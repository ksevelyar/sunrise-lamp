# Wiring
## Mains
* One mains cable enters a fused and switched inlet.
* Split live and neutral after the inlet to power the XLG-100-H-AB LED driver and the isolated 12 V AC/DC supply in parallel.
* Connect protective earth to the XLG protective-earth terminal and to a conductive enclosure.
* Enclose mains terminals, provide cable strain relief, and keep mains wiring separate from low-voltage wiring.

## LED panel
* Connect XLG `V+` to LED panel positive.
* Connect XLG `V-` to LED panel negative.
* Do not connect XLG `V-` to any controller or dimming connection.

## 12 V supply
* Connect 12 V positive to PW2VB01 `Vin` and to the 12 V input of the 5 V buck converter.
* Connect 12 V negative to PW2VB01 input and PWM `GND` and to the buck converter input negative.

## ESP32-C3 SuperMini
* Connect the buck converter 5 V output to the SuperMini `5V` pin.
* Connect the buck converter output negative to a SuperMini `GND` pin.
* Connect SuperMini `GPIO1` to PW2VB01 `PWM`.
* Configure `GPIO1` as a 1 kHz PWM output.
* Do not use boot-strapping pins `GPIO2`, `GPIO8`, or `GPIO9` for dimming.

## Dimming control
* Configure the PW2VB01 for 0 to 10 V output.
* Connect PW2VB01 `Vo` to XLG `DIM+`.
* Connect PW2VB01 output `GND` to XLG `DIM-`.
* Keep the PW2VB01 output connection separate from XLG LED `V+` and `V-`.
