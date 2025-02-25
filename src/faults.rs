//! The fault module defines the faults returned by the L9961 driver when making measurements
//! It implements the logic to translate the fault registers into the measurement struct,
//! and to clear the fault registers while doing so.

use crate::{
    measurement::Measurement,
    registers::{DiagCurr, DiagOvOtUt, DiagUv},
    Registers, L9961,
};

#[cfg(not(feature = "defmt"))]
use bitflags::bitflags;
#[cfg(feature = "defmt")]
use defmt::bitflags;

bitflags! {
    /// Cell fault flags
    pub struct CellFaults:u8 {
        /// Cell is over its configured over-voltage threshold
        const OVER_VOLTAGE = 0x01;
        /// Cell is over it's configured severe over-voltage threshold
        const EXTREME_OVER_VOLTAGE = 0x02;
        /// Cell is over its configured over-voltage threshold
        const UNDER_VOLTAGE = 0x04;
        /// Cell is below its configured under-voltage threshold for balancing
        const UNDER_VOLTAGE_FOR_BALANCING = 0x08;
        /// Cell is over it's configured severe over-voltage threshold
        const EXTREME_UNDER_VOLTAGE = 0x10;
        // Ensure that the reserved bits are always 0
        // TODO: This requires Bitflags 2.x
        // const _ = 0x007F;
    }
}

bitflags! {
    /// Pack and BMS fault flags
    pub struct PackFaults:u16 {
        /// Cell is over its configured over-voltage threshold
        const OVER_VOLTAGE = 0x01;
        /// Cell is over its configured over-voltage threshold
        const UNDER_VOLTAGE = 0x02;
        /// Pack is over it's configured over-temperature threshold
        const NTC_OVER_TEMP = 0x04;
        /// Pack is under it's configured under-temperature threshold
        const NTC_UNDER_TEMP = 0x08;
        /// Pack is over it's configured severe over-temperature threshold
        const NTC_SEVERE_OVER_TEMP = 0x10;
        /// BMS Die over-temperature
        const DIE_OVER_TEMP = 0x20;
        /// Mismatch between cell measurements and pack voltage
        const CELL_VOLTAGE_SUM_VB_MISMATCH = 0x40;

        /// Coulomb counter saturation
        const CC_SAT = 0x8000;
        // Ensure that the reserved bits are always 0
        // TODO: This requires Bitflags 2.x
        // const _ = 0x007F;
    }
}

impl<I2C, I, O> L9961<I2C, I, O>
where
    I2C: embedded_hal_async::i2c::I2c,
    I: embedded_hal_async::digital::Wait,
    O: embedded_hal::digital::OutputPin,
{
    /// Update the measurement with fault registers
    pub(crate) async fn read_fault_registers(
        &mut self,
        measurement: &mut Measurement,
    ) -> Result<(), I2C::Error> {
        let register_values = self.read_registers(Registers::DiagOvOtUt, 2).await.unwrap();
        let diag_1 = DiagOvOtUt::from_bits_truncate(register_values[0]);
        let diag_2 = DiagUv::from_bits_truncate(register_values[1]);
        let diag3 = self.read_diag_curr().await?;

        // Set any cell 1 faults'
        if diag_1.contains(DiagOvOtUt::CELL1_OV) {
            measurement.cell_1.faults |= CellFaults::OVER_VOLTAGE;
        }
        if diag_1.contains(DiagOvOtUt::CELL1_SEVERE_OV) {
            measurement.cell_1.faults |= CellFaults::EXTREME_OVER_VOLTAGE;
        }

        // Set any pack faults
        if diag_1.contains(DiagOvOtUt::DIE_OT) {
            measurement.pack_faults |= PackFaults::DIE_OVER_TEMP;
        }
        if diag_1.contains(DiagOvOtUt::NTC_OT) {
            measurement.pack_faults |= PackFaults::NTC_OVER_TEMP;
        }
        if diag_1.contains(DiagOvOtUt::NTC_SEVERE_OT) {
            measurement.pack_faults |= PackFaults::DIE_OVER_TEMP;
        }
        if diag_1.contains(DiagOvOtUt::NTC_UT) {
            measurement.pack_faults |= PackFaults::NTC_UNDER_TEMP;
        }
        if diag_1.contains(DiagOvOtUt::PACK_OV) {
            measurement.pack_faults |= PackFaults::OVER_VOLTAGE;
        }
        if diag_1.contains(DiagOvOtUt::VB_SUM_CHECK_FAIL) {
            measurement.pack_faults |= PackFaults::NTC_OVER_TEMP;
        }
        if diag_2.contains(DiagUv::VB_UV) {
            measurement.pack_faults |= PackFaults::UNDER_VOLTAGE;
        }
        if diag3.contains(DiagCurr::CC_SAT) {
            measurement.pack_faults |= PackFaults::DIE_OVER_TEMP;
        }

        Ok(())
    }

    pub(crate) async fn clear_fault_registers(&mut self) -> Result<(), I2C::Error> {
        self.write_diag_ov_ot_ut(DiagOvOtUt::all()).await?;
        self.write_diag_uv(DiagUv::all()).await?;
        self.write_diag_curr(DiagCurr::all()).await
    }
}
