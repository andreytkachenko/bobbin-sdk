#![no_std]
#![no_main]
#![feature(asm)]

extern crate feather_m0 as board;
extern crate examples;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();    
    let brd = board::board();
    let app = examples::led::BlinkLed::new(brd.led0(), brd, 500);
    app.run()
}
