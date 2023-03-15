//! Board definition for Arduino BLESense v1
use embassy_nrf::{
    gpio::{AnyPin, Level, Output, OutputDrive, Pin, Pull},
    interrupt::{self, InterruptExt, Priority},
    peripherals::{TWISPI0, UARTE0, TWISPI1},
    twim::Twim,
    uarte::{self, Uarte},
    saadc::{self, AnyInput, Saadc, Input},
};

pub struct Board {
    /// Onboard LED
    pub led: Output<'static, AnyPin>,    
    pub twim0: Twim<'static, TWISPI0>,
    pub twim1: Twim<'static, TWISPI1>,    
    pub adc: Saadc<'static, 1>,    
    pub vdd_env: Output<'static, AnyPin>,    
    pub r_pullup: Output<'static, AnyPin>,     
    //pub uart: Uarte<'static, UARTE0>,
}

impl Board {
    /// Return board with concrete peripherals
    pub fn init(p: embassy_nrf::Peripherals) -> Board {

        let led = Output::new(p.P0_13.degrade(), Level::Low, OutputDrive::Standard);

        let adc_config = embassy_nrf::saadc::Config::default();
        let channel_cfg = embassy_nrf::saadc::ChannelConfig::single_ended(p.P0_04.degrade_saadc());
        let adc_irq = interrupt::take!(SAADC);
        adc_irq.set_priority(interrupt::Priority::P3);
        let adc = saadc::Saadc::new(p.SAADC, adc_irq, adc_config, [channel_cfg]);

        
        // TWIM0 for the EXTERNAL sensors                
        let twim_config = embassy_nrf::twim::Config::default();
        let twim0_irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);
        twim0_irq.set_priority(Priority::P3);
        let twim0 = Twim::new(p.TWISPI0, twim0_irq, p.P0_31, p.P0_02, twim_config);


        // TWIM1 for the INTERNAL sensors                
        let twim_config = embassy_nrf::twim::Config::default();
        let twim1_irq = interrupt::take!(SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1);
        twim1_irq.set_priority(Priority::P3);
        let twim1 = Twim::new(p.TWISPI1, twim1_irq, p.P0_14, p.P0_15, twim_config);
        
        // internal pins necessary for the sensor to work - control power supply to the sensors
        let vdd_env = Output::new(p.P0_22.degrade(), Level::High, OutputDrive::Standard);
        let r_pullup = Output::new(p.P1_00.degrade(), Level::High, OutputDrive::Standard);

       
        Board {             
            twim0,
            twim1,            
            adc,
            led,
            vdd_env,
            r_pullup,
            //uart,
        }

    }
}

