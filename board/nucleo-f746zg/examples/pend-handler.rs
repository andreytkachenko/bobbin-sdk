#![no_std]
#![no_main]

extern crate nucleo_f746zg as board;
extern crate examples;

#[no_mangle]
pub extern "C" fn main() -> ! {
    examples::pend_handler::run_with_sys(board::init())
}
