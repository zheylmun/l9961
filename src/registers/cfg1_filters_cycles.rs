use core::ops::Deref;

const TCELL_FILTER_MASK: u16 = 0b11;
const TCELL_FILTER_SHIFT: u16 = 0;
const T_SC_FILTER_MASK: u16 = 0b111;
const T_SC_FILTER_SHIFT: u16 = 2;
const T_CUR_FILTER_MASK: u16 = 0b11;
const T_CUR_FILTER_SHIFT: u16 = 5;
const T_MEAS_CYCLE_MASK: u16 = 0b11111;
const T_MEAS_CYCLE_SHIFT: u16 = 7;

/// Programmable cell voltage sample acquisition time (2 bit)
pub enum TCellFilter {
    /// 0.8ms
    T0_8Ms = 0b00,
    /// 1.31ms
    T1_31Ms = 0b01,
    /// 4.38ms
    T4_38Ms = 0b10,
    /// 16.67ms
    T16_67Ms = 0b11,
}

impl From<u16> for TCellFilter {
    #[inline]
    fn from(value: u16) -> Self {
        match value {
            0b00 => TCellFilter::T0_8Ms,
            0b01 => TCellFilter::T1_31Ms,
            0b10 => TCellFilter::T4_38Ms,
            0b11 => TCellFilter::T16_67Ms,
            _ => panic!("Invalid T_CELL_FILTER"),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for TCellFilter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "T_CELL_FILTER: ");
        match self {
            TCellFilter::T0_8Ms => defmt::write!(f, "0.8 ms"),
            TCellFilter::T1_31Ms => defmt::write!(f, "1.31 ms"),
            TCellFilter::T4_38Ms => defmt::write!(f, "4.38 ms"),
            TCellFilter::T16_67Ms => defmt::write!(f, "16.67 ms"),
        }
    }
}

/// Programmable short-circuit in discharge filter time (3 bit)
pub enum TSCFilter {
    /// 32 us
    T32us = 0b000,
    /// 64 us
    T64us = 0b001,
    /// 128 us
    T128us = 0b010,
    /// 256us
    T256us = 0b011,
    /// 512 us
    T512us = 0b100,
}

impl From<u16> for TSCFilter {
    #[inline]
    fn from(value: u16) -> Self {
        match value {
            0b000 => TSCFilter::T32us,
            0b001 => TSCFilter::T64us,
            0b010 => TSCFilter::T128us,
            0b011 => TSCFilter::T256us,
            0b100 => TSCFilter::T512us,
            _ => panic!("Invalid T_SC_FILTER"),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for TSCFilter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "T_SC_FILTER: ");
        match self {
            TSCFilter::T32us => defmt::write!(f, "32 us"),
            TSCFilter::T64us => defmt::write!(f, "64 us"),
            TSCFilter::T128us => defmt::write!(f, "128 us"),
            TSCFilter::T256us => defmt::write!(f, "256 us"),
            TSCFilter::T512us => defmt::write!(f, "512 us"),
        }
    }
}

/// Programmable current sense sample acquisition time (2 bit)
pub enum TCurFilter {
    /// 4.22 ms
    T4_22Ms = 0b00,
    /// 8.44 ms
    T8_44Ms = 0b01,
    /// 16.9 ms
    T16_9Ms = 0b10,
    /// 33.8 ms
    T33_8Ms = 0b11,
}

impl From<u16> for TCurFilter {
    #[inline]
    fn from(value: u16) -> Self {
        match value {
            0b00 => TCurFilter::T4_22Ms,
            0b01 => TCurFilter::T8_44Ms,
            0b10 => TCurFilter::T16_9Ms,
            0b11 => TCurFilter::T33_8Ms,
            _ => panic!("Invalid T_CUR_FILTER"),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for TCurFilter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "T_CUR_FILTER: ");
        match self {
            TCurFilter::T4_22Ms => defmt::write!(f, "4.22 ms"),
            TCurFilter::T8_44Ms => defmt::write!(f, "8.44 ms"),
            TCurFilter::T16_9Ms => defmt::write!(f, "16.9 ms"),
            TCurFilter::T33_8Ms => defmt::write!(f, "33.8 ms"),
        }
    }
}

/// Programmable voltage conversion routine execution period (5 bit)
/// The period is equal to T_MEAS_CYCLE * 10 ms
pub struct TMeasCycle(u8);

