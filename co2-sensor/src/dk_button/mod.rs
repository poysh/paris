use nrf52840_hal::{
    gpio::{Pin, Input, PullUp},
    prelude::*,
};


pub struct Button {
    pin: Pin<Input<PullUp>>,
    was_pressed: bool,
}

impl Button {
    // static method
    pub fn new<Mode>(pin: Pin<Mode>) -> Self {
        Button {
            pin: pin.into_pullup_input(),
            was_pressed: false,
        }
    }

    // instance method
    pub fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()
    }

    pub fn check_rising_edge(&mut self) -> bool {

        let mut rising_edge = false;

        let is_pressed = self.is_pressed();
        // Only trigger on "rising edge" of the signal
        // Term: "Edge trigger"
        if self.was_pressed && !is_pressed {
            // Was pressed, now isn't:
            rising_edge = true;
        }
        self.was_pressed = is_pressed;
        rising_edge
    }
}