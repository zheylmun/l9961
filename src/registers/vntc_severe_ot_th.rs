use core::ops::Deref;

/// NTC severe over temp monitoring threshold configuration register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VNTCSevereOTTh(u16);

impl VNTCSevereOTTh {
    /// Get the programmable severe over temp fault threshold (12bit)
    #[inline]
    pub const fn get_ntc_severe_ot_th(&self) -> u16 {
        (self.0 & 0x0FFF) as u16
    }

    /// Set the the programmable severe over temp fault threshold (12bit)
    #[inline]
    pub const fn set_ntc_severe_ot_th(&mut self, ntc_ot_th: u16) {
        debug_assert!(
            ntc_ot_th & 0x0FFF == ntc_ot_th,
            "Invalid ntc severe overtemp threshold value"
        );
        self.0 = self.0 & 0xF000 | (ntc_ot_th as u16);
    }
}

impl Deref for VNTCSevereOTTh {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for VNTCSevereOTTh {
    fn from(id: u16) -> Self {
        VNTCSevereOTTh(id)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for VNTCSevereOTTh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VNTC_SEVERE_OT_TH: {{\n  NTC_SEVERE_OT_TH: {},\n}}",
            self.get_ntc_severe_ot_th(),
        )
    }
}