impl From<u8> for TMeasCycle {
    #[inline]
    fn from(value: u8) -> Self {
        debug_assert!(value & 0b1110000 == 0, "Invalid T_MEAS_CYCLE");
        Self(value)
    }
}

impl Deref for TMeasCycle {
    type Target = u8;
    #[inline]
    fn deref(&self) -> &u8 {
        &self.0
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for TMeasCycle {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "T_MEAS_CYCLE: {} ms", (self.0 * 10).max(10))
    }
}

/// Registers CFG1_FILTERS_CYCLES
/// Used to configure the filter and cycle times for the voltage and current measurements
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cfg1FiltersCycles(u16);

impl Cfg1FiltersCycles {
    /// Create a new Cfg1 Filters register value
    pub const fn new(
        tcell_filter: TCellFilter,
        t_sc_filter: TSCFilter,
        t_cur_filter: TCurFilter,
        t_meas_cycle: u8,
    ) -> Self {
        Self(
            tcell_filter as u16 & TCELL_FILTER_MASK << TCELL_FILTER_SHIFT
                | (t_sc_filter as u16 & T_SC_FILTER_MASK) << T_SC_FILTER_SHIFT
                | (t_cur_filter as u16 & T_CUR_FILTER_MASK) << T_CUR_FILTER_SHIFT
                | (t_meas_cycle as u16 & T_MEAS_CYCLE_MASK) << T_MEAS_CYCLE_SHIFT,
        )
    }

    /// Get the current cell voltage conversion time
    #[inline]
    pub fn get_t_cell_filter(&self) -> TCellFilter {
        TCellFilter::from((self.0 >> TCELL_FILTER_SHIFT) & TCELL_FILTER_MASK)
    }

    /// Set a new cell voltage conversion time
    #[inline]
    pub const fn set_t_cell_filter(&mut self, filter: TCellFilter) {
        self.0 = self.0 & !(TCELL_FILTER_MASK << TCELL_FILTER_SHIFT)
            | (filter as u16) << TCELL_FILTER_SHIFT;
    }

    /// Get the current short circuit measurement time
    #[inline]
    pub fn get_t_sc_filter(&self) -> TSCFilter {
        TSCFilter::from((self.0 >> T_SC_FILTER_SHIFT) & T_SC_FILTER_MASK)
    }

    /// Set a new short circuit measurement time
    #[inline]
    pub const fn set_t_sc_filter(&mut self, filter: TSCFilter) {
        self.0 = self.0 & !(T_SC_FILTER_MASK << T_SC_FILTER_SHIFT)
            | (filter as u16) << T_SC_FILTER_SHIFT;
    }

    /// Get the current current sense acquisition time
    #[inline]
    pub fn get_t_curr_filter(&self) -> TCurFilter {
        TCurFilter::from((self.0 >> T_CUR_FILTER_SHIFT) & T_CUR_FILTER_MASK)
    }

    /// Set a new current sense acquisition time
    #[inline]
    pub const fn set_t_curr_filter(&mut self, filter: TCurFilter) {
        self.0 = self.0 & !(T_CUR_FILTER_MASK << T_CUR_FILTER_SHIFT)
            | (filter as u16) << T_CUR_FILTER_SHIFT;
    }

    /// Get the current measurement cycle period
    #[inline]
    pub fn get_t_meas_cycle(&self) -> TMeasCycle {
        TMeasCycle::from(((self.0 >> T_MEAS_CYCLE_SHIFT) & TCELL_FILTER_MASK) as u8)
    }

    /// Set a new measurement cycle period
    #[inline]
    pub fn set_t_meas_cycle(&mut self, filter: TMeasCycle) {
        let filter_value = *filter as u16;
        self.0 = self.0 & !(T_MEAS_CYCLE_MASK << T_MEAS_CYCLE_SHIFT)
            | filter_value << T_MEAS_CYCLE_SHIFT;
    }
}

impl Deref for Cfg1FiltersCycles {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for Cfg1FiltersCycles {
    #[inline]
    fn from(id: u16) -> Self {
        debug_assert!(id & 0xF000 == 0, "Invalid CFG1_FILTERS_CYCLES");
        Cfg1FiltersCycles(id)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Cfg1FiltersCycles {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Config 1 Filters, Cycles: {{\n {},\n  {},\n  {},\n  {}\n}}",
            self.get_t_cell_filter(),
            self.get_t_sc_filter(),
            self.get_t_curr_filter(),
            self.get_t_meas_cycle()
        )
    }
}
