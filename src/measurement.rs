//! The measurement module contains the entry point for periodically measuring the cell voltages and temperatures,
//! as well as initiating the fault handling process should faults occur during measurement.

#[cfg(feature = "ntc")]
use crate::registers::NtcGpio;
use crate::{
    conversions::{cell_voltage_measurement_mv_from_code, pack_voltage_measurement_mv_from_code},
    faults::{CellFaults, PackFaults},
    registers::{DieTemp, TMeasCycle, VCell, VCellSum, VB},
    Registers, L9961,
};

use defmt::info;
use embassy_futures::select::select3;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::{delay::DelayNs, digital::Wait, i2c::I2c};

/// A single cell measurement
#[derive(Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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

/// Struct representing data collected from a single measurement cycle
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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

impl<I2C, I, O> L9961<I2C, I, O>
where
    I2C: I2c,
    I: Wait,
    O: OutputPin,
{
    /// Wait for the device to complete a measurement
    pub async fn make_measurement(
        &mut self,
        delay: &mut impl DelayNs,
    ) -> Result<Option<Measurement>, I2C::Error> {
        let cycle_time = self
            .config
            .measurement_cycles
            .get_t_meas_cycle()
            .period_ms();

        match select3(
            self.ready.wait_for_any_edge(),
            self.fault.wait_for_low(),
            delay.delay_ms(cycle_time as u32),
        )
        .await
        {
            embassy_futures::select::Either3::First(result) => {
                result.unwrap();
                let mut measurement = Measurement::default();
                self.read_measurement_registers(&mut measurement).await?;
                Ok(Some(measurement))
            }
            embassy_futures::select::Either3::Second(result) => {
                result.unwrap();
                let mut measurement = Measurement::default();
                self.read_fault_registers(&mut measurement).await?;
                self.clear_fault_registers().await?;
                self.read_measurement_registers(&mut measurement).await?;
                self.ready.wait_for_any_edge().await.unwrap();
                Ok(Some(measurement))
            }
            embassy_futures::select::Either3::Third(()) => {
                info!("Timed out after waiting for {}ms", cycle_time);
                Ok(None)
            }
        }
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
}
