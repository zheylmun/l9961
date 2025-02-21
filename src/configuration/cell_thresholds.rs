use defmt::{write, Format, Formatter};

use crate::conversions::round_trip_cell_voltage;

use super::CounterThreshold;

/// Cell threshold configuration struct
pub struct CellThresholds {
    pub(crate) cell_over_voltage_threshold_mv: u16,
    // TODO: Sample code and documentation disagree here.  Test with simulator cells to determine correct value.
    pub(crate) cell_severe_over_voltage_delta_threshold_mv: u16,
    pub(crate) cell_under_voltage_threshold_mv: u16,
    pub(crate) cell_severe_under_voltage_threshold_mv: u16,
    pub(crate) cell_balancing_under_voltage_delta_threshold_mv: u16,
    pub(crate) fault_counter_threshold: CounterThreshold,
    pub(crate) max_pack_cell_sum_delta_mv: u16,
}

impl CellThresholds {
    /// Create a new CellThresholds struct with the default values.
    pub const fn new() -> Self {
        CellThresholds {
            cell_over_voltage_threshold_mv: 4196,
            cell_severe_over_voltage_delta_threshold_mv: 4392,
            cell_under_voltage_threshold_mv: 2986,
            cell_severe_under_voltage_threshold_mv: 2693,
            cell_balancing_under_voltage_delta_threshold_mv: 3181,
            fault_counter_threshold: CounterThreshold::default(),
            max_pack_cell_sum_delta_mv: 995,
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

    /// Set the cell fault counter thresholds
    /// The fault counter threshold is a 4-bit value (0-15) used to determine how many times a fault condition must be measured before the fault is triggered.
    /// The default value is 10.
    pub const fn with_fault_counter_threshold(mut self, threshold: u8) -> Self {
        let threshold = CounterThreshold::new(threshold);
        self.fault_counter_threshold = threshold;
        self
    }

    /// Set the allowed delta between the sum of the cell voltages and the pack voltage
    pub const fn with_max_pack_cell_sum_delta_mv(mut self, delta_mv: u16) -> Self {
        self.max_pack_cell_sum_delta_mv = delta_mv;
        self
    }
}

impl Format for CellThresholds {
    fn format(&self, f: Formatter) {
        write!(
            f,
            "CellThresholds {{
    cell_over_voltage_threshold_mv: {},
    cell_severe_over_voltage_delta_threshold_mv: {},
    cell_under_voltage_threshold_mv: {},
    cell_severe_under_voltage_threshold_mv: {},
    cell_balancing_under_voltage_delta_threshold_mv: {},
    fault_counter_threshold: {},
    max_pack_cell_sum_delta_mv: {},
}}",
            round_trip_cell_voltage(self.cell_over_voltage_threshold_mv),
            round_trip_cell_voltage(self.cell_severe_over_voltage_delta_threshold_mv),
            round_trip_cell_voltage(self.cell_under_voltage_threshold_mv),
            round_trip_cell_voltage(self.cell_severe_under_voltage_threshold_mv),
            round_trip_cell_voltage(self.cell_balancing_under_voltage_delta_threshold_mv),
            self.fault_counter_threshold.value(),
            round_trip_cell_voltage(self.max_pack_cell_sum_delta_mv)
        )
    }
}
