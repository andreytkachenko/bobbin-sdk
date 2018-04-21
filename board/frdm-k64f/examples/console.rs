#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;

use board::prelude::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init().run(|brd| {
        println!("Running Console");
        loop {
            if let Some(console) = brd.console() {
                console.write(b"Tick...\r\n");
            }
            brd.tick().delay(500);
        }
    })
}