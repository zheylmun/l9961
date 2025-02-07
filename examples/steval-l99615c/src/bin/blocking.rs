#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use l9961::L9961;
use steval_l99615c as functions;
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

    // Read the chip ID
    let id = l9961.read_chip_id().unwrap();
    defmt::info!("{}", id);

    // Read the Cfg3Act register
    let cfg3_act = l9961.read_cfg3_act().unwrap();
    defmt::info!("{}", cfg3_act);

    // Read the Cfg1FiltersCycles register
    let filters = l9961.read_cfg1_filters_cycles().unwrap();
    defmt::info!("{}", filters);

    // Read the Device Address Register
    let device_address = l9961.read_device_address().unwrap();
    defmt::info!("{}", device_address);

    // Read the Cfg2Enables register
    let cfg2_enables = l9961.read_cfg2_enables().unwrap();
    defmt::info!("{}", cfg2_enables);

    // Read the CSA Gain Factor register
    let csa_gain_factor = l9961.read_csa_gain_factor().unwrap();
    defmt::info!("{}", csa_gain_factor);

    // Read the VCellOvTh register
    let vcell_ov_th = l9961.read_vcell_ov_th().unwrap();
    defmt::info!("VCell OV Threshold: {}", vcell_ov_th);

    // Read the VCellUvTh register
    let vcell_uv_th = l9961.read_vcell_uv_th().unwrap();
    defmt::info!("VCell Undervoltage Threshold: {}", vcell_uv_th);

    // Read the VCellSevereDeltaThrs register
    let vcell_severe_delta_thrs = l9961.read_vcell_severe_delta_thrs().unwrap();
    defmt::info!("{}", vcell_severe_delta_thrs);

    // Read the VCellBalUvDeltaTh register
    let vcell_bal_uv_delta_th = l9961.read_vcell_bal_uv_delta_th().unwrap();
    defmt::info!("{}", vcell_bal_uv_delta_th);

    functions::exit()
}
