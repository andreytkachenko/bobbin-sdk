#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

use board::common::Channel;
use board::common::bits::*;
use board::led::*;
use board::pot::*;
use board::hal::pcc;
use board::hal::port::*;
use board::hal::ftm::*;


#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("POT Test");

    let led0 = LED0;
    let pwm_ch = FTM0_CH2;
    led0.port_pin().mode_ftm(&pwm_ch);

    pwm_ch.periph()
        .pcc_set_clock_source(pcc::ClockSource::SPLLDIV2)
        .set_prescale(64);

    // LED is active low, use pwm_low

    pwm_ch.pwm_low(0, 1024);

    loop {
        let v: U10 = POT0.analog_read();
        pwm_ch.set_compare(v.into());
    }
}