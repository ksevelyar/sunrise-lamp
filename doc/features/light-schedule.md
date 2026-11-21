# Light Schedule

## Requirements
* The lamp starts with light output disabled before Wi-Fi or time synchronization starts.
* The lamp stays off until a valid time is available.
* The daily schedule controls perceived brightness in the configured timezone.
* Brightness changes linearly between schedule points and updates every second.
* The final schedule point transitions to the first point at midnight.
* Gamma 2.2 converts perceived brightness to PWM duty cycle. Zero brightness requests zero duty cycle.
* Dimming is flicker-free.

## User scenarios

### 1. Lamp stays off while starting without valid time
1. The lamp starts without a valid RTC time.
2. Before Wi-Fi and time synchronization start, the PWM duty cycle is 0%.
3. Time synchronization has not succeeded.
4. The light remains off.

### 2. Lamp stays off after a nighttime restart
1. The local schedule is `00:00=0,08:00=0,09:00=100,17:00=100,19:00=40,23:00=40`.
2. The lamp restarts with a valid local RTC time of 04:00.
3. The PWM duty cycle starts at 0%.
4. The schedule keeps the light off.

### 3. Lamp brightens at sunrise
1. The local schedule is `00:00=0,08:00=0,09:00=100,17:00=100,19:00=40,23:00=40`.
2. At 08:00, perceived brightness is 0%.
3. At 1, 2 and 3 seconds after 08:00, perceived brightness follows the linear transition to 100% at 09:00.
4. At 08:30, perceived brightness is 50%.
5. At 09:00, perceived brightness is 100%.

### 4. Lamp holds daytime brightness
1. The local schedule is `00:00=0,08:00=0,09:00=100,17:00=100,19:00=40,23:00=40`.
2. At 09:00, perceived brightness is 100%.
3. At 12:00 and 17:00, perceived brightness is 100%.

### 5. Lamp dims in the evening
1. The local schedule is `00:00=0,08:00=0,09:00=100,17:00=100,19:00=40,23:00=40`.
2. At 17:00, perceived brightness is 100%.
3. At 1, 2 and 3 seconds after 17:00, perceived brightness follows the linear transition to 40% at 19:00.
4. At 18:00, perceived brightness is 70%.
5. At 19:00, perceived brightness is 40%.

### 6. Lamp holds evening brightness
1. The local schedule is `00:00=0,08:00=0,09:00=100,17:00=100,19:00=40,23:00=40`.
2. At 19:00, perceived brightness is 40%.
3. At 21:00 and 23:00, perceived brightness is 40%.

### 7. Lamp dims to off at midnight
1. The local schedule is `00:00=0,08:00=0,09:00=100,17:00=100,19:00=40,23:00=40`.
2. At 23:00, perceived brightness is 40%.
3. At 1, 2 and 3 seconds after 23:00, perceived brightness follows the linear transition to 0% at midnight.
4. At 23:30, perceived brightness is 20%.
5. At 00:00 and 04:00, the light is off.

### 8. Lamp converts perceived brightness to PWM duty
1. The local schedule is `00:00=0,08:00=0,09:00=100,17:00=100,19:00=40,23:00=40`.
2. At 08:00, perceived brightness is 0% and the PWM duty cycle is 0%.
3. At 08:30, perceived brightness is 50% and the PWM duty cycle is approximately 22%.
4. At 09:00, perceived brightness is 100% and the PWM duty cycle is 100%.

## Implementation
* ESP32-C3 `GPIO1` supplies PWM to the PW2VB01 converter configured for 0 to 10 V output.
* The converter supplies a steady analog dimming signal to the LED driver.
* A gamma 2.2 lookup table converts perceived brightness to PWM duty cycle. Measured lamp values can replace the initial table.
* [Wiring](../hardware/wiring.md) and [power supply](../hardware/psu.md) describe the dimming connections.
