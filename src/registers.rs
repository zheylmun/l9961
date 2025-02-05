//! # Registers
//! Definitions for the registers of the L9961 chip

mod chip_id;
pub use chip_id::ChipID;

mod cfg1_filters_cycles;
pub use cfg1_filters_cycles::Cfg1FiltersCycles;

mod cfg2_enables;
pub use cfg2_enables::Cfg2Enables;

mod cfg3_act;
pub use cfg3_act::Cfg3Act;

mod csa_gain_factor;
pub use csa_gain_factor::CsaGainFactor;

mod dev_addr;
pub use dev_addr::DevAddr;

mod vcell_ov_th;
pub use vcell_ov_th::VCellOvTh;

/// The registers of the L9961 chip represented as their addresses
#[repr(u8)]
pub enum Registers {
    /// The chip ID register
    ChipID = 0x00,
    /// The chip revision register
    Cfg3Act = 0x01,
    /// The configuration register for the filters and cycles
    Cfg1FiltersCycles = 0x02,
    /// The device address register
    DevAddr = 0x03,
    /// The configuration register for enables
    Cfg2Enables = 0x04,
    /// The configuration register for the CSA gain factor
    CsaGainFactor = 0x05,
    /// The configuration register for the VCELL overvoltage threshold
    VCellOvTh = 0x06,
}
