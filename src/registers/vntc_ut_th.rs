use core::ops::Deref;

/// NTC under temp monitoring threshold configuration register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VNTCUTTh(u16);

impl VNTCUTTh {
    /// Create a new VNTCUTTh struct
    pub const fn new(threshold: u16, fault_count: u8) -> Self {
        debug_assert!(
            threshold & 0x0FFF == threshold,
            "Invalid ntc under temp threshold value"
        );
        debug_assert!(
            fault_count & 0x0F == fault_count,
            "Invalid ntc under temp counter threshold value"
        );
        Self((threshold & 0x0FFF) | ((fault_count as u16) << 12))
    }
    /// Get the programmable under temp fault threshold (12bit)
    pub const fn get_ntc_ut_th(&self) -> u16 {
        (self.0 & 0x0FFF) as u16
    }

    /// Set the the programmable under temp fault threshold (12bit)
    pub const fn set_ntc_ut_th(&mut self, ntc_ut_th: u16) {
        debug_assert!(
            ntc_ut_th & 0x0FFF == ntc_ut_th,
            "Invalid ntc under temp threshold value"
        );
        self.0 = self.0 & 0xF000 | (ntc_ut_th as u16);
    }

    /// Get the programmable under temp event counter threshold (4 bit)
    pub const fn get_nntc_ut_cnt_th(&self) -> u8 {
        ((self.0 & 0x0F00) >> 12) as u8
    }

    /// Set the programmable under temp event counter threshold (4 bit)
    pub const fn set_nntc_ut_cnt_th(&mut self, nntc_ut_cnt_th: u8) {
        self.0 = self.0 & 0x0FFF | ((nntc_ut_cnt_th as u16) << 12);
    }
}

impl Deref for VNTCUTTh {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for VNTCUTTh {
    fn from(id: u16) -> Self {
        VNTCUTTh(id)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for VNTCUTTh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VNTC_UT_TH: {{\n  NTC_UT_TH: {},\n  NNTC_UT_CNT_TH: {}\n}}",
            self.get_ntc_ut_th(),
            self.get_nntc_ut_cnt_th()
        )
    }
}
