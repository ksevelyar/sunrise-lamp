# Sunrise Lamp
A sunrise lamp to fight SAD and grow plants.

## Documentation
* [Boundaries](doc/boundaries.md)
* [Light schedule](doc/features/light-schedule.md)
* [Time sync](doc/features/time-sync.md)
* [Hardware](doc/hardware/hardware.md)
* [Power supply](doc/hardware/psu.md)
* [Wiring](doc/hardware/wiring.md)
* [udev setup](doc/hardware/udev.md)

## Configuration
Set these build-time environment variables:

* `SSID`: Wi-Fi network name
* `PASS`: Wi-Fi password
* `TIMEZONE`: IANA timezone, for example `Europe/Moscow`
* `SCHEDULE`: comma-separated `HH:MM=brightness` points
* `UPDATE_INTERVAL`: whole seconds between brightness updates, set to `1` for the required schedule behavior
* `WIFI_TRANSMIT_POWER`: Wi-Fi transmit power from `2` to `20` dBm

`SCHEDULE` starts at `00:00`. Times are strictly increasing, and brightness is an integer percentage from `0` to `100`. Brightness changes linearly between adjacent points. Repeating a brightness value creates a hold interval.

```
SCHEDULE="00:00=0,08:00=0,09:00=100,17:00=100,19:00=40,23:00=40"
UPDATE_INTERVAL=1
```

This schedule keeps the lamp off from 00:00 to 08:00, fades it to 100% by 09:00, holds it until 17:00, fades it to 40% by 19:00, holds it until 23:00, and fades it off by midnight.

## Build and flash

```
nix develop
cargo build --release
```

## Test
Tests require a connected MCU.

```
nix develop
cargo test
```

