use core::ops::Deref;

use defmt::debug_assert;

/// VB Measurement Register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VB(u16);

impl VB {
    /// Get the measurement code of the battery pack
    #[inline]
    pub const fn get_vb_meas(&self) -> u16 {
        (self.0 & 0x7FFF) as u16
    }

    /// Get the voltage measurement of the pack in mV
    #[inline]
    pub const fn get_vb_meas_mv(&self) -> u16 {
        61 * self.get_vb_meas() / 10
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
        defmt::write!(f, "VB : {}mv,", self.get_vb_meas_mv())
    }
}
