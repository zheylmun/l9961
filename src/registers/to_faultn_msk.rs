#[cfg(not(feature = "defmt"))]
use bitflags::bitflags;
#[cfg(feature = "defmt")]
use defmt::bitflags;

bitflags! {
    /// Programmable mask register for whether faults assert the on the PRDRV and balance FET outputs
    pub struct ToFaultnMsk:u16 {
        /// Mask for the CELL_UF_FAULTN_MSK bit
        const CELL_UF_FAULTN_MSK = 0x0001;
        /// Mask for the CELL_SEVERE_UV_FAULTN_MSK bit
        const CELL_SEVERE_UV_FAULTN_MSK = 0x0002;
        /// Mask for the CELL_OV_FAULTN_MSK bit
        const CELL_OV_FAULTN_MSK = 0x0004;
        /// Mask for the CELL_SEVERE_OV_FAULTN_MSK bit
        const CELL_SEVERE_OV_FAULTN_MSK = 0x0008;
        /// Mask for the BAL_UV_FAULTN_MSK bit
        const BAL_UV_FAULTN_MSK = 0x0010;
        /// Mask for the VB_UV_FAULTN_MSK bit
        const VB_UV_FAULTN_MSK = 0x0020;
        /// Mask for the VB_OV_FAULTN_MSK bit
        const VB_OV_FAULTN_MSK = 0x0040;
        /// Mask for the VB_SUM_CHECK_FAULTN_MSK bit
        const VB_SUM_CHECK_FAULTN_MSK = 0x0080;
        /// Mask for the NTC_OT_FAULTN_MSK bit
        const NTC_OT_FAULTN_MSK = 0x0100;
        /// Mask for the NTC_SEVERE_OT_FAULTN_MSK bit
        const NTC_SEVERE_OT_FAULTN_MSK = 0x0200;
        /// Mask for the NTC_UT_FAULTN_MSK bit
        const NTC_UT_FAULTN_MSK = 0x0400;
        /// Mask for the DIE_OT_FAULTN_MSK bit
        const DIE_OT_FAULTN_MSK = 0x0800;
        // Ensure that the reserved bits are always 0
        // TODO: This requires Bitflags 2.x
        // const _ = 0x0FFF;
    }
}
