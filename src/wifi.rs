use defmt::{error, info, warn};
use embassy_net::{Runner, Stack, StackResources};
use embassy_time::{Duration, Timer};
use esp_hal::rng::Rng;
pub use esp_radio::wifi::Interface;
use static_cell::ConstStaticCell;

use esp_radio::wifi::{Config, ControllerConfig, WifiController, sta::StationConfig};

extern crate alloc;

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASS");
const WIFI_TRANSMIT_POWER: &str = env!("WIFI_TRANSMIT_POWER");

pub fn init(
    wifi: esp_hal::peripherals::WIFI<'static>,
) -> (
    WifiController<'static>,
    Stack<'static>,
    Runner<'static, Interface<'static>>,
) {
    static NETWORK_RESOURCES: ConstStaticCell<StackResources<6>> =
        ConstStaticCell::new(StackResources::new());

    let (mut controller, interfaces) = initialize_controller(wifi);
    configure_transmit_power(&mut controller);

    let config = embassy_net::Config::dhcpv4(embassy_net::DhcpConfig::default());
    let seed = u64::from(Rng::new().random()) << 32 | u64::from(Rng::new().random());

    let (stack, runner) =
        embassy_net::new(interfaces.station, config, NETWORK_RESOURCES.take(), seed);

    (controller, stack, runner)
}

fn initialize_controller(
    wifi: esp_hal::peripherals::WIFI<'static>,
) -> (
    WifiController<'static>,
    esp_radio::wifi::Interfaces<'static>,
) {
    let station_config = Config::Station(
        StationConfig::default()
            .with_ssid(SSID)
            .with_password(PASSWORD.into()),
    );

    esp_radio::wifi::new(
        wifi,
        ControllerConfig::default().with_initial_config(station_config),
    )
    .expect("Failed to initialize Wi-Fi controller")
}

fn configure_transmit_power(controller: &mut WifiController<'static>) {
    let max_tx_power_dbm: i8 = WIFI_TRANSMIT_POWER
        .parse()
        .expect("WIFI_TRANSMIT_POWER must be a valid integer");
    controller
        .set_max_tx_power(max_tx_power_dbm * 4)
        .expect("Failed to set max TX power");
}

#[embassy_executor::task]
pub async fn connection(mut controller: WifiController<'static>) {
    let mut failed_attempts: u32 = 0;

    loop {
        info!("wifi: connecting to \"{}\"", env!("SSID"));

        match controller.connect_async().await {
            Ok(info) => {
                failed_attempts = 0;
                info!("wifi: connected to \"{}\"", info.ssid.as_str());

                wait_for_disconnect(&mut controller).await;
            }
            Err(err) => {
                failed_attempts += 1;
                warn!("wifi: connection attempt {} failed", failed_attempts);
                match err {
                    esp_radio::wifi::WifiError::Disconnected(info) => {
                        error!(
                            "wifi: disconnected: SSID: \"{}\", reason: {:?}, RSSI: {}",
                            info.ssid.as_str(),
                            info.reason,
                            info.rssi,
                        );
                    }
                    _ => {
                        error!("wifi: connection failed: {:?}", err);
                    }
                }
            }
        }

        Timer::after(Duration::from_millis(5000)).await;
    }
}

async fn wait_for_disconnect(controller: &mut WifiController<'static>) {
    match controller.wait_for_disconnect_async().await {
        Ok(info) => {
            info!(
                "wifi: disconnected from \"{}\" (reason: {:?}, rssi: {})",
                info.ssid.as_str(),
                info.reason,
                info.rssi,
            );
        }
        Err(error) => warn!("wifi: disconnect wait failed: {:?}", error),
    }
}

#[embassy_executor::task]
pub async fn net_task(mut runner: Runner<'static, Interface<'static>>) {
    runner.run().await;
}
