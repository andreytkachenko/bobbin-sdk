#![no_std]
#![no_main]
#![feature(asm)]

extern crate feather_m0 as board;

use core::fmt::Write;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    let mut u = board::usart::usart0(115_200);

    let mut i = 0u32;
    loop {
        write!(u, "Hello, {}\r\n", i).unwrap();
        board::delay(1024);
        i = i.wrapping_add(1);
    }
}

