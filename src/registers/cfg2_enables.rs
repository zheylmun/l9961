use core::ops::Deref;

use defmt::Format;

const VCELL_EN_1_MASK: u16 = 0x0001;
const VCELL_EN_2_MASK: u16 = 0x0002;
const VCELL_EN_3_MASK: u16 = 0x0004;
const VCELL_EN_4_MASK: u16 = 0x0008;
const VCELL_EN_5_MASK: u16 = 0x0010;
const VB_EN: u16 = 0x0020;
const NTC_EN: u16 = 0x0040;
const CSA_EN: u16 = 0x0080;
const CC_ACC_EN: u16 = 0x0100;
const OVC_EN: u16 = 0x0200;
const SC_EN: u16 = 0x0400;
const DCHG_HS_LS: u16 = 0x0800;
const CHG_HS_LS: u16 = 0x1000;
const CRC_EN: u16 = 0x2000;

/// Configuration for high-side vs. low side FETs for charge and discharge
pub enum FetConfig {
    /// High-side FET configuration
    HighSide,
    /// Low-side FET configuration
    LowSide,
}

impl From<FetConfig> for u16 {
    fn from(fet_config: FetConfig) -> u16 {
        match fet_config {
            FetConfig::HighSide => 0,
            FetConfig::LowSide => 1,
        }
    }
}

impl Format for FetConfig {
    fn format(&self, f: defmt::Formatter) {
        match self {
            FetConfig::HighSide => defmt::write!(f, "High Side"),
            FetConfig::LowSide => defmt::write!(f, "Low Side"),
        }
    }
}

/// Register CFG2_ENABLES
/// Used to enable or disable features such as VCELL_EN (cell voltage monitoring),
/// VB (battery pack voltage) monitoring, NTC (cell temperature monitoring),
/// CSA (Current Sense ADC), CC_AC (Coulomb Counting), OVC (Overcurrent Protection),
/// SC (Short Circuit Protection), and CRC (Cyclic Redundancy Check for I2C data integrity)
/// Additionally, the register configures the charge and discharge FET drivers for use with
/// high side or low side FETs
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cfg2Enables(u16);

impl Cfg2Enables {
    /// Create a new Cfg2 Enables register value
    #[inline]
    pub const fn new(
        vcell_en_1: bool,
        vcell_en_2: bool,
        vcell_en_3: bool,
        vcell_en_4: bool,
        vcell_en_5: bool,
        vb_en: bool,
        ntc_en: bool,
        csa_en: bool,
        cc_acc_en: bool,
        ovc_en: bool,
        sc_en: bool,
        dchg_hs_ls: FetConfig,
        chg_hs_ls: FetConfig,
        crc_en: bool,
    ) -> Self {
        Self(
            (vcell_en_1 as u16) << 0
                | (vcell_en_2 as u16) << 1
                | (vcell_en_3 as u16) << 2
                | (vcell_en_4 as u16) << 3
                | (vcell_en_5 as u16) << 4
                | (vb_en as u16) << 5
                | (ntc_en as u16) << 6
                | (csa_en as u16) << 7
                | (cc_acc_en as u16) << 8
                | (ovc_en as u16) << 9
                | (sc_en as u16) << 10
                | (dchg_hs_ls as u16) << 11
                | (chg_hs_ls as u16) << 12
                | (crc_en as u16) << 13,
        )
    }

    /// Get the VCELL_EN_1 flag
    #[inline]
    pub const fn get_vcell_en_1(&self) -> bool {
        (self.0 & VCELL_EN_1_MASK) != 0
    }

    /// Set the VCELL_EN_1 flag
    #[inline]
    pub const fn set_vcell_en_1(&mut self, value: bool) {
        self.0 = self.0 & !VCELL_EN_1_MASK | ((value as u16) << 0);
    }

    /// Get the VCELL_EN_2 flag
    #[inline]
    pub const fn get_vcell_en_2(&self) -> bool {
        (self.0 & VCELL_EN_2_MASK) != 0
    }

    /// Set the VCELL_EN_2 flag
    #[inline]
    pub const fn set_vcell_en_2(&mut self, value: bool) {
        self.0 = self.0 & !VCELL_EN_2_MASK | ((value as u16) << 1);
    }

    /// Get the VCELL_EN_3 flag
    #[inline]
    pub const fn get_vcell_en_3(&self) -> bool {
        (self.0 & VCELL_EN_3_MASK) != 0
    }

    /// Set the VCELL_EN_3 flag
    #[inline]
    pub const fn set_vcell_en_3(&mut self, value: bool) {
        self.0 = self.0 & !VCELL_EN_3_MASK | ((value as u16) << 2);
    }

    /// Get the VCELL_EN_4 flag
    #[inline]
    pub const fn get_vcell_en_4(&self) -> bool {
        (self.0 & VCELL_EN_4_MASK) != 0
    }

    /// Set the VCELL_EN_4 flag
    #[inline]
    pub const fn set_vcell_en_4(&mut self, value: bool) {
        self.0 = self.0 & !VCELL_EN_4_MASK | ((value as u16) << 3);
    }

    /// Get the VCELL_EN_5 flag
    #[inline]
    pub const fn get_vcell_en_5(&self) -> bool {
        (self.0 & VCELL_EN_5_MASK) != 0
    }

    /// Set the VCELL_EN_5 flag
    #[inline]
    pub const fn set_vcell_en_5(&mut self, value: bool) {
        self.0 = self.0 & !VCELL_EN_5_MASK | ((value as u16) << 4);
    }

