use core::ops::Deref;

use defmt::debug_assert;

/// Die temp  Measurement Register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DieTemp(u16);

impl DieTemp {
    /// Get the measurement code of the DIE_TEMP_MEAS value
    pub const fn get_die_temp(&self) -> u16 {
        (self.0 & 0x0FFF) as u16
    }

    /// Get the voltage measurement of the pack in mV
    pub const fn get_die_temp_kelvin(&self) -> u16 {
        ((343165u32 - 196u32 * self.get_die_temp() as u32) / 1000u32) as u16
    }
}

impl Deref for DieTemp {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for DieTemp {
    fn from(value: u16) -> Self {
        debug_assert!(value & 0x0FFF == value);
        Self(value)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for DieTemp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DIE_TEMP : {} k,", self.get_die_temp_kelvin())
    }
}
