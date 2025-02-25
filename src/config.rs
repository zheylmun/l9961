//! # Configuration
//! The configuration module provides high-level abstractions for configuring the L9961.
//! All configuration structures include reasonable defaults for the various configuration registers,
//! with validation of the values to ensure they are within the valid range for the L9961.
//! The various configuration structs are used to set the configuration registers on the L9961.

#[cfg(feature = "ntc")]
mod ntc_thresholds;
mod voltage_thresholds;

#[cfg(feature = "ntc")]
pub use ntc_thresholds::NtcThresholds;
pub use voltage_thresholds::VoltageThresholds;

use crate::{
    registers::{Cfg1FiltersCycles, DevAddr},
    L9961,
};

/// Configuration struct for the L9961
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    /// I2C address of the l9961 device
    pub address: u8,
    /// Configuration block for cell and pack voltage thresholds
    pub voltage_thresholds: VoltageThresholds,
    /// Configuration block for NTC monitoring thresholds
    #[cfg(feature = "ntc")]
    pub ntc_thresholds: NtcThresholds,
    /// Configuration block the timing of measurements
    pub measurement_cycles: Cfg1FiltersCycles,
}

impl Config {
    /// Create a new L9961 Config with default address, voltage thresholds, NTC thresholds, and measurement cycles
    pub const fn default() -> Self {
        Self {
            address: 0x49,
            voltage_thresholds: VoltageThresholds::new(),
            #[cfg(feature = "ntc")]
            ntc_thresholds: NtcThresholds::new(),
            measurement_cycles: Cfg1FiltersCycles::default(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::default()
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

impl<I2C, I, O> L9961<I2C, I, O>
where
    I2C: embedded_hal_async::i2c::I2c,
    I: embedded_hal_async::digital::Wait,
    O: embedded_hal::digital::OutputPin,
{
    /// Apply the given configuration to the L9961
    pub async fn apply_config(&mut self) -> Result<(), I2C::Error> {
        self.write_device_address(DevAddr::from(self.config.address as u16))
            .await?;
        self.apply_voltage_threshold_configuration().await?;
        #[cfg(feature = "ntc")]
        self.apply_ntc_threshold_configuration().await?;

        Ok(())
    }
}
