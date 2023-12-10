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

/*
pub fn init_adc(adc_pin: AnyInput, adc: SAADC) -> Saadc<'static, 1> {
    // Then we initialize the ADC. We are only using one channel in this example.
    let config = saadc::Config::default();
    let channel_cfg = saadc::ChannelConfig::single_ended(adc_pin.degrade_saadc());
    let irq = interrupt::take!(SAADC);
    irq.set_priority(interrupt::Priority::P3);
    let saadc = saadc::Saadc::new(adc, irq, config, [channel_cfg]);
    saadc
}
 */

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("This is BLESense!");

    // First we get the peripherals access crate.
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = interrupt::Priority::P2;
    config.time_interrupt_priority = interrupt::Priority::P2;
    let p = embassy_nrf::init(config);

    let board = Board::init(p);

    let mut red = board.red;
    let mut green = board.green;
    let mut blue = board.blue;

    // get the ADC
    //let mut saadc = helpers::init_adc(adc_pin, p.SAADC);
    // Indicated: wait for ADC calibration.
    // adc.calibrate().await;

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
        // let mut buf = [0i16; 1];
        // adc.sample(&mut buf).await;

        // We only sampled one ADC channel.
        // let adc_raw_value: i16 = buf[0];

        let batt_level: u8 = 42_u8; // (((adc_raw_value / 256) + 128) * 100 / 255) as u8;

        /*
        batt_level = match batt_level {
            101u8 => 0u8,
            _ => batt_level + 1,
        };
         */

        messages::ADC_SIGNAL.signal(batt_level);

        Timer::after(Duration::from_millis(1000)).await;

        if led.is_set_high() {
            green.set_low()
        } else {
            green.set_high()
        }
    }
}
