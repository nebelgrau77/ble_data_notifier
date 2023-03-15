//! BLE GATT services

/// Battery service
#[nrf_softdevice::gatt_service(uuid = "180f")]
pub struct BatteryService {
    #[characteristic(uuid = "2a19", read, notify)]
    pub battery_level: u8,
}

/// Environmental sensing service - pressure, humidity, temperature
#[nrf_softdevice::gatt_service(uuid = "181a")]
pub struct EnviroSensingService {    
    #[characteristic(uuid = "2a6e", read, notify)]
    pub temperature: u32,
    #[characteristic(uuid = "2a6f", read, notify)]
    pub humidity: u32,
    #[characteristic(uuid = "2a6d", read, notify)]
    pub pressure: u32,
}