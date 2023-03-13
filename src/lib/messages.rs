use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

pub static ADC_SIGNAL: Signal<CriticalSectionRawMutex, u8> = Signal::new();