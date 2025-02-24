//! The measurement module contains the entry point for periodically measuring the cell voltages and temperatures,
//! as well as initiating the fault handling process should faults occur during measurement.

use crate::registers::VCell;
pub use crate::{
    registers::{
        Cfg1FiltersCycles, DiagCurr, DiagOvOtUt, DiagUv, TCellFilter, TCurFilter, TSCFilter,
        ToFaultnMsk, ToFuseRstMask, ToPrdrvBalMask,
    },
    Registers, L9961,
};
use embassy_futures::select::select3;

use embedded_hal::digital::OutputPin;
use embedded_hal_async::{delay::DelayNs, digital::Wait, i2c::I2c};

impl<I2C, I, O, const CELL_COUNT: u8> L9961<I2C, I, O, CELL_COUNT>
where
    I2C: I2c,
    I: Wait,
    O: OutputPin,
{
    /// Wait for the device to complete a measurement
    pub async fn make_measurement(&mut self, delay: &mut impl DelayNs) -> Result<(), I2C::Error> {
        match select3(
            self.ready.wait_for_any_edge(),
            self.fault.wait_for_low(),
            delay.delay_ms(305),
        )
        .await
        {
            embassy_futures::select::Either3::First(result) => {
                result.unwrap();
                self.read_measurement_registers().await?;
            }
            embassy_futures::select::Either3::Second(result) => {
                result.unwrap();
                self.read_fault_registers().await?;
                self.clear_fault_registers().await?;
                self.read_measurement_registers().await?;
                self.ready.wait_for_any_edge().await.unwrap();
            }
            embassy_futures::select::Either3::Third(()) => (),
        }

        Ok(())
    }

    async fn read_measurement_registers(&mut self) -> Result<(), I2C::Error> {
        let register_values = self.read_registers(Registers::VCell1, 9).await.unwrap();
        let vcell1 = VCell::new(1, register_values[0]);
        let vcell2 = VCell::new(2, register_values[1]);
        let vcell3 = VCell::new(3, register_values[2]);
        let vcell4 = VCell::new(4, register_values[3]);
        let vcell5 = VCell::new(5, register_values[4]);
        defmt::info!(
            "{},\n{},\n{},\n{},\n{}",
            vcell1,
            vcell2,
            vcell3,
            vcell4,
            vcell5
        );
        Ok(())
    }

    async fn read_fault_registers(&mut self) -> Result<(DiagOvOtUt, DiagUv, DiagCurr), I2C::Error> {
        let register_values = self.read_registers(Registers::DiagOvOtUt, 2).await.unwrap();
        let diag_1 = DiagOvOtUt::from_bits_truncate(register_values[0]);
        let diag_2 = DiagUv::from_bits_truncate(register_values[1]);
        let diag3 = self.read_diag_curr().await?;
        defmt::info!(
            "FAULT:{{
  DiagOvOtUt: {},
  DiagUv: {},
  DiagCurr: {},
  }}",
            diag_1,
            diag_2,
            diag3
        );

        Ok((diag_1, diag_2, diag3))
    }

    async fn clear_fault_registers(&mut self) -> Result<(), I2C::Error> {
        self.write_diag_ov_ot_ut(DiagOvOtUt::all()).await?;
        self.write_diag_uv(DiagUv::all()).await?;
        self.write_diag_curr(DiagCurr::all()).await
    }
}
