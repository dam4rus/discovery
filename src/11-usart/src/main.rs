#![no_main]
#![no_std]

use core::fmt::{self, Write};
use heapless::Vec;

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1, bkpt};

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\r\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\r\n"), $($arg)*)
    };
}

struct SerialPort<'a> {
    usart1: &'a mut usart1::RegisterBlock,
}

impl<'a> fmt::Write for SerialPort<'a> {
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

// #[entry]
// fn main() -> ! {
//     let (usart1, _mono_timer, _itm) = aux11::init();

//     let mut serial = SerialPort { usart1 };

//     uprintln!(serial, "The answer is {}", 40 + 2);

//     loop {}
// }

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();
    let mut buffer = Vec::<u8, 32>::new();

    loop {
        buffer.clear();
        loop {
            // Wait until there's data available
            while usart1.isr.read().rxne().bit_is_clear() {}

            // Retrieve the data
            match usart1.rdr.read().rdr().bits() as u8 {
                b'\r' => {
                    for byte in buffer.iter().rev() {
                        while usart1.isr.read().txe().bit_is_clear() {}
                        usart1
                            .tdr
                            .write(|w| w.tdr().bits(u16::from(*byte)));
                    }

                    let mut serial = SerialPort { usart1 };
                    uprintln!(serial, "");
                    break;
                }
                byte => if buffer.push(byte).is_err() {
                    let mut serial = SerialPort { usart1 };
                    uprintln!(serial, "error: buffer is full");
                    break;
                }
            }
        }
    }
}

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
