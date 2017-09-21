#![no_std]
#![feature(lang_items)]

#![feature(asm)]

extern crate r0;

extern crate log;

#[macro_use] pub mod itm;
#[macro_use] pub mod console;
#[macro_use] pub mod logger;

extern crate stm32f40x;
pub use stm32f40x::{common, chip, hal};

pub mod exceptions;
#[cfg(target_os="none")]
pub mod lang_items;

pub mod pin;
pub mod clock;
pub mod led;
// pub mod btn;
pub mod tim;

pub use tim::delay;

// pub fn delay(ms: usize) {
//     for _ in 0..(ms * 5000) {
//         unsafe { asm!("nop"); }
//     }
// }

pub fn init() {
    clock::init();
    led::init();
    // btn::init();
    tim::init();
    console::init();
}