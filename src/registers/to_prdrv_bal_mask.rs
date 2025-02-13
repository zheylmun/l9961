#[cfg(not(feature = "defmt"))]
use bitflags::bitflags;
#[cfg(feature = "defmt")]
use defmt::bitflags;

bitflags! {
    /// Programmable mask register for whether faults assert the on the PRDRV and balance FET outputs
    pub struct ToPrdrvBalMask:u16 {
        /// Mask for the CELL_UV_PRDRV bit
        const CELL_UV_PRDRV_MSK = 0x0001;
        /// Mask for the CELL_SEVERE_UV_PRDRV bit
        const CELL_SEVERE_UV_PRDRV_MSK = 0x0002;
        /// Mask for the CELL_OV_PRDRV bit
        const CELL_OV_PRDRV_MSK = 0x0004;
        /// Mask for the CELL_SEVERE_OV_PRDRV bit
        const CELL_SEVERE_OV_PRDRV_MSK = 0x0008;
        /// Mask for the VB_UV_PRDRV bit
        const VB_UV_PRDRV_MSK = 0x0010;
        /// Mask for the VB_OV_PRDRV bit
        const VB_OV_PRDRV_MSK = 0x0020;
        /// Mask for the VB_SUM_CHECK_PRDRV bit
        const VB_SUM_CHECK_PRDRV_MSK = 0x0040;
        /// Mask for the NTC_OT_PRDRV bit
        const NTC_OT_PRDRV_MSK = 0x0080;
        /// Mask for the NTC_SEVERE_OT_PRDRV bit
        const NTC_SEVERE_OT_PRDRV_MSK = 0x0100;
        /// Mask for the NTC_UT_PRDRV bit
        const NTC_UT_PRDRV_MSK = 0x0200;
        /// Mask for the DIE_OT_PRDRV bit
        const DIE_OT_PRDRV_MSK = 0x0400;
        /// Mask for the BAL_UV_BAL bit
        const BAL_UV_BAL_MSK = 0x0800;
        /// Mask for the NTC_SEVERE_OT_BAL bit
        const NTC_SEVERE_OT_BAL_MSK = 0x1000;
        /// Mask for the DIE_OT_BAL bit
        const DIE_OT_BAL_MSK = 0x2000;
        /// Mask for the VB_SUM_CHECK_BAL bit
        const VB_SUM_CHECK_BAL_MSK = 0x4000;
        // Ensure that the reserved bit is always 0
        // TODO: This requires Bitflags 2.x
        // const _ = 0x7FFF;
    }
}
