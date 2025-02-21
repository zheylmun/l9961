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

/// Convert from mv to code and back to get the actual value which will be set given a target mv value
#[inline]
pub const fn round_trip_cell_voltage(voltage_mv: u16) -> u16 {
    cell_voltage_mv_from_code(cell_voltage_code_from_mv(voltage_mv))
}

/// Convert a pack voltage register code to mV
#[inline]
pub const fn pack_voltage_mv_from_code(code: u16) -> u16 {
    ((1000u32 * code as u32) / 97600u32) as u16
}

/// Convert a pack voltage in mV to a register code
#[inline]
pub const fn pack_voltage_code_from_mv(voltage_mv: u16) -> u16 {
    ((voltage_mv as u32 * 97600u32) / 1000u32) as u16
}

/// Convert from mv to code and back to get the actual value which will be set given a target mv value
#[inline]
pub const fn round_trip_pack_voltage(voltage_mv: u16) -> u16 {
    pack_voltage_mv_from_code(pack_voltage_code_from_mv(voltage_mv))
}
