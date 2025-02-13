use core::ops::Deref;
use defmt::{write, Format, Formatter};

/// Programmable short circuit protection threshold register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCThreshold(u16);

impl SCThreshold {
    /// Get the programmable short circuit threshold (4 bit)
    #[inline]
    pub const fn get_sc_th(&self) -> u8 {
        (self.0 & 0x000F) as u8
    }

    /// Set the the programmable short circuit threshold (4 bit)
    #[inline]
    pub const fn set_sc_th(&mut self, sc_th: u8) {
        debug_assert!(sc_th & 0x000F == sc_th, "Invalid SC_TH value");
        self.0 = self.0 & 0xFFF0 | (sc_th as u16);
    }

    /// Get the programmable persistent short circuit threshold (4 bit)
    #[inline]
    pub const fn get_sc_persist_th(&self) -> u8 {
        (self.0 & 0x00F0 >> 4) as u8
    }

    /// Set the the programmable persistent short circuit threshold (4 bit)
    #[inline]
    pub const fn set_sc_persist_th(&mut self, sc_th: u8) {
        debug_assert!(sc_th & 0x000F == sc_th, "Invalid SC_PERSIST_TH value");
        self.0 = self.0 & 0xFF0F | ((sc_th as u16) << 4);
    }
}

impl Deref for SCThreshold {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for SCThreshold {
    fn from(sc_th: u16) -> Self {
        debug_assert!(sc_th & 0x000F == sc_th, "Invalid SC_TH value");
        Self(sc_th)
    }
}

#[cfg(feature = "defmt")]
impl Format for SCThreshold {
    fn format(&self, f: Formatter) {
        write!(
            f,
            "SC_THRESHOLD: {{\n  SC_TH: {},\n  SC_PERSIST_TH: {}\n}}",
            self.get_sc_th(),
            self.get_sc_persist_th(),
        )
    }
}
