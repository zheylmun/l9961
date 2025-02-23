use core::ops::Deref;

use defmt::debug_assert;

/// NTC GPIO Measurement Register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NtcGpio(u16);

impl NtcGpio {
    /// Get the measurement code of the NTC_MEAS value
    pub const fn get_ntc_meas(&self) -> u16 {
        (self.0 & 0x0FFF) as u16
    }

    /// Get the converted NTC_MEAS value in mV
    pub const fn get_ntc_meas_mv(&self) -> u16 {
        806 * self.get_ntc_meas() / 1000
    }
}

impl Deref for NtcGpio {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for NtcGpio {
    fn from(value: u16) -> Self {
        debug_assert!(value & 0x0FFF == value);
        Self(value)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for NtcGpio {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "NTC_MEAS : {} mV", self.get_ntc_meas_mv())
    }
}
