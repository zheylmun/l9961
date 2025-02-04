use core::ops::Deref;

/// Programmable cell voltage sample acquisition time (2 bit)
enum TCellFilter {
    /// 0.8ms
    T0_8Ms = 0b00,
    /// 1.31ms
    T1_31Ms = 0b01,
    /// 4.38ms
    T4_38Ms = 0b10,
    /// 16.67ms
    T16_67Ms = 0b11,
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
enum TSCFilter {
    /// 32 us
    T32us = 0b000,
    /// 64 us
    T64us = 0b001,
    /// 128 us
    T128s = 0b010,
    /// 256s
    T256us = 0b011,
    /// 512 us
    T512us = 0b100,
}

#[cfg(feature = "defmt")]
impl defmt::Format for TSCFilter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "T_SC_FILTER: ");
        match self {
            TSCFilter::T32us => defmt::write!(f, "32 us"),
            TSCFilter::T64us => defmt::write!(f, "64 us"),
            TSCFilter::T128s => defmt::write!(f, "128 us"),
            TSCFilter::T256us => defmt::write!(f, "256 us"),
            TSCFilter::T512us => defmt::write!(f, "512 us"),
        }
    }
}

/// Programmable current sense sample acquisition time (2 bit)
enum TCurFilter {
    /// 4.22 ms
    T4_22Ms = 0b00,
    /// 8.44 ms
    T8_44Ms = 0b01,
    /// 16.9 ms
    T16_9Ms = 0b10,
    /// 33.8 ms
    T33_8Ms = 0b11,
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
struct TMeasCycle(u8);

impl TMeasCycle {
    pub const fn new(t_meas_cycle: u8) -> Self {
        debug_assert!(t_meas_cycle & 0b1110000 == 0, "Invalid T_MEAS_CYCLE");
        TMeasCycle(t_meas_cycle)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for TMeasCycle {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "T_MEAS_CYCLE: {} ms", self.0 * 10)
    }
}

/// Registers CFG1_FILTERS_CYCLES
/// Used to configure the filter and cycle times for the voltage and current measurements
pub struct Cfg1FiltersCycles(u16);

impl Cfg1FiltersCycles {
    pub fn new(
        tcell_filter: TCellFilter,
        t_sc_filter: u8,
        t_cur_filter: u8,
        t_meas_cycle: u8,
    ) -> Self {
    }
}

impl Deref for Cfg1FiltersCycles {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl From<u16> for Cfg1FiltersCycles {
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
            "T: {{\n {},\n  {},\n  {},\n  {}\n}}",
            self.tcell_filter(),
            self.t_sc_filter(),
            self.t_cur_filter(),
            self.t_meas_cycle()
        )
    }
}
