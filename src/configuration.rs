//! # Configuration
//! The configuration module provides high-level abstractions for configuring the L9961.
//! All configuration structures include reasonable defaults for the various configuration registers,
//! with validation of the values to ensure they are within the valid range for the L9961.
//! The various configuration structs are used to set the configuration registers on the L9961.

/// Cell threshold configuration struct
pub struct CellThresholds {
    pub(crate) cell_over_voltage_threshold_mv: u16,
    pub(crate) cell_over_voltage_counter_threshold: CounterThreshold,
    // TODO: Sample code and documentation disagree here.  Test with simulator cells to determine correct value.
    pub(crate) cell_severe_over_voltage_delta_threshold_mv: u16,
    pub(crate) cell_under_voltage_threshold_mv: u16,
    pub(crate) cell_severe_under_voltage_threshold_mv: u16,
    pub(crate) cell_under_voltage_counter_threshold: CounterThreshold,
    pub(crate) cell_balancing_under_voltage_delta_threshold_mv: u16,
    pub(crate) cell_balancing_under_voltage_counter_threshold: CounterThreshold,
}

impl CellThresholds {
    /// Create a new CellThresholds struct with the default values.
    pub const fn new() -> Self {
        CellThresholds {
            cell_over_voltage_threshold_mv: 4200,
            cell_severe_over_voltage_delta_threshold_mv: 4400,
            cell_over_voltage_counter_threshold: CounterThreshold::default(),
            cell_under_voltage_threshold_mv: 3000,
            cell_severe_under_voltage_threshold_mv: 2800,
            cell_under_voltage_counter_threshold: CounterThreshold::default(),
            cell_balancing_under_voltage_delta_threshold_mv: 3200,
            cell_balancing_under_voltage_counter_threshold: CounterThreshold::default(),
        }
    }

    /// Set the cell over-voltage threshold in mV
    pub const fn with_cell_over_voltage_threshold_mv(mut self, voltage_mv: u16) -> Self {
        self.cell_over_voltage_threshold_mv = voltage_mv;
        self
    }

    /// Set the cell severe over-voltage threshold in mV
    pub const fn with_cell_severe_over_voltage_threshold_mv(mut self, voltage_mv: u16) -> Self {
        self.cell_severe_over_voltage_delta_threshold_mv = voltage_mv;
        self
    }

    /// Set the cell over-voltage counter threshold
    pub const fn with_cell_over_voltage_counter_threshold(mut self, threshold: u8) -> Self {
        let threshold = CounterThreshold::new(threshold);
        self.cell_over_voltage_counter_threshold = threshold;
        self
    }

    /// Set the cell under-voltage threshold in mV
    pub const fn with_cell_under_voltage_threshold_mv(mut self, voltage_mv: u16) -> Self {
        self.cell_under_voltage_threshold_mv = voltage_mv;
        self
    }

    /// Set the cell severe under-voltage threshold in mV
    pub const fn with_cell_severe_under_voltage_threshold_mv(mut self, voltage_mv: u16) -> Self {
        self.cell_severe_under_voltage_threshold_mv = voltage_mv;
        self
    }

    /// Set the cell under-voltage counter threshold
    pub const fn with_cell_under_voltage_counter_threshold(mut self, threshold: u8) -> Self {
        let threshold = CounterThreshold::new(threshold);
        self.cell_under_voltage_counter_threshold = threshold;
        self
    }

    /// Set the cell balancing under-voltage threshold in mV
    pub const fn with_cell_balancing_under_voltage_threshold_mv(mut self, voltage_mv: u16) -> Self {
        self.cell_balancing_under_voltage_delta_threshold_mv = voltage_mv;
        self
    }

    /// Set the cell balancing under-voltage counter threshold
    pub const fn with_cell_balancing_under_voltage_counter_threshold(
        mut self,
        threshold: u8,
    ) -> Self {
        let threshold = CounterThreshold::new(threshold);
        self.cell_balancing_under_voltage_counter_threshold = threshold;
        self
    }
}

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
