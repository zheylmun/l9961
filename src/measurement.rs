//! The measurement module contains the entry point for periodically measuring the cell voltages and temperatures,
//! as well as initiating the fault handling process should faults occur during measurement.

use crate::{
    conversions::{cell_voltage_measurement_mv_from_code, pack_voltage_measurement_mv_from_code},
    registers::{DiagCurr, DiagOvOtUt, DiagUv, DieTemp, TMeasCycle, VCell, VCellSum, VB},
    Registers, L9961,
};

use bitflags::bitflags;
use embassy_futures::select::select3;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::{delay::DelayNs, digital::Wait, i2c::I2c};

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

/// A single cell measurement
pub struct CellMeasurement {
    /// Cell voltage in mV
    pub voltage_mv: u16,
    /// Active faults, if any
    pub faults: CellFaults,
}

impl Default for CellMeasurement {
    fn default() -> Self {
        Self {
            voltage_mv: 0,
            faults: CellFaults::empty(),
        }
    }
}

bitflags! {
    /// Cell fault flags
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

/// Struct representing data collected from a single measurement cycle
pub struct Measurement {
    /// Cell 1 measurement
    pub cell_1: CellMeasurement,
    /// Cell 2 measurement
    pub cell_2: CellMeasurement,
    /// Cell 3 measurement
    pub cell_3: CellMeasurement,
    /// Cell 4 measurement
    #[cfg(feature = "4_cells")]
    pub cell_4: CellMeasurement,
    /// Cell 5 measurement
    #[cfg(feature = "5_cells")]
    pub cell_5: CellMeasurement,
    /// Sum of all cell voltages in millivo
    pub cell_sum_mv: u16,
    /// Battery voltage in millivolts
    pub vbat_mv: u16,
    /// VNTC measurement
    #[cfg(feature = "ntc")]
    pub ntc_mv: u16,
    /// Die temp in degrees Celsius
    pub die_temp: u16,
    /// Pack or BMS level faults
    pub pack_faults: PackFaults,
}

impl Default for Measurement {
    fn default() -> Self {
        Self {
            cell_1: CellMeasurement::default(),
            cell_2: CellMeasurement::default(),
            cell_3: CellMeasurement::default(),
            #[cfg(feature = "4_cells")]
            cell_4: CellMeasurement::default(),
            #[cfg(feature = "5_cells")]
            cell_5: CellMeasurement::default(),
            cell_sum_mv: 0,
            vbat_mv: 0,
            #[cfg(feature = "ntc")]
            ntc_mv: 0,
            die_temp: 0,
            pack_faults: PackFaults::empty(),
        }
    }
}

impl<I2C, I, O, const CELL_COUNT: u8> L9961<I2C, I, O, CELL_COUNT>
where
    I2C: I2c,
    I: Wait,
    O: OutputPin,
{
    /// Wait for the device to complete a measurement
    pub async fn make_measurement(&mut self, delay: &mut impl DelayNs) -> Result<(), I2C::Error> {
        let cycle_time: u32 =
            if let TMeasCycle::Period10ms(t) = self.config.measurement_cycles.get_t_meas_cycle() {
                t as u32 * 10
            } else {
                // If the measurement cycle isn't specified, wait for 300ms
                300
            };
        match select3(
            self.ready.wait_for_any_edge(),
            self.fault.wait_for_low(),
            delay.delay_ms(cycle_time),
        )
        .await
        {
            embassy_futures::select::Either3::First(result) => {
                result.unwrap();
                let mut measurement = Measurement::default();
                self.read_measurement_registers(&mut measurement).await?;
            }
            embassy_futures::select::Either3::Second(result) => {
                result.unwrap();
                let mut measurement = Measurement::default();
                self.read_fault_registers(&mut measurement).await?;
                self.clear_fault_registers().await?;
                self.read_measurement_registers(&mut measurement).await?;
                self.ready.wait_for_any_edge().await.unwrap();
            }
            embassy_futures::select::Either3::Third(()) => (),
        }

        Ok(())
    }

    async fn read_measurement_registers(
        &mut self,
        measurement: &mut Measurement,
    ) -> Result<(), I2C::Error> {
        let register_values = self.read_registers(Registers::VCell1, 9).await.unwrap();
        let vcell1 = VCell::new(1, register_values[0]);
        measurement.cell_1.voltage_mv =
            cell_voltage_measurement_mv_from_code(vcell1.get_vcell_meas_code());

        let vcell2 = VCell::new(2, register_values[1]);
        measurement.cell_2.voltage_mv =
            cell_voltage_measurement_mv_from_code(vcell2.get_vcell_meas_code());
        let vcell3 = VCell::new(3, register_values[2]);
        measurement.cell_3.voltage_mv =
            cell_voltage_measurement_mv_from_code(vcell3.get_vcell_meas_code());
        #[cfg(feature = "4_cells")]
        {
            let vcell4 = VCell::new(4, register_values[3]);
            measurement.cell_4.voltage_mv =
                cell_voltage_measurement_mv_from_code(vcell4.get_vcell_meas_code());
        }
        #[cfg(feature = "5_cells")]
        {
            let vcell5 = VCell::new(5, register_values[4]);
            measurement.cell_5.voltage_mv =
                cell_voltage_measurement_mv_from_code(vcell5.get_vcell_meas_code());
        }
        let vcell_sum = VCellSum::from(register_values[5]);
        measurement.cell_sum_mv =
            cell_voltage_measurement_mv_from_code(vcell_sum.get_vcellsum_meas());
        let vbat = VB::from(register_values[6]);
        measurement.vbat_mv = pack_voltage_measurement_mv_from_code(vbat.get_vb_meas_code());
        #[cfg(feature = "ntc")]
        {
            let ntc = NtcGpio::from(register_values[7]);
            measurement.ntc_mv = ntc.get_ntc_meas_mv();
        }
        measurement.die_temp = DieTemp::from(register_values[8]).get_die_temp_celsius();
        #[cfg(feature = "coulomb_counting")]
        {
            let cc_registers = self.read_registers(Registers::CCInstMeas, 3).await.unwrap();
            let cc_inst_meas = cc_registers[0];
            let cc_acc_msb = cc_registers[1];
            let cc_inst_meas = CCAccLsbCntr::from(cc_registers[2]);
        }
        Ok(())
    }

    /// Update the measurement with fault registers
    async fn read_fault_registers(
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

    async fn clear_fault_registers(&mut self) -> Result<(), I2C::Error> {
        self.write_diag_ov_ot_ut(DiagOvOtUt::all()).await?;
        self.write_diag_uv(DiagUv::all()).await?;
        self.write_diag_curr(DiagCurr::all()).await
    }
}
