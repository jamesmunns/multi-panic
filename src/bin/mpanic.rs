#![no_main]
#![no_std]

use multi_panic as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Here we go!");

    panic!();
}
