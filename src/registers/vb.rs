use core::ops::Deref;

use defmt::debug_assert;

use crate::conversions::pack_voltage_measurement_mv_from_code;

/// Battery Pack Voltage Measurement Register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VB(u16);

impl VB {
    /// Get the measurement code of the battery pack
    pub const fn get_vb_meas_code(&self) -> u16 {
        (self.0 & 0x7FFF) as u16
    }
}

impl Deref for VB {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for VB {
    fn from(value: u16) -> Self {
        debug_assert!(value & 0x0FFF == value);
        Self(value)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for VB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VB : {}mv",
            pack_voltage_measurement_mv_from_code(self.get_vb_meas_code())
        )
    }
}
