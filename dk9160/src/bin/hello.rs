#![no_main]
#![no_std]

use dk9160 as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    dk9160::exit()
}
