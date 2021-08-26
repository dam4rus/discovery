#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let delay_ms = 50u32;

    leds[0].on().unwrap();
    leds[1].on().unwrap();
    let mut index = 1;

    loop {
        delay.delay_ms(delay_ms);
        let prev_index = (index as isize - 1 + leds.len() as isize) as usize % leds.len();
        leds[prev_index].off().unwrap();
        delay.delay_ms(delay_ms);
        let next_index = (index + 1) % leds.len();
        leds[next_index].on().unwrap();
        index = next_index;
    }
}
