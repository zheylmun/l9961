#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use l9961::L9961;
use steval_l99615c as _;
use stm32g0xx_hal::{
    i2c::{Config, I2cExt},
    prelude::*,
    stm32,
};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let gpiob = dp.GPIOB.split(&mut rcc);
    let scl = gpiob.pb13.into_open_drain_output_in_state(PinState::High);
    let sda = gpiob.pb14.into_open_drain_output_in_state(PinState::High);
    let i2c = dp
        .I2C2
        .i2c(sda, scl, Config::with_timing(0x2020_151b), &mut rcc);

    let mut l9961 = L9961::new(i2c, 0x49);
    let id = l9961.read_chip_id().unwrap();
    defmt::println!(
        "Chip ID:\n  Metal Id: {}\n  Silicon Id: {}",
        id.metal_id(),
        id.silicon_id()
    );
    defmt::println!("{}", id);
    let mut cfg3_act = l9961.read_cfg3_act().unwrap();
    defmt::println!("{}", cfg3_act);
    cfg3_act.set_cell_1_balance_enabled(false);
    defmt::println!("Activating Cell 1 Balancing");
    l9961.write_cfg3_act(cfg3_act).unwrap();
    let cfg3_act = l9961.read_cfg3_act().unwrap();
    defmt::println!("{}", cfg3_act);

    loop {}
}
