//! # L9961 Industrial BMS Driver
//!
//! This driver aims to provide a robust API for building battery packs utilizing the [STMicro L9961 BMS Chip](https://www.st.com/en/power-management/l9961.html).
//! The crate is built on top of the [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) and [`embedded-hal-async`](https://github.com/rust-embedded/embedded-hal/tree/master/embedded-hal-async)  traits.
//! The driver requires an async runtime to function, and has been tested with both [Embassy](https://embassy.dev) as well as [RTIC 2.x](https://rtic.rs).

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
    pub fn new(i2c: I2C, ready: I, fault: I, wake: O, config: Config) -> Self {
        Self {
            i2c,
            ready,
            fault,
            wake,
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
