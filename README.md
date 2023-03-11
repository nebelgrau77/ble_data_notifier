### HOW TO BUILD AND FLASH

#### On MicroMod with JLink:

* run the code: ```cargo run --features "ble-gatt-server"```

#### On ItsyBitsy/BLESense/Micromod with Adafruit bootloader

* build the code: ```cargo build --release --features "ble-gatt-server"```
* convert to .hex file: ```arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabihf/release/blinky blinky.hex```
* create a dfu package: ```adafruit-nrfutil dfu genpkg --dev-type 0x0052 --application blinky.hex blinky.zip``` or `nrf_package blinky.hex blinky.zip` (aliased)
  
* put the board into bootloader mode (hold the reset button, the red LED will fade in and out, will show as _BLESENSE730_ as it is the new bootloader with SoftDevice S140 v.7.3.0)
* flash the firmware: ```adafruit-nrfutil dfu serial --package blinky.zip -p /dev/ttyACM0 -b 115200```

