#[cfg(not(feature = "defmt"))]
use bitflags::bitflags;
#[cfg(feature = "defmt")]
use defmt::bitflags;

bitflags! {
    /// Diagnostic Over-Voltage, Over-Temperature, and Under-Temperature flags
    /// All are clear-on-write, and must be cleared by the mcu to re-enable masked behavior
    pub struct DiagOvOtUt:u16 {
        /// Cell 1 Over-Voltage flag
        const CELL1_OV = 0x0001;
        /// Cell 2 Over-Voltage flag
        const CELL2_OV = 0x0002;
        /// Cell 3 Over-Voltage flag
        const CELL3_OV = 0x0004;
        /// Cell 4 Over-Voltage flag
        const CELL4_OV = 0x0008;
        /// Cell 5 Over-Voltage flag
        const CELL5_OV = 0x0010;
        /// Pack Over-Voltage flag
        const PACK_OV = 0x0020;
        /// Cell 1 Severe Over-Voltage flag
        const CELL1_SEVERE_OV = 0x0040;
        /// Cell 2 Severe Over-Voltage flag
        const CELL2_SEVERE_OV = 0x0080;
        /// Cell 3 Severe Over-Voltage flag
        const CELL3_SEVERE_OV = 0x0100;
        /// Cell 4 Severe Over-Voltage flag
        const CELL4_SEVERE_OV = 0x0200;
        /// Cell 5 Severe Over-Voltage flag
        const CELL5_SEVERE_OV = 0x0400;
        /// Cell voltage sum and pack measurement comparison check failed
        const VB_SUM_CHECK_FAIL = 0x0800;
        /// NTC Over-Temperature flag
        const NTC_OT = 0x1000;
        /// NTC Severe Over-Temperature flag
        const NTC_SEVERE_OT = 0x2000;
        /// NTC Under-Temperature flag
        const NTC_UT = 0x4000;
        /// Die Over-Temperature flag
        const DIE_OT = 0x8000;
    }
}
