//! Board definition for Arduino BLESense v1
use embassy_nrf::{
    gpio::{AnyPin, Level, Output, OutputDrive, Pin, Pull},
    interrupt::{self, InterruptExt, Priority},
    //peripherals::{TWISPI0, UARTE0, TWISPI1},
    //twim::Twim,
    //uarte::{self, Uarte},
    saadc::{self, AnyInput, Input, Saadc},
};

pub struct Board {
    /// Onboard LED
    pub red: Output<'static, AnyPin>,
    pub green: Output<'static, AnyPin>,
    pub blue: Output<'static, AnyPin>,
    //pub twim0: Twim<'static, TWISPI0>,
    //pub uart: Uarte<'static, UARTE0>,
    //pub adc: Saadc<'static, 1>,
}

impl Board {
    /// Return board with concrete peripherals
    pub fn init(p: embassy_nrf::Peripherals) -> Board {
        let red = Output::new(p.P0_26.degrade(), Level::Low, OutputDrive::Standard);
        let green = Output::new(p.P0_30.degrade(), Level::Low, OutputDrive::Standard);
        let blue = Output::new(p.P0_06.degrade(), Level::Low, OutputDrive::Standard);

        /*
        let adc_config = embassy_nrf::saadc::Config::default();
        let channel_cfg = embassy_nrf::saadc::ChannelConfig::single_ended(p.P0_04.degrade_saadc());
        let adc_irq = interrupt::take!(SAADC);
        adc_irq.set_priority(interrupt::Priority::P3);
        let adc = saadc::Saadc::new(p.SAADC, adc_irq, adc_config, [channel_cfg]);
         */

        Board {
            //twim0,
            //uart,
            red,
            green,
            blue,
        }
    }
}
