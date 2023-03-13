//! This example showcases how to notify a connected client via BLE of new data.
//! Using, for example, nRF-Connect on iOS/Android we can connect to the device "BlueTest"
//! and see some data getting updated in real-time.
//!
//! Data is not gathered unless a valid connection exists with a client. This is guaranteed
//! by using the "select" crate to wait for either the `gatt_server::run` future or the `data_future` future
//! to complete.
//!
//! Only a single BLE connection is supported in this example so that RAM usage remains minimal.
//!
//! The internal RC oscillator is used to generate the LFCLK.
//!

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::{info, *};
use embassy_executor::Spawner;
use embassy_nrf::interrupt::InterruptExt;
use embassy_nrf::peripherals::SAADC;
use embassy_nrf::saadc::{AnyInput, Input, Saadc};
use embassy_nrf::{interrupt, saadc};
use embassy_time::{Duration, Timer};
use futures::future::{select, Either};
use futures::pin_mut;
use nrf_softdevice::ble::{gatt_server, peripheral, Connection};
use nrf_softdevice::{raw, Softdevice};

use ble_softdev_test::{
    self as _,
    ble::{sd, server},
    device::Board,
    messages,
    };



//-- not necessary right now --//




#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("This is Micromod!");

    // First we get the peripherals access crate.
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = interrupt::Priority::P2;
    config.time_interrupt_priority = interrupt::Priority::P2;
    let p = embassy_nrf::init(config);


    let board = Board::init(p);

    //let mut led = board.led;

    let mut adc_pin = board.a0;

    let mut batt_level: u8 = 0u8;
   

    // get the ADC
    //let mut saadc = init_adc(adc_pin, p.SAADC);
    // Indicated: wait for ADC calibration.
    //saadc.calibrate().await;

    // Enable SoftDevice
    let sd = nrf_softdevice::Softdevice::enable(&sd::softdevice_config());

    // let server = unwrap!(Server::new(sd));

    // Create BLE GATT server
    let server = unwrap!(server::Server::new(sd));

    // Run SoftDevice task
    unwrap!(spawner.spawn(sd::softdevice_task(sd)));

    // Run BLE server task - is that necessary?
    unwrap!(spawner.spawn(server::ble_server_task(spawner, server, sd)));



    loop {
        
        
        batt_level = match batt_level {
            101u8 => 0u8,
            _ => batt_level + 1,
        };
        

        //batt_level += 1;

        messages::ADC_SIGNAL.signal(batt_level);
        
        Timer::after(Duration::from_millis(500)).await;
    }
}

