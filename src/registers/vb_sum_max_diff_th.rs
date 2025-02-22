use core::ops::Deref;

/// Programmable plausibility check threshold between VB and sum of cells register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VBSumMaxDiffTh(u16);

impl VBSumMaxDiffTh {
    /// Get the programmable plausibility check threshold between VB and sum of cells in volts (8 bit)
    pub const fn get_vb_sum_max_diff_th_volts(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    /// Set the the Get the programmable VB OV fault threshold (8 bit)
    pub const fn set_vb_sum_max_diff_th_volts(&mut self, vb_sum_max_diff: u8) {
        debug_assert!(vb_sum_max_diff < 25, "Invalid VB_SUM_MAX_DIFF_TH value");
        self.0 = self.0 & 0xFF00 | (vb_sum_max_diff as u16);
    }
}

impl Deref for VBSumMaxDiffTh {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for VBSumMaxDiffTh {
    fn from(diff_th: u16) -> Self {
        debug_assert!(diff_th < 25, "Invalid VB_SUM_MAX_DIFF_TH value");
        VBSumMaxDiffTh(diff_th)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for VBSumMaxDiffTh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VB_SUM_MAX_DIFF_TH: {{\n  VB_SUM_MAX_DIFF_TH: {},\n}}",
            self.get_vb_sum_max_diff_th_volts(),
        )
    }
}
