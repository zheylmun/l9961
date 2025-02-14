#[cfg(not(feature = "defmt"))]
use bitflags::bitflags;
#[cfg(feature = "defmt")]
use defmt::bitflags;

bitflags! {
    /// Programmable mask register for whether current related faults assert the on the PRDRV FET and FAULTN outputs
    pub struct CurrMsk : u16 {
        /// Masks Over Current in Charge fault asserting on the charge pin
        const OVC_CHG_PRDRV_MSK = 0x0001;
        /// Masks Over Current in Discharge fault asserting on the discharge pin
        const OVC_DCHG_PRDRV_MSK = 0x0002;
        /// Masks Short Circuit in Discharge fault asserting on the discharge pin
        const SC_DCHG_PRDRV_MSK = 0x0004;
        /// Masks Persistent Over Current in Charge fault asserting on the fuse pin
        const PERSIST_OVC_CHG_FUSE_MSK = 0x0008;
        /// Masks Persistent Over Current in Discharge fault asserting on the fuse pin
        const PERSIST_OVC_DCHG_FUSE_MSK = 0x0010;
        /// Masks Persistent Short Circuit in Discharge fault asserting on the fuse pin
        const PERSIST_SC_DCHG_FUSE_MSK = 0x0020;
        /// Masks Over Current in Charge fault asserting on the FAULTN pin
        const OVC_CHG_FAULTN_MSK = 0x0040;
        /// Masks Over Current in Discharge fault asserting on the FAULTN pin
        const OV_DCHG_FAULTN_MSK = 0x0080;
        /// Masks Short Circuit in Discharge fault asserting on the FAULTN pin
        const SC_DCHG_FAULTN_MSK = 0x0100;
        /// Masks Persistent Short Cirtcuit in Discharge fault asserting on the FAULTN pin
        const PERSIST_SC_DCHG_FAULTN_MSK = 0x0200;
        /// Masks Persistent Over Current in Charge fault asserting on the FAULTN pin
        const PERSIST_OVC_CHG_FAULTN_MSK = 0x0400;
        /// Masks Persistent Over Current in Discharge fault asserting on the FAULTN pin
        const PERSIST_OVC_DCHG_FAULTN_MSK = 0x0800;

        // Ensure that the reserved bit is always 0
        // TODO: This requires Bitflags 2.x
        // const _ = 0x7FFF;
    }
}
