#![no_std]
#![cfg_attr(test, no_main)]

use dk9160 as _; // memory layout + panic handler

#[defmt_test::tests]
mod tests {}
