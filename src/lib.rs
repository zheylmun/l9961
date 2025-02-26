//! # L9961 Industrial BMS Driver
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

pub mod commands;
pub mod config;
pub mod conversions;
pub mod faults;
pub mod measurement;
pub mod registers;

pub use config::Config;
pub use registers::Registers;

use registers::{
    Cfg1FiltersCycles, DiagCurr, DiagOvOtUt, DiagUv, ToFaultnMsk, ToFuseRstMask, ToPrdrvBalMask,
};

use embassy_futures::select::select;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::{delay::DelayNs, digital::Wait, i2c::I2c};

/// Number of cells in the battery pack
/// This is used to compile out unnecessary variables and functionality
#[cfg(not(feature = "4_cells"))]
pub(crate) const CELL_COUNT: u8 = 3;
#[cfg(all(feature = "4_cells", not(feature = "5_cells")))]
pub(crate) const CELL_COUNT: u8 = 4;
#[cfg(all(feature = "5_cells", feature = "4_cells"))]
pub(crate) const CELL_COUNT: u8 = 5;

/// L9961 Industrial BMS Driver
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct L9961<I2C, I, O> {
    i2c: I2C,
    ready: I,
    fault: I,
    wake: O,
    config: Config,
    //keep a large enough buffer to read measurement run of 9 registers
    // write address + register + read address + (2 bytes + crc * each register)
    i2c_scratch_buffer: [u8; 30],
    // result buffer
    i2c_results: [u16; 9],
}

impl<I2C, I, O> L9961<I2C, I, O>
where
    I2C: I2c,
    I: Wait,
    O: OutputPin,
{
    /// Create a new instance of the ST L9961 driver for the given blocking I2C bus and address.
    pub fn new(i2c: I2C, ready: I, fault: I, wakeup: O, config: Config) -> Self {
        Self {
            i2c,
            ready,
            fault,
            wake: wakeup,
            config,
            i2c_scratch_buffer: [0; 30],
            i2c_results: [0; 9],
        }
    }

    /// Wake up the l9961 if it is asleep
    pub async fn wake_if_asleep(&mut self, delay: &mut impl DelayNs) {
        self.wake.set_high().unwrap();
        match select(self.ready.wait_for_any_edge(), delay.delay_ms(100)).await {
            embassy_futures::select::Either::First(result) => result.unwrap(),
            embassy_futures::select::Either::Second(_) => (),
        }
        self.wake.set_low().unwrap();
    }

    /// Ensure that the device is in standby mode
    pub async fn disable_measurements(&mut self) -> Result<(), I2C::Error> {
        // Setting the cycle period to 0 disables all measurement
        self.write_cfg1_filters_cycles(Cfg1FiltersCycles::deactivate())
            .await
    }

    /// Enable the measurement cycle
    pub async fn enable_measurements(&mut self) -> Result<(), I2C::Error> {
        self.write_cfg1_filters_cycles(self.config.measurement_cycles)
            .await
    }

    /// Clear all fault registers
    pub async fn clear_all_faults(&mut self) -> Result<(), I2C::Error> {
        self.write_diag_ov_ot_ut(DiagOvOtUt::all()).await?;
        self.write_diag_curr(DiagCurr::all()).await?;
        self.write_diag_uv(DiagUv::all()).await
    }

    /// Mask all fault assertions for development purposes
    pub async fn mask_all_faults(&mut self) -> Result<(), I2C::Error> {
        self.write_to_faultn_msk(ToFaultnMsk::all()).await?;
        self.write_to_prdrv_bal_mask(ToPrdrvBalMask::all()).await?;
        self.write_to_fuse_rst_msk(ToFuseRstMask::all()).await
    }
}
