use core::ops::Deref;

/// The Chip ID register
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChipID(u16);

impl ChipID {
    /// Get the Metal ID of the chip
    pub fn metal_id(&self) -> u8 {
        (self.0 & 0x000F) as u8
    }

    /// Get the Silicon ID of the chip
    pub fn silicon_id(&self) -> u8 {
        ((self.0 & 0x00F0) >> 4) as u8
    }
}

impl Deref for ChipID {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for ChipID {
    fn from(id: u16) -> Self {
        debug_assert!(id & 0xFF00 == 0, "Invalid chip ID");
        ChipID(id)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChipID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ChipID: {{\n  metal_id: {},\n  silicon_id: {}\n}}",
            self.metal_id(),
            self.silicon_id()
        )
    }
}
