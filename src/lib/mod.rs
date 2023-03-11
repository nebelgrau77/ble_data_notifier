#![feature(type_alias_impl_trait)]
#![no_main]
#![no_std]


#![macro_use]

use defmt_rtt as _; // global logger
use embassy_nrf as _; // time driver
use panic_probe as _;

pub mod ble;
pub mod device;

