#![no_main]
#![no_std]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::Delay;
use l9961::{
    configuration::VoltageThresholds,
    registers::{Cfg2Enables, FetConfig},
};
use steval_l99615c::{self as functions, initialize_l9961};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let peripherals = embassy_stm32::init(Default::default());
    let mut l9961 = initialize_l9961(peripherals);
    let mut delay = Delay;
    info!("L9961 Initialized");
    l9961.wake_if_asleep(&mut delay).await;
    l9961.disable_measurements().await.unwrap();
    l9961.mask_all_faults().await.unwrap();
    l9961.clear_all_faults().await.unwrap();
    // See if the l9961 is awake by trying to read the chip ID

    let cell_thresholds = VoltageThresholds::new()
        .with_cell_under_voltage_threshold_mv(1000)
        .with_cell_severe_under_voltage_threshold_mv(500);
    l9961
        .configure_voltage_thresholds(cell_thresholds)
        .await
        .unwrap();

    let enables = Cfg2Enables::new(
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        FetConfig::HighSide,
        FetConfig::HighSide,
        false,
    );
    l9961.write_cfg2_enables(enables).await.unwrap();
    /*
        let cell_filters = Cfg1FiltersCycles::new(
            TCellFilter::T1_31Ms,
            TSCFilter::T64us,
            TCurFilter::T8_44Ms,
            30,
        );
        l9961.write_cfg1_filters_cycles(cell_filters).unwrap();
        //l9961.download_configuration_from_nvm().unwrap();
    */
    // Read the chip ID
    let id = l9961.read_chip_id().await.unwrap();
    defmt::info!("{}", id);

    // Read the Cfg3Act register
    let cfg3_act = l9961.read_cfg3_act().await.unwrap();
    defmt::info!("{}", cfg3_act);

    // Read the Cfg1FiltersCycles register
    let filters = l9961.read_cfg1_filters_cycles().await.unwrap();
    defmt::info!("{}", filters);

    // Read the Device Address Register
    let device_address = l9961.read_device_address().await.unwrap();
    defmt::info!("{}", device_address);

    // Read the Cfg2Enables register
    let cfg2_enables = l9961.read_cfg2_enables().await.unwrap();
    defmt::info!("{}", cfg2_enables);

    // Read the CSA Gain Factor register
    let csa_gain_factor = l9961.read_csa_gain_factor().await.unwrap();
    defmt::info!("{}", csa_gain_factor);

    // Read the VCellOvTh register
    let vcell_ov_th = l9961.read_vcell_ov_th().await.unwrap();
    defmt::info!("VCell OV Threshold: {}", vcell_ov_th);

    // Read the VCellUvTh register
    let vcell_uv_th = l9961.read_vcell_uv_th().await.unwrap();
    defmt::info!("VCell Undervoltage Threshold: {}", vcell_uv_th);

    // Read the VCellSevereDeltaThrs register
    let vcell_severe_delta_thrs = l9961.read_vcell_severe_delta_thrs().await.unwrap();
    defmt::info!("{}", vcell_severe_delta_thrs);

    // Read the VCellBalUvDeltaTh register
    let vcell_bal_uv_delta_th = l9961.read_vcell_bal_uv_delta_th().await.unwrap();
    defmt::info!("{}", vcell_bal_uv_delta_th);

    // Read the VBOvTh register
    let vb_ov_th = l9961.read_vb_ov_th().await.unwrap();
    defmt::info!("{}", vb_ov_th);

    // Read the VBUvTh register
    let vb_uv_th = l9961.read_vb_uv_th().await.unwrap();
    defmt::info!("{}", vb_uv_th);

    // Read the VBSumMaxDiffTh register
    let vb_sum_max_diff_th = l9961.read_vb_sum_max_diff_th().await.unwrap();
    defmt::info!("{}", vb_sum_max_diff_th);

    // Read the VNTCOTTh register
    let vntc_ot_th = l9961.read_vntc_ot_th().await.unwrap();
    defmt::info!("{}", vntc_ot_th);

    // Read the VNTCUTTh register
    let vntc_ut_th = l9961.read_vntc_ut_th().await.unwrap();
    defmt::info!("{}", vntc_ut_th);

    // Read the VNTCSevereOTTh register
    let vntc_severe_ot_th = l9961.read_vntc_severe_ot_th().await.unwrap();
    defmt::info!("{}", vntc_severe_ot_th);

    // Read the OvCTHresholds register
    let ovc_thresholds = l9961.read_ovc_thresholds().await.unwrap();
    defmt::info!("{}", ovc_thresholds);

    // Read the PersistentOvCThresholds register
    let persistent_ovc_thresholds = l9961.read_persistent_ovc_thresholds().await.unwrap();
    defmt::info!("{}", persistent_ovc_thresholds);

    // Test this new bitmask a bit more than the others
    let to_prdrv_bal_mask = l9961.read_to_prdrv_bal_mask().await.unwrap();
    defmt::info!("{}", to_prdrv_bal_mask);

    let to_fuse_rst_mask = l9961.read_to_fuse_rst_msk().await.unwrap();
    defmt::info!("{}", to_fuse_rst_mask);

    let to_faultn_mask = l9961.read_to_faultn_msk().await.unwrap();
    defmt::info!("{}", to_faultn_mask);

    let manufacturer_name_msb = l9961.read_manufacturer_name_msb().await.unwrap();
    let manufacturer_name_lsb = l9961.read_manufacturer_name_lsb().await.unwrap();
    let manufacturer_name = (manufacturer_name_msb as u32) << 16 | manufacturer_name_lsb as u32;
    defmt::info!("Manufacturer Name: {:#010x}", manufacturer_name);

    let manufacturing_date = l9961.read_manufacturing_date().await.unwrap();
    defmt::info!("Manufacturing Date: {}", manufacturing_date);

    let first_usage_date = l9961.read_first_usage_date().await.unwrap();
    defmt::info!("First Usage Date: {}", first_usage_date);

    let serial_number_msb = l9961.read_serial_number_msb().await.unwrap();
    let serial_number_lsb = l9961.read_serial_number_lsb().await.unwrap();
    defmt::info!(
        "Serial Number: {}",
        ((serial_number_msb as u32) << 16) | serial_number_lsb as u32
    );

    // Check for NVM CRC Faults
    let crc_faults = l9961.read_vcell_1_faults().await.unwrap();
    defmt::info!("{}", crc_faults);

    let diag_ot_ov_ut = l9961.read_diag_ov_ot_ut().await.unwrap();
    defmt::info!("{}", diag_ot_ov_ut);

    let diag_uv = l9961.read_diag_uv().await.unwrap();
    defmt::info!("{}", diag_uv);

    let cc_inst_meas = l9961.read_cc_inst_meas().await.unwrap();
    defmt::info!("{}", cc_inst_meas);

    let cc_acc_msb = l9961.read_cc_acc_msb().await.unwrap();
    let cc_acc_lsb = l9961.read_cc_acc_lsb_cntr().await.unwrap();
    let cc_acc = (cc_acc_msb as u32) << 8 | cc_acc_lsb.get_cc_acc_lsb() as u32;
    defmt::info!("CC_ACC: {}", cc_acc);
    defmt::info!("CC_ACC_SAMPLE_CNT: {}", cc_acc_lsb.get_cc_sample_cnt());
    let diag_curr = l9961.read_diag_curr().await.unwrap();
    defmt::info!("{}", diag_curr);
    // Read the cell voltages

    for i in 1..=5 {
        let cell_voltage = l9961.read_vcell(i).await.unwrap();
        defmt::info!("{}", cell_voltage);
    }

    let vcell_sum = l9961.read_vcellsum().await.unwrap();
    defmt::info!("{}", vcell_sum);

    let vb = l9961.read_vb().await.unwrap();
    defmt::info!("{}", vb);

    let ntc_gpio = l9961.read_ntc_gpio().await.unwrap();
    defmt::info!("{}", ntc_gpio);

    let die_temp = l9961.read_die_temp().await.unwrap();
    defmt::info!("{}", die_temp);

    l9961.go_2_standby().await.unwrap();

    functions::exit()
}
