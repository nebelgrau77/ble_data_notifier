# ENS160 idea

- needs shared bus
- can work with BLESense at first, using the HTS221 temperature and humidity to calibrate the other sensor
- maybe to avoid some timing issues the idea would be to:
  1. read temperature and humidity
  2. read the ENS air quality values
  3. calibrate ENS using values from point i.
  4. advertise

this may be necessary (dependencies updates): https://github.com/embassy-rs/nrf-softdevice/blob/master/examples/Cargo.toml
