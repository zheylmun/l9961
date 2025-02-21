//! # Conversion functions for the L9961
//! The L9961 uses coded values for many of its registers.
//! These functions convert between the coded values and the actual values in mV, mA, etc.

/// Convert a cell voltage register code to mV
#[inline]
pub const fn cell_voltage_mv_from_code(code: u8) -> u16 {
    ((19520u32 * code as u32) / 1000u32) as u16
}

/// Convert a cell voltage in mV to a register code
#[inline]
pub const fn cell_voltage_code_from_mv(voltage_mv: u16) -> u8 {
    ((voltage_mv as u32 * 1000u32) / 19520u32) as u8
}
