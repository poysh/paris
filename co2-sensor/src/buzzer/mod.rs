use embedded_hal::blocking::delay::DelayMs;
use nrf52840_hal::{Timer, gpio::{Level, PushPull, Pin, Output}, pac::TIMER0, prelude::*, timer::OneShot};

pub struct Buzzer {
    pin: Pin<Output<PushPull>>
}

impl Buzzer {
    pub fn init<Mode>(buzzer_pin: Pin<Mode>) -> Self {
        Buzzer {
            pin: buzzer_pin.into_push_pull_output(Level::Low),
        }
    }

    fn high(&mut self) {
        self.pin.set_high().unwrap();
    }

    fn low(&mut self) {
        self.pin.set_low().unwrap();
    }

    pub fn buzz(&mut self, timer: &mut Timer<TIMER0, OneShot>) {
        for _i in 0..250 {
            self.high();
            timer.delay_ms(10_u32);
            self.low();
            timer.delay_ms(10_u32);
        }
        
    }
}