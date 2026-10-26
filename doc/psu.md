# Power supply
## Model
* Mean Well XLG-100-H-AB

## Dimming interface
* `DIM+` and `DIM-` accept additive 0 to 10 V, 10 V PWM, or resistance dimming.
* Do not connect `DIM-` to LED output `V-`.
* The dimming source current is typically 100 µA.
* 0 V, 0% duty 10 V PWM, or 0 Ω requests 0% output.
* The minimum dimming level is about 8% for other dimming inputs.

## Integration constraints
* An ESP32-C3 GPIO provides a 3.3 V high level, while the driver specifies a 10 V PWM dimming signal.
* Any selected interface must accept 3.3 V PWM directly or include a specified level conversion stage.
* Do not connect the control ground to XLG LED output `V-`.
