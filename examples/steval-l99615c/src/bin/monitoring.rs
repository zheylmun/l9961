#![no_main]
#![no_std]
#![deny(warnings, unsafe_code)]

use embassy_executor::Spawner;
use embassy_time::Delay;
use l9961::{
    config::{CounterThreshold, VoltageThresholds},
    registers::{Cfg2Enables, FetConfig},
    Config,
};
use steval_l99615c::{configure_l9961_peripherals, exit};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let peripherals = embassy_stm32::init(Default::default());

    let config = Config {
        // Configure the voltage monitoring with extreme thresholds to avoid faults triggering
        voltage_thresholds: VoltageThresholds {
            cell_over_voltage_threshold_mv: 4200,
            cell_severe_over_voltage_delta_threshold_mv: 100,
            cell_under_voltage_threshold_mv: 2900,
            cell_severe_under_voltage_delta_threshold_mv: 400,
            cell_balancing_under_voltage_delta_threshold_mv: 600,
            pack_over_voltage_threshold_mv: 20750,
            pack_under_voltage_threshold_mv: 15000,
            max_pack_cell_sum_delta_mv: 500,
            fault_counter_threshold: CounterThreshold::default(),
        },
        ..Default::default()
    };

    let mut l9961 = configure_l9961_peripherals(peripherals, config);

    let mut delay = Delay;
    l9961.wake_if_asleep(&mut delay).await;

    l9961.apply_config().await.unwrap();
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
        true,                // Enable temperature measurement
        true,                // Enable current measurement
        true,                // Enable coulomb counter
        false,               // Disable over-current detection
        false,               // Disable short-circuit detection
        FetConfig::HighSide, // Set the discharge FET configuration to high-side
        FetConfig::HighSide, // Set the charge FET configuration to high-side
        false,               // Disable data CRC
    );
    l9961.write_cfg2_enables(enables).await.unwrap();

    // Clear any faults that may have been triggered
    l9961.clear_all_faults().await.unwrap();
    // Start the measurement loop
    l9961.enable_measurements().await.unwrap();
    let mut counter = 0;
    while counter < 100 {
        let measurement = l9961.make_measurement(&mut delay).await.unwrap();
        defmt::println!("{}", measurement);
        counter += 1;
    }

    l9961.go_2_standby().await.unwrap();

    exit()
}
