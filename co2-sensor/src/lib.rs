#![no_std]

use defmt_rtt as _; // global logger

use nrf52840_hal as _;

use panic_probe as _;

pub mod dk_button;
pub mod rgb_led;
pub mod number_representations;
pub mod scd30;
pub mod buzzer;

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
