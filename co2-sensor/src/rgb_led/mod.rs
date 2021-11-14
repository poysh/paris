use nrf52840_hal::{
    gpio::{Pin, PushPull, Output, Level},
    pac::TIMER0,
    prelude::*,
    Timer,
    timer::OneShot,
};

pub struct LEDColor {
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

    pub fn red(&mut self) {
        self.r.set_low().unwrap();
        self.g.set_high().unwrap();
        self.b.set_high().unwrap();
    }

    pub fn green(&mut self) {
        self.r.set_high().unwrap();
        self.g.set_low().unwrap();
        self.b.set_high().unwrap();
    }

    pub fn blue(&mut self) {
        self.r.set_high().unwrap();
        self.g.set_high().unwrap();
        self.b.set_low().unwrap();
    }

    pub fn light_blue(&mut self) {
        self.r.set_high().unwrap();
        self.g.set_low().unwrap();
        self.b.set_low().unwrap();
    }

    pub fn yellow(&mut self) {
        self.r.set_low().unwrap();
        self.g.set_high().unwrap();
        self.b.set_low().unwrap();
    }

    pub fn white(&mut self) {
        self.r.set_low().unwrap();
        self.g.set_low().unwrap();
        self.b.set_low().unwrap();
    }

    pub fn off(&mut self) {
        self.r.set_high().unwrap();
        self.g.set_high().unwrap();
        self.b.set_high().unwrap();
    }

    pub fn blinky(&mut self, timer: &mut Timer<TIMER0, OneShot>) {
        self.red();
        timer.delay_ms(1000_u32);
        self.green();
        timer.delay_ms(1000_u32);
        self.blue();
        timer.delay_ms(1000_u32);
    }
}