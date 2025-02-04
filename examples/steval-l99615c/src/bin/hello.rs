#![no_main]
#![no_std]

use steval_l99615c as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");
    defmt::println!("Hello, world!");
    defmt::println!("Hello, world!");
    defmt::println!("Hello, world!");
    defmt::println!("Hello, world!");
    defmt::info!("Some Info");

    steval_l99615c::exit()
}
