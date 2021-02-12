#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux16::{entry, iprint, iprintln, prelude::*, Direction,I16x3, Sensitivity};
use m::Float;
use heapless::{consts, Vec};
use core::f32::consts::PI;

#[entry]
fn main() -> ! {
    let mut data: Vec<u8, consts::U32> = Vec::new();
    let ( mono_timer, mut leds, mut lsm303dlhc, mut delay, mut itm,usart1) = aux16::init();

    const SENSITIVITY: f32 = 12. / (1 << 14) as f32;
    const THRESHOLD: f32 = 0.5;

    lsm303dlhc.set_accel_sensitivity(Sensitivity::G12).unwrap();

    let measurement_time = mono_timer.frequency().0; // 1 second in ticks
    let mut instant = None;
    let mut max_g = 0.;
    iprintln!(&mut itm.stim[0], "Welcome");
    loop {
        let g_x = f32::from(lsm303dlhc.accel().unwrap().x).abs() * SENSITIVITY;
        let g_y = f32::from(lsm303dlhc.accel().unwrap().y).abs() * SENSITIVITY;
        let g_z = f32::from(lsm303dlhc.accel().unwrap().z).abs() * SENSITIVITY;

        let I16x3 { x, y, z } = lsm303dlhc.accel().unwrap();

        let x = f32::from(x) * SENSITIVITY;
        let y = f32::from(y) * SENSITIVITY;
        let z = f32::from(z) * SENSITIVITY;
        iprintln!(&mut itm.stim[0], "{:?}", (x, y, z));
        // iprintln!(&mut itm.stim[0], "{:?} {:?} {:?}", g_x, g_y, g_z);

        match instant {
            None => {
                // If acceleration goes above a threshold, we start measuring
                if g_x > THRESHOLD {
                    iprintln!(&mut itm.stim[0], "START!");
                    iprintln!(&mut itm.stim[0], "{:?}", (x, y, z));
                    iprintln!(&mut itm.stim[0], "{:?} {:?} {:?}", g_x, g_y, g_z);

                    max_g = g_x;
                    instant = Some(mono_timer.now());
                }
            }
            // Still measuring
            Some(ref instant) if instant.elapsed() < measurement_time => {
                if g_x > max_g {
                    max_g = g_x;
                }
            }
            _ => {
                // Report max value
                iprintln!(&mut itm.stim[0], "Max acceleration: {}g", max_g);
                iprintln!(&mut itm.stim[0], "{:?}", (x, y, z));
                iprintln!(&mut itm.stim[0], "{:?}", (g_x, g_y, g_z));

                // Measurement done
                instant = None;

                // Reset
                max_g = 0.;
            }
        }

        while usart1.isr.read().txe().bit_is_clear() {}
        usart1.tdr.write(|w| w.tdr().bits(u16::from('0' as u8)));
        while usart1.isr.read().txe().bit_is_clear() {}
        usart1.tdr.write(|w| w.tdr().bits(u16::from('.' as u8)));       
        //let (mut tx,mut rx) = usart1.split();
        
        data.clear();
        let numf32 : f32 = max_g*1000.;
        if max_g>0.0 {}
        else{  }
        let numu16 :u16 = numf32.abs() as u16;
        if numu16<1000 {
            match numu16 {
                0...9 => if data.push(numu16 as u8+48).is_err(){},
                10...99 => { //29
                    let tens = numu16 as f32 /10.0;
                    let tens = tens as u8;
                    if data.push(tens as u8+48).is_err(){};
                    if data.push(numu16 as u8-(tens as u8 *10)+48).is_err(){};
                },
                100...999 => { //546
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
        { if data.push(9+48).is_err(){}
          if data.push(9+48).is_err(){}
          if data.push(9+48).is_err(){} }
 
        if data.push(13).is_err(){}
        
        for byte in data.iter(){
            while usart1.isr.read().txe().bit_is_clear() {}
            usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
        };
 
        delay.delay_ms(500_u16);
    }
}