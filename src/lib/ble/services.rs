//! BLE GATT services

/// Battery service
#[nrf_softdevice::gatt_service(uuid = "180f")]
pub struct BatteryService {
    #[characteristic(uuid = "2a19", read, notify)]
    pub battery_level: u8,
}
/*
/// Thermometer service
#[nrf_softdevice::gatt_service(uuid = "1809")]
pub struct Thermometer {
    #[characteristic(uuid = "2a6e", read, notify)]
    pub temperature: u32,
}
*/
