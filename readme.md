# Sunrise Lamp

## Requirements
* The lamp initializes with light output disabled. A nighttime restart must not wake user.
* One mains power cable powers the complete lamp.
* The daily schedule controls perceived brightness.
* Brightness transitions use gamma 2.2 so that fades appear visually uniform.
* The LED panel is at least 1 m above the plants.
* Side shields prevent direct viewing of the LED panel from normal room positions.

## User scenarios
* At startup, the PWM output is 0% before Wi-Fi and time synchronization start.
* From 08:00 to 09:00, brightness rises from 0% to 100%.
* From 09:00 to 17:00, brightness remains at 100%.
* From 17:00 to 19:00, brightness falls from 100% to 40%.
* From 19:00 to 23:00, brightness remains at 40%.
* From 23:00 to 00:00, brightness falls from 40% to 0%.
* Verify every transition at 1 s, 2 s, and 3 s after it begins.
* Verify that a nighttime restart leaves the light off.

## Implementation
* A fused and switched mains inlet powers the Mean Well XLG-100-H-AB LED driver and an isolated 12 V AC/DC power supply in parallel.
* The XLG-100-H-AB powers the LED panel.
* The 12 V power supply powers the PW2VB01 PWM-to-voltage converter.
* A 12 V to 5 V DC/DC buck converter powers the ESP32-C3 SuperMini through its `5V` input.
* ESP32-C3 `GPIO1` supplies 3.3 V PWM to the PW2VB01.
* A gamma 2.2 lookup table converts scheduled perceived brightness into PWM duty cycle.
* The PW2VB01 is configured for 0 to 10 V output. Its output connects to XLG `DIM+` and `DIM-`.
* ESP32, 12 V, and PW2VB01 ground must not connect to XLG LED output `V-`.
* The LED panel is supported by eight 40 mm wooden battens and secured by eight cable ties. Kraft-paper side shields reduce glare.

## Configuration
Set these build-time environment variables:

* `SSID`: Wi-Fi network name
* `PASS`: Wi-Fi password
* `TIMEZONE`: IANA timezone, for example `Europe/Moscow`
* `SCHEDULE`: comma-separated `HH:MM=brightness` points
* `UPDATE_INTERVAL`: whole seconds between brightness updates
* `WIFI_TRANSMIT_POWER`: Wi-Fi transmit power from `2` to `20` dBm

`SCHEDULE` starts at `00:00`. Times are strictly increasing, and brightness is an integer percentage from `0` to `100`. Brightness changes linearly between adjacent points. Repeating a brightness value creates a hold interval.

```sh
SCHEDULE="00:00=0,08:00=0,09:00=100,17:00=100,19:00=40,23:00=40"
UPDATE_INTERVAL=1
```

This schedule keeps the lamp off from 00:00 to 08:00, fades it to 100% by 09:00, holds it until 17:00, fades it to 40% by 19:00, holds it until 23:00, and fades it off by midnight.

## Build
Enter the Nix development shell and build the firmware:

```sh
nix develop
cargo build --release
```

## Documentation
* [Hardware](doc/hardware.md)
* [Light dimming](doc/light.md)
* [Power supply](doc/psu.md)
* [Wiring](doc/wiring.md)
