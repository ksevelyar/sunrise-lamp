# Light
## Requirements
* Lamp boots with light off.
* Dimming is flicker-free.

## Dimming options
### Optocoupler-switched PWM
An optocoupler transistor connects between DIM+ and DIM-. An ESP32 GPIO drives the optocoupler LED through a current-limiting resistor.

* A closed optocoupler requests minimum output or off.
* An open optocoupler requests full output.
* Firmware PWM duty is inverted because longer DIM+ to DIM- shorts mean less light.
* This can introduce brightness modulation.

### Isolated PWM-to-0-10 V converter
An isolated converter accepts ESP32 PWM and produces a steady 0 to 10 V control signal. Connect the converter output positive terminal to DIM+ and negative terminal to DIM-.

* The driver receives an analog dimming voltage rather than direct PWM switching.
* This is the preferred option for smooth low-brightness transitions.
* Use a converter with a 3.3 V-compatible input and isolated output.

The dimming interface is undecided.

## Gamma conversion
Schedule brightness represents perceived brightness. PWM duty cycle controls the electrical dimming voltage, so direct linear duty-cycle changes do not appear visually uniform.

Use a lookup table to convert requested brightness from 0% to 100% into PWM duty cycle. The initial table uses gamma 2.2 and can later be replaced with measured lamp values.

* Linear conversion maps 50% requested brightness to 50% PWM duty cycle. It is electrically predictable but visually uneven.
* Gamma 2.2 maps 50% requested brightness to about 22% PWM duty cycle. It is a conventional visual-brightness curve.
* A measured lookup table can compensate for the PWM converter, LED driver, LED panel, and low-brightness threshold.

The conversion sets 0% requested brightness to 0% PWM duty cycle so the lamp stays off.
