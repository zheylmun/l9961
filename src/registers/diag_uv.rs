#[cfg(not(feature = "defmt"))]
use bitflags::bitflags;
#[cfg(feature = "defmt")]
use defmt::bitflags;

bitflags! {
    /// Diagnostic Under Voltage flags
    /// All are clear-on-write, and must be cleared by the mcu to re-enable masked behavior
    pub struct DiagUv:u16 {
        /// Cell 1 Under-Voltage flag
        const CELL1_UV = 0x0001;
        /// Cell 2 Under-Voltage flag
        const CELL2_UV = 0x0002;
        /// Cell 3 Under-Voltage flag
        const CELL3_UV = 0x0004;
        /// Cell 4 Under-Voltage flag
        const CELL4_UV = 0x0008;
        /// Cell 5 Under-Voltage flag
        const CELL5_UV = 0x0010;
        /// Cell 1 under balance voltage flag
        const BAL1_UV = 0x0020;
        /// Cell 2 under balance voltage flag
        const BAL2_UV = 0x0040;
        /// Cell 3 under balance voltage flag
        const BAL3_UV = 0x0080;
        /// Cell 4 under balance voltage flag
        const BAL4_UV = 0x0100;
        /// Cell 5 under balance voltage flag
        const BAL5_UV = 0x0200;
        /// Pack Under-Voltage flag
        const VB_UV = 0x0400;
        /// Cell 1 Severe Under-Voltage flag
        const V_SEVERE_CELL1_UV = 0x0800;
        /// Cell 2 Severe Under-Voltage flag
        const V_SEVERE_CELL2_UV = 0x1000;
        /// Cell 3 Severe Under-Voltage flag
        const V_SEVERE_CELL3_UV = 0x2000;
        /// Cell 4 Severe Under-Voltage flag
        const V_SEVERE_CELL4_UV = 0x4000;
        /// Cell 5 Severe Under-Voltage flag
        const V_SEVERE_CELL5_UV = 0x8000;
    }
}
