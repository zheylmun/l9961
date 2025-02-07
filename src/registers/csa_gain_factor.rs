use core::ops::Deref;

/// Current sense ADC gain factor configuration register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CsaGainFactor(u16);

impl Deref for CsaGainFactor {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for CsaGainFactor {
    fn from(id: u16) -> Self {
        Self(id)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for CsaGainFactor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CSA Gain Factor : {:#04x}", self.0,)
    }
}
