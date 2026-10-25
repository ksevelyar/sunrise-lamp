# Sunrise Lamp

## Configuration
Set these build-time environment variables:

* `SSID`: Wi-Fi network name
* `PASS`: Wi-Fi password
* `TIMEZONE`: IANA timezone, for example `Europe/Moscow`
* `SUNRISE`: sunrise time in 24-hour `HH:MM` format, for example `08:00`
* `SUNSET`: sunset time in 24-hour `HH:MM` format, for example `23:00`
* `WIFI_TRANSMIT_POWER`: Wi-Fi transmit power from `2` to `20` dBm

`SUNRISE` must be earlier than `SUNSET`.

## Build
Enter the Nix development shell and build the firmware:

```sh
nix develop
cargo build --release
```

## Doc
* [Hardware](doc/hardware.md)
* [Light dimming options](doc/light.md)
* [Wiring](doc/wiring.md)
