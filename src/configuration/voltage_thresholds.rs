use defmt::{write, Format, Formatter};

use crate::{
    conversions::{
        cell_voltage_code_from_mv, pack_voltage_code_from_mv, round_trip_cell_voltage,
        round_trip_pack_voltage,
    },
    registers::{
        VBOvTh, VBSumMaxDiffTh, VBUvTh, VCellBalUvDeltaTh, VCellOvTh, VCellSevereDeltaThrs,
        VCellUvTh,
    },
};

use super::CounterThreshold;

/// Cell threshold configuration struct
pub struct VoltageThresholds {
    pub(crate) cell_over_voltage_threshold_mv: u16,
    // TODO: Sample code and documentation disagree here.  Test with simulator cells to determine correct value.
    pub(crate) cell_severe_over_voltage_delta_threshold_mv: u16,
    pub(crate) cell_under_voltage_threshold_mv: u16,
    pub(crate) cell_severe_under_voltage_delta_threshold_mv: u16,
    pub(crate) cell_balancing_under_voltage_delta_threshold_mv: u16,
    pub(crate) fault_counter_threshold: CounterThreshold,
    pub(crate) max_pack_cell_sum_delta_mv: u16,
    pub(crate) pack_over_voltage_threshold_mv: u16,
    pub(crate) pack_under_voltage_threshold_mv: u16,
}

impl VoltageThresholds {
    /// Create a new CellThresholds struct with the default values.
    pub const fn new() -> Self {
        VoltageThresholds {
            cell_over_voltage_threshold_mv: 4196,
            cell_severe_over_voltage_delta_threshold_mv: 4392,
            cell_under_voltage_threshold_mv: 2986,
            cell_severe_under_voltage_delta_threshold_mv: 2693,
            cell_balancing_under_voltage_delta_threshold_mv: 3181,
            fault_counter_threshold: CounterThreshold::default(),
            max_pack_cell_sum_delta_mv: 995,
            pack_over_voltage_threshold_mv: 21000,
            pack_under_voltage_threshold_mv: 15000,
        }
    }

    /// Get the cell over-voltage register config based on this configuration
    pub(crate) fn cell_over_voltage_configuration(&self) -> VCellOvTh {
        let cell_over_voltage_code = cell_voltage_code_from_mv(self.cell_over_voltage_threshold_mv);
        VCellOvTh::new(cell_over_voltage_code, self.fault_counter_threshold.value())
    }

    /// Get the cell under-voltage register config based on this configuration
    pub(crate) fn cell_under_voltage_configuration(&self) -> VCellUvTh {
        let cell_under_voltage_code =
            cell_voltage_code_from_mv(self.cell_under_voltage_threshold_mv);
        VCellUvTh::new(
            cell_under_voltage_code,
            self.fault_counter_threshold.value(),
        )
    }

    /// Get the cell balancing under-voltage delta register config based on this configuration
    pub(crate) fn cell_balancing_under_voltage_delta_configuration(&self) -> VCellBalUvDeltaTh {
        let cell_balancing_under_voltage_delta_code =
            cell_voltage_code_from_mv(self.cell_balancing_under_voltage_delta_threshold_mv);
        VCellBalUvDeltaTh::new(
            cell_balancing_under_voltage_delta_code,
            self.fault_counter_threshold.value(),
        )
    }

    /// Get the cell severe over-voltage delta register config based on this configuration
    pub(crate) fn cell_severe_voltage_threshold_delta_configuration(&self) -> VCellSevereDeltaThrs {
        let cell_severe_over_voltage_delta_code =
            cell_voltage_code_from_mv(self.cell_severe_over_voltage_delta_threshold_mv);
        let cell_severe_under_voltage_delta_code =
            cell_voltage_code_from_mv(self.cell_severe_under_voltage_delta_threshold_mv);
        VCellSevereDeltaThrs::new(
            cell_severe_over_voltage_delta_code,
            cell_severe_under_voltage_delta_code,
        )
    }

    /// Get the pack over-voltage threshold register config based on this configuration
    pub(crate) fn pack_over_voltage_threshold(&self) -> VBOvTh {
        let over_voltage_code = pack_voltage_code_from_mv(self.pack_over_voltage_threshold_mv);
        VBOvTh::new(over_voltage_code, self.fault_counter_threshold.value())
    }

    /// Get the pack under-voltage threshold register config based on this configuration
    pub(crate) fn pack_under_voltage_threshold(&self) -> VBUvTh {
        let under_voltage_code = pack_voltage_code_from_mv(self.pack_under_voltage_threshold_mv);
        VBUvTh::new(under_voltage_code, self.fault_counter_threshold.value())
    }

    /// Get the pack vs cell sum voltage delta threshold register config based on this configuration
    pub(crate) fn pack_vs_cell_sum_delta_threshold(&self) -> VBSumMaxDiffTh {
        VBSumMaxDiffTh::from(pack_voltage_code_from_mv(self.max_pack_cell_sum_delta_mv) as u16)
    }
}

impl Format for VoltageThresholds {
    fn format(&self, f: Formatter) {
        write!(
            f,
            "CellThresholds {{
    cell over voltage threshold mv: {},
    cell severe over voltage delta threshold mv: {},
    cell under voltage threshold mv: {},
    cell severe under voltage threshold mv: {},
    cell balancing under voltage delta threshold mv: {},
    fault counter threshold: {},
    max pack cell sum delta mv: {},
    pack over voltage threshold mv: {}
    pack under voltage threshold mv: {}
}}",
            round_trip_cell_voltage(self.cell_over_voltage_threshold_mv),
            round_trip_cell_voltage(self.cell_severe_over_voltage_delta_threshold_mv),
            round_trip_cell_voltage(self.cell_under_voltage_threshold_mv),
            round_trip_cell_voltage(self.cell_severe_under_voltage_delta_threshold_mv),
            round_trip_cell_voltage(self.cell_balancing_under_voltage_delta_threshold_mv),
            self.fault_counter_threshold.value(),
            round_trip_pack_voltage(self.max_pack_cell_sum_delta_mv),
            round_trip_pack_voltage(self.pack_over_voltage_threshold_mv),
            round_trip_pack_voltage(self.pack_under_voltage_threshold_mv)
        )
    }
}
