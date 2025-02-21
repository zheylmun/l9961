//! # L9961 Industrial BMS Driver
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

pub mod commands;
pub mod configuration;
pub mod conversions;
pub mod registers;

pub use configuration::*;
use defmt::info;
pub use registers::*;

use conversions::*;
#[cfg(feature = "is_sync")]
use embedded_hal::i2c::I2c;
#[cfg(not(feature = "is_sync"))]
use embedded_hal_async::i2c::I2c;

/// L9961 Industrial BMS Driver
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct L9961<I2C, const CELL_COUNT: u8 = 3> {
    i2c: I2C,
    address: u8,
}

impl<I2C, const CELL_COUNT: u8> L9961<I2C, CELL_COUNT>
where
    I2C: I2c,
{
    /// Create a new instance of the ST L9961 driver for the given blocking I2C bus and address.
    pub fn new(i2c: I2C, address: u8) -> Self {
        debug_assert!(CELL_COUNT >= 3 && CELL_COUNT <= 5);
        Self { i2c, address }
    }

    /// Configure the cell voltage thresholds
    /// Pack level thresholds will be set automatically based on the per-cell thresholds and the number of cells.
    #[maybe_async::maybe_async]
    pub async fn configure_voltage_thresholds(
        &mut self,
        config: CellThresholds,
    ) -> Result<(), I2C::Error> {
        info!("Applying cell voltage thresholds:\n{}", config);

        let cell_over_voltage_code =
            cell_voltage_code_from_mv(config.cell_over_voltage_threshold_mv);
        let cell_severe_over_voltage_code =
            cell_voltage_code_from_mv(config.cell_severe_over_voltage_delta_threshold_mv);
        let cell_under_voltage_code =
            cell_voltage_code_from_mv(config.cell_under_voltage_threshold_mv);
        let cell_severe_under_voltage_code =
            cell_voltage_code_from_mv(config.cell_severe_under_voltage_threshold_mv);
        let cell_balancing_under_voltage_delta_code =
            cell_voltage_code_from_mv(config.cell_balancing_under_voltage_delta_threshold_mv);
        let _max_pack_cell_sum_delta_mv = config.max_pack_cell_sum_delta_mv;

        // Program the cell over-voltage threshold and counter threshold
        let vcell_ov_th = VCellOvTh::new(
            cell_over_voltage_code,
            config.fault_counter_threshold.value(),
        );
        self.write_vcell_ov_th(vcell_ov_th).await?;

        // Program the cell under-voltage threshold and counter threshold
        let vcell_uv_th = VCellUvTh::new(
            cell_under_voltage_code,
            config.fault_counter_threshold.value(),
        );
        self.write_vcell_uv_th(vcell_uv_th).await?;

        // Program the cell balancing under-voltage delta threshold and counter threshold
        let vcell_bal_uv_delta_th = VCellBalUvDeltaTh::new(
            cell_balancing_under_voltage_delta_code,
            config.fault_counter_threshold.value(),
        );
        self.write_vcell_bal_uv_delta_th(vcell_bal_uv_delta_th)
            .await?;

        // Program the cell severe under/over-voltage threshold
        let severe_delta_thresholds = VCellSevereDeltaThrs::new(
            cell_severe_over_voltage_code,
            cell_severe_under_voltage_code,
        );
        self.write_vcell_severe_delta_thrs(severe_delta_thresholds)
            .await?;

        Ok(())
    }
}
