//! # Conversion functions for the L9961
//! The L9961 uses coded values for many of its registers.
//! These functions convert between the coded values and the actual values in mV, mA, etc.

/// Convert a cell voltage threshold register code to mV
pub const fn cell_voltage_threshold_mv_from_code(code: u8) -> u16 {
    ((19520 * code as u32) / 1000) as u16
}

/// Convert a cell voltage in mV to a register code
pub const fn cell_voltage_threshold_code_from_mv(voltage_mv: u16) -> u8 {
    ((voltage_mv as u32 * 1000) / 19520) as u8
}

/// Convert from mv to code and back to get the actual value which will be set given a target mv value
pub const fn round_trip_cell_voltage_threshold(voltage_mv: u16) -> u16 {
    cell_voltage_threshold_mv_from_code(cell_voltage_threshold_code_from_mv(voltage_mv))
}

/// Convert a cell voltage measurement register code to mV
pub const fn cell_voltage_measurement_mv_from_code(code: u16) -> u16 {
    ((122 * code as u32) / 100) as u16
}

/// Convert a cell voltage in mV to a register code
pub const fn cell_voltage_measurement_code_from_mv(voltage_mv: u16) -> u16 {
    ((voltage_mv as u32 * 100) / 122) as u16
}

/// Convert from mv to code and back to get the actual value which will be set given a target mv value
pub const fn round_trip_cell_voltage_measurement(voltage_mv: u16) -> u16 {
    cell_voltage_measurement_mv_from_code(cell_voltage_measurement_code_from_mv(voltage_mv))
}

/// Convert a pack voltage threshold register code to mV
pub const fn pack_voltage_threshold_mv_from_code(code: u8) -> u16 {
    ((code as u32 * 97600) / 1000) as u16
}

/// Convert a pack voltage in mV to a register code
pub const fn pack_voltage_threshold_code_from_mv(voltage_mv: u16) -> u8 {
    ((1000 * voltage_mv as u32) / 97600) as u8
}

/// Convert from pack mv to code and back to get the actual value which will be set given a target mv value
pub const fn round_trip_pack_voltage_threshold(voltage_mv: u16) -> u16 {
    pack_voltage_threshold_mv_from_code(pack_voltage_threshold_code_from_mv(voltage_mv))
}

/// Convert a pack voltage measurement register code to mV
pub const fn pack_voltage_measurement_mv_from_code(code: u16) -> u16 {
    ((61 * code as u32) / 10) as u16
}

/// Convert a pack voltage measurement in mV to a register code
pub const fn pack_voltage_measurement_code_from_mv(voltage_mv: u16) -> u16 {
    ((voltage_mv as u32 * 10) / 61) as u16
}

/// Convert from pack mv to code and back to get the actual value which will be set given a target mv value
pub const fn round_trip_pack_voltage_measurement(voltage_mv: u16) -> u16 {
    pack_voltage_measurement_mv_from_code(pack_voltage_measurement_code_from_mv(voltage_mv))
}

/// Convert an ntc register code to mV
pub const fn ntc_voltage_mv_from_code(code: u16) -> u16 {
    ((code as u32 * 806) / 1000) as u16
}

/// Convert an ntc voltage in mV to a register code
pub const fn ntc_voltage_code_from_mv(voltage_mv: u16) -> u16 {
    ((1000 * voltage_mv as u32) / 806) as u16
}

/// Convert from ntc mv to code and back to get the actual value which will be set given a target mv value
pub const fn round_trip_ntc_voltage(voltage_mv: u16) -> u16 {
    ntc_voltage_mv_from_code(ntc_voltage_code_from_mv(voltage_mv))
}

const CURRENT_RESOLUTION_VAL: u32 = 9155;
const CURRENT_RESISTOR_SHUNT_VALUE_MOHM: u32 = 10;

/// Convert a current measurement from a combined register code to mA
pub const fn current_measurement_ma_from_code(code: u32) -> u16 {
    (((code * CURRENT_RESOLUTION_VAL) / CURRENT_RESISTOR_SHUNT_VALUE_MOHM) / 1000u32) as u16
}
