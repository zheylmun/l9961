use core::ops::Deref;

/// NTC over temp monitoring threshold configuration register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VNTCOTTh(u16);

impl VNTCOTTh {
    /// Create a new VNTCOTTh struct
    pub const fn new(ntc_ot_th: u16, nntc_ot_cnt_th: u8) -> Self {
        debug_assert!(
            ntc_ot_th & 0x0FFF == ntc_ot_th,
            "Invalid ntc overtemp threshold value"
        );
        debug_assert!(
            nntc_ot_cnt_th & 0x0F == nntc_ot_cnt_th,
            "Invalid ntc overtemp counter threshold value"
        );
        Self((ntc_ot_th & 0x0FFF) | ((nntc_ot_cnt_th as u16) << 12))
    }

    /// Get the programmable over temp fault threshold (12bit)
    pub const fn get_ntc_ot_th(&self) -> u16 {
        (self.0 & 0x0FFF) as u16
    }

    /// Set the the programmable over temp fault threshold (12bit)
    pub const fn set_ntc_ot_th(&mut self, ntc_ot_th: u16) {
        debug_assert!(
            ntc_ot_th & 0x0FFF == ntc_ot_th,
            "Invalid ntc overtemp threshold value"
        );
        self.0 = self.0 & 0xF000 | (ntc_ot_th as u16);
    }

    /// Get the programmable over temp event counter threshold (4 bit)
    pub const fn get_nntc_ot_cnt_th(&self) -> u8 {
        ((self.0 & 0x0F00) >> 12) as u8
    }

    /// Set the programmable over temp event counter threshold (4 bit)
    pub const fn set_nntc_ot_cnt_th(&mut self, nntc_ot_cnt_th: u8) {
        self.0 = self.0 & 0x0FFF | ((nntc_ot_cnt_th as u16) << 12);
    }
}

impl Deref for VNTCOTTh {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for VNTCOTTh {
    fn from(id: u16) -> Self {
        VNTCOTTh(id)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for VNTCOTTh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VNTC_OT_TH: {{\n  NTC_OT_TH: {},\n  NNTC_OT_CNT_TH: {}\n}}",
            self.get_ntc_ot_th(),
            self.get_nntc_ot_cnt_th()
        )
    }
}
