//! Board definition for Arduino BLESense v1
use embassy_nrf::{
    gpio::{AnyPin, Level, Output, OutputDrive, Pin, Pull},
    interrupt::{self, InterruptExt, Priority},
    //peripherals::{TWISPI0, UARTE0, TWISPI1},
    //twim::Twim,
    //uarte::{self, Uarte},
    saadc::{self, AnyInput, Saadc, Input},
};

pub struct Board {
    /// Onboard LED
    pub led: Output<'static, AnyPin>,    
    //pub twim0: Twim<'static, TWISPI0>,
    //pub uart: Uarte<'static, UARTE0>,
    pub adc: Saadc<'static, 1>,    
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

        /*

        // TWIM0 for the EXTERNAL sensors                
        let twim_config = embassy_nrf::twim::Config::default();
        let twim_irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);
        twim_irq.set_priority(Priority::P3);
        let twim0 = Twim::new(p.TWISPI0, twim_irq, p.P0_31, p.P0_02, twim_config);

        /*

        // for internal need TWIM1
        
        TWIM1 sda: p.P0_14, scl: p.P0_15

        let twim1 = 
        
        */

        /*
        let _vdd_env = port0.p0_22.into_push_pull_output(Level::High); // powers the HTS221 sensor, as per board schematics
        let _r_pullup = port1.p1_00.into_push_pull_output(Level::High); // necessary for SDA1 and SCL1 to work, as per board schematics
        */

        let mut uart_config = uarte::Config::default();
        uart_config.parity = uarte::Parity::EXCLUDED;
        uart_config.baudrate = uarte::Baudrate::BAUD115200;
        let uart_irq = interrupt::take!(UARTE0_UART0);
        uart_irq.set_priority(Priority::P3);
        let uart = uarte::Uarte::new(p.UARTE0, uart_irq, p.P1_10, p.P1_03, uart_config);
            */
        Board {             
            //twim0,
            //uart,
            adc,
            led,
        }

    }
}
