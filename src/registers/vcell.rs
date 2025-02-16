use core::ops::Deref;

/// VCell Measurement Register
/// Packs the cell number and measurement value into a single u16.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VCell(u16);

impl VCell {
    /// Create a new VCell register value
    #[inline]
    pub(super) fn new(cell: u8, measurement: u16) -> Self {
        debug_assert!(cell > 0 && cell < 6);
        VCell(((cell as u16) << 12) | (measurement & 0x0FFF))
    }

    /// Get the cell voltage measurement code
    #[inline]
    pub const fn get_vcell_meas(&self) -> u16 {
        (self.0 & 0x0FFF) as u16
    }

    /// Get the cell voltage measurement in mV
    #[inline]
    pub const fn get_vcell_meas_mv(&self) -> u16 {
        122 * self.get_vcell_meas() / 100
    }

    /// Get the cell number
    #[inline]
    pub const fn get_cell(&self) -> u8 {
        ((self.0 >> 12) & 0x0F) as u8
    }
}

impl Deref for VCell {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for VCell {
    fn format(&self, f: defmt::Formatter) {
        let cell = self.get_cell();
        defmt::write!(
            f,
            "VCELL{}: {{\n  VCELL{} : {}mv,\n}}",
            cell,
            cell,
            self.get_vcell_meas_mv()
        )
    }
}
