//! # Registers
//! Definitions and read/write functions for the registers of the L9961 chip

mod cc_acc_lsb_cntr;
mod cfg1_filters_cycles;
mod cfg2_enables;
mod cfg3_act;
mod chip_id;
mod csa_gain_factor;
mod curr_msk;
mod dev_addr;
mod diag_curr;
mod diag_ov_ot_ut;
mod diag_uv;
mod die_temp;
mod ntc_gpio;
mod ovc_thresholds;
mod persistent_ovc_thresholds;
mod sc_threshold;
mod to_faultn_msk;
mod to_fuse_rst_msk;
mod to_prdrv_bal_mask;
mod vb;
mod vb_ov_th;
mod vb_sum_max_diff_th;
mod vb_uv_th;
mod vcell;
mod vcell_1_faults;
mod vcell_bal_uv_delta_th;
mod vcell_ov_th;
mod vcell_severe_delta_thrs;
mod vcell_uv_th;
mod vcellsum;
mod vntc_ot_th;
mod vntc_severe_ot_th;
mod vntc_ut_th;

pub use self::{
    cc_acc_lsb_cntr::CCAccLsbCntr,
    cfg1_filters_cycles::{Cfg1FiltersCycles, TCellFilter, TCurFilter, TMeasCycle, TSCFilter},
    cfg2_enables::{Cfg2Enables, FetConfig},
    cfg3_act::Cfg3Act,
    chip_id::ChipID,
    csa_gain_factor::CsaGainFactor,
    curr_msk::CurrMsk,
    dev_addr::DevAddr,
    diag_curr::DiagCurr,
    diag_ov_ot_ut::DiagOvOtUt,
    diag_uv::DiagUv,
    die_temp::DieTemp,
    ntc_gpio::NtcGpio,
    ovc_thresholds::OvCThresholds,
    persistent_ovc_thresholds::PersistentOvCThresholds,
    sc_threshold::SCThreshold,
    to_faultn_msk::ToFaultnMsk,
    to_fuse_rst_msk::ToFuseRstMask,
    to_prdrv_bal_mask::ToPrdrvBalMask,
    vb::VB,
    vb_ov_th::VBOvTh,
    vb_sum_max_diff_th::VBSumMaxDiffTh,
    vb_uv_th::VBUvTh,
    vcell::VCell,
    vcell_1_faults::VCell1Faults,
    vcell_bal_uv_delta_th::VCellBalUvDeltaTh,
    vcell_ov_th::VCellOvTh,
    vcell_severe_delta_thrs::VCellSevereDeltaThrs,
    vcell_uv_th::VCellUvTh,
    vcellsum::VCellSum,
    vntc_ot_th::VNTCOTTh,
    vntc_severe_ot_th::VNTCSevereOTTh,
    vntc_ut_th::VNTCUTTh,
};

use crate::L9961;

use defmt::Format;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::{digital::Wait, i2c::I2c};

use maybe_async::maybe_async;

