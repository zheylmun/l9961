use core::ops::Deref;

/// Cell undervoltage during balancing monitoring threshold configuration register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VCellBalUvDeltaTh(u16);

impl VCellBalUvDeltaTh {
    /// Get the programmable balancing UV threshold (positive delta in respect to cell UV threshold, 8 bit)
    #[inline]
    pub const fn get_vcell_bal_uv_delta_th(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    /// Set the the programmable cell undervoltage fault threshold (8bit)
    #[inline]
    pub const fn set_vcell_uv_th(&mut self, vcell_uv_th: u8) {
        self.0 = self.0 & 0xFF00 | (vcell_uv_th as u16);
    }

    /// Get the programmable cell undervoltage event counter threshold (4 bit)
    #[inline]
    pub const fn get_ncell_uv_cnt_th(&self) -> u8 {
        ((self.0 & 0x0F00) >> 8) as u8
    }

    /// Set the programmable cell undervoltage event counter threshold (4 bit)
    #[inline]
    pub const fn set_ncell_uv_cnt_th(&mut self, ncell_uv_cnt_th: u8) {
        self.0 = self.0 & 0xF0FF | ((ncell_uv_cnt_th as u16) << 8);
    }
}

impl Deref for VCellBalUvDeltaTh {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for VCellBalUvDeltaTh {
    fn from(id: u16) -> Self {
        debug_assert!(id & 0xF000 == 0, "Invalid VCellBalUvDeltaTh value");
        VCellBalUvDeltaTh(id)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for VCellBalUvDeltaTh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VCELL_BAL_UV_DELTA_TH: {{\n  VCELL_BAL_UV_DELTA_TH: {},\n  NCELL_BAL_UV_CNT_TH: {}\n}}",
            self.get_vcell_bal_uv_delta_th(),
            self.get_ncell_uv_cnt_th()
        )
    }
}
