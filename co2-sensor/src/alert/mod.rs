use core::ops::Range;
use crate::rgb_led;
use crate::buzzer;
use nrf52840_hal::{Timer, pac::TIMER0};

const WARNING_LEVEL: Range<f32> = 1000.0..1999.99;
const UPPER_LIMIT: f32 = 2000_f32;

pub fn alert(
    co2: &f32, 
    rgb_led: &mut rgb_led::LEDColor, 
    buzzer: &mut buzzer::Buzzer, 
    timer: &mut Timer<TIMER0>
) {
    if WARNING_LEVEL.contains(co2) {
        rgb_led.yellow();
    } else if co2 > &UPPER_LIMIT {
        rgb_led.red();
        buzzer.buzz(timer)
    } else {
        rgb_led.green();
    }
}