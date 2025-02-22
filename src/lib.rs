//! # L9961 Industrial BMS Driver
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

pub mod commands;
pub mod configuration;
pub mod conversions;
pub mod registers;

pub use registers::Registers;

use embedded_hal::digital::OutputPin;
use embedded_hal_async::{digital::Wait, i2c::I2c};

/// L9961 Industrial BMS Driver
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct L9961<I2C, I, O, const CELL_COUNT: u8 = 3> {
    i2c: I2C,
    ready: I,
    fault: I,
    wakeup: O,
    address: u8,
}

impl<I2C, I, O, const CELL_COUNT: u8> L9961<I2C, I, O, CELL_COUNT>
where
    I2C: I2c,
    I: Wait,
    O: OutputPin,
{
    /// Create a new instance of the ST L9961 driver for the given blocking I2C bus and address.
    pub fn new(i2c: I2C, ready: I, fault: I, wakeup: O, address: u8) -> Self {
        debug_assert!(CELL_COUNT >= 3 && CELL_COUNT <= 5);
        Self {
            i2c,
            ready,
            fault,
            wakeup,
            address,
        }
    }
}
