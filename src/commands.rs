//! # Commands
//! Functions to interact with the L9961 device commands.
//! Note that these functions overlap with the register definitions, but are broken out due to the
//! higher level abstraction of the device commands.

use crate::{registers::Registers, L9961};

/// NVM command value to upload configuration to NVM
const NVM_WRITE_READ_CODE_CMD_UPLOAD: u16 = 0xAAAA;

/// NVM command value to load configuration from NVM
const NVM_WRITE_READ_CODE_CMD_DOWNLOAD: u16 = 0x5555;

/// Register value for GO2 commands
const CMD_VAL: u16 = 0x2000;

#[cfg(not(feature = "async"))]
impl<I2C> L9961<I2C>
where
    I2C: embedded_hal::i2c::I2c,
{
    /// Download the stored device configuration from NVM
    pub fn download_configuration_from_nvm(&mut self) -> Result<(), I2C::Error> {
        self.write_register(Registers::Nvm2, NVM_WRITE_READ_CODE_CMD_DOWNLOAD)
    }

    /// Upload the current device configuration to NVM
    pub fn upload_configuration_to_nvm(&mut self) -> Result<(), I2C::Error> {
        self.write_register(Registers::Nvm2, NVM_WRITE_READ_CODE_CMD_UPLOAD)
    }

    /// Send the GO2SHIP command to the device
    pub fn go_2_ship(&mut self) -> Result<(), I2C::Error> {
        self.write_register(Registers::VCell1, CMD_VAL)
    }

    /// Send the GO2STBY command to the device
    pub fn go_2_stby(&mut self) -> Result<(), I2C::Error> {
        self.write_register(Registers::VCell2, CMD_VAL)
    }

    /// Arm the fuse trigger
    pub fn fuse_trig_arm(&mut self) -> Result<(), I2C::Error> {
        self.write_register(Registers::VCell3, CMD_VAL)
    }

    /// Fire the fuse trigger if arm state has not expired
    /// **WARNING** this will blow the fuse and permanently disconnect the battery
    pub fn fuse_trig_fire(&mut self) -> Result<(), I2C::Error> {
        self.write_register(Registers::VCell4, CMD_VAL)
    }
}

#[cfg(feature = "async")]
impl<I2C: embedded_hal_async::i2c::I2c> L9961<I2C> {
    /// Create a new instance of the ST L9961 driver for the given async I2C bus and address.
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }
}
