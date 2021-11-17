#![no_main]
#![no_std]

use co2_sensor::{self as _, alert::{self, alert}, buzzer, epaper_helper, rgb_led, scd30}; // global logger + panicking-behavior + memory layout

// access to functionality:
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

// access to waveshare display
use epd_waveshare::{
    epd4in2::{self, Display4in2, EPD4in2},
    graphics::Display,
    prelude::*,
};
// access to board peripherals:
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, p1::Parts as P1Parts, Level},
    prelude::*,
    twim::{self, Twim},
    spim::{self, Spim},
    Timer,
};

// definde local altitude
const ALTITUDE: u16 = 0_u16;
//define local pressure
const PRESSURE: u16 = 1025_u16;

// position for measurements on the display
const CO2_POSITION: (i32, i32) = (220, 90);
const CO2_UNIT: &str = "ppm";

const TEMP_POSITION: (i32, i32) = (220, 130);
const TEMP_UNIT: &str = "°C";

const HUMIDITY_POSITION: (i32, i32) = (220, 170);
const HUMIDITY_UNIT: &str = "%";

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");
    // take() returns all peripherals, so we can access them
    let board = hal::pac::Peripherals::take().unwrap();
    // first peripheral: initialize timer
    let mut timer = Timer::new(board.TIMER0);
    // second peripheral: access to P0 pins
    let pins = P0Parts::new(board.P0);
    // third peripheral: access to P1 pins
    let pins_1 = P1Parts::new(board.P1);
    // set pin p0_13 into push pull output, with the initial level HIGH
    let mut led_1 = pins.p0_13.into_push_pull_output(Level::Low);
    timer.delay_ms(1000_u32);

    let din = pins_1.p1_01.into_push_pull_output(Level::Low).degrade();
    let clk = pins_1.p1_02.into_push_pull_output(Level::Low).degrade();
    let cs = pins_1.p1_03.into_push_pull_output(Level::Low);
    let dc = pins_1.p1_04.into_push_pull_output(Level::Low);
    let rst = pins_1.p1_05.into_push_pull_output(Level::Low);
    let busy = pins_1.p1_06.into_floating_input();

    let spi_pins = spim::Pins {
        sck: clk,
        miso: None,
        mosi: Some(din),
    };

    let mut spi = Spim::new(
        board.SPIM3,
        spi_pins,
        spim::Frequency::K500,
        spim::MODE_0,
        0,
    );

    // instantiate Timer for epaper
    let mut delay = Timer::new(board.TIMER1);

    // instantiate epaper display on SPIM3
    let mut epd4in2 = EPD4in2::new(&mut spi, cs, busy, dc, rst, &mut delay).unwrap();

    //external led
    let led_channel_red = pins.p0_03.degrade();
    let led_channel_green = pins.p0_04.degrade();
    let led_channel_blue = pins.p0_28.degrade();

    let mut led_light =
        rgb_led::LEDColor::init(led_channel_red, led_channel_green, led_channel_blue);

    // configure pins p0_30 and p0_31 as floating pins
    // sda = data signal
    // scl = clock signal
    let scl = pins.p0_30.degrade().into_floating_input();
    let sda = pins.p0_31.degrade().into_floating_input();

    // configure pin p0_29 as buzzer pin
    let mut buzzer = buzzer::Buzzer::init(pins.p0_29.degrade());

    // instantiate the pins as `twim::Pins`
    let pins = twim::Pins { scl, sda };

    // create new Twim instance, takes TWIM peripherals, the pins, frequency
    let i2c = Twim::new(board.TWIM0, pins, twim::Frequency::K100);

    let mut sensor = scd30::SCD30::init(i2c);

    let firmware_version = sensor.get_firmware_version().unwrap_or_else(|error| {
        led_light.error_blink(&mut timer);
        panic!("Error getting firmware version: {:?}", error)
    });

    defmt::println!(
        "Firmware version: {=u8}.{=u8} CRC: {=u8}",
        firmware_version[0],
        firmware_version[1],
        firmware_version[2]
    );

    // print Automatic self-calibration status (ASC)
    defmt::println!("ASC Status: {=bool}", sensor.get_asc_status().unwrap());

    // start continuous measurements
    sensor.start_continuous_measurement(PRESSURE).unwrap();

    // print current altitude compensation
    defmt::println!(
        "Altitude compensation: {=u16}",
        sensor.get_altitude().unwrap()
    );

    loop {
        defmt::info!("Entering loop");
        //check if data is ready to be read
        if sensor.data_ready(&mut timer).unwrap() {
            defmt::info!("Result ready");
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

            let display = Display4in2::default();
            let display = epaper_helper::draw_text(display);

            let display = epaper_helper::draw_numbers(co2, CO2_UNIT, CO2_POSITION, display);
            let display = epaper_helper::draw_numbers(temp, TEMP_UNIT, TEMP_POSITION, display);
            let display = epaper_helper::draw_numbers(humidity, HUMIDITY_UNIT, HUMIDITY_POSITION, display);

            epd4in2.update_frame(&mut spi, &display.buffer()).unwrap();
            epd4in2.display_frame(&mut spi).expect("display frame new graphics"); 

            // TODO toggle buzzer with button
            alert::alert(&co2, &mut led_light, &mut buzzer, &mut timer);
        }

        timer.delay_ms(30000_u32);
        led_1.set_high().unwrap();
        timer.delay_ms(30000_u32);
        led_1.set_low().unwrap();
    }

    // loop {
    //     led_1.set_high().unwrap();
    //     timer.delay_ms(1000_u32);
    //     led_1.set_low().unwrap();
    //     timer.delay_ms(1000_u32);
    // }

    // co2_sensor::exit()
}
