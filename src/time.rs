use core::net::{IpAddr, SocketAddr};
use core::sync::atomic::{AtomicU32, Ordering};

use defmt::{error, info, warn};
use embassy_net::{
    Stack,
    dns::DnsQueryType,
    udp::{PacketMetadata, UdpSocket},
};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Instant, Timer, with_timeout};
use esp_hal::rtc_cntl::Rtc;
use jiff::Timestamp;
use sntpc::{NtpContext, NtpTimestampGenerator, get_time};
use sntpc_net_embassy::UdpSocketWrapper;
use static_cell::StaticCell;

use crate::schedule::{TIMEZONE, determine_brightness_percentage};

const NTP_SERVER: &str = "pool.ntp.org";
const MINIMUM_VALID_EPOCH: u32 = 1_700_000_000;
const MICROSECONDS_PER_SECOND: u64 = 1_000_000;

static EPOCH_BASE: AtomicU32 = AtomicU32::new(0);
static INSTANT_BASE: AtomicU32 = AtomicU32::new(0);

pub static RTC: StaticCell<Mutex<CriticalSectionRawMutex, Rtc<'static>>> = StaticCell::new();

pub fn epoch_seconds() -> Option<u32> {
    let epoch_base = EPOCH_BASE.load(Ordering::Acquire);
    if epoch_base == 0 {
        return None;
    }

    let current_instant = Instant::now().as_secs();
    let instant_base = INSTANT_BASE.load(Ordering::Relaxed) as u64;
    let elapsed_seconds = current_instant.saturating_sub(instant_base) as u32;
    Some(epoch_base + elapsed_seconds)
}

pub fn local_clock(epoch_seconds: u32) -> (u8, u8) {
    let timestamp = Timestamp::new(i64::from(epoch_seconds), 0).unwrap();
    let local_time = timestamp.to_zoned(TIMEZONE);
    (local_time.hour() as u8, local_time.minute() as u8)
}

fn set_epoch_seconds(epoch_seconds: u32) {
    INSTANT_BASE.store(Instant::now().as_secs() as u32, Ordering::Relaxed);
    EPOCH_BASE.store(epoch_seconds, Ordering::Release);
}

fn seed_from_rtc(rtc: &Rtc) {
    let epoch_seconds = (rtc.current_time_us() / MICROSECONDS_PER_SECOND) as u32;
    if epoch_seconds > MINIMUM_VALID_EPOCH {
        set_epoch_seconds(epoch_seconds);
        log_local_time("rtc seeded");
    }
}

fn log_local_time(event: &str) {
    let Some(epoch_seconds) = epoch_seconds() else {
        return;
    };

    let timestamp = Timestamp::new(i64::from(epoch_seconds), 0).unwrap();
    let local_time = timestamp.to_zoned(TIMEZONE);
    let brightness_percent =
        determine_brightness_percentage(local_time.hour() as u8, local_time.minute() as u8);

    info!(
        "time: {} {} {=i16}-{=i8:02}-{=i8:02} {=i8:02}:{=i8:02} ({}%)",
        event,
        TIMEZONE.iana_name().unwrap(),
        local_time.year(),
        local_time.month(),
        local_time.day(),
        local_time.hour(),
        local_time.minute(),
        brightness_percent,
    );
}

#[derive(Clone, Copy)]
struct NtpTimestamp {
    current_time_microseconds: u64,
}

impl NtpTimestampGenerator for NtpTimestamp {
    fn init(&mut self) {}

    fn timestamp_sec(&self) -> u64 {
        self.current_time_microseconds / MICROSECONDS_PER_SECOND
    }

    fn timestamp_subsec_micros(&self) -> u32 {
        (self.current_time_microseconds % MICROSECONDS_PER_SECOND) as u32
    }
}

async fn sync_with_ntp(
    current_time_microseconds: u64,
    stack: &Stack<'static>,
    receive_metadata: &mut [PacketMetadata; 16],
    receive_buffer: &mut [u8; 4096],
    transmit_metadata: &mut [PacketMetadata; 16],
    transmit_buffer: &mut [u8; 4096],
) -> Option<u64> {
    let ntp_addresses = match stack.dns_query(NTP_SERVER, DnsQueryType::A).await {
        Ok(addresses) if !addresses.is_empty() => addresses,
        _ => {
            error!("time: DNS failed");
            return None;
        }
    };

    let socket = {
        let mut socket = UdpSocket::new(
            *stack,
            receive_metadata,
            receive_buffer,
            transmit_metadata,
            transmit_buffer,
        );
        socket.bind(0).unwrap();
        UdpSocketWrapper::new(socket)
    };

    let ntp_address: IpAddr = ntp_addresses[0].into();
    let result = get_time(
        SocketAddr::from((ntp_address, 123)),
        &socket,
        NtpContext::new(NtpTimestamp {
            current_time_microseconds,
        }),
    )
    .await;

    match result {
        Ok(time) => Some(
            time.sec() as u64 * MICROSECONDS_PER_SECOND
                + ((time.sec_fraction() as u64 * MICROSECONDS_PER_SECOND) >> 32),
        ),
        Err(_) => {
            error!("time: NTP failed");
            None
        }
    }
}

#[embassy_executor::task]
pub async fn ntp_task(
    stack: Stack<'static>,
    rtc: &'static Mutex<CriticalSectionRawMutex, Rtc<'static>>,
) {
    seed_from_rtc(&*rtc.lock().await);
    stack.wait_config_up().await;

    let mut receive_metadata = [PacketMetadata::EMPTY; 16];
    let mut receive_buffer = [0; 4096];
    let mut transmit_metadata = [PacketMetadata::EMPTY; 16];
    let mut transmit_buffer = [0; 4096];
    let mut failed_attempts: u32 = 0;

    loop {
        let current_time_microseconds = rtc.lock().await.current_time_us();
        let result = with_timeout(
            Duration::from_secs(30),
            sync_with_ntp(
                current_time_microseconds,
                &stack,
                &mut receive_metadata,
                &mut receive_buffer,
                &mut transmit_metadata,
                &mut transmit_buffer,
            ),
        )
        .await;

        match result {
            Ok(Some(epoch_microseconds)) => {
                let epoch_seconds = (epoch_microseconds / MICROSECONDS_PER_SECOND) as u32;
                rtc.lock().await.set_current_time_us(epoch_microseconds);
                set_epoch_seconds(epoch_seconds);
                log_local_time("synced");
                break;
            }
            Ok(None) | Err(_) => {
                failed_attempts += 1;
                warn!("time: NTP sync attempt {} failed", failed_attempts);
                Timer::after(Duration::from_secs(10)).await;
            }
        }
    }
}
