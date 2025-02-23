use core::ops::Deref;

/// NTC severe over temp monitoring threshold configuration register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VNTCSevereOTTh(u16);

impl VNTCSevereOTTh {
    /// Create a new NTC Severe Over Temperature Delta Threshold struct
    pub const fn new(ntc_severe_ot_th: u16) -> Self {
        debug_assert!(
            ntc_severe_ot_th & 0x0FFF == ntc_severe_ot_th,
            "Invalid ntc severe over-temp threshold value"
        );
        Self(ntc_severe_ot_th)
    }

    /// Get the programmable severe over temp delta threshold (12bit)
    pub const fn get_ntc_severe_ot_th(&self) -> u16 {
        (self.0 & 0x0FFF) as u16
    }

    /// Set the the programmable severe over temp delta threshold (12bit)
    pub const fn set_ntc_severe_ot_th(&mut self, ntc_ot_th: u16) {
        debug_assert!(
            ntc_ot_th & 0x0FFF == ntc_ot_th,
            "Invalid ntc severe over-temp threshold value"
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
        defmt::write!(f, "VNTC_SEVERE_OT_TH: {}", self.get_ntc_severe_ot_th(),)
    }
}
