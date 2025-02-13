//! # Registers
//! Definitions for the registers of the L9961 chip

mod cfg1_filters_cycles;
mod cfg2_enables;
mod cfg3_act;
mod chip_id;
mod csa_gain_factor;
mod dev_addr;
mod ovc_thresholds;
mod persistent_ovc_thresholds;
mod sc_threshold;
mod to_faultn_msk;
mod to_fuse_rst_msk;
mod to_prdrv_bal_mask;
mod vb_ov_th;
mod vb_sum_max_diff_th;
mod vb_uv_th;
mod vcell_bal_uv_delta_th;
mod vcell_ov_th;
mod vcell_severe_delta_thrs;
mod vcell_uv_th;
mod vntc_ot_th;
mod vntc_severe_ot_th;
mod vntc_ut_th;

#[cfg(feature = "blocking")]
mod blocking;

pub use self::{
    cfg1_filters_cycles::Cfg1FiltersCycles, cfg2_enables::Cfg2Enables, cfg3_act::Cfg3Act,
    chip_id::ChipID, csa_gain_factor::CsaGainFactor, dev_addr::DevAddr,
    ovc_thresholds::OvCThresholds, persistent_ovc_thresholds::PersistentOvCThresholds,
    sc_threshold::SCThreshold, to_faultn_msk::ToFaultnMsk, to_fuse_rst_msk::ToFuseRstMask,
    to_prdrv_bal_mask::ToPrdrvBalMask, vb_ov_th::VBOvTh, vb_sum_max_diff_th::VBSumMaxDiffTh,
    vb_uv_th::VBUvTh, vcell_bal_uv_delta_th::VCellBalUvDeltaTh, vcell_ov_th::VCellOvTh,
    vcell_severe_delta_thrs::VCellSevereDeltaThrs, vcell_uv_th::VCellUvTh, vntc_ot_th::VNTCOTTh,
    vntc_severe_ot_th::VNTCSevereOTTh, vntc_ut_th::VNTCUTTh,
};

/// The registers of the L9961 chip represented as their addresses
#[repr(u8)]
pub enum Registers {
    /// The chip ID register
    ChipID = 0x00,
    /// The chip revision register
    Cfg3Act = 0x01,
    /// Configuration register for the filters and cycles
    Cfg1FiltersCycles = 0x02,
    /// The device address register
    DevAddr = 0x03,
    /// Configuration register for enables
    Cfg2Enables = 0x04,
    /// Configuration register for the CSA gain factor
    CsaGainFactor = 0x05,
    /// Configuration register for the VCELL overvoltage threshold
    VCellOvTh = 0x06,
    /// Configuration register for the VCELL undervoltage threshold
    VCellUvTh = 0x07,
    /// Configuration register for the VCELL severe undervoltage and overvoltage thresholds
    VCellSevereDeltaThrs = 0x08,
    /// Configuration register for the VCELL balancing undervoltage delta threshold
    VCellBalUvDeltaTh = 0x09,
    /// Configuration register for the VBAT overvoltage threshold
    VBOvTh = 0x0A,
    /// Configuration register for the VBAT undervoltage threshold
    VBUvTh = 0x0B,
    /// Configuration register for the VBAT sum max diff threshold
    VBSumMaxDiffTh = 0x0C,
    /// Configuration register for the VNTC over temperature threshold
    VNTCOTTh = 0x0D,
    /// Configuration register for the VNTC under temperature threshold
    VNTCUTTh = 0x0E,
    /// Configuration register for the VNTC severe overtemperature threshold
    VNTCSevereOTTh = 0x0F,
    /// Configuration register for overcurrent protection
    OvCThresholds = 0x10,
    /// Configuration register for persistent overcurrent protection
    PersistentOvCThresholds = 0x11,
    /// Configuration register for short circuit threshold protection
    SCThreshold = 0x12,
    /// Configuration register for the TO_PRDRV_BAL_MASK
    ToPrdrvBalMask = 0x13,
    /// Configuration register for the TO_FUSE_RST_MSK
    ToFuseRstMask = 0x14,
    /// Configuration register for the TO_FAULTN_MSK
    ToFaultnMsk = 0x15,
}
