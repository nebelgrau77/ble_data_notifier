//! BLE Server config and tasks
use super::services::*;
use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::block_on;
use embassy_nrf::saadc::Saadc;
use embassy_sync::pubsub::publisher;
use embassy_time::{Duration, Timer};
use futures::{
    future::{select, Either},
    pin_mut,
};
use nrf_softdevice::{
    ble::{gatt_server, peripheral, Connection},
    Softdevice,
};
use static_cell::StaticCell;

/// BLE advertising data

#[rustfmt::skip]
const ADV_DATA: &[u8] = 
    &[
        0x02, 0x01, nrf_softdevice::raw::BLE_GAP_ADV_FLAGS_LE_ONLY_GENERAL_DISC_MODE as u8,
        0x05, 0x03, 0x0F, 0x18, 0x1a, 0x18, 
        0x09, 0x09, b'S', b'i', b'k', b'o', b'r', b'a', b'-', b'2', 
    ];

// BLE scan data
#[rustfmt::skip]
const SCAN_DATA: &[u8] = &[
    0x05, 0x03, 0x0F, 0x18, 0x1a, 0x18, 
    ];


/// BLE GATT server

#[nrf_softdevice::gatt_server]
pub struct Server {
    /// Battery service
    batt: BatteryService,
    enviro: EnviroSensingService,    
}

/// GATT server task: is this necessary?

#[embassy_executor::task]
pub async fn ble_server_task(spawner: Spawner, server: Server, sd: &'static Softdevice) {
    static SERVER: StaticCell<Server> = StaticCell::new();
    let server: &'static mut Server = SERVER.init(server);

    info!("BLE is ON!");

    let config = peripheral::Config::default();

    let adv = peripheral::ConnectableAdvertisement::ScannableUndirected { 
        adv_data: ADV_DATA,
        scan_data: SCAN_DATA,        
    };
    
    loop {

    
        match peripheral::advertise_connectable(sd, adv, &config).await {
            Ok(conn) => {                
                unwrap!(spawner.spawn(conn_task(server, conn)));
            }
            Err(e) => error!("{:?}",e),
         }

    }
     

}

/// BLE connection task. - is this needed???
#[embassy_executor::task]
async fn conn_task(
    server: &'static Server,
    conn: Connection,    

) {
    let data_future = notify_data(server, &conn);  // why can't saadc be borrowed as mutable?
    let gatt_future = gatt_server::run(&conn, server, |e| match e {
        ServerEvent::Batt(BatteryServiceEvent::BatteryLevelCccdWrite { notifications }) => {
            info!("battery notifications: {}", notifications);
        }
        ServerEvent::Enviro(EnviroSensingServiceEvent::HumidityCccdWrite { notifications }) => {
            info!("humidity notifications: {}", notifications);
        }
        ServerEvent::Enviro(EnviroSensingServiceEvent::TemperatureCccdWrite { notifications }) => {
            info!("temperature notifications: {}", notifications);
        }
        ServerEvent::Enviro(EnviroSensingServiceEvent::PressureCccdWrite { notifications }) => {
            info!("pressure notifications: {}", notifications);
        }    
    });

    pin_mut!(data_future);
    pin_mut!(gatt_future);

    match select(data_future, gatt_future).await {
        Either::Left((_, _)) => {
            info!("notification service encountered an error and stopped")
        }
        Either::Right((res, _)) => {
            if let Err(e) = res {
                info!("gatt_server run exited with error: {:?}", e);
            }
        }
    };

}


/// Reads the current ADC value every second and notifies the connected client.
async fn notify_data<'a>(server: &'a Server, 
                        connection: &'a Connection,
                        ) 
{
    loop {
                
        let batt_level: u8 = crate::messages::ADC_SIGNAL.wait().await;

        //let pressure: u32 = crate::messages::PRESS_SIGNAL.wait().await;
        
        let envdata = crate::messages::ENVIRO_SIGNAL.wait().await;

        // Try and notify the connected client of the new ADC value.
        match server.batt.battery_level_notify(connection, &batt_level) {
            Ok(_) => info!("Battery adc_raw_value: {=u8}", &batt_level),
            Err(_) => unwrap!(server.batt.battery_level_set(&batt_level)),
        };

        // Try and notify the connected client of the new presure value.
        match server.enviro.pressure_notify(connection, &envdata.pressure) {
            Ok(_) => info!("Pressure value: {=u32}", &envdata.pressure),
            Err(_) => unwrap!(server.enviro.pressure_set(&envdata.pressure)),
        };
        // Try and notify the connected client of the new temperature value.
        match server.enviro.temperature_notify(connection, &envdata.temperature) {
            Ok(_) => info!("Temperature value: {=u32}", &envdata.temperature),
            Err(_) => unwrap!(server.enviro.temperature_set(&envdata.temperature)),
        };
        // Try and notify the connected client of the new humidity value.
        match server.enviro.humidity_notify(connection, &envdata.humidity) {
            Ok(_) => info!("Humidity value: {=u32}", &envdata.humidity),
            Err(_) => unwrap!(server.enviro.humidity_set(&envdata.humidity)),
};

        // Sleep for one second.        
        //Timer::after(Duration::from_secs(1)).await
        
    }
}