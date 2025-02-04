//! # L9961 Industrial BMS Driver

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use registers::{Cfg3Act, ChipID, Registers};

pub mod registers;

/// L9961 Industrial BMS Driver
#[maybe_async_cfg::maybe(sync(keep_self), async(feature = "async"))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct L9961<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> L9961<I2C>
where
    I2C: embedded_hal::i2c::I2c,
{
    /// Create a new instance of the ST L9961 driver for the given blocking I2C bus and address.
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }

    /// Read a register from the L9961.
    pub fn read_register(&mut self, register: Registers) -> Result<u16, I2C::Error> {
        let mut buffer = [0, 0];
        self.i2c
            .write_read(self.address, &[register as u8], &mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }

    /// Write a new value to a register on the l9961
    pub fn write_register(&mut self, register: Registers, value: u16) -> Result<(), I2C::Error> {
        let buffer = value.to_be_bytes();
        self.i2c
            .write(self.address, &[register as u8, buffer[0], buffer[1]])?;
        Ok(())
    }

    /// Read the chip ID.
    pub fn read_chip_id(&mut self) -> Result<ChipID, I2C::Error> {
        Ok(self.read_register(Registers::ChipID)?.into())
    }

    /// Read the Cfg3Act register
    pub fn read_cfg3_act(&mut self) -> Result<Cfg3Act, I2C::Error> {
        Ok(self.read_register(Registers::Cfg3Act)?.into())
    }

    /// Write a new value to the Cfg3Act register
    pub fn write_cfg3_act(&mut self, new_config: Cfg3Act) -> Result<(), I2C::Error> {
        self.write_register(Registers::Cfg3Act, *new_config)
    }
}
