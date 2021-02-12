#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*, I16x3};
use m::Float;
use heapless::{consts, Vec};

#[entry]
fn main() -> ! {
    const XY_GAIN: f32 = 1100.; // LSB / G
    const Z_GAIN: f32 = 980.; // LSB / G

    let (_leds, mut lsm303dlhc, mut delay, mut itm,usart1) = aux15::init();
    let mut data: Vec<u8, consts::U32> = Vec::new();
    loop {
        let I16x3 { x, y, z } = lsm303dlhc.mag().unwrap();

        let x = f32::from(x) / XY_GAIN;
        let y = f32::from(y) / XY_GAIN;
        let z = f32::from(z) / Z_GAIN;

        let mag = (x * x + y * y + z * z).sqrt();

        // iprintln!(&mut itm.stim[0], "{} mG", mag * 1_000.);
        
        
        while usart1.isr.read().txe().bit_is_clear() {}
        usart1.tdr.write(|w| w.tdr().bits(u16::from('x' as u8)));
        //let (mut tx,mut rx) = usart1.split();
        
        data.clear();
        let numf32 : f32 = mag*1_000.;
        if numf32>0.0 {if data.push(43).is_err(){}}
        else{if data.push(45).is_err(){}}
        let numu16 :u16 = numf32.abs() as u16;
        if numu16<1000 {
            match numu16 {
                0...9 => if data.push(numu16 as u8+48).is_err(){},
                10...99 => { 
                    let tens = numu16 as f32 /10.0;
                    let tens = tens as u8;
                    if data.push(tens as u8+48).is_err(){};
                    if data.push(numu16 as u8-(tens as u8 *10)+48).is_err(){};
                },
                100...999 => { 
                    let hundred = numu16 as f32 /100.0;
                    let hundred = hundred as u8;
                    if data.push(hundred as u8+48).is_err(){};
                    let rem= numu16 as u16-(hundred as u16 *100);
                    let tens = rem as f32 /10.0;
                    let tens = tens as u8;
                    if data.push(tens as u8+48).is_err(){};
                    if data.push(rem as u8-(tens as u8 *10)+48).is_err(){};
                },
                _ => ()
            }
        } else 
        {if data.push(9+48).is_err(){}
         if data.push(9+48).is_err(){}
         if data.push(9+48).is_err(){}}
        if data.push(13).is_err(){}

        for byte in data.iter(){
            while usart1.isr.read().txe().bit_is_clear() {}
            usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
        };
 
        delay.delay_ms(500_u16);
    }
}