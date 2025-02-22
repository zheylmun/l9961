//! # Commands
//! Functions to interact with the L9961 device commands.
//! Note that these functions overlap with the register definitions, but are broken out due to the
//! higher level abstraction of the device commands.

use crate::{Registers, L9961};

use embedded_hal::digital::OutputPin;
use embedded_hal_async::{digital::Wait, i2c::I2c};
use maybe_async::maybe_async;

/// NVM command value to upload configuration to NVM
const NVM_WRITE_READ_CODE_CMD_UPLOAD: u16 = 0xAAAA;
/// NVM command value to load configuration from NVM
const NVM_WRITE_READ_CODE_CMD_DOWNLOAD: u16 = 0x5555;
/// Register value for GO2 commands
const CMD_VAL: u16 = 0x2000;

impl<I2C, I, O, const CELL_COUNT: u8> L9961<I2C, I, O, CELL_COUNT>
where
    I2C: I2c,
    I: Wait,
    O: OutputPin,
{
    /// Download the stored device configuration from NVM
    #[maybe_async]
    pub async fn download_configuration_from_nvm(&mut self) -> Result<(), I2C::Error> {
        self.write_register(Registers::Nvm2, NVM_WRITE_READ_CODE_CMD_DOWNLOAD)
            .await
    }

    /// Upload the current device configuration to NVM
    #[maybe_async]
    pub async fn upload_configuration_to_nvm(&mut self) -> Result<(), I2C::Error> {
        self.write_register(Registers::Nvm2, NVM_WRITE_READ_CODE_CMD_UPLOAD)
            .await
    }

    /// Send the GO2SHIP command to the device
    #[maybe_async]
    pub async fn go_2_ship(&mut self) -> Result<(), I2C::Error> {
        self.write_register(Registers::VCell1, CMD_VAL).await
    }

    /// Send the GO2STBY command to the device
    #[maybe_async]
    pub async fn go_2_standby(&mut self) -> Result<(), I2C::Error> {
        self.write_register(Registers::VCell2, CMD_VAL).await
    }

    /// Arm the fuse trigger
    #[maybe_async]
    pub async fn fuse_trig_arm(&mut self) -> Result<(), I2C::Error> {
        self.write_register(Registers::VCell3, CMD_VAL).await
    }

    /// Fire the fuse trigger if arm state has not expired
    /// **WARNING** this will blow the fuse and permanently disconnect the battery
    #[maybe_async]
    pub async fn fuse_trig_fire(&mut self) -> Result<(), I2C::Error> {
        self.write_register(Registers::VCell4, CMD_VAL).await
    }
}
