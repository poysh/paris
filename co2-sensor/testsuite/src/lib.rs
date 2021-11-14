#![no_std]
#![cfg_attr(test, no_main)]

use co2_sensor as _; // memory layout + panic handler

#[defmt_test::tests]
mod tests {}
