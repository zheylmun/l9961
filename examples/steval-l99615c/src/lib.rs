#![no_main]
#![no_std]

use cortex_m_semihosting::debug;

use defmt_rtt as _; // global logger

use embassy_stm32::{
    self as _, bind_interrupts,
    exti::ExtiInput,
    gpio::{Level, Output, Pull, Speed},
    i2c::{self, I2c},
    mode::Async,
    peripherals::{self},
    time::Hertz,
    Peripherals,
}; // memory layout

use l9961::{Config, L9961};
use panic_probe as _;

bind_interrupts!(struct Irqs {
    I2C2 => i2c::EventInterruptHandler<peripherals::I2C2>, i2c::ErrorInterruptHandler<peripherals::I2C2>;
});

pub fn configure_l9961<'a>(
    peripherals: Peripherals,
    config: Config,
) -> L9961<I2c<'a, Async>, ExtiInput<'a>, Output<'a>> {
    let i2c = I2c::new(
        peripherals.I2C2,
        peripherals.PB13,
        peripherals.PB14,
        Irqs,
        peripherals.DMA1_CH1,
        peripherals.DMA1_CH2,
        Hertz(100_000),
        Default::default(),
    );

    let ready = ExtiInput::new(peripherals.PB0, peripherals.EXTI0, Pull::None);
    let faultn = ExtiInput::new(peripherals.PA6, peripherals.EXTI6, Pull::None);
    let wakeup = Output::new(peripherals.PA5, Level::Low, Speed::Low);
    L9961::new(i2c, ready, faultn, wakeup, config)
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

/// Terminates the application and makes a semihosting-capable debug tool exit
/// with status code 0.
pub fn exit() -> ! {
    loop {
        debug::exit(debug::EXIT_SUCCESS);
    }
}

/// Hardfault handler.
///
/// Terminates the application and makes a semihosting-capable debug tool exit
/// with an error. This seems better than the default, which is to spin in a
/// loop.
#[cortex_m_rt::exception]
#[allow(non_snake_case)]
unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    loop {
        debug::exit(debug::EXIT_FAILURE);
    }
}

// defmt-test 0.3.0 has the limitation that this `#[tests]` attribute can only be used
// once within a crate. the module can be in any file but there can only be at most
// one `#[tests]` module in this library crate
#[cfg(test)]
#[defmt_test::tests]
mod unit_tests {
    use defmt::assert;

    #[test]
    fn it_works() {
        assert!(true)
    }
}
