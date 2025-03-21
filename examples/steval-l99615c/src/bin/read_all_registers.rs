#![no_main]
#![no_std]

use embassy_executor::Spawner;
use embassy_time::Delay;
use l9961::{
    registers::{Cfg2Enables, FetConfig},
    Config,
};
use steval_l99615c::{self as functions, configure_l9961_peripherals};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let peripherals = embassy_stm32::init(Default::default());
    let config = Config::default();
    let mut l9961 = configure_l9961_peripherals(peripherals, config);
    let mut delay = Delay;
    l9961.wake_if_asleep(&mut delay).await;
    l9961.disable_measurements().await.unwrap();
    // Make sure everything is turned off so we can read all of the registers
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

    // Read em all
    let chip_id = l9961.read_chip_id().await.unwrap();
    let cfg3_act = l9961.read_cfg3_act().await.unwrap();
    let filters = l9961.read_cfg1_filters_cycles().await.unwrap();
    let device_address = l9961.read_device_address().await.unwrap();
    let cfg2_enables = l9961.read_cfg2_enables().await.unwrap();
    let csa_gain_factor = l9961.read_csa_gain_factor().await.unwrap();
    let vcell_ov_th = l9961.read_vcell_ov_th().await.unwrap();
    let vcell_uv_th = l9961.read_vcell_uv_th().await.unwrap();
    let vcell_severe_delta_thrs = l9961.read_vcell_severe_delta_thrs().await.unwrap();
    let vcell_bal_uv_delta_th = l9961.read_vcell_bal_uv_delta_th().await.unwrap();
    let vb_ov_th = l9961.read_vb_ov_th().await.unwrap();
    let vb_uv_th = l9961.read_vb_uv_th().await.unwrap();
    let vb_sum_max_diff_th = l9961.read_vb_sum_max_diff_th().await.unwrap();
    let vntc_ot_th = l9961.read_vntc_ot_th().await.unwrap();
    let vntc_ut_th = l9961.read_vntc_ut_th().await.unwrap();
    let vntc_severe_ot_th = l9961.read_vntc_severe_ot_th().await.unwrap();
    let ovc_thresholds = l9961.read_ovc_thresholds().await.unwrap();
    let persistent_ovc_thresholds = l9961.read_persistent_ovc_thresholds().await.unwrap();
    let to_prdrv_bal_mask = l9961.read_to_prdrv_bal_mask().await.unwrap();
    let to_fuse_rst_mask = l9961.read_to_fuse_rst_msk().await.unwrap();
    let to_faultn_mask = l9961.read_to_faultn_msk().await.unwrap();

    let manufacturer_name_msb = l9961.read_manufacturer_name_msb().await.unwrap();
    let manufacturer_name_lsb = l9961.read_manufacturer_name_lsb().await.unwrap();
    let manufacturer_name = (manufacturer_name_msb as u32) << 16 | manufacturer_name_lsb as u32;

    let manufacturing_date = l9961.read_manufacturing_date().await.unwrap();

    let first_usage_date = l9961.read_first_usage_date().await.unwrap();

    let serial_number_msb = l9961.read_serial_number_msb().await.unwrap();
    let serial_number_lsb = l9961.read_serial_number_lsb().await.unwrap();
    let serial_number = (serial_number_msb as u32) << 16 | serial_number_lsb as u32;
    // Check for NVM CRC Faults
    let crc_faults = l9961.read_vcell_1_faults().await.unwrap();
    let diag_ot_ov_ut = l9961.read_diag_ov_ot_ut().await.unwrap();
    let diag_uv = l9961.read_diag_uv().await.unwrap();
    let cc_inst_meas = l9961.read_cc_inst_meas().await.unwrap();

    let cc_acc_msb = l9961.read_cc_acc_msb().await.unwrap();
    let cc_acc_lsb = l9961.read_cc_acc_lsb_cntr().await.unwrap();
    let cc_acc = (cc_acc_msb as u32) << 8 | cc_acc_lsb.get_cc_acc_lsb() as u32;
    let diag_curr = l9961.read_diag_curr().await.unwrap();

    let cell_voltage_1 = l9961.read_vcell(1).await.unwrap();
    let cell_voltage_2 = l9961.read_vcell(2).await.unwrap();
    let cell_voltage_3 = l9961.read_vcell(3).await.unwrap();
    let cell_voltage_4 = l9961.read_vcell(4).await.unwrap();
    let cell_voltage_5 = l9961.read_vcell(5).await.unwrap();

    let vcell_sum = l9961.read_vcellsum().await.unwrap();
    let vb = l9961.read_vb().await.unwrap();
    let ntc_gpio = l9961.read_ntc_gpio().await.unwrap();
    let die_temp = l9961.read_die_temp().await.unwrap();

    // Print it all out
    defmt::println!(
        "
{},
{},
{},
{},
{},
{},
{},
{},
{},
{},
{},
{},
{},
{},
{},
{},
{},
{},
To PreDrivers and Balance Mask: {},
To Fuse and Reset Mask: {},
To Fault Line Mask: {},
Manufacturer Name: {:#010x},
Manufacturing Date: {},
First Usage Date: {},
Serial Number: {},
{},
Diag Over Temp, Over Voltage, Under Temp faults: {},
Diag Under Voltage faults: {},
CC_INST_MEAS: {},
CC_ACC: {},
CC_ACC_SAMPLE_CNT: {},
Diag Current faults: {},
{},
{},
{},
{},
{},
{},
{},
{},
{}
",
        chip_id,
        cfg3_act,
        filters,
        device_address,
        cfg2_enables,
        csa_gain_factor,
        vcell_ov_th,
        vcell_uv_th,
        vcell_severe_delta_thrs,
        vcell_bal_uv_delta_th,
        vb_ov_th,
        vb_uv_th,
        vb_sum_max_diff_th,
        vntc_ot_th,
        vntc_ut_th,
        vntc_severe_ot_th,
        ovc_thresholds,
        persistent_ovc_thresholds,
        to_prdrv_bal_mask,
        to_fuse_rst_mask,
        to_faultn_mask,
        manufacturer_name,
        manufacturing_date,
        first_usage_date,
        serial_number,
        crc_faults,
        diag_ot_ov_ut,
        diag_uv,
        cc_inst_meas,
        cc_acc,
        cc_acc_lsb.get_cc_sample_cnt(),
        diag_curr,
        cell_voltage_1,
        cell_voltage_2,
        cell_voltage_3,
        cell_voltage_4,
        cell_voltage_5,
        vcell_sum,
        vb,
        ntc_gpio,
        die_temp,
    );

    l9961.go_2_standby().await.unwrap();

    functions::exit()
}
