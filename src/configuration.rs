//! # Configuration
//! The configuration module provides high-level abstractions for configuring the L9961.
//! All configuration structures include reasonable defaults for the various configuration registers,
//! with validation of the values to ensure they are within the valid range for the L9961.
//! The various configuration structs are used to set the configuration registers on the L9961.

mod ntc_thresholds;
mod voltage_thresholds;

pub use ntc_thresholds::NtcThresholds;
pub use voltage_thresholds::VoltageThresholds;

use crate::L9961;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::{digital::Wait, i2c::I2c};

impl<I2C, I, O, const CELL_COUNT: u8> L9961<I2C, I, O, CELL_COUNT>
where
    I2C: I2c,
    I: Wait,
    O: OutputPin,
{
    /// Configure the cell voltage thresholds
    /// Pack level thresholds will be set automatically based on the per-cell thresholds and the number of cells.
    pub async fn configure_voltage_thresholds(
        &mut self,
        config: VoltageThresholds,
    ) -> Result<(), I2C::Error> {
        // Program the cell over-voltage threshold and counter threshold register
        self.write_vcell_ov_th(config.cell_over_voltage_configuration())
            .await?;
        // Program the cell under-voltage threshold and counter threshold register
        self.write_vcell_uv_th(config.cell_under_voltage_configuration())
            .await?;
        // Program the cell balancing under-voltage delta threshold and counter threshold register
        self.write_vcell_bal_uv_delta_th(config.cell_balancing_under_voltage_delta_configuration())
            .await?;
        // Program the cell severe under/over-voltage threshold register
        self.write_vcell_severe_delta_thrs(
            config.cell_severe_voltage_threshold_delta_configuration(),
        )
        .await?;
        // Program the pack over voltage threshold register
        self.write_vb_ov_th(config.pack_over_voltage_threshold())
            .await?;
        // Program the pack under voltage threshold register
        self.write_vb_uv_th(config.pack_under_voltage_threshold())
            .await?;
        // Program the pack vs cell sum plausibility check register
        self.write_vb_sum_max_diff_th(config.pack_vs_cell_sum_delta_threshold())
            .await?;
        Ok(())
    }

    /// Configure the NTC thresholds
    pub async fn configure_ntc_thresholds(
        &mut self,
        thresholds: NtcThresholds,
    ) -> Result<(), I2C::Error> {
        self.write_vntc_ot_th(thresholds.over_temperature_configuration())
            .await?;
        self.write_vntc_ut_th(thresholds.under_temperature_configuration())
            .await?;
        self.write_vntc_severe_ot_th(thresholds.severe_over_temp_delta_configuration())
            .await?;
        Ok(())
    }
}

/// Newtype for the counter threshold value to ensure a valid range.
/// The counter threshold is a 4-bit value used to determine how many times a fault condition must occur before the fault is triggered.
/// Default value is 10.
pub struct CounterThreshold(u8);

impl CounterThreshold {
    /// Create a new CounterThreshold with the given value
    pub const fn new(value: u8) -> Self {
        debug_assert!(value < 16);
        CounterThreshold(value)
    }

    /// Create a new CounterThreshold with the default value of 10
    pub const fn default() -> Self {
        Self(10)
    }

    /// Get the internal value
    pub(crate) const fn value(&self) -> u8 {
        self.0
    }
}
