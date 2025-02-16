#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use l9961::{registers::ToPrdrvBalMask, L9961};
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

    l9961.download_configuration_from_nvm().unwrap();

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

    // Read the VBOvTh register
    let vb_ov_th = l9961.read_vb_ov_th().unwrap();
    defmt::info!("{}", vb_ov_th);

    // Read the VBUvTh register
    let vb_uv_th = l9961.read_vb_uv_th().unwrap();
    defmt::info!("{}", vb_uv_th);

    // Read the VBSumMaxDiffTh register
    let vb_sum_max_diff_th = l9961.read_vb_sum_max_diff_th().unwrap();
    defmt::info!("{}", vb_sum_max_diff_th);

    // Read the VNTCOTTh register
    let vntc_ot_th = l9961.read_vntc_ot_th().unwrap();
    defmt::info!("{}", vntc_ot_th);

    // Read the VNTCUTTh register
    let vntc_ut_th = l9961.read_vntc_ut_th().unwrap();
    defmt::info!("{}", vntc_ut_th);

    // Read the VNTCSevereOTTh register
    let vntc_severe_ot_th = l9961.read_vntc_severe_ot_th().unwrap();
    defmt::info!("{}", vntc_severe_ot_th);

    // Read the OvCTHresholds register
    let ovc_thresholds = l9961.read_ovc_thresholds().unwrap();
    defmt::info!("{}", ovc_thresholds);

    // Read the PersistentOvCThresholds register
    let persistent_ovc_thresholds = l9961.read_persistent_ovc_thresholds().unwrap();
    defmt::info!("{}", persistent_ovc_thresholds);

    // Test this new bitmask a bit more than the others
    let to_prdrv_bal_mask = l9961.read_to_prdrv_bal_mask().unwrap();
    defmt::info!("{}", to_prdrv_bal_mask);
    let masking = ToPrdrvBalMask::CELL_UV_PRDRV_MSK | ToPrdrvBalMask::CELL_SEVERE_UV_PRDRV_MSK;
    l9961.write_to_prdrv_bal_mask(masking).unwrap();
    let to_prdrv_bal_mask = l9961.read_to_prdrv_bal_mask().unwrap();
    defmt::info!("{}", to_prdrv_bal_mask);
    let masking = ToPrdrvBalMask::empty();
    l9961.write_to_prdrv_bal_mask(masking).unwrap();
    let to_prdrv_bal_mask = l9961.read_to_prdrv_bal_mask().unwrap();
    defmt::info!("{}", to_prdrv_bal_mask);
    let to_fuse_rst_mask = l9961.read_to_fuse_rst_msk().unwrap();
    defmt::info!("{}", to_fuse_rst_mask);
    let to_faultn_mask = l9961.read_to_faultn_msk().unwrap();
    defmt::info!("{}", to_faultn_mask);

    let manufacturer_name_msb = l9961.read_manufacturer_name_msb().unwrap();
    let manufacturer_name_lsb = l9961.read_manufacturer_name_lsb().unwrap();
    let manufacturer_name = (manufacturer_name_msb as u32) << 16 | manufacturer_name_lsb as u32;
    defmt::info!("Manufacturer Name: {:#010x}", manufacturer_name);

    let manufacturing_date = l9961.read_manufacturing_date().unwrap();
    defmt::info!("Manufacturing Date: {}", manufacturing_date);

    let first_usage_date = l9961.read_first_usage_date().unwrap();
    defmt::info!("First Usage Date: {}", first_usage_date);

    let serial_number_msb = l9961.read_serial_number_msb().unwrap();
    defmt::info!("Serial Number MSB: {}", serial_number_msb);

    functions::exit()
}
