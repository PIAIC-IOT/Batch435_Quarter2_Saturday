#![no_main]
#![no_std]

use aux9::{entry, tim6};

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    // TODO implement this
    // const K: u16 = 800; // this value needs to be tweaked
    // for _ in 0..(K * ms) {
    //     aux9::nop()
    // }
    for _ in 0..1000_u16{}
}

#[entry]
fn main() -> ! {
    let (mut leds, rcc, tim6) = aux9::init();

    // TODO initialize TIM6

    let ms = 50;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on();
            delay(tim6, ms);
            leds[curr].off();
            delay(tim6, ms);
        }
    }
}
