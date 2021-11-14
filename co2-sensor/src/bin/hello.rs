#![no_main]
#![no_std]

use co2_sensor as _; // global logger + panicking-behavior + memory layout
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Level},
    Timer,
    Temp
};
use embedded_hal::{
    blocking::delay::DelayMs,
    digital::v2::OutputPin
};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    // take() returns all peripherals, so we can access them
    let board = hal::pac::Peripherals::take().unwrap();
    // first peripheral: initialize timer
    let mut timer = Timer::new(board.TIMER0);
    // second peripheral: initialize temperature sensor
    let mut temp = Temp::new(board.TEMP);


    loop {
        let temperature: f32 = temp.measure().to_num();
        defmt::info!("{=f32} C", temperature);
        timer.delay_ms(60000_u32);
    }


    co2_sensor::exit()
}
