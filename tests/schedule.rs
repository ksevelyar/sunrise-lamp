#![no_std]
#![no_main]

esp_bootloader_esp_idf::esp_app_desc!();

#[cfg(test)]
#[embedded_test::tests(executor = esp_rtos::embassy::Executor::new())]
mod tests {
    use defmt::assert_eq;
    use sunrise_lamp::schedule::{
        LampState, SUNRISE_MINUTES, SUNSET_MINUTES, determine_lamp_state,
    };

    #[init]
    fn initialize_test_hardware() {
        let peripherals = esp_hal::init(esp_hal::Config::default());
        let timer_group = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);
        let software_interrupt =
            esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
        esp_rtos::start(timer_group.timer0, software_interrupt.software_interrupt0);
        rtt_target::rtt_init_defmt!();
    }

    #[test]
    async fn sunrise_starts_daylight() {
        let sunrise_hour = (SUNRISE_MINUTES / 60) as u8;
        let sunrise_minute = (SUNRISE_MINUTES % 60) as u8;

        assert_eq!(
            determine_lamp_state(sunrise_hour, sunrise_minute),
            LampState::Day
        );
    }

    #[test]
    async fn sunset_starts_night() {
        let sunset_hour = (SUNSET_MINUTES / 60) as u8;
        let sunset_minute = (SUNSET_MINUTES % 60) as u8;

        assert_eq!(
            determine_lamp_state(sunset_hour, sunset_minute),
            LampState::Night
        );
    }
}
