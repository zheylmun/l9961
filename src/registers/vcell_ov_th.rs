use core::ops::Deref;

/// Cell overvoltage monitoring threshold configuration register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VCellOvTh(u16);

impl VCellOvTh {
    /// Get the Programmable cell overvoltage fault threshold (8bit)
    #[inline]
    pub const fn get_vcell_ov_th(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    /// Set the Programmable cell overvoltage fault threshold (8bit)
    #[inline]
    pub const fn set_vcell_ov_th(&mut self, vcell_ov_th: u8) {
        self.0 = self.0 & 0xFF00 | (vcell_ov_th as u16);
    }

    /// Get the Programmable cell overvoltage event counter threshold (4 bit)
    #[inline]
    pub const fn get_ncell_ov_cnt_th(&self) -> u8 {
        ((self.0 & 0x0F00) >> 8) as u8
    }

    /// Set the Programmable cell overvoltage event counter threshold (4 bit)
    #[inline]
    pub const fn set_ncell_ov_cnt_th(&mut self, ncell_ov_cnt_th: u8) {
        self.0 = self.0 & 0xF0FF | ((ncell_ov_cnt_th as u16) << 8);
    }
}

impl Deref for VCellOvTh {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for VCellOvTh {
    fn from(id: u16) -> Self {
        debug_assert!(id & 0xF000 == 0, "Invalid VCellOvTh value");
        VCellOvTh(id)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for VCellOvTh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VCELL_OV_TH: {{\n  CELL_OV_TH: {},\n  NCELL_OV_CNT_TH: {}\n}}",
            self.get_vcell_ov_th(),
            self.get_ncell_ov_cnt_th()
        )
    }
}
