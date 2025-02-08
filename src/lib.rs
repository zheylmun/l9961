//! # L9961 Industrial BMS Driver

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use registers::{
    Cfg1FiltersCycles, Cfg2Enables, Cfg3Act, ChipID, CsaGainFactor, DevAddr, OvCThresholds,
    Registers, VBOvTh, VBSumMaxDiffTh, VBUvTh, VCellBalUvDeltaTh, VCellOvTh, VCellSevereDeltaThrs,
    VCellUvTh, VNTCOTTh, VNTCSevereOTTh, VNTCUTTh,
};

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

    /// Read the Cfg1FiltersCycles register
    pub fn read_cfg1_filters_cycles(&mut self) -> Result<Cfg1FiltersCycles, I2C::Error> {
        Ok(self.read_register(Registers::Cfg1FiltersCycles)?.into())
    }

    /// Write a new value to the Cfg1FiltersCycles register
    pub fn write_cfg1_filters_cycles(
        &mut self,
        new_config: Cfg1FiltersCycles,
    ) -> Result<(), I2C::Error> {
        self.write_register(Registers::Cfg1FiltersCycles, *new_config)
    }

    /// Read the Device Address Register
    pub fn read_device_address(&mut self) -> Result<DevAddr, I2C::Error> {
        Ok(self.read_register(Registers::DevAddr)?.into())
    }

    /// Write a new value to the Device Address Register
    pub fn write_device_address(&mut self, new_config: DevAddr) -> Result<(), I2C::Error> {
        self.write_register(Registers::DevAddr, *new_config)
    }

    /// Read the Cfg2Enables register
    pub fn read_cfg2_enables(&mut self) -> Result<Cfg2Enables, I2C::Error> {
        Ok(self.read_register(Registers::Cfg2Enables)?.into())
    }

    /// Write a new value to the Cfg2Enables register
    pub fn write_cfg2_enables(&mut self, new_config: Cfg2Enables) -> Result<(), I2C::Error> {
        self.write_register(Registers::Cfg2Enables, *new_config)
    }

    /// Read the CSA (Current Sense ADC) gain factor register
    pub fn read_csa_gain_factor(&mut self) -> Result<CsaGainFactor, I2C::Error> {
        Ok(self.read_register(Registers::CsaGainFactor)?.into())
    }

    /// Write a new value to the CSA (Current Sense ADC) gain factor register
    pub fn write_csa_gain_factor(&mut self, new_config: CsaGainFactor) -> Result<(), I2C::Error> {
        self.write_register(Registers::CsaGainFactor, *new_config)
    }

    /// Read the VCell Ov Threshold register
    pub fn read_vcell_ov_th(&mut self) -> Result<VCellOvTh, I2C::Error> {
        Ok(self.read_register(Registers::VCellOvTh)?.into())
    }

    /// Write a new value to the VCellOv Threshold register
    pub fn write_vcell_ov_th(&mut self, new_config: VCellOvTh) -> Result<(), I2C::Error> {
        self.write_register(Registers::VCellOvTh, *new_config)
    }

    /// Read the VCellUv Threshold register
    pub fn read_vcell_uv_th(&mut self) -> Result<VCellUvTh, I2C::Error> {
        Ok(self.read_register(Registers::VCellUvTh)?.into())
    }

    /// Write a new value to the VCellUv Threshold register
    pub fn write_vcell_uv_th(&mut self, new_config: VCellUvTh) -> Result<(), I2C::Error> {
        self.write_register(Registers::VCellUvTh, *new_config)
    }

    /// Read the VCellSevereDeltaThrs register
    pub fn read_vcell_severe_delta_thrs(&mut self) -> Result<VCellSevereDeltaThrs, I2C::Error> {
        Ok(self.read_register(Registers::VCellSevereDeltaThrs)?.into())
    }

    /// Write a new value to the VCellSevereDeltaThrs register
    pub fn write_vcell_severe_delta_thrs(
        &mut self,
        new_config: VCellSevereDeltaThrs,
    ) -> Result<(), I2C::Error> {
        self.write_register(Registers::VCellSevereDeltaThrs, *new_config)
    }

    /// Read the VCellBalUvDeltaTh register
    pub fn read_vcell_bal_uv_delta_th(&mut self) -> Result<VCellBalUvDeltaTh, I2C::Error> {
        Ok(self.read_register(Registers::VCellBalUvDeltaTh)?.into())
    }

    /// Write a new value to the VCellBalUvDeltaTh register
    pub fn write_vcell_bal_uv_delta_th(
        &mut self,
        new_config: VCellBalUvDeltaTh,
    ) -> Result<(), I2C::Error> {
        self.write_register(Registers::VCellBalUvDeltaTh, *new_config)
    }

    /// Read the VBOvTh register
    pub fn read_vb_ov_th(&mut self) -> Result<VBOvTh, I2C::Error> {
        Ok(self.read_register(Registers::VBOvTh)?.into())
    }

    /// Write a new value to the VBOvTh register
    pub fn write_vb_ov_th(&mut self, new_config: VBOvTh) -> Result<(), I2C::Error> {
        self.write_register(Registers::VBOvTh, *new_config)
    }

    /// Read the VBUvTh register
    pub fn read_vb_uv_th(&mut self) -> Result<VBUvTh, I2C::Error> {
        Ok(self.read_register(Registers::VBUvTh)?.into())
    }

    /// Write a new value to the VBUvTh register
    pub fn write_vb_uv_th(&mut self, new_config: VBUvTh) -> Result<(), I2C::Error> {
        self.write_register(Registers::VBUvTh, *new_config)
    }

    /// Read the VBSumMaxDiffTh register
    pub fn read_vb_sum_max_diff_th(&mut self) -> Result<VBSumMaxDiffTh, I2C::Error> {
        Ok(self.read_register(Registers::VBSumMaxDiffTh)?.into())
    }

    /// Write a new value to the VBSumMaxDiffTh register
    pub fn write_vb_sum_max_diff_th(
        &mut self,
        new_config: VBSumMaxDiffTh,
    ) -> Result<(), I2C::Error> {
        self.write_register(Registers::VBSumMaxDiffTh, *new_config)
    }

    /// Read the VNTCOTTh register
    pub fn read_vntc_ot_th(&mut self) -> Result<VNTCOTTh, I2C::Error> {
        Ok(self.read_register(Registers::VNTCOTTh)?.into())
    }

    /// Write a new value to the VNTCOTTh register
    pub fn write_vntc_ot_th(&mut self, new_config: VNTCOTTh) -> Result<(), I2C::Error> {
        self.write_register(Registers::VNTCOTTh, *new_config)
    }

    /// Read the VNTCUTTh register
    pub fn read_vntc_ut_th(&mut self) -> Result<VNTCUTTh, I2C::Error> {
        Ok(self.read_register(Registers::VNTCUTTh)?.into())
    }

    /// Write a new value to the VNTCUTTh register
    pub fn write_vntc_ut_th(&mut self, new_config: VNTCUTTh) -> Result<(), I2C::Error> {
        self.write_register(Registers::VNTCUTTh, *new_config)
    }

    /// Read the VNTCSevereOtTh register
    pub fn read_vntc_severe_ot_th(&mut self) -> Result<VNTCSevereOTTh, I2C::Error> {
        Ok(self.read_register(Registers::VNTCSevereOTTh)?.into())
    }

    /// Write a new value to the VNTCSevereOtTh register
    pub fn write_vntc_severe_ot_th(
        &mut self,
        new_config: VNTCSevereOTTh,
    ) -> Result<(), I2C::Error> {
        self.write_register(Registers::VNTCSevereOTTh, *new_config)
    }

    /// Read the OVC_THRESHOLDS register
    pub fn read_ovc_thresholds(&mut self) -> Result<OvCThresholds, I2C::Error> {
        Ok(self.read_register(Registers::OvCThresholds)?.into())
    }

    /// Write a new value to the OVC_THRESHOLDS register
    pub fn write_ovc_thresholds(&mut self, new_config: OvCThresholds) -> Result<(), I2C::Error> {
        self.write_register(Registers::OvCThresholds, *new_config)
    }
}
