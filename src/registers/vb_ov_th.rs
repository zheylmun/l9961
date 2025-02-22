use core::ops::Deref;

/// Battery pack monitoring over-voltage threshold configuration register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VBOvTh(u16);

impl VBOvTh {
    /// Create a new VBOvTh register value
    #[inline]
    pub const fn new(vb_ov_th: u8, nvb_ov_cnt_th: u8) -> Self {
        VBOvTh((vb_ov_th as u16) | ((nvb_ov_cnt_th as u16) << 8))
    }

    /// Get the programmable VB OV fault threshold (8 bit)
    #[inline]
    pub const fn get_vb_ov_th(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    /// Set the the Get the programmable VB OV fault threshold (8 bit)
    #[inline]
    pub const fn set_vb_ov_th(&mut self, vcell_ov_th: u8) {
        self.0 = self.0 & 0xFF00 | (vcell_ov_th as u16);
    }

    /// Get the programmable VB OV event counter threshold (4 bit)
    #[inline]
    pub const fn get_nvb_ov_cnt_th(&self) -> u8 {
        ((self.0 & 0x0F00) >> 8) as u8
    }

    /// Set the programmable VB OV event counter threshold (4 bit)
    #[inline]
    pub const fn set_ncvb_ov_cnt_th(&mut self, ncell_ov_cnt_th: u8) {
        self.0 = self.0 & 0xF0FF | ((ncell_ov_cnt_th as u16) << 8);
    }
}

impl Deref for VBOvTh {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for VBOvTh {
    fn from(id: u16) -> Self {
        debug_assert!(id & 0xF000 == 0, "Invalid VBOvTh value");
        VBOvTh(id)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for VBOvTh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VB_OV_TH: {{\n  VB_OV_TH: {},\n  NVB_OV_CNT_TH: {}\n}}",
            self.get_vb_ov_th(),
            self.get_nvb_ov_cnt_th()
        )
    }
}
