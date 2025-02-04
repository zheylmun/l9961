use core::ops::Deref;

use defmt::debug_assert;

/// The device address register
pub struct DevAddr(u16);

impl DevAddr {
    /// Get the configured device_address
    #[inline]
    pub fn get_device_address(&self) -> u8 {
        (self.0 & 0b01111111) as u8
    }

    /// Set the device_address
    #[inline]
    pub fn set_device_address(&mut self, device_address: u8) {
        debug_assert!(
            device_address & 0b01111111 == device_address,
            "Invalid device address"
        );
        self.0 = device_address as u16;
    }
}

impl Deref for DevAddr {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for DevAddr {
    fn from(id: u16) -> Self {
        debug_assert!(id & 0b01111111 == id, "Invalid device address");
        Self(id)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for DevAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Device Address: {:#02x}", self.get_device_address(),)
    }
}
