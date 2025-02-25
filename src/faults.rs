use bitflags::bitflags;

bitflags! {
    /// Cell fault flags
    pub struct CellFaults:u8 {
        /// Cell is over its configured over-voltage threshold
        const OVER_VOLTAGE = 0x01;
        /// Cell is over it's configured severe over-voltage threshold
        const EXTREME_OVER_VOLTAGE = 0x02;
        /// Cell is over its configured over-voltage threshold
        const UNDER_VOLTAGE = 0x04;
        /// Cell is below its configured under-voltage threshold for balancing
        const UNDER_VOLTAGE_FOR_BALANCING = 0x08;
        /// Cell is over it's configured severe over-voltage threshold
        const EXTREME_UNDER_VOLTAGE = 0x10;
        // Ensure that the reserved bits are always 0
        // TODO: This requires Bitflags 2.x
        // const _ = 0x007F;
    }
}

bitflags! {
    /// Pack and BMS fault flags
    pub struct PackFaults:u16 {
        /// Cell is over its configured over-voltage threshold
        const OVER_VOLTAGE = 0x01;
        /// Cell is over its configured over-voltage threshold
        const UNDER_VOLTAGE = 0x02;
        /// Pack is over it's configured over-temperature threshold
        const NTC_OVER_TEMP = 0x04;
        /// Pack is under it's configured under-temperature threshold
        const NTC_UNDER_TEMP = 0x08;
        /// Pack is over it's configured severe over-temperature threshold
        const NTC_SEVERE_OVER_TEMP = 0x10;
        /// BMS Die over-temperature
        const DIE_OVER_TEMP = 0x20;
        /// Mismatch between cell measurements and pack voltage
        const CELL_VOLTAGE_SUM_VB_MISMATCH = 0x40;

        /// Coulomb counter saturation
        const CC_SAT = 0x8000;
        // Ensure that the reserved bits are always 0
        // TODO: This requires Bitflags 2.x
        // const _ = 0x007F;
    }
}
