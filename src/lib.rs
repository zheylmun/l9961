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
pub use registers::*;

use conversions::*;
#[cfg(feature = "is_sync")]
use embedded_hal::i2c::I2c;
#[cfg(not(feature = "is_sync"))]
use embedded_hal_async::i2c::I2c;

/// L9961 Industrial BMS Driver
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct L9961<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> L9961<I2C>
where
    I2C: I2c,
{
    /// Create a new instance of the ST L9961 driver for the given blocking I2C bus and address.
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }

    /// Configure the cell protection thresholds
    #[maybe_async::maybe_async]
    pub async fn configure_cell_thresholds(
        &mut self,
        config: CellThresholds,
    ) -> Result<(), I2C::Error> {
        // Program the cell over-voltage threshold and counter threshold
        let vcell_ov_th = VCellOvTh::new(
            cell_voltage_code_from_mv(config.cell_over_voltage_threshold_mv),
            config.cell_over_voltage_counter_threshold.value(),
        );
        self.write_vcell_ov_th(vcell_ov_th).await?;
        // Program the cell under-voltage threshold and counter threshold
        let vcell_uv_th = VCellUvTh::new(
            cell_voltage_code_from_mv(config.cell_under_voltage_threshold_mv),
            config.cell_under_voltage_counter_threshold.value(),
        );
        self.write_vcell_uv_th(vcell_uv_th).await?;
        // Program the cell balancing under-voltage delta threshold and counter threshold
        let vcell_bal_uv_delta_th = VCellBalUvDeltaTh::new(
            cell_voltage_code_from_mv(config.cell_balancing_under_voltage_delta_threshold_mv),
            config
                .cell_balancing_under_voltage_counter_threshold
                .value(),
        );
        self.write_vcell_bal_uv_delta_th(vcell_bal_uv_delta_th)
            .await?;
        // Program the cell severe under/over-voltage threshold
        let severe_delta_thresholds = VCellSevereDeltaThrs::new(
            cell_voltage_code_from_mv(config.cell_severe_over_voltage_delta_threshold_mv),
            cell_voltage_code_from_mv(config.cell_severe_under_voltage_threshold_mv),
        );
        self.write_vcell_severe_delta_thrs(severe_delta_thresholds)
            .await?;
        Ok(())
    }
}
