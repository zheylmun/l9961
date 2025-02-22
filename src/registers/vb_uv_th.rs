use core::ops::Deref;

/// Battery pack monitoring under-voltage threshold configuration register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VBUvTh(u16);

impl VBUvTh {
    /// Create a new VBUvTh register value
    #[inline]
    pub const fn new(vb_uv_th: u8, nvb_uv_cnt_th: u8) -> Self {
        VBUvTh((vb_uv_th as u16) | ((nvb_uv_cnt_th as u16) << 8))
    }

    /// Get the programmable VB UV fault threshold (8 bit)
    #[inline]
    pub const fn get_vb_uv_th(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    /// Set the the Get the programmable VB UV fault threshold (8 bit)
    #[inline]
    pub const fn set_vb_uv_th(&mut self, vcell_uv_th: u8) {
        self.0 = self.0 & 0xFF00 | (vcell_uv_th as u16);
    }

    /// Get the programmable VB UV event counter threshold (4 bit)
    #[inline]
    pub const fn get_nvb_uv_cnt_th(&self) -> u8 {
        ((self.0 & 0x0F00) >> 8) as u8
    }

    /// Set the programmable VB UV event counter threshold (4 bit)
    #[inline]
    pub const fn set_ncvb_uv_cnt_th(&mut self, ncell_uv_cnt_th: u8) {
        self.0 = self.0 & 0xF0FF | ((ncell_uv_cnt_th as u16) << 8);
    }
}

impl Deref for VBUvTh {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for VBUvTh {
    fn from(id: u16) -> Self {
        debug_assert!(id & 0xF000 == 0, "Invalid VBUvTh value");
        VBUvTh(id)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for VBUvTh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VB_UV_TH: {{\n  VB_UV_TH: {},\n  NVB_UV_CNT_TH: {}\n}}",
            self.get_vb_uv_th(),
            self.get_nvb_uv_cnt_th()
        )
    }
}
