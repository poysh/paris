#![no_main]
#![no_std]

use co2_sensor as _; // global logger + panicking-behavior + memory layout
use co2_sensor::{scd30, alert, rgb_led, buzzer};
use embedded_hal::blocking::delay::DelayMs;
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Level},
    prelude::*,
    twim::{self, Twim},
    Temp, Timer,
};

// definde local altitude
const ALTITUDE: u16 = 0_u16;
//define local pressure
const PRESSURE: u16 = 1025_u16;

#[cortex_m_rt::entry]
fn main() -> ! {
    // take() returns all peripherals, so we can access them
    let board = hal::pac::Peripherals::take().unwrap();
    // first peripheral: initialize timer
    let mut timer = Timer::new(board.TIMER0);
    // second peripheral: initialize temperature sensor
    let mut temp = Temp::new(board.TEMP);
    // third peripheral: access to gpio P0 pins
    let pins = P0Parts::new(board.P0);

    let mut led = pins.p0_13.into_push_pull_output(Level::Low);


    //external led 
    let led_channel_red = pins.p0_03.degrade();
    let led_channel_green = pins.p0_04.degrade();
    let led_channel_blue = pins.p0_28.degrade();

    let mut led_light = rgb_led::LEDColor::init(led_channel_red, led_channel_green, led_channel_blue);

    // configure pins p0_30 and p0_31 as floating pins
    // sda = data signal
    // scl = clock signal
    let scl = pins.p0_30.degrade().into_floating_input();
    let sda = pins.p0_31.degrade().into_floating_input();

    // configure pin p0_29 as buzzer pin
    let mut buzzer = buzzer::Buzzer::init(pins.p0_29.degrade());

    buzzer.buzz(&mut timer);

    // instantiate the pins as `twim::Pins`
    let pins = twim::Pins { scl, sda };

    // create new Twim instance, takes TWIM peripherals, the pins, frequency
    let i2c = Twim::new(board.TWIM0, pins, twim::Frequency::K100);

    let mut sensor = scd30::SCD30::init(i2c);

    let firmware_version = sensor.get_firmware_version().unwrap();
    defmt::println!(
        "Firmware version: {=u8}.{=u8} CRC: {=u8}",
        firmware_version[0],
        firmware_version[1],
        firmware_version[2]
    );

    // print Automatic self-calibration status (ASC)
    defmt::println!("ASC Status: {=bool}", sensor.get_asc_status().unwrap());

    // print current altitude compensation
    defmt::println!(
        "Altitude compensation: {=u16}",
        sensor.get_altitude().unwrap()
    );

    // set altitude compensation
    sensor.set_altitude_compensation(ALTITUDE).unwrap();

    // start continuous measurements
    sensor.start_continuous_measurement(PRESSURE).unwrap();

    // print current altitude compensation
    defmt::println!(
        "Altitude compensation: {=u16}",
        sensor.get_altitude().unwrap()
    );

    loop {
        //check if data is ready to be read
        if sensor.data_ready(&mut timer).unwrap() {
            let result = sensor.read_measurement(&mut timer).unwrap();

            let co2 = result.co2;
            let temp = result.temperature;
            let humidity = result.humidity;

            defmt::println!(
                "
            CO2: {=f32} ppm
            Temperature: {=f32} C
            Humidity: {=f32} %
            ",
                co2,
                temp,
                humidity
            );

            alert::alert(&co2, &mut led_light, &mut buzzer, &mut timer);
        }

        timer.delay_ms(1000_u32);
        led.set_high().unwrap();
        timer.delay_ms(1000_u32);
        led.set_low().unwrap();
    }
}
