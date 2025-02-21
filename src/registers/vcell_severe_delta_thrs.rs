use core::ops::Deref;

/// Cell voltage severe undervoltage and overvoltage monitoring threshold configuration register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VCellSevereDeltaThrs(u16);

impl VCellSevereDeltaThrs {
    /// Create a new VCellSevereDeltaThrs register value
    #[inline]
    pub const fn new(
        cell_severe_over_voltage_delta_threshold: u8,
        cell_severe_under_voltage_delta_threshold: u8,
    ) -> Self {
        VCellSevereDeltaThrs(
            (cell_severe_under_voltage_delta_threshold as u16) << 8
                | cell_severe_over_voltage_delta_threshold as u16,
        )
    }

    /// Get the programmable cell severe OV threshold (positive delta in respect to cell OV threshold, 8 bit)
    #[inline]
    pub const fn get_vcell_severe_ov_delta_th(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    /// Set the programmable cell severe OV threshold (positive delta in respect to cell OV threshold, 8 bit)
    #[inline]
    pub const fn set_vcell_severe_ov_delta_th(&mut self, severe_ov_threshold: u8) {
        self.0 = self.0 & 0xFF00 | (severe_ov_threshold as u16);
    }

    /// Get the programmable cell severe UV threshold (negative delta in respect to cell UV threshold, 8 bit)
    #[inline]
    pub const fn get_vcell_severe_uv_delta_th(&self) -> u8 {
        ((self.0 & 0xFF00) >> 8) as u8
    }

    /// Set the programmable cell severe UV threshold (negative delta in respect to cell UV threshold, 8 bit)
    #[inline]
    pub const fn set_vcell_severe_uv_delta_th(&mut self, severe_uv_threshold: u8) {
        self.0 = self.0 & 0x00FF | ((severe_uv_threshold as u16) << 8);
    }
}

impl Deref for VCellSevereDeltaThrs {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for VCellSevereDeltaThrs {
    fn from(id: u16) -> Self {
        VCellSevereDeltaThrs(id)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for VCellSevereDeltaThrs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VCELL_SEVERE_DELTA_THRS: {{\n  VCELL_SEVERE_UV_DELTA_TH: {},\n  VCELL_SEVERE_OV_DELTA_TH: {}\n}}",
            self.get_vcell_severe_uv_delta_th(),
            self.get_vcell_severe_ov_delta_th()
        )
    }
}
