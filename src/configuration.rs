//! # Configuration
//! The configuration module provides high-level abstractions for configuring the L9961.
//! All configuration structures include reasonable defaults for the various configuration registers,
//! with validation of the values to ensure they are within the valid range for the L9961.
//! The various configuration structs are used to set the configuration registers on the L9961.

mod cell_thresholds;

pub use cell_thresholds::CellThresholds;

/// Newtype for the counter threshold value to ensure a valid range.
/// The counter threshold is a 4-bit value used to determine how many times a fault condition must occur before the fault is triggered.
/// Default value is 10.
pub(crate) struct CounterThreshold(u8);

impl CounterThreshold {
    #[inline]
    const fn new(value: u8) -> Self {
        debug_assert!(value < 16);
        CounterThreshold(value)
    }
    #[inline]
    pub const fn default() -> Self {
        Self(10)
    }

    #[inline]
    pub(crate) const fn value(&self) -> u8 {
        self.0
    }
}
