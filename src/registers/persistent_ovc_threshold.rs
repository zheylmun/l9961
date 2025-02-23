use core::ops::Deref;

/// Overcurrent monitoring persistent covercurrent threshold configuration register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PersistentOvCThreshold(u16);

impl PersistentOvCThreshold {
    /// Get the programmable persistent overcurrent fault threshold (8bit)
    pub const fn get_persistent_ovc_th(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    /// Set the programmable persistent overcurrent fault threshold (8bit)
    pub const fn set_persistent_ovc_th(&mut self, peristent_ovc_th: u8) {
        self.0 = self.0 & 0xFF00 | (peristent_ovc_th as u16);
    }
}

impl Deref for PersistentOvCThreshold {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for PersistentOvCThreshold {
    fn from(id: u16) -> Self {
        debug_assert!(id & 0xFF00 == 0, "Invalid OvCThresholds value");
        Self(id)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for PersistentOvCThreshold {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PERSISTENT_OVC_THRESHOLD: {}",
            self.get_persistent_ovc_th(),
        )
    }
}
