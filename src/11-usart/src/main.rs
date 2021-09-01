#![no_main]
#![no_std]

use core::fmt::{self, Write};

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1};

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n"), $($arg)*)
    };
}

struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.as_bytes() {
            while self.usart1.isr.read().txe().bit_is_clear() {}
            self.usart1
                .tdr
                .write(|w| w.tdr().bits(u16::from(*c)));
        }
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();

    let mut serial = SerialPort { usart1 };

    uprintln!(serial, "The answer is {}", 40 + 2);

    loop {}
}

// #[allow(unused_imports)]
// use aux11::{entry, iprint, iprintln};

// #[entry]
// fn main() -> ! {
//     let (usart1, _mono_timer, _itm) = aux11::init();

//     for c in b"The quick brown fox jumps over the lazy dog." {
//         while usart1.isr.read().txe().bit_is_clear() {}
//         usart1
//             .tdr
//             .write(|w| w.tdr().bits(u16::from(*c)));

//     }

//     loop {}
// }
