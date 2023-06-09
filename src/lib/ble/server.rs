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
        0x03, 0x03, 0x0F, 0x18,
        0x09, 0x09, b'B', b'l', b'u', b'e', b'T', b'e', b's', b't', 
    ];

// BLE scan data
#[rustfmt::skip]
const SCAN_DATA: &[u8] = &[
    0x03, 0x03, 0x0F, 0x18, 
    ];


/// BLE GATT server

#[nrf_softdevice::gatt_server]
pub struct Server {
    /// Battery service
    batt: BatteryService,
    //temp: Thermometer,
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
    let data_future = notify_data(&server, &conn); // some function
    let gatt_future = gatt_server::run(&conn, server, |e| match e {
        ServerEvent::Batt(BatteryServiceEvent::BatteryLevelCccdWrite { notifications }) => {
            info!("battery notifications: {}", notifications);
        }
        /*
        ServerEvent::Temp(ThermometerEvent::TemperatureCccdWrite { notifications }) => {
            info!("temperature notifications: {}", notifications);
        }
         */
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

/// Send some data out

async fn notify_data<'a>(
    server: &'a Server,
    conn: &'a Connection,
    //saadc: &'a mut Saadc<'_, 1>,
) {
    loop {
        
        // no real data gathering, just a static value to test it

        let batt_level: u8 = 123;

        // try and notify the connected client of the new ADC value
        match server.batt.battery_level_notify(conn, &batt_level) {
            Ok(_) => info!("Battery adc_raw_value: {=u8}", &batt_level),
            Err(_) => unwrap!(server.batt.battery_level_set(&batt_level))
        };

        // sleep for some time
        Timer::after(Duration::from_secs(1)).await
    }
}