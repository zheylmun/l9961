use defmt::{write, Format, Formatter};

use crate::{
    conversions::ntc_voltage_code_from_mv,
    registers::{VNTCOTTh, VNTCSevereOTTh, VNTCUTTh},
};

use super::CounterThreshold;

/// Temperature threshold configuration struct
pub struct NtcThresholds {
    /// NTC over temperature threshold in mV
    pub over_temp_threshold_mv: u16,
    /// NTC severe over temperature delta threshold in mV
    pub severe_over_temp_delta_threshold_mv: u16,
    /// NTC under temperature threshold in mV
    pub under_temp_threshold_mv: u16,
    /// NTC fault counter threshold
    pub fault_counter_threshold: CounterThreshold,
}

impl NtcThresholds {
    /// Create a new CellThresholds struct with the default values.
    pub const fn new() -> Self {
        Self {
            over_temp_threshold_mv: 0,
            severe_over_temp_delta_threshold_mv: 3300,
            under_temp_threshold_mv: 3300,
            fault_counter_threshold: CounterThreshold::default(),
        }
    }

    /// Get the ntc over-temperature register value based on this configuration
    pub(crate) fn over_temperature_configuration(&self) -> VNTCOTTh {
        let over_temp_code = ntc_voltage_code_from_mv(self.over_temp_threshold_mv);
        VNTCOTTh::new(over_temp_code, self.fault_counter_threshold.value())
    }

    /// Get the cell under-voltage register config based on this configuration
    pub(crate) fn under_temperature_configuration(&self) -> VNTCUTTh {
        let under_temp_code = ntc_voltage_code_from_mv(self.under_temp_threshold_mv);
        VNTCUTTh::new(under_temp_code, self.fault_counter_threshold.value())
    }

    /// Get the cell severe over-voltage delta register config based on this configuration
    pub(crate) fn severe_over_temp_delta_configuration(&self) -> VNTCSevereOTTh {
        let severe_over_temp_delta_threshold_code =
            ntc_voltage_code_from_mv(self.severe_over_temp_delta_threshold_mv);
        VNTCSevereOTTh::new(severe_over_temp_delta_threshold_code)
    }
}

impl Format for NtcThresholds {
    fn format(&self, f: Formatter) {
        write!(
            f,
            "Ntc Thresholds {{
over temp threshold mv: {},
severe over temp delta threshold mv: {},
under under temp threshold mv: {},
fault counter threshold: {},
}}",
            self.over_temp_threshold_mv,
            self.severe_over_temp_delta_threshold_mv,
            self.under_temp_threshold_mv,
            self.fault_counter_threshold.value(),
        )
    }
}