/// The registers of the L9961 chip represented as their addresses
#[derive(Clone, Copy, Debug, Format)]
#[repr(u8)]
pub enum Registers {
    /// The chip ID register
    ChipID = 0x00,
    /// The chip revision register
    Cfg3Act = 0x01,
    /// Configuration register for the filters and cycles
    Cfg1FiltersCycles = 0x02,
    /// The device address register
    DevAddr = 0x03,
    /// Configuration register for enables
    Cfg2Enables = 0x04,
    /// Configuration register for the CSA gain factor
    CsaGainFactor = 0x05,
    /// Configuration register for the VCELL overvoltage threshold
    VCellOvTh = 0x06,
    /// Configuration register for the VCELL undervoltage threshold
    VCellUvTh = 0x07,
    /// Configuration register for the VCELL severe undervoltage and overvoltage thresholds
    VCellSevereDeltaThrs = 0x08,
    /// Configuration register for the VCELL balancing undervoltage delta threshold
    VCellBalUvDeltaTh = 0x09,
    /// Configuration register for the VBAT overvoltage threshold
    VBOvTh = 0x0A,
    /// Configuration register for the VBAT undervoltage threshold
    VBUvTh = 0x0B,
    /// Configuration register for the VBAT sum max diff threshold
    VBSumMaxDiffTh = 0x0C,
    /// Configuration register for the VNTC over temperature threshold
    VNTCOTTh = 0x0D,
    /// Configuration register for the VNTC under temperature threshold
    VNTCUTTh = 0x0E,
    /// Configuration register for the VNTC severe overtemperature threshold
    VNTCSevereOTTh = 0x0F,
    /// Configuration register for overcurrent protection
    OvCThresholds = 0x10,
    /// Configuration register for persistent overcurrent protection
    PersistentOvCThresholds = 0x11,
    /// Configuration register for short circuit threshold protection
    SCThreshold = 0x12,
    /// Configuration register for the TO_PRDRV_BAL_MASK
    ToPrdrvBalMask = 0x13,
    /// Configuration register for the TO_FUSE_RST_MSK
    ToFuseRstMask = 0x14,
    /// Configuration register for the TO_FAULTN_MSK
    ToFaultnMsk = 0x15,
    /// Configuration register for the CURR_MASK
    CurrMsk = 0x16,
    /// Manufacturer Name MSB Register
    ManufacturerNameMsb = 0x17,
    /// Manufacturer Name LSB Register
    ManufacturerNameLsb = 0x18,
    /// Manufacturing Date Register
    ManufacturingDate = 0x19,
    /// First usage date register
    FirstUsageDate = 0x1A,
    /// Serial Number MSB Register
    SerialNumberMsb = 0x1B,
    /// Serial Number LSB Register
    SerialNumberLsb = 0x1C,
    /// Device Name MSB Register
    DeviceNameMsb = 0x1D,
    /// Device Name LSB Register
    DeviceNameLsb = 0x1E,
    /// NVM1 Register
    Nvm1 = 0x1F,
    /// NVM2 Register
    Nvm2 = 0x20,
    /// VCELL1 Register
    VCell1 = 0x21,
    /// VCELL2 Register
    VCell2 = 0x22,
    /// VCELL3 Register
    VCell3 = 0x23,
    /// VCELL4 Register
    VCell4 = 0x24,
    /// VCELL5 Register
    VCell5 = 0x25,
    /// VCELLSUM Register
    VCellSum = 0x26,
    /// VB Register
    VB = 0x27,
    /// NTC_GPIO Register
    NtcGpio = 0x28,
    /// Die Temp Register
    DieTemp = 0x29,
    /// DIAG_OV_OT_UT Register
    DiagOvOtUt = 0x2A,
    /// DIAG_UV Register
    DiagUv = 0x2B,
    /// CC_INST_MEAS Register
    CCInstMeas = 0x2C,
    /// CC_ACC_MSB Register
    CCAccMsb = 0x2D,
    /// CC_ACC_LSB_CNTR Register
    CCAccLsbCntr = 0x2E,
    /// DIAG_CURR Register
    DiagCurr = 0x2F,
}

impl<I2C, I, O, const CELL_COUNT: u8> L9961<I2C, I, O, CELL_COUNT>
where
    I2C: I2c,
    I: Wait,
    O: OutputPin,
{
    /// Read one or more registers from the l9961
    #[maybe_async]
    pub async fn read_registers(
        &mut self,
        register: Registers,
        count: usize,
    ) -> Result<&[u16], I2C::Error> {
        let crc = false;
        let stride = match crc {
            true => 3,
            false => 2,
        };
        let bytes_to_read = count * stride;
        // TODO:To validate CRC
        // CRC of the data is calculated over the following values
        // [address << 1, register, address<< 1 | 1, value]
        self.i2c
            .write_read(
                self.address,
                &[register as u8],
                &mut self.i2c_scratch_buffer[3..3 + bytes_to_read],
            )
            .await?;
        for i in 0..count {
            self.i2c_results[i] = u16::from_be_bytes(
                self.i2c_scratch_buffer[3 + i * stride..3 + i * stride + 2]
                    .try_into()
                    .unwrap(),
            );
    }
        Ok(&self.i2c_results[0..count])
    }

    /// Convenience function to read a single register from the l9961
    #[inline]
    pub async fn read_register(&mut self, register: Registers) -> Result<u16, I2C::Error> {
        Ok(self.read_registers(register, 1).await?[0].into())
    }

    /// Write a new value to a register on the l9961
    #[maybe_async]
    pub async fn write_register(
        &mut self,
        register: Registers,
        value: u16,
    ) -> Result<(), I2C::Error> {
        // TODO:To calculate CRC
        // CRC of the data is calculated over the following values
        // [address << 1, register, value]
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
