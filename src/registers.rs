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

mod vb_ov_th;
pub use vb_ov_th::VBOvTh;

mod vb_uv_th;
pub use vb_uv_th::VBUvTh;

mod vb_sum_max_diff_th;
pub use vb_sum_max_diff_th::VBSumMaxDiffTh;

mod vcell_bal_uv_delta_th;
pub use vcell_bal_uv_delta_th::VCellBalUvDeltaTh;

mod vcell_ov_th;
pub use vcell_ov_th::VCellOvTh;

mod vcell_severe_delta_thrs;
pub use vcell_severe_delta_thrs::VCellSevereDeltaThrs;

mod vcell_uv_th;
pub use vcell_uv_th::VCellUvTh;

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
    /// The configuration register for the VCELL undervoltage threshold
    VCellUvTh = 0x07,
    /// The configuration register for the VCELL severe undervoltage and overvoltage thresholds
    VCellSevereDeltaThrs = 0x08,
    /// The configuration register for the VCELL balancing undervoltage delta threshold
    VCellBalUvDeltaTh = 0x09,
    /// The configuration register for the VBAT overvoltage threshold
    VBOvTh = 0x0A,
    /// The configuration register for the VBAT undervoltage threshold
    VBUvTh = 0x0B,
    /// The configuration register for the VBAT sum max diff threshold
    VBSumMaxDiffTh = 0x0C,
}
