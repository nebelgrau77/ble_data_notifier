//! BLE GATT services

/// Battery service
#[nrf_softdevice::gatt_service(uuid = "180f")]
pub struct BatteryService {
    #[characteristic(uuid = "2a19", read, notify)]
    pub battery_level: u8,
}

/// Raw ADC reading
#[nrf_softdevice::gatt_service(uuid = "999e06eb-bc32-401d-abd9-d88781031d04")]
pub struct RawAdcService {
    #[characteristic(uuid = "999e06eb-bc32-401d-abd9-d88781041d04", read, notify)]
    pub adc_value: i16,
}

/// Environmental sensing service - pressure, humidity, temperature, irradiance
#[nrf_softdevice::gatt_service(uuid = "181a")]
pub struct EnviroSensingService {    
    #[characteristic(uuid = "2a6e", read, notify)]
    pub temperature: i16,
    #[characteristic(uuid = "2a6f", read, notify)]
    pub humidity: u16,
    #[characteristic(uuid = "2a6d", read, notify)]
    pub pressure: u32,
    #[characteristic(uuid = "2a77", read, notify)]
    pub irradiance: u16,
}