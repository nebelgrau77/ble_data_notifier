/// Converts raw temperature reading to the Humidity characteristic format
pub fn temp_conv(raw_temp: i16) -> i16 {
    
    let mut conv_temp: i16 = (raw_temp >> 3) * 100;
    
    if raw_temp < 0 {
        conv_temp -= 125 * (raw_temp & 0b111);
    } else {
        conv_temp += 125 * (raw_temp & 0b111)
    }

    conv_temp

}

/// Converts raw temperature reading to the Humidity characteristic format
pub fn hum_conv(raw_hum: u16) -> u16 {

    let mut conv_hum: u16 = (raw_hum >> 1) * 100; // integer part
    
    conv_hum += 5 * (raw_hum & 0b1); // decimal part
    
    conv_hum

}