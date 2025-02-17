use core::ops::Deref;

use defmt::debug_assert;

/// NTC GPIO Measurement Register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NtcGpio(u16);

impl NtcGpio {
    /// Get the measurement code of the NTC_MEAS value
    #[inline]
    pub const fn get_ntc_meas(&self) -> u16 {
        (self.0 & 0x7FFF) as u16
    }

    /// Get the voltage measurement of the pack in mV
    #[inline]
    pub const fn get_ntc_meas_unspecified(&self) -> u16 {
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
        defmt::write!(
            f,
            "NTC_MEAD : {} godknowswhatunits,",
            self.get_ntc_meas_unspecified()
        )
    }
}
