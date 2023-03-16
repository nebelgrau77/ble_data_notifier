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
    messages::Enviro,
    };

use lps22hb::interface::{I2cInterface, i2c::I2cAddress};
use lps22hb::LPS22HB;

use hts221;

use shared_bus;

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

 const BOOT_DELAY_MS: u64 = 100; //small delay for the I2C to initiate correctly and start on boot without having to reset the board

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("This is BLESense!");

    // First we get the peripherals access crate.
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = interrupt::Priority::P2;
    config.time_interrupt_priority = interrupt::Priority::P2;
    let p = embassy_nrf::init(config);

    let board = Board::init(p);
    
    let mut led = board.led;

    let mut adc = board.adc;

    // get the ADC
    //let mut saadc = helpers::init_adc(adc_pin, p.SAADC);
    // Indicated: wait for ADC calibration.
    adc.calibrate().await;



    let _vdd_env = board.vdd_env; // powers the LPS22HB sensor, as per board schematics
    let _r_pullup = board.r_pullup; // necessary for SDA1 and SCL1 to work, as per board schematics


    Timer::after(Duration::from_millis(BOOT_DELAY_MS)).await;

    let mut i2c1 = board.twim1;

    //let manager = shared_bus::CortexMBusManager::new(i2c1);

    //let bus = shared_bus::BusManagerSimple::new(i2c1);


    //let lps_bus = bus.acquire_i2c();

    /*

    // configure I2C interface for the LPS22HB driver
    let i2c_interface = I2cInterface::init(lps_bus, I2cAddress::SA0_GND);
       
    // create a new driver instance with the I2C interface    
    let mut lps22 = LPS22HB::new(i2c_interface);

    */
    
    //let mut hts_bus = bus.acquire_i2c();

    let mut hts221 = hts221::Builder::new()                
    .with_default_7bit_address()
    .with_avg_t(hts221::AvgT::Avg256)
    .with_avg_h(hts221::AvgH::Avg512)    
    .build(&mut i2c1).unwrap();

     

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
        
        // reading ADC value (battery level)

        let mut buf = [0i16; 1];
        adc.sample(&mut buf).await;

        // We only sampled one ADC channel.
        let adc_raw_value: i16 = buf[0];

        let batt_level: u8 = (((adc_raw_value / 256) + 128) * 100 / 255) as u8;

        messages::ADC_SIGNAL.signal(batt_level);

        // reading pressure value

        let mut enviro = Enviro {
            temperature: 9999,
            pressure: 10135,
            humidity: 9999,
        };

        /*

        lps22.one_shot().unwrap();

        let press = lps22.read_pressure().unwrap();

        enviro.pressure = (press * 10.0) as u32;
         */

        let temperature_x8 = hts221.temperature_x8(&mut i2c1).unwrap();
        let temp = temperature_x8 / 8;

        let humidity_x2 = hts221.humidity_x2(&mut i2c1).unwrap();
        let hum = humidity_x2 / 2;

        enviro.temperature = temp  as u32;
        enviro.humidity = hum as u32;
 
        //let pressure: u32 = (press * 100.0) as u32;

        //messages::PRESS_SIGNAL.signal(pressure);

        messages::ENVIRO_SIGNAL.signal(enviro);

        Timer::after(Duration::from_millis(1000)).await;

        if led.is_set_high() {
            led.set_low()
        } else {
            led.set_high()
        }

    }
}


