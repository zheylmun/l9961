#[cfg(not(feature = "defmt"))]
use bitflags::bitflags;
#[cfg(feature = "defmt")]
use defmt::bitflags;

bitflags! {
    /// Programmable mask register for whether faults assert the on the PRDRV and balance FET outputs
    pub struct ToFuseRstMask:u16 {
        /// Mask for the CELL_SEVERE_UV_FUSE bit
        const CELL_SEVERE_UV_FUSE_MSK = 0x0001;
        /// Mask for the CELL_SEVERE_OV_FUSE bit
        const CELL_SEVERE_OV_FUSE_MSK = 0x0002;
        /// Mask for the VB_SUM_CHECK_FUSE bit
        const VB_SUM_CHECK_FUSE_MSK = 0x0004;
        /// Mask for the NTC_SEVERE_OT_FUSE bit
        const NTC_SEVERE_OT_FUSE_MSK = 0x0008;
        /// Mask for the CELL_OV_RST bit
        const CELL_OV_RST_MSK = 0x0010;
        /// Mask for the CELL_SEVERE_OV_RST bit
        const CELL_SEVERE_OV_RST_MSK = 0x0020;
        /// Mask for the VB_OV_RST bit
        const VB_OV_RST_MSK = 0x0040;
        // Ensure that the reserved bits are always 0
        // TODO: This requires Bitflags 2.x
        // const _ = 0x007F;
    }
}
