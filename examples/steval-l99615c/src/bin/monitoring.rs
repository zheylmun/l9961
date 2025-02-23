#![no_main]
#![no_std]

use embassy_executor::Spawner;
use embassy_time::Delay;
use l9961::{
    configuration::{CounterThreshold, VoltageThresholds},
    registers::{Cfg2Enables, FetConfig},
};
use steval_l99615c::{self as functions, initialize_l9961};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let peripherals = embassy_stm32::init(Default::default());
    let mut l9961 = initialize_l9961(peripherals);
    let mut delay = Delay;
    l9961.wake_if_asleep(&mut delay).await;
    // Make sure measurements are disabled before changing settings
    l9961.disable_measurements().await.unwrap();

    // Configure cell and battery measurements
    let enables = Cfg2Enables::new(
        true,                // Enable cell 1 voltage measurement
        true,                // Enable cell 2 voltage measurement
        true,                // Enable cell 3 voltage measurement
        true,                // Enable cell 4 voltage measurement
        true,                // Enable cell 5 voltage measurement
        true,                // Enable Battery voltage measurement
        false,               // Disable temperature measurement
        false,               // Disable current measurement
        false,               // Disable coulomb counter
        false,               // Disable over-current detection
        false,               // Disable short-circuit detection
        FetConfig::HighSide, // Set the discharge FET configuration to high-side
        FetConfig::HighSide, // Set the charge FET configuration to high-side
        false,               // Disable data CRC
    );
    l9961.write_cfg2_enables(enables).await.unwrap();

    // Configure the voltage monitoring with extreme thresholds to avoid faults triggering
    let voltage = VoltageThresholds {
        cell_over_voltage_threshold_mv: 2000,
        cell_severe_over_voltage_delta_threshold_mv: 100,
        cell_under_voltage_threshold_mv: 1000,
        cell_severe_under_voltage_delta_threshold_mv: 500,
        cell_balancing_under_voltage_delta_threshold_mv: 500,
        pack_over_voltage_threshold_mv: 24000,
        pack_under_voltage_threshold_mv: 0,
        max_pack_cell_sum_delta_mv: 1000,
        fault_counter_threshold: CounterThreshold::default(),
    };
    l9961.configure_voltage_thresholds(voltage).await.unwrap();
    // Clear any faults that may have been triggered
    l9961.clear_all_faults().await.unwrap();
    l9961.enable_measurements().await.unwrap();
    let mut counter = 0;
    while counter < 100 {
        l9961.make_measurement(&mut delay).await.unwrap();
        counter += 1;
    }

    l9961.go_2_standby().await.unwrap();

    functions::exit()
}
