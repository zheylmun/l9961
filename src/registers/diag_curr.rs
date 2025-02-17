#[cfg(not(feature = "defmt"))]
use bitflags::bitflags;
#[cfg(feature = "defmt")]
use defmt::bitflags;

bitflags! {
    /// Diagnostic fault flags
    /// These flags have different behavior by bit position
    pub struct DiagCurr:u16 {
        /// Mask for the CC SAT bit
        /// Read Only
        const CC_SAT = 0x0001;
        /// Mask for the OVC CHG bit
        /// Clear on Write
        const OVC_CHG = 0x0002;
        /// Mask for the OVC DCHG bit
        /// Clear on Write
        const OVC_DCHG = 0x0004;
        /// Mask for the PERSIST_OVC_CHG bit
        /// Clear on Write
        const PERSIST_OVC_CHG = 0x0008;
        /// Mask for the PERSIST_OVC_DCHG bit
        /// Clear on Write
        const PERSIST_OVC_DCHG = 0x0010;
        /// Mask for the SC_DCHG bit
        /// Clear on Write
        const SC_DCHG = 0x0020;
        /// Mask for the PERSIST_SC_DCHG bit
        /// Clear on Write
        const PERSIST_SC_DCHG = 0x0040;
        /// Mask for the FUSE_EXT bit
        /// Read Only
        const FUSE_EXT = 0x0080;
        /// Mask for the FAULTN_EXT bit
        /// Clear on Write
        const FAULTN_EXT = 0x0100;
        // Ensure that the reserved bit is always 0
        // TODO: This requires Bitflags 2.x
        // const _ = 0x0EFF;
    }
}
