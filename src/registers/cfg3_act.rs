use core::ops::Deref;

/// Configuration 3 Activation register
/// Contains the configuration for the activation of the balancing,
/// charge, and discharge FETs
pub struct Cfg3Act(u16);

const CELL_1_BAL_EN: u16 = 1 << 0;
const CELL_2_BAL_EN: u16 = 1 << 1;
const CELL_3_BAL_EN: u16 = 1 << 2;
const CELL_4_BAL_EN: u16 = 1 << 3;
const CELL_5_BAL_EN: u16 = 1 << 4;
const CHG_EN: u16 = 1 << 5;
const DISCHG_EN: u16 = 1 << 6;

impl Cfg3Act {
    /// Create a new Cfg3Act object
    pub fn new(
        cell_1_bal_en: bool,
        cell_2_bal_en: bool,
        cell_3_bal_en: bool,
        cell_4_bal_en: bool,
        cell_5_bal_en: bool,
        chg_en: bool,
        dischg_en: bool,
    ) -> Self {
        Self(
            (cell_1_bal_en as u16) << 0
                | (cell_2_bal_en as u16) << 1
                | (cell_3_bal_en as u16) << 2
                | (cell_4_bal_en as u16) << 3
                | (cell_5_bal_en as u16) << 4
                | (chg_en as u16) << 5
                | (dischg_en as u16) << 6,
        )
    }

    /// Cell 1 Charge balance enabled
    pub fn get_cell_1_balance_enabled(&self) -> bool {
        self.0 & CELL_1_BAL_EN != 0
    }

    /// Set cell 1 charge balance enable
    pub fn set_cell_1_balance_enabled(&mut self, enabled: bool) {
        if enabled {
            self.0 |= CELL_1_BAL_EN;
        } else {
            self.0 &= !CELL_1_BAL_EN;
        }
    }

    /// Cell 2 Charge balance enable
    pub fn get_cell_2_balance_enabled(&self) -> bool {
        self.0 & CELL_2_BAL_EN != 0
    }

    /// Set cell 2 charge balance enable
    pub fn set_cell_2_balance_enabled(&mut self, enabled: bool) {
        if enabled {
            self.0 |= CELL_2_BAL_EN;
        } else {
            self.0 &= !CELL_2_BAL_EN;
        }
    }

    /// Cell 3 Charge balance enable
    pub fn get_cell_3_balance_enabled(&self) -> bool {
        self.0 & CELL_3_BAL_EN != 0
    }

    /// Set cell 3 charge balance enable
    pub fn set_cell_3_balance_enabled(&mut self, enabled: bool) {
        if enabled {
            self.0 |= CELL_3_BAL_EN;
        } else {
            self.0 &= !CELL_3_BAL_EN;
        }
    }

    /// Cell 4 Charge balance enable
    pub fn get_cell_4_balance_enabled(&self) -> bool {
        self.0 & CELL_4_BAL_EN != 0
    }

    /// Set cell 4 charge balance enable
    pub fn set_cell_4_balance_enabled(&mut self, enabled: bool) {
        if enabled {
            self.0 |= CELL_4_BAL_EN;
        } else {
            self.0 &= !CELL_4_BAL_EN;
        }
    }

    /// Cell 5 Charge balance enable
    pub fn get_cell_5_balance_enabled(&self) -> bool {
        self.0 & CELL_5_BAL_EN != 0
    }

    /// Set cell 5 charge balance enable
    pub fn set_cell_5_balance_enabled(&mut self, enabled: bool) {
        if enabled {
            self.0 |= CELL_5_BAL_EN;
        } else {
            self.0 &= !CELL_5_BAL_EN;
        }
    }

    /// Charge FET enable
    pub fn get_charge_enabled(&self) -> bool {
        self.0 & CHG_EN != 0
    }

    /// Set FET charge enable
    pub fn set_charge_enabled(&mut self, enabled: bool) {
        if enabled {
            self.0 |= CHG_EN;
        } else {
            self.0 &= !CHG_EN;
        }
    }

    /// Discharge FET enable
    pub fn get_discharge_enabled(&self) -> bool {
        self.0 & DISCHG_EN != 0
    }

    /// Set FET discharge enable
    pub fn set_discharge_enabled(&mut self, enabled: bool) {
        if enabled {
            self.0 |= DISCHG_EN;
        } else {
            self.0 &= !DISCHG_EN;
        }
    }
}

impl Deref for Cfg3Act {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for Cfg3Act {
    fn from(id: u16) -> Self {
        debug_assert!(id & 0b1111111110000000 == 0, "Invalid CFG3_ACT value");
        Cfg3Act(id)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Cfg3Act {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Config 3 Active: {{\n  Cell 1 balancing active: {},\n  Cell 2 balancing active: {},\n  Cell 3 balancing active: {},\n  Cell 4 balancing active: {},\n  Cell 5 balancing active: {},\n  Charge FET active: {},\n  Discharge FET active: {}\n}}",
            self.get_cell_1_balance_enabled(),
            self.get_cell_2_balance_enabled(),
            self.get_cell_3_balance_enabled(),
            self.get_cell_4_balance_enabled(),
            self.get_cell_5_balance_enabled(),
            self.get_charge_enabled(),
            self.get_discharge_enabled()
        )
    }
}
