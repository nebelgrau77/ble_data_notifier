use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

pub static ADC_SIGNAL: Signal<CriticalSectionRawMutex, u8> = Signal::new();
pub static PRESS_SIGNAL: Signal<CriticalSectionRawMutex, u32> = Signal::new();


pub struct Enviro {
    pub temperature: i16,
    pub pressure: u32,
    pub humidity: u16,
}

pub static ENVIRO_SIGNAL: Signal<CriticalSectionRawMutex, Enviro> = Signal::new();
