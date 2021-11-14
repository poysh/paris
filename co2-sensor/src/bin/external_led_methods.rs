#![no_main]
#![no_std]

use co2_sensor as _;
// global logger + panicking-behavior + memory layout
use nrf52840_hal::{
    self as hal,
    prelude::*,
    gpio::{
        {p0::Parts as P0Parts},
        Pin,
        Level,
        Output,
        PushPull,
    },
    pac::TIMER0,
    timer::OneShot,
    Timer,
};
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

struct LEDColor {
    r: Pin<Output<PushPull>>,
    g: Pin<Output<PushPull>>,
    b: Pin<Output<PushPull>>,
}

impl LEDColor {
    // static methods:
    // they don't need to be called by and instance
    // they are used as constructors
    // they don't have `self` as an argument

    pub fn init<Mode>(led_red: Pin<Mode>, led_green: Pin<Mode>, led_blue: Pin<Mode>) -> LEDColor {

        LEDColor {
            r: led_red.into_push_pull_output(Level::High),
            g: led_green.into_push_pull_output(Level::High),
            b: led_blue.into_push_pull_output(Level::High),
        }
    }

    // instance methods
    // they are called by the instance
    // they have a reference `self` as an argument

    fn red(&mut self) {
        self.r.set_low().unwrap();
        self.g.set_high().unwrap();
        self.b.set_high().unwrap();
    }

    fn green(&mut self) {
        self.r.set_high().unwrap();
        self.g.set_low().unwrap();
        self.b.set_high().unwrap();
    }

    fn blue(&mut self) {
        self.r.set_high().unwrap();
        self.g.set_high().unwrap();
        self.b.set_low().unwrap();
    }

    fn blinky(&mut self, timer: &mut Timer<TIMER0, OneShot>) {
        self.red();
        timer.delay_ms(1000_u32);
        self.green();
        timer.delay_ms(1000_u32);
        self.blue();
        timer.delay_ms(1000_u32);
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    // take() returns all peripherals, so we can access them
    let board = hal::pac::Peripherals::take().unwrap();
    // first peripheral: initialize timer
    let mut timer = Timer::new(board.TIMER0);
    // second peripheral: access to P0_Pins
    let pins = P0Parts::new(board.P0);

    let led_channel_red = pins.p0_03.degrade();
    let led_channel_green = pins.p0_04.degrade();
    let led_channel_blue = pins.p0_28.degrade();

    let mut light = LEDColor::init(led_channel_red, led_channel_green, led_channel_blue);

    light.blinky(&mut timer);

    loop {
        light.red();

        timer.delay_ms(1000_u32);

        light.blue();

        timer.delay_ms(1000_u32);

        light.green();

        timer.delay_ms(1000_u32);
    }

    co2_sensor::exit()
}
