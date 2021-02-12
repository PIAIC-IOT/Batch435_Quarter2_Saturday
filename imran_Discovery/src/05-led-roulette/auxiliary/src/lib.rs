//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

pub use cortex_m_rt::entry;
pub use f3::{
    hal::{delay::Delay, prelude},
    led::Leds,
};

use f3::hal::{prelude::*, stm32f30x};
  
pub fn init() -> (Delay, Leds) {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = stm32f30x::Peripherals::take().unwrap();
    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // Configure the syst timer to trigger an update
    let delay = Delay::new(cp.SYST, clocks);
    // Configure gpioE as a push-pull output. The `rcc` register is passed to the function
    // in order to configure the port. ahb should be passed instead.
    let leds = Leds::new(dp.GPIOE.split(&mut rcc.ahb));

    (delay, leds)
}
