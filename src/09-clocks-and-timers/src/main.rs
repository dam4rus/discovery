#![no_main]
#![no_std]

use aux9::{entry, switch_hal::OutputSwitch, tim6, nop};

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    tim6.arr.write(|w| w.arr().bits(ms));
    tim6.cr1.write(|w| w.cen().set_bit());
    while !tim6.sr.read().uif().bit_is_set() {
        nop();
    }
    tim6.sr.write(|w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
    let (leds, rcc, tim6) = aux9::init();
    let mut leds = leds.into_array();

    rcc.apb1enr.write(|w| w.tim6en().set_bit());
    tim6.cr1.write(|w| {
        w.opm().set_bit();
        w.cen().clear_bit()
    });
    tim6.psc.write(|w| w.psc().bits(7_999));

    let ms = 50;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().unwrap();
            delay(tim6, ms);
            leds[curr].off().unwrap();
            delay(tim6, ms);
        }
    }
}
