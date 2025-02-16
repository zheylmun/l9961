//! # L9961 Industrial BMS Driver

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

pub mod registers;

/// L9961 Industrial BMS Driver
#[maybe_async_cfg::maybe(sync(keep_self), async(feature = "async"))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct L9961<I2C> {
    i2c: I2C,
    address: u8,
}

#[cfg(not(feature = "async"))]
impl<I2C> L9961<I2C>
where
    I2C: embedded_hal::i2c::I2c,
{
    /// Create a new instance of the ST L9961 driver for the given blocking I2C bus and address.
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }
}

#[cfg(feature = "async")]
impl<I2C: embedded_hal_async::i2c::I2c> L9961<I2C> {
    /// Create a new instance of the ST L9961 driver for the given async I2C bus and address.
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }
}
