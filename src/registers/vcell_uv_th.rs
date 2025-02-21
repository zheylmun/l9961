use core::ops::Deref;
use defmt::{Format, Formatter, write};

/// Cell undervoltage monitoring threshold configuration register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VCellUvTh(u16);

impl VCellUvTh {
    /// Get the programmable cell under-voltage fault threshold (8bit)
    #[inline]
    pub const fn get_vcell_uv_th(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    /// Set the the programmable cell undervoltage fault threshold (8bit)
    #[inline]
    pub const fn set_vcell_uv_th(&mut self, vcell_uv_th: u8) {
        self.0 = self.0 & 0xFF00 | (vcell_uv_th as u16);
    }

    /// Get the programmable cell under-voltage event counter threshold (4 bit)
    #[inline]
    pub const fn get_ncell_uv_cnt_th(&self) -> u8 {
        ((self.0 & 0x0F00) >> 8) as u8
    }

    /// Set the programmable cell under-voltage event counter threshold (4 bit)
    #[inline]
    pub const fn set_ncell_uv_cnt_th(&mut self, ncell_uv_cnt_th: u8) {
        self.0 = self.0 & 0xF0FF | ((ncell_uv_cnt_th as u16) << 8);
    }
}

impl Deref for VCellUvTh {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for VCellUvTh {
    fn from(id: u16) -> Self {
        debug_assert!(id & 0xF000 == 0, "Invalid VCellUvTh value");
        VCellUvTh(id)
    }
}

#[cfg(feature = "defmt")]
impl Format for VCellUvTh {
    fn format(&self, f: Formatter) {
        write!(
            f,
            "VCELL_UV_TH: {{\n  VCELL_UV_TH: {},\n  NCELL_UV_CNT_TH: {}\n}}",
            self.get_vcell_uv_th(),
            self.get_ncell_uv_cnt_th()
        )
    }
}
