#![no_std]
#![no_main]

extern crate teensy_lc as board;

use board::hal::port::GpioPin;
use board::hal::gpio::DigitalOutput;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::LED0.gpio_pin();
    loop {
        led0.toggle_output();
        board::delay(500);
    }
}
