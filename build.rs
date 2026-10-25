fn main() {
    let timezone = std::env::var("TIMEZONE").expect("TIMEZONE must be set");
    let sunrise_minutes = parse_clock_minutes("SUNRISE");
    let sunset_minutes = parse_clock_minutes("SUNSET");
    validate_wifi_transmit_power();

    assert!(
        sunrise_minutes < sunset_minutes,
        "SUNRISE must be earlier than SUNSET"
    );

    let output_directory = std::env::var("OUT_DIR").expect("OUT_DIR must be set");
    let configuration_path = std::path::Path::new(&output_directory).join("configuration.rs");
    let configuration = format!(
        "pub const TIMEZONE: ::jiff::tz::TimeZone = ::jiff::tz::get!(\"{timezone}\");\n\
         pub const SUNRISE_MINUTES: u16 = {sunrise_minutes};\n\
         pub const SUNSET_MINUTES: u16 = {sunset_minutes};\n"
    );
    std::fs::write(configuration_path, configuration).expect("failed to write configuration");

    for variable_name in ["TIMEZONE", "SUNRISE", "SUNSET", "WIFI_TRANSMIT_POWER"] {
        println!("cargo:rerun-if-env-changed={variable_name}");
    }

    linker_be_nice();
    println!("cargo:rustc-link-arg-tests=-Tembedded-test.x");
    println!("cargo:rustc-link-arg=-Tdefmt.x");
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}

fn parse_clock_minutes(variable_name: &str) -> u16 {
    let value =
        std::env::var(variable_name).unwrap_or_else(|_| panic!("{variable_name} must be set"));
    let Some((hour, minute)) = value.split_once(':') else {
        panic!("{variable_name} must use HH:MM format");
    };

    assert!(
        hour.len() == 2 && minute.len() == 2 && value.len() == 5,
        "{variable_name} must use HH:MM format"
    );

    let hour: u16 = hour
        .parse()
        .unwrap_or_else(|_| panic!("{variable_name} must use HH:MM format"));
    let minute: u16 = minute
        .parse()
        .unwrap_or_else(|_| panic!("{variable_name} must use HH:MM format"));

    assert!(
        hour < 24 && minute < 60,
        "{variable_name} must be a valid time"
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
