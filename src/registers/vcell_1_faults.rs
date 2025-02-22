use core::ops::Deref;

/// VCell1 Measurement Register packed faults
/// VCell1 also contains the crc_cfg_fail and crc_trim_cal_fail flags
/// This ignores the measurement and command bits, and simply returns the faults
/// Measurement should be disabled when working with the NVM,
/// so reads to this register should be for one purpose only
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VCell1Faults(u16);

impl VCell1Faults {
    /// Get the Programmable cell overvoltage event counter threshold (4 bit)
    pub const fn get_crc_trim_cal_fail(&self) -> bool {
        self.0 & 0x4000 != 0
    }

    /// Get the Programmable cell overvoltage event counter threshold (4 bit)
    pub const fn get_crc_cfg_fail(&self) -> bool {
        self.0 & 0x8000 != 0
    }
}

impl Deref for VCell1Faults {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for VCell1Faults {
    fn from(val: u16) -> Self {
        // Make sure the GO2SHIP bits are not set on accident
        debug_assert!(val & 0x3000 == 0);
        VCell1Faults(val)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for VCell1Faults {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VCELL1_FAULTS: {{\n  CRC_TRIM_CAL_FAIL: {},\n  CRC_CFG_FAIL: {}\n}}",
            self.get_crc_trim_cal_fail(),
            self.get_crc_cfg_fail()
        )
    }
}
