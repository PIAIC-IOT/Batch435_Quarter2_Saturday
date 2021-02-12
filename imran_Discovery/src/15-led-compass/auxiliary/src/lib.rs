 //! Initialization code

#![no_std]
 
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;
pub use f3::{
    hal::{delay::Delay, prelude, stm32f30x::i2c1},
    led::{Direction, Leds},
    lsm303dlhc::I16x3,
};

pub use f3::hal::{serial::Serial, stm32f30x::usart1};

use f3::hal::{
    prelude::*,
    stm32f30x::USART1,
};

use f3::{
    hal::{i2c::I2c, prelude::*, stm32f30x},
    Lsm303dlhc,
};

pub fn init() -> (Leds, Lsm303dlhc, Delay, ITM,&'static mut usart1::RegisterBlock) {

    //New
    //(Leds, Lsm303dlhc, Delay, ITM,&'static mut usart1::RegisterBlock,) {
    //Old
    //(Leds, Lsm303dlhc, Delay, ITM) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let leds = Leds::new(gpioe);
 
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);
    
    let lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();

    let delay = Delay::new(cp.SYST, clocks);
 
    //new IMRAN START
    let (tx, rx) = match () {
        #[cfg(feature = "adapter")]
        () => {
            let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

            let tx = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
            let rx = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
 
            (tx, rx)
        }
        #[cfg(not(feature = "adapter"))]
        () => {
            let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);

            let tx = gpioc.pc4.into_af7(&mut gpioc.moder, &mut gpioc.afrl);
            let rx = gpioc.pc5.into_af7(&mut gpioc.moder, &mut gpioc.afrl);

            (tx, rx)
        }
    };
      Serial::usart1(dp.USART1, (tx, rx),    9600.bps(), clocks, &mut rcc.apb2);
    //Serial::usart1(dp.USART1, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb2);
    // If you are having trouble sending/receiving data to/from the
    // HC-05 bluetooth module, try this configuration instead:
    // Serial::usart1(dp.USART1, (tx, rx), 9600.bps(), clocks, &mut rcc.apb2);

    //new IMRAN End
    //Original   (leds, lsm303dlhc, delay, cp.ITM)
    
    //New Imran
    unsafe {
    (leds, lsm303dlhc, delay, cp.ITM,&mut *(USART1::ptr() as *mut _)) }
}
