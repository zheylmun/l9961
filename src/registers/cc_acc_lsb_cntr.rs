use core::ops::Deref;

/// Coulomb Counter access LSB and counter register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CCAccLsbCntr(u16);

impl CCAccLsbCntr {
    /// Get the number of samples stored in the accumulator
    #[inline]
    pub const fn get_cc_sample_cnt(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    /// Get the lsb of the cc accumulator
    #[inline]
    pub const fn get_cc_acc_lsb(&self) -> u8 {
        ((self.0 & 0xFF00) >> 8) as u8
    }
}

impl Deref for CCAccLsbCntr {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for CCAccLsbCntr {
    fn from(id: u16) -> Self {
        CCAccLsbCntr(id)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for CCAccLsbCntr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CC_ACC_LSB_CNTR: {{\n  CC_SAMPLE_CNT: {},\n  CC_ACC_LSB: {}\n}}",
            self.get_cc_sample_cnt(),
            self.get_cc_acc_lsb()
        )
    }
}
