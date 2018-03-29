#![no_std]
#![no_main]

extern crate arduino_zero as board;
extern crate examples;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();    
    
    let b = board::board();
    examples::leds::run(b);
}