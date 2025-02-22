use core::ops::Deref;

/// Overcurrent monitoring threshold configuration register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OvCThresholds(u16);

impl OvCThresholds {
    /// Get the programmable discharge overcurrent fault threshold (8bit)
    pub const fn get_ovc_dchg_th(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    /// Set the programmable discharge overcurrent fault threshold (8bit)
    pub const fn set_ovc_dchg_th(&mut self, ovc_dchg_th: u8) {
        self.0 = self.0 & 0xFF00 | (ovc_dchg_th as u16);
    }

    /// Get the programmable charge overcurrent fault threshold (8bit)
    pub const fn get_ovc_chg_th(&self) -> u8 {
        ((self.0 & 0xFF00) >> 8) as u8
    }

    /// Set the programmable charge overcurrent fault threshold (8bit)
    pub const fn set_ovc_chg_th(&mut self, ovc_chg_th: u8) {
        self.0 = self.0 & 0x00FF | ((ovc_chg_th as u16) << 8);
    }
}

impl Deref for OvCThresholds {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for OvCThresholds {
    fn from(id: u16) -> Self {
        debug_assert!(id & 0xF000 == 0, "Invalid OvCThresholds value");
        OvCThresholds(id)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for OvCThresholds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OVC_THRESHOLDS: {{\n  OVC_DCHG_TH: {},\n  OVC_CHG_TH: {}\n}}",
            self.get_ovc_dchg_th(),
            self.get_ovc_chg_th()
        )
    }
}
