use crate::{
    config::CounterThreshold,
    conversions::ntc_voltage_code_from_mv,
    registers::{VNTCOTTh, VNTCSevereOTTh, VNTCUTTh},
    L9961,
};

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

impl defmt::Format for NtcThresholds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
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
impl<I2C, I, O, const CELL_COUNT: u8> L9961<I2C, I, O, CELL_COUNT>
where
    I2C: embedded_hal_async::i2c::I2c,
    I: embedded_hal_async::digital::Wait,
    O: embedded_hal::digital::OutputPin,
{
    /// Configure the NTC thresholds
    pub async fn apply_ntc_threshold_configuration(&mut self) -> Result<(), I2C::Error> {
        self.write_vntc_ot_th(self.config.ntc_thresholds.over_temperature_configuration())
            .await?;
        self.write_vntc_ut_th(self.config.ntc_thresholds.under_temperature_configuration())
            .await?;
        self.write_vntc_severe_ot_th(
            self.config
                .ntc_thresholds
                .severe_over_temp_delta_configuration(),
        )
        .await?;
        Ok(())
    }
}
