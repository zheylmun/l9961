use core::ops::Deref;

use defmt::debug_assert;

/// VCellSum Measurement Register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VCellSum(u16);

impl VCellSum {
    /// Get the sum of cell voltages measurement code
    pub const fn get_vcellsum_meas(&self) -> u16 {
        (self.0 & 0x7FFF) as u16
    }

    /// Get the sum of cell voltages measurement in mV
    pub const fn get_vcellsum_meas_mv(&self) -> u16 {
        122 * self.get_vcellsum_meas() / 100
    }
}

impl Deref for VCellSum {
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
            "VCELLSUM: {{\n  VCELLSUM : {}mv,\n}}",
            self.get_vcellsum_meas_mv()
        )
    }
}
