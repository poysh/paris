#![no_main]
#![no_std]

use co2_sensor as _; // global logger + panicking-behavior + memory layout

// access to functionality:
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

// access to graphics
use embedded_graphics::{
    egtext, 
    fonts::{Font12x16, Font24x32, Text},
    geometry::Point,
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Triangle},
    style::PrimitiveStyle,
    style::TextStyle,
    text_style,
};

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
    spim::{self, Spim},
    Timer,
};

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

    // instantiate ePaper
    let mut delay = Timer::new(board.TIMER1);
    let mut epd4in2 = EPD4in2::new(&mut spi, cs, busy, dc, rst, &mut delay).unwrap();

    let mut display = Display4in2::default();

    let c1 = Circle::new(Point::new(171, 110), 30)
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display);

    let c2 = Circle::new(Point::new(229, 110), 30)
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display);

    let t1 = Triangle::new(
        Point::new(259, 120),
        Point::new(141, 120),
        Point::new(200, 200),
    )
    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
    .draw(&mut display);

    let text1 = Text::new("I love you Lily!", Point::new(100, 220))
        .into_styled(TextStyle::new(Font12x16, BinaryColor::On))
        .draw(&mut display);

    epd4in2.update_frame(&mut spi, &display.buffer()).unwrap();
    epd4in2
        .display_frame(&mut spi)
        .expect("display frame new graphics");

    // loop {
    //     led_1.set_high().unwrap();
    //     timer.delay_ms(1000_u32);
    //     led_1.set_low().unwrap();
    //     timer.delay_ms(1000_u32);
    // }

    co2_sensor::exit()
}
