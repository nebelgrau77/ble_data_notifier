//! Board definition for MicroMod with ML carrier board
use embassy_nrf::{
    gpio::{AnyPin, Level, Output, OutputDrive, Pin, Pull},
    interrupt::{self, InterruptExt, Priority},
    peripherals::{TWISPI0, UARTE0},
    twim::Twim,
    uarte::{self, Uarte},
    saadc::{self, AnyInput, Saadc, Input},
};

pub struct Board {
    /// Onboard LED
    pub led: Output<'static, AnyPin>,    
    pub twim: Twim<'static, TWISPI0>,
    pub uart: Uarte<'static, UARTE0>,
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
        
        
        let twim_config = embassy_nrf::twim::Config::default();
        let twim_irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);
        twim_irq.set_priority(Priority::P3);
        let twim = Twim::new(p.TWISPI0, twim_irq, p.P0_08, p.P0_11, twim_config);

        let mut uart_config = uarte::Config::default();
        uart_config.parity = uarte::Parity::EXCLUDED;
        uart_config.baudrate = uarte::Baudrate::BAUD115200;
        let uart_irq = interrupt::take!(UARTE0_UART0);
        uart_irq.set_priority(Priority::P3);
        let uart = uarte::Uarte::new(p.UARTE0, uart_irq, p.P1_10, p.P1_03, uart_config);

        Board {             
            twim,
            uart,
            adc,
            led,
        }

    }
}
