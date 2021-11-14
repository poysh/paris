#![no_main]
#![no_std]

use co2_sensor as _;
// global logger + panicking-behavior + memory layout
use co2_sensor::{dk_button, number_representations::{self, Unit}, rgb_led};
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts},
    prelude::*,
    Temp,
    Timer,
};
use nb::block;

use core::ops::Range;

const FREEZING_TEMPERATURE: f32 = 19.9;
const CRISP_TEMPERATURES: Range<f32> = 20.00..21.99;
const COMFORTABLE_TEMPERATURES: Range<f32> = 22.0..23.99;
const A_BIT_TOO_STEAMY_TEMPERATURES: Range<f32> = 24.0..25.99;
const BOILING_TEMPERATURE: f32 = 26.0;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    // take() returns all peripherals, so we can access them
    let board = hal::pac::Peripherals::take().unwrap();
    // first peripheral: access to the pins
    let pins = P0Parts::new(board.P0);
    // second peripheral: access to internal temp sensor
    let mut temp = Temp::new(board.TEMP);

    // periodic timer
    let mut periodic_timer = Timer::periodic(board.TIMER0);
    let mut millis: u64 = 0;

    let mut button1 = dk_button::Button::new(pins.p0_11.degrade());

    let led_channel_red = pins.p0_03.degrade();
    let led_channel_green = pins.p0_04.degrade();
    let led_channel_blue = pins.p0_28.degrade();

    let mut rgb_light = rgb_led::LEDColor::init(led_channel_red, led_channel_green, led_channel_blue);

    let mut current_unit = number_representations::Unit::Celcius;

    loop {
        // Start by setting/resetting the timer for the next interval
        // Timer counts in microseconds/at 1Mhz, we care about milliseconds
        periodic_timer.start(1000_u32);

        // Every 1000ms
        // read temperature
        // light led in appropriate color
        // print the current temperature reading

        if (millis % 1000) == 0 {
            defmt::println!("Tick (milliseconds): {=u64}", millis);
            // measure temperature
            let temperature: f32 = temp.measure().to_num();

            if temperature < FREEZING_TEMPERATURE {
                rgb_light.blue();
            } else if CRISP_TEMPERATURES.contains(&temperature) {
                rgb_light.light_blue();
            } else if COMFORTABLE_TEMPERATURES.contains(&temperature) {
                rgb_light.green();
            } else if A_BIT_TOO_STEAMY_TEMPERATURES.contains(&temperature) {
                rgb_light.yellow()
            } else if temperature > BOILING_TEMPERATURE {
                rgb_light.red()
            }

            // display temperature
            let converted_temp = current_unit.convert_temperature(&temperature);

            match current_unit {
                Unit::Fahrenheit => defmt::println!("{=f32} F", converted_temp),
                Unit::Kelvin => defmt::println!("{=f32} K", converted_temp),
                Unit::Celcius => defmt::println!("{=f32} C", converted_temp),
            }
        }
        if (millis % 5) == 0 {
            // read and update the button status
            if button1.check_rising_edge() {
                current_unit = match current_unit {
                    Unit::Fahrenheit => Unit::Kelvin,
                    Unit::Kelvin => Unit::Celcius,
                    Unit::Celcius => Unit::Fahrenheit,
                };
                defmt::println!("Unit changed");
            }
        };

        // Now wait for timer to complete
        block!(periodic_timer.wait()).unwrap();

        // Increment our millisecond count
        millis = millis.saturating_add(1);
    }
}