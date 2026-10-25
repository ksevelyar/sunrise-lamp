#![no_std]
#![no_main]

use esp_alloc as _;
use esp_hal::{clock::CpuClock, rtc_cntl::Rtc, timer::timg::TimerGroup};
use panic_rtt_target as _;
use sunrise_lamp::{time, wifi};

use defmt::info;
use embassy_executor::Spawner;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Timer};

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    rtt_target::rtt_init_defmt!();

    let configuration = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(configuration);
    let rtc = Rtc::new(peripherals.LPWR);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 66320);

    let timer_group = TimerGroup::new(peripherals.TIMG0);
    let software_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timer_group.timer0, software_interrupt.software_interrupt0);
    info!("sunrise-lamp: embassy initialized");

    let (controller, stack, runner) = wifi::init(peripherals.WIFI).await;
    spawner.spawn(wifi::connection(controller).unwrap());
    spawner.spawn(wifi::net_task(runner).unwrap());

    let rtc = time::RTC.init(Mutex::new(rtc));
    spawner.spawn(time::ntp_task(stack, rtc).unwrap());

    loop {
        Timer::after(Duration::from_secs(3600)).await;
    }
}
