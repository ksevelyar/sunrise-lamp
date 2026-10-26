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