    /// Get the VB_EN flag
    #[inline]
    pub const fn get_vb_en(&self) -> bool {
        (self.0 & VB_EN) != 0
    }

    /// Set the VB_EN flag
    #[inline]
    pub const fn set_vb_en(&mut self, value: bool) {
        self.0 = self.0 & !VB_EN | ((value as u16) << 5);
    }

    /// Get the NTC_EN flag
    #[inline]
    pub const fn get_ntc_en(&self) -> bool {
        (self.0 & NTC_EN) != 0
    }

    /// Set the cell temperature  flag
    #[inline]
    pub const fn set_ntc_en(&mut self, value: bool) {
        self.0 = self.0 & !NTC_EN | ((value as u16) << 6);
    }

    /// Get the current sense adc enable status
    #[inline]
    pub const fn get_csa_en(&self) -> bool {
        (self.0 & CSA_EN) != 0
    }

    /// Set the current sense adc enable status
    #[inline]
    pub const fn set_csa_en(&mut self, value: bool) {
        self.0 = self.0 & !CSA_EN | ((value as u16) << 7);
    }

    /// Get the couloumb counting enable status
    #[inline]
    pub const fn get_cc_acc_en(&self) -> bool {
        (self.0 & CC_ACC_EN) != 0
    }

    /// Set the couloumb counting enable status
    #[inline]
    pub const fn set_cc_acc_en(&mut self, value: bool) {
        self.0 = self.0 & !CC_ACC_EN | ((value as u16) << 8);
    }

    /// Get the overcurrent protection enable status
    #[inline]
    pub const fn get_ovc_en(&self) -> bool {
        (self.0 & OVC_EN) != 0
    }

    /// Set the overcurrent protection enable statua
    #[inline]
    pub const fn set_ovc_en(&mut self, value: bool) {
        self.0 = self.0 & !OVC_EN | ((value as u16) << 9);
    }

    /// Get the enable status of the short circuit protection
    #[inline]
    pub const fn get_sc_en(&self) -> bool {
        (self.0 & SC_EN) != 0
    }

    /// Enable or disable short circuit protection
    #[inline]
    pub const fn set_sc_en(&mut self, value: bool) {
        self.0 = self.0 & !SC_EN | ((value as u16) << 10);
    }

    /// Get the Discahrge FET configuration
    /// - False configured for High Side FET
    /// - False configured for Low Side FET
    #[inline]
    pub const fn get_dchg_hs_ls(&self) -> FetConfig {
        match (self.0 & DCHG_HS_LS) == 0 {
            true => FetConfig::HighSide,
            false => FetConfig::LowSide,
        }
    }

    /// Set the discharge FET configuration
    /// - True configured for High Side FET
    /// - False configured for Low Side FET
    #[inline]
    pub const fn set_dchg_hs_ls(&mut self, value: FetConfig) {
        self.0 = self.0 & !DCHG_HS_LS | ((value as u16) << 11);
    }

    /// Get the charge FET configuration
    /// - True configured for High Side FET
    /// - False configured for Low Side FET
    #[inline]
    pub const fn get_chg_hs_ls(&self) -> FetConfig {
        match (self.0 & CHG_HS_LS) == 0 {
            true => FetConfig::HighSide,
            false => FetConfig::LowSide,
        }
    }

    /// Set the charge FET configuration
    /// - True configured for High Side FET
    /// - False configured for Low Side FET
    #[inline]
    pub const fn set_chg_hs_ls(&mut self, value: FetConfig) {
        self.0 = self.0 & !CHG_HS_LS | ((value as u16) << 12);
    }

    /// Get the CRC_EN flag
    #[inline]
    pub const fn get_crc_en(&self) -> bool {
        (self.0 & CRC_EN) != 0
    }

    /// Set the CRC_EN flag
    #[inline]
    pub const fn set_crc_en(&mut self, value: bool) {
        self.0 = self.0 & !CRC_EN | ((value as u16) << 13);
    }
}

impl Deref for Cfg2Enables {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for Cfg2Enables {
    fn from(id: u16) -> Self {
        debug_assert!(id & 0x3FFF == id, "Invalid CFG2_ENABLES value");
        Self(id)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Cfg2Enables {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Config 2 Enables: {{\n  Cell 1 voltage conversion enabled: {},\n  Cell 2 voltage conversion enabled: {},\n  Cell 3 voltage conversion enabled: {},\n  Cell 4 voltage conversion enabled: {},\n  Cell 5 voltage conversion enabled: {},\n  Battery voltage conversion enabled: {},\n  Temperature conversion enabled: {},\n  Current sense enabled: {},\n  Coulomb counter accumulator enabled: {},\n  Overcurrent protection enabled: {},\n  Short circuit monitor enabled: {},\n  Discharge high side low side: {},\n  Charge high side low side: {},\n  CRC enabled: {}\n}}",
            self.get_vcell_en_1(),
            self.get_vcell_en_2(),
            self.get_vcell_en_3(),
            self.get_vcell_en_4(),
            self.get_vcell_en_5(),
            self.get_vb_en(),
            self.get_ntc_en(),
            self.get_csa_en(),
            self.get_cc_acc_en(),
            self.get_ovc_en(),
            self.get_sc_en(),
            self.get_dchg_hs_ls(),
            self.get_chg_hs_ls(),
            self.get_crc_en()
        )
    }
}
