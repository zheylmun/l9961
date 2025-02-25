use crate::conversions::cell_voltage_measurement_mv_from_code;

/// VCellSum Measurement Register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VCellSum(u16);

impl VCellSum {
    /// Get the sum of cell voltages measurement code
    pub const fn get_vcellsum_meas(&self) -> u16 {
        (self.0 & 0x7FFF) as u16
    }
}

impl core::ops::Deref for VCellSum {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for VCellSum {
    fn from(value: u16) -> Self {
        debug_assert!(value & !0x8000 == value);
        Self(value)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for VCellSum {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VCELLSUM:  {}mv",
            cell_voltage_measurement_mv_from_code(self.get_vcellsum_meas())
        )
    }
}
