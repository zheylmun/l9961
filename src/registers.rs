//! # Registers
//! Definitions for the registers of the L9961 chip

mod chip_id;
pub use chip_id::ChipID;

mod cfg3_act;
pub use cfg3_act::Cfg3Act;

/// The registers of the L9961 chip represented as their addresses
#[repr(u8)]
pub enum Registers {
    /// The chip ID register
    ChipID = 0x00,
    /// The chip revision register
    Cfg3Act = 0x01,
}
