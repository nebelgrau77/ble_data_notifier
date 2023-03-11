//! Board definition for MicroMod with ML carrier board
use embassy_nrf::{
    gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull},
    interrupt::{self, InterruptExt, Priority},
    peripherals::{TWISPI0, UARTE0},
    twim::Twim,
    uarte::{self, Uarte}
};

pub struct Board {
    /// Onboard LED
    pub led: Output<'static, AnyPin>,
    pub a0: Input<'static, AnyPin>,
    pub twim: Twim<'static, TWISPI0>,
    pub uart: Uarte<'static, UARTE0>,
}

impl Board {
    /// Return board with concrete peripherals
    pub fn init(p: embassy_nrf::Peripherals) -> Board {
        let led = Output::new(p.P0_13.degrade(), Level::Low, OutputDrive::Standard);

        let a0 = Input::new(p.P0_04.degrade(), Pull::Up);

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
            led,
            a0,
            twim,
            uart
        }

    }
}
