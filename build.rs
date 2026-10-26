fn main() {
    let timezone = std::env::var("TIMEZONE").expect("TIMEZONE must be set");
    let schedule_points = parse_schedule();
    let update_interval_seconds = parse_update_interval_seconds();
    validate_wifi_transmit_power();

    let output_directory = std::env::var("OUT_DIR").expect("OUT_DIR must be set");
    let configuration_path = std::path::Path::new(&output_directory).join("configuration.rs");
    let schedule_point_constants = schedule_points
        .iter()
        .map(|(minutes, brightness_percent)| format!("({minutes}, {brightness_percent})"))
        .collect::<Vec<_>>()
        .join(", ");
    let configuration = format!(
        "pub const TIMEZONE: ::jiff::tz::TimeZone = ::jiff::tz::get!(\"{timezone}\");\n\
         pub const SCHEDULE_POINTS: &[(u16, u8)] = &[{schedule_point_constants}];\n\
         pub const UPDATE_INTERVAL_SECONDS: u64 = {update_interval_seconds};\n"
    );
    std::fs::write(configuration_path, configuration).expect("failed to write configuration");

    for variable_name in [
        "TIMEZONE",
        "SCHEDULE",
        "UPDATE_INTERVAL",
        "WIFI_TRANSMIT_POWER",
    ] {
        println!("cargo:rerun-if-env-changed={variable_name}");
    }

    linker_be_nice();
    println!("cargo:rustc-link-arg-tests=-Tembedded-test.x");
    println!("cargo:rustc-link-arg=-Tdefmt.x");
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}

fn parse_schedule() -> Vec<(u16, u8)> {
    let schedule = std::env::var("SCHEDULE").expect("SCHEDULE must be set");
    let schedule_points = schedule
        .split(',')
        .map(parse_schedule_point)
        .collect::<Vec<_>>();

    assert!(
        schedule_points.len() >= 2,
        "SCHEDULE must contain at least two points"
    );
    assert!(schedule_points[0].0 == 0, "SCHEDULE must start with 00:00");

    for schedule_point_pair in schedule_points.windows(2) {
        let previous_minutes = schedule_point_pair[0].0;
        let current_minutes = schedule_point_pair[1].0;
        assert!(
            previous_minutes < current_minutes,
            "SCHEDULE times must be strictly increasing"
        );
    }

    schedule_points
}

fn parse_schedule_point(schedule_point: &str) -> (u16, u8) {
    let Some((clock_time, brightness_percent)) = schedule_point.split_once('=') else {
        panic!("SCHEDULE points must use HH:MM=brightness format");
    };

    assert!(
        !brightness_percent.contains('='),
        "SCHEDULE points must use HH:MM=brightness format"
    );

    let minutes = parse_clock_minutes(clock_time, "SCHEDULE times");
    let brightness_percent = brightness_percent
        .parse::<u8>()
        .expect("SCHEDULE brightness must be an integer");

    assert!(
        brightness_percent <= 100,
        "SCHEDULE brightness must be between 0 and 100"
    );

    (minutes, brightness_percent)
}

fn parse_update_interval_seconds() -> u64 {
    let update_interval_seconds = std::env::var("UPDATE_INTERVAL")
        .expect("UPDATE_INTERVAL must be set")
        .parse::<u64>()
        .expect("UPDATE_INTERVAL must be a whole number of seconds");

    assert!(
        (1..=60).contains(&update_interval_seconds),
        "UPDATE_INTERVAL must be between 1 and 60 seconds"
    );

    update_interval_seconds
}

fn parse_clock_minutes(value: &str, value_description: &str) -> u16 {
    let Some((hour, minute)) = value.split_once(':') else {
        panic!("{value_description} must use HH:MM format");
    };

    assert!(
        !minute.contains(':') && hour.len() == 2 && minute.len() == 2 && value.len() == 5,
        "{value_description} must use HH:MM format"
    );

    let hour = hour
        .parse::<u16>()
        .unwrap_or_else(|_| panic!("{value_description} must use HH:MM format"));
    let minute = minute
        .parse::<u16>()
        .unwrap_or_else(|_| panic!("{value_description} must use HH:MM format"));

    assert!(
        hour < 24 && minute < 60,
        "{value_description} must be a valid time"
    );
    hour * 60 + minute
}

fn validate_wifi_transmit_power() {
    let transmit_power = std::env::var("WIFI_TRANSMIT_POWER")
        .expect("WIFI_TRANSMIT_POWER must be set")
        .parse::<i8>()
        .expect("WIFI_TRANSMIT_POWER must be a valid integer");

    assert!(
        (2..=20).contains(&transmit_power),
        "WIFI_TRANSMIT_POWER must be between 2 and 20 dBm"
    );
}

fn linker_be_nice() {
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() > 1 {
        let error_kind = &arguments[1];
        let error_value = &arguments[2];

        match error_kind.as_str() {
            "undefined-symbol" => match error_value.as_str() {
                value if value.starts_with("_defmt_") => {
                    eprintln!();
                    eprintln!("💡 defmt not found: add defmt.x as a linker script");
                    eprintln!();
                }
                "_stack_start" => {
                    eprintln!();
                    eprintln!("💡 linkall.x is missing");
                    eprintln!();
                }
                value if value.starts_with("esp_rtos_") => {
                    eprintln!();
                    eprintln!("💡 esp-radio requires esp-rtos initialization");
                    eprintln!();
                }
                "embedded_test_linker_file_not_added_to_rustflags" => {
                    eprintln!();
                    eprintln!("💡 embedded-test requires embedded-test.x");
                    eprintln!();
                }
                "free"
                | "malloc"
                | "calloc"
                | "get_free_internal_heap_size"
                | "malloc_internal"
                | "realloc_internal"
                | "calloc_internal"
                | "free_internal" => {
                    eprintln!();
                    eprintln!("💡 esp-alloc is missing");
                    eprintln!();
                }
                _ => (),
            },
            _ => std::process::exit(1),
        }

        std::process::exit(0);
    }

    println!(
        "cargo:rustc-link-arg=--error-handling-script={}",
        std::env::current_exe()
            .expect("failed to get build script path")
            .display()
    );
}
