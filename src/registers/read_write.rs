use super::{
    CCAccLsbCntr, Cfg1FiltersCycles, Cfg2Enables, Cfg3Act, ChipID, CsaGainFactor, CurrMsk, DevAddr,
    DiagCurr, DiagOvOtUt, DiagUv, DieTemp, NtcGpio, OvCThresholds, PersistentOvCThresholds,
    Registers, SCThreshold, ToFaultnMsk, ToFuseRstMask, ToPrdrvBalMask, VB, VBOvTh, VBSumMaxDiffTh,
    VBUvTh, VCell, VCell1Faults, VCellBalUvDeltaTh, VCellOvTh, VCellSevereDeltaThrs, VCellSum,
    VCellUvTh, VNTCOTTh, VNTCSevereOTTh, VNTCUTTh,
};

use crate::L9961;

#[cfg(feature = "is_sync")]
use embedded_hal::i2c::I2c;
#[cfg(not(feature = "is_sync"))]
use embedded_hal_async::i2c::I2c;

use maybe_async::maybe_async;

impl<I2C> L9961<I2C>
where
    I2C: I2c,
{
    /// Read a register from the L9961.
    #[maybe_async]
    pub async fn read_register(&mut self, register: Registers) -> Result<u16, I2C::Error> {
        let mut buffer = [0, 0];
        self.i2c
            .write_read(self.address, &[register as u8], &mut buffer)
            .await?;
        Ok(u16::from_be_bytes(buffer))
    }

    /// Write a new value to a register on the l9961
    #[maybe_async]
    pub async fn write_register(
        &mut self,
        register: Registers,
        value: u16,
    ) -> Result<(), I2C::Error> {
        let buffer = value.to_be_bytes();
        self.i2c
            .write(self.address, &[register as u8, buffer[0], buffer[1]])
            .await?;
        Ok(())
    }

    /// Read the chip ID.
    #[maybe_async]
    pub async fn read_chip_id(&mut self) -> Result<ChipID, I2C::Error> {
        Ok(self.read_register(Registers::ChipID).await?.into())
    }

    /// Read the Cfg3Act register
    #[maybe_async]
    pub async fn read_cfg3_act(&mut self) -> Result<Cfg3Act, I2C::Error> {
        Ok(self.read_register(Registers::Cfg3Act).await?.into())
    }

    /// Write a new value to the Cfg3Act register
    #[maybe_async]
    pub async fn write_cfg3_act(&mut self, new_config: Cfg3Act) -> Result<(), I2C::Error> {
        self.write_register(Registers::Cfg3Act, *new_config).await
    }

    /// Read the Cfg1FiltersCycles register
    #[maybe_async]
    pub async fn read_cfg1_filters_cycles(&mut self) -> Result<Cfg1FiltersCycles, I2C::Error> {
        Ok(self
            .read_register(Registers::Cfg1FiltersCycles)
            .await?
            .into())
    }

    /// Write a new value to the Cfg1FiltersCycles register
    #[maybe_async]
    pub async fn write_cfg1_filters_cycles(
        &mut self,
        new_config: Cfg1FiltersCycles,
    ) -> Result<(), I2C::Error> {
        self.write_register(Registers::Cfg1FiltersCycles, *new_config)
            .await
    }

    /// Read the Device Address Register
    #[maybe_async]
    pub async fn read_device_address(&mut self) -> Result<DevAddr, I2C::Error> {
        Ok(self.read_register(Registers::DevAddr).await?.into())
    }

    /// Write a new value to the Device Address Register
    /// Note that this will also update the internal I2C address of the `L9961` driver upon success
    #[maybe_async]
    pub async fn write_device_address(&mut self, new_config: DevAddr) -> Result<(), I2C::Error> {
        self.write_register(Registers::DevAddr, *new_config).await?;
        self.address = new_config.get_device_address();
        Ok(())
    }

    /// Read the Cfg2Enables register
    #[maybe_async]
    pub async fn read_cfg2_enables(&mut self) -> Result<Cfg2Enables, I2C::Error> {
        Ok(self.read_register(Registers::Cfg2Enables).await?.into())
    }

    /// Write a new value to the Cfg2Enables register
    #[maybe_async]
    pub async fn write_cfg2_enables(&mut self, new_config: Cfg2Enables) -> Result<(), I2C::Error> {
        self.write_register(Registers::Cfg2Enables, *new_config)
            .await
    }

    /// Read the CSA (Current Sense ADC) gain factor register
    #[maybe_async]
    pub async fn read_csa_gain_factor(&mut self) -> Result<CsaGainFactor, I2C::Error> {
        Ok(self.read_register(Registers::CsaGainFactor).await?.into())
    }

    /// Write a new value to the CSA (Current Sense ADC) gain factor register
    #[maybe_async]
    pub async fn write_csa_gain_factor(
        &mut self,
        new_config: CsaGainFactor,
    ) -> Result<(), I2C::Error> {
        self.write_register(Registers::CsaGainFactor, *new_config)
            .await
    }

    /// Read the VCell Ov Threshold register
    #[maybe_async]
    pub async fn read_vcell_ov_th(&mut self) -> Result<VCellOvTh, I2C::Error> {
        Ok(self.read_register(Registers::VCellOvTh).await?.into())
    }

    /// Write a new value to the VCellOv Threshold register
    #[maybe_async]
    pub async fn write_vcell_ov_th(&mut self, new_config: VCellOvTh) -> Result<(), I2C::Error> {
        self.write_register(Registers::VCellOvTh, *new_config).await
    }

    /// Read the VCellUv Threshold register
    #[maybe_async]
    pub async fn read_vcell_uv_th(&mut self) -> Result<VCellUvTh, I2C::Error> {
        Ok(self.read_register(Registers::VCellUvTh).await?.into())
    }

    /// Write a new value to the VCellUv Threshold register
    #[maybe_async]
    pub async fn write_vcell_uv_th(&mut self, new_config: VCellUvTh) -> Result<(), I2C::Error> {
        self.write_register(Registers::VCellUvTh, *new_config).await
    }

    /// Read the VCellSevereDeltaThrs register
    #[maybe_async]
    pub async fn read_vcell_severe_delta_thrs(
        &mut self,
    ) -> Result<VCellSevereDeltaThrs, I2C::Error> {
        Ok(self
            .read_register(Registers::VCellSevereDeltaThrs)
            .await?
            .into())
    }

    /// Write a new value to the VCellSevereDeltaThrs register
    #[maybe_async]
    pub async fn write_vcell_severe_delta_thrs(
        &mut self,
        new_config: VCellSevereDeltaThrs,
    ) -> Result<(), I2C::Error> {
        self.write_register(Registers::VCellSevereDeltaThrs, *new_config)
            .await
    }

    /// Read the VCellBalUvDeltaTh register
    #[maybe_async]
    pub async fn read_vcell_bal_uv_delta_th(&mut self) -> Result<VCellBalUvDeltaTh, I2C::Error> {
        Ok(self
            .read_register(Registers::VCellBalUvDeltaTh)
            .await?
            .into())
    }

    /// Write a new value to the VCellBalUvDeltaTh register
    #[maybe_async]
    pub async fn write_vcell_bal_uv_delta_th(
        &mut self,
        new_config: VCellBalUvDeltaTh,
    ) -> Result<(), I2C::Error> {
        self.write_register(Registers::VCellBalUvDeltaTh, *new_config)
            .await
    }

    /// Read the VBOvTh register
    #[maybe_async]
    pub async fn read_vb_ov_th(&mut self) -> Result<VBOvTh, I2C::Error> {
        Ok(self.read_register(Registers::VBOvTh).await?.into())
    }

    /// Write a new value to the VBOvTh register
    #[maybe_async]
    pub async fn write_vb_ov_th(&mut self, new_config: VBOvTh) -> Result<(), I2C::Error> {
        self.write_register(Registers::VBOvTh, *new_config).await
    }

    /// Read the VBUvTh register
    #[maybe_async]
    pub async fn read_vb_uv_th(&mut self) -> Result<VBUvTh, I2C::Error> {
        Ok(self.read_register(Registers::VBUvTh).await?.into())
    }

    /// Write a new value to the VBUvTh register
    #[maybe_async]
    pub async fn write_vb_uv_th(&mut self, new_config: VBUvTh) -> Result<(), I2C::Error> {
        self.write_register(Registers::VBUvTh, *new_config).await
    }

    /// Read the VBSumMaxDiffTh register
    #[maybe_async]
    pub async fn read_vb_sum_max_diff_th(&mut self) -> Result<VBSumMaxDiffTh, I2C::Error> {
        Ok(self.read_register(Registers::VBSumMaxDiffTh).await?.into())
    }

    /// Write a new value to the VBSumMaxDiffTh register
    #[maybe_async]
    pub async fn write_vb_sum_max_diff_th(
        &mut self,
        new_config: VBSumMaxDiffTh,
    ) -> Result<(), I2C::Error> {
        self.write_register(Registers::VBSumMaxDiffTh, *new_config)
            .await
    }

    /// Read the VNTCOTTh register
    #[maybe_async]
    pub async fn read_vntc_ot_th(&mut self) -> Result<VNTCOTTh, I2C::Error> {
        Ok(self.read_register(Registers::VNTCOTTh).await?.into())
    }

    /// Write a new value to the VNTCOTTh register
    #[maybe_async]
    pub async fn write_vntc_ot_th(&mut self, new_config: VNTCOTTh) -> Result<(), I2C::Error> {
        self.write_register(Registers::VNTCOTTh, *new_config).await
    }

    /// Read the VNTCUTTh register
    #[maybe_async]
    pub async fn read_vntc_ut_th(&mut self) -> Result<VNTCUTTh, I2C::Error> {
        Ok(self.read_register(Registers::VNTCUTTh).await?.into())
    }

    /// Write a new value to the VNTCUTTh register
    #[maybe_async]
    pub async fn write_vntc_ut_th(&mut self, new_config: VNTCUTTh) -> Result<(), I2C::Error> {
        self.write_register(Registers::VNTCUTTh, *new_config).await
    }

    /// Read the VNTCSevereOtTh register
    #[maybe_async]
    pub async fn read_vntc_severe_ot_th(&mut self) -> Result<VNTCSevereOTTh, I2C::Error> {
        Ok(self.read_register(Registers::VNTCSevereOTTh).await?.into())
    }

    /// Write a new value to the VNTCSevereOtTh register
    #[maybe_async]
    pub async fn write_vntc_severe_ot_th(
        &mut self,
        new_config: VNTCSevereOTTh,
    ) -> Result<(), I2C::Error> {
        self.write_register(Registers::VNTCSevereOTTh, *new_config)
            .await
    }

    /// Read the OVC_THRESHOLDS register
    #[maybe_async]
    pub async fn read_ovc_thresholds(&mut self) -> Result<OvCThresholds, I2C::Error> {
        Ok(self.read_register(Registers::OvCThresholds).await?.into())
    }

    /// Write a new value to the OVC_THRESHOLDS register
    #[maybe_async]
    pub async fn write_ovc_thresholds(
        &mut self,
        new_config: OvCThresholds,
    ) -> Result<(), I2C::Error> {
        self.write_register(Registers::OvCThresholds, *new_config)
            .await
    }

    /// Read the PERSISTENT_OVC_THRESHOLDS register
    #[maybe_async]
    pub async fn read_persistent_ovc_thresholds(
        &mut self,
    ) -> Result<PersistentOvCThresholds, I2C::Error> {
        Ok(self
            .read_register(Registers::PersistentOvCThresholds)
            .await?
            .into())
    }

    /// Write a new value to the PERSISTENT_OVC_THRESHOLDS register
    #[maybe_async]
    pub async fn write_persistent_ovc_thresholds(
        &mut self,
        new_config: PersistentOvCThresholds,
    ) -> Result<(), I2C::Error> {
        self.write_register(Registers::PersistentOvCThresholds, *new_config)
            .await
    }

    /// Read the SC_THRESHOLD register
    #[maybe_async]
    pub async fn read_sc_threshold(&mut self) -> Result<SCThreshold, I2C::Error> {
        Ok(self.read_register(Registers::SCThreshold).await?.into())
    }

    /// Write the SC_THRESHOLD register
    #[maybe_async]
    pub async fn write_sc_threshold(&mut self, new_config: SCThreshold) -> Result<(), I2C::Error> {
        self.write_register(Registers::SCThreshold, *new_config)
            .await
    }

    /// Read the TO_PRDRV_BAL_MASK register
    #[maybe_async]
    pub async fn read_to_prdrv_bal_mask(&mut self) -> Result<ToPrdrvBalMask, I2C::Error> {
        Ok(ToPrdrvBalMask::from_bits_truncate(
            self.read_register(Registers::ToPrdrvBalMask).await?,
        ))
    }

    /// Write the TO_PRDRV_BAL_MASK register
    #[maybe_async]
    pub async fn write_to_prdrv_bal_mask(
        &mut self,
        new_config: ToPrdrvBalMask,
    ) -> Result<(), I2C::Error> {
        self.write_register(Registers::ToPrdrvBalMask, new_config.bits())
            .await
    }

    /// Read the TO_FUSE_RST_MSK register
    #[maybe_async]
    pub async fn read_to_fuse_rst_msk(&mut self) -> Result<ToFuseRstMask, I2C::Error> {
        Ok(ToFuseRstMask::from_bits_truncate(
            self.read_register(Registers::ToFuseRstMask).await?,
        ))
    }

    /// Write the TO_FUSE_RST_MSK register
    #[maybe_async]
    pub async fn write_to_fuse_rst_msk(
        &mut self,
        new_config: ToFuseRstMask,
    ) -> Result<(), I2C::Error> {
        self.write_register(Registers::ToFuseRstMask, new_config.bits())
            .await
    }

    /// Read the TO_FAULTN_MSK register
    #[maybe_async]
    pub async fn read_to_faultn_msk(&mut self) -> Result<ToFaultnMsk, I2C::Error> {
        Ok(ToFaultnMsk::from_bits_truncate(
            self.read_register(Registers::ToFaultnMsk).await?,
        ))
    }

    /// Write the TO_FAULTN_MSK register
    #[maybe_async]
    pub async fn write_to_faultn_msk(&mut self, new_config: ToFaultnMsk) -> Result<(), I2C::Error> {
        self.write_register(Registers::ToFaultnMsk, new_config.bits())
            .await
    }

    /// Read the CURR_MSK register
    #[maybe_async]
    pub async fn read_curr_msk(&mut self) -> Result<CurrMsk, I2C::Error> {
        Ok(CurrMsk::from_bits_truncate(
            self.read_register(Registers::CurrMsk).await?,
        ))
    }

    /// Write the CURR_MSK register
    #[maybe_async]
    pub async fn write_curr_msk(&mut self, new_config: CurrMsk) -> Result<(), I2C::Error> {
        self.write_register(Registers::CurrMsk, new_config.bits())
            .await
    }

    /// Read the Manufacturer Name msb
    #[maybe_async]
    pub async fn read_manufacturer_name_msb(&mut self) -> Result<u16, I2C::Error> {
        Ok(self.read_register(Registers::ManufacturerNameMsb).await?)
    }

    /// Write the Manufacturer Name msb
    #[maybe_async]
    pub async fn write_manufacturer_name_msb(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.write_register(Registers::ManufacturerNameMsb, value)
            .await
    }

    /// Read the Manufacturer Name Lsb
    #[maybe_async]
    pub async fn read_manufacturer_name_lsb(&mut self) -> Result<u16, I2C::Error> {
        Ok(self.read_register(Registers::ManufacturerNameLsb).await?)
    }

    /// Write the Manufacturer Name lsb
    #[maybe_async]
    pub async fn write_manufacturer_name_lsb(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.write_register(Registers::ManufacturerNameLsb, value)
            .await
    }

    /// Read the Manufacturing Date Register
    #[maybe_async]
    pub async fn read_manufacturing_date(&mut self) -> Result<u16, I2C::Error> {
        Ok(self.read_register(Registers::ManufacturingDate).await?)
    }

    /// Write the Manufacturing date register
    #[maybe_async]
    pub async fn write_manufacturing_date(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.write_register(Registers::ManufacturingDate, value)
            .await
    }

    /// Read the First Usage Date Register
    #[maybe_async]
    pub async fn read_first_usage_date(&mut self) -> Result<u16, I2C::Error> {
        Ok(self.read_register(Registers::FirstUsageDate).await?)
    }

    /// Write the First Usage Date Register
    #[maybe_async]
    pub async fn write_first_usage_date(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.write_register(Registers::FirstUsageDate, value).await
    }

    /// Read the Serial Number MSB Register
    #[maybe_async]
    pub async fn read_serial_number_msb(&mut self) -> Result<u16, I2C::Error> {
        Ok(self.read_register(Registers::SerialNumberMsb).await?)
    }

    /// Write the Serial Number MSB Register
    #[maybe_async]
    pub async fn write_serial_number_msb(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.write_register(Registers::SerialNumberMsb, value).await
    }

    /// Read the Serial Number LSB Register
    #[maybe_async]
    pub async fn read_serial_number_lsb(&mut self) -> Result<u16, I2C::Error> {
        Ok(self.read_register(Registers::SerialNumberLsb).await?)
    }

    /// Write the Serial Number LSB Register
    #[maybe_async]
    pub async fn write_serial_number_lsb(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.write_register(Registers::SerialNumberLsb, value).await
    }

    /// Read the device name MSB register
    #[maybe_async]
    pub async fn read_device_name_msb(&mut self) -> Result<u16, I2C::Error> {
        Ok(self.read_register(Registers::DeviceNameMsb).await?)
    }

    /// Write the device name MSB register
    #[maybe_async]
    pub async fn write_device_name_msb(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.write_register(Registers::DeviceNameMsb, value).await
    }

    /// Read the device name LSB register
    #[maybe_async]
    pub async fn read_device_name_lsb(&mut self) -> Result<u16, I2C::Error> {
        Ok(self.read_register(Registers::DeviceNameLsb).await?)
    }

    /// Write the device name LSB register
    #[maybe_async]
    pub async fn write_device_name_lsb(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.write_register(Registers::DeviceNameLsb, value).await
    }

    /// Read the faults from the VCell 1 register
    #[maybe_async]
    pub async fn read_vcell_1_faults(&mut self) -> Result<VCell1Faults, I2C::Error> {
        Ok(self.read_register(Registers::VCell1).await?.into())
    }

    /// Read one of the 5 VCell registers (1 indexed per device)
    #[maybe_async]
    pub async fn read_vcell(&mut self, cell: u8) -> Result<VCell, I2C::Error> {
        let measurement = match cell {
            1 => self.read_register(Registers::VCell1).await?,
            2 => self.read_register(Registers::VCell2).await?,
            3 => self.read_register(Registers::VCell3).await?,
            4 => self.read_register(Registers::VCell4).await?,
            5 => self.read_register(Registers::VCell5).await?,
            _ => panic!("Attempt to read non-existent cell"),
        };
        Ok(VCell::new(cell, measurement))
    }

    /// Read the VCellSum measurement register
    #[maybe_async]
    pub async fn read_vcellsum(&mut self) -> Result<VCellSum, I2C::Error> {
        Ok(self.read_register(Registers::VCellSum).await?.into())
    }

    /// Read the VB measurement register
    #[maybe_async]
    pub async fn read_vb(&mut self) -> Result<VB, I2C::Error> {
        Ok(self.read_register(Registers::VB).await?.into())
    }

    /// Read the NTC_GPIO register
    #[maybe_async]
    pub async fn read_ntc_gpio(&mut self) -> Result<NtcGpio, I2C::Error> {
        Ok(self.read_register(Registers::NtcGpio).await?.into())
    }

    /// Read the Die Temperature register
    #[maybe_async]
    pub async fn read_die_temp(&mut self) -> Result<DieTemp, I2C::Error> {
        Ok(self.read_register(Registers::DieTemp).await?.into())
    }

    /// Read the DIAG_OV_OT_UT register
    #[maybe_async]
    pub async fn read_diag_ov_ot_ut(&mut self) -> Result<DiagOvOtUt, I2C::Error> {
        Ok(DiagOvOtUt::from_bits_truncate(
            self.read_register(Registers::DiagOvOtUt).await?,
        ))
    }

    /// Write a new value to the DIAG_OV_OT_UT register
    #[maybe_async]
    pub async fn write_diag_ov_ot_ut(&mut self, new_config: DiagOvOtUt) -> Result<(), I2C::Error> {
        self.write_register(Registers::DiagOvOtUt, new_config.bits())
            .await
    }

    /// Read the DIAG_UV register
    #[maybe_async]
    pub async fn read_diag_uv(&mut self) -> Result<DiagUv, I2C::Error> {
        Ok(DiagUv::from_bits_truncate(
            self.read_register(Registers::DiagUv).await?,
        ))
    }

    /// Write to the DIAG_UV register
    #[maybe_async]
    pub async fn write_diag_uv(&mut self, new_config: DiagUv) -> Result<(), I2C::Error> {
        self.write_register(Registers::DiagUv, new_config.bits())
            .await
    }

    /// Read the CC_INST_MEAS register
    #[maybe_async]
    pub async fn read_cc_inst_meas(&mut self) -> Result<u16, I2C::Error> {
        Ok(self.read_register(Registers::CCInstMeas).await?)
    }

    /// Read the CC_ACC_MSB register
    #[maybe_async]
    pub async fn read_cc_acc_msb(&mut self) -> Result<u16, I2C::Error> {
        Ok(self.read_register(Registers::CCAccMsb).await?)
    }

    /// Write the CC_ACC_MSB register
    #[maybe_async]
    pub async fn write_cc_acc_msb(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.write_register(Registers::CCAccMsb, value).await
    }

    /// Read the CC_ACC_LSB_CNTR register
    #[maybe_async]
    pub async fn read_cc_acc_lsb_cntr(&mut self) -> Result<CCAccLsbCntr, I2C::Error> {
        Ok(self.read_register(Registers::CCAccLsbCntr).await?.into())
    }

    /// Read the DIAG_CURR register
    #[maybe_async]
    pub async fn read_diag_curr(&mut self) -> Result<DiagCurr, I2C::Error> {
        Ok(DiagCurr::from_bits_truncate(
            self.read_register(Registers::DiagCurr).await?,
        ))
    }

    /// Write to the DIAG_CURR register
    #[maybe_async]
    pub async fn write_diag_curr(&mut self, new_config: DiagCurr) -> Result<(), I2C::Error> {
        self.write_register(Registers::DiagCurr, new_config.bits())
            .await
    }
}
