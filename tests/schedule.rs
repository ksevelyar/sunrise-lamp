#![no_std]
#![no_main]

esp_bootloader_esp_idf::esp_app_desc!();

#[cfg(test)]
#[embedded_test::tests(executor = esp_rtos::embassy::Executor::new())]
mod tests {
    use defmt::assert_eq;
    use sunrise_lamp::schedule::{
        SCHEDULE_POINTS, UPDATE_INTERVAL_SECONDS, determine_brightness_percentage,
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
    async fn returns_the_configured_brightness_at_each_schedule_point() {
        for (minutes, expected_brightness_percent) in SCHEDULE_POINTS {
            let hour = (minutes / 60) as u8;
            let minute = (minutes % 60) as u8;

            assert_eq!(
                determine_brightness_percentage(hour, minute),
                *expected_brightness_percent
            );
        }
    }

    #[test]
    async fn holds_the_nighttime_brightness() {
        assert_eq!(determine_brightness_percentage(4, 0), 0);
    }

    #[test]
    async fn interpolates_the_sunrise_brightness() {
        assert_eq!(determine_brightness_percentage(8, 30), 50);
    }

    #[test]
    async fn interpolates_the_evening_fade_brightness() {
        assert_eq!(determine_brightness_percentage(18, 0), 70);
    }

    #[test]
    async fn interpolates_the_sunset_brightness() {
        assert_eq!(determine_brightness_percentage(23, 30), 20);
    }

    #[test]
    async fn uses_the_configured_update_interval() {
        assert_eq!(UPDATE_INTERVAL_SECONDS, 1);
    }
}
