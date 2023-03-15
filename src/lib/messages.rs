use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

pub static ADC_SIGNAL: Signal<CriticalSectionRawMutex, u8> = Signal::new();
pub static PRESS_SIGNAL: Signal<CriticalSectionRawMutex, u32> = Signal::new();


pub struct Enviro {
    pub temperature: u32,
    pub pressure: u32,
    pub humidity: u32,
}

pub static ENVIRO_SIGNAL: Signal<CriticalSectionRawMutex, Enviro> = Signal::new();

/* idea: have a struct like this:


pub struct Enviro {
    temperature: f32,
    pressure: f32,
    humidity: f32,
}

and send the whole struct in the signal. 

it needs some default values, like 1234.5


 */