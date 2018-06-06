#![no_std]
#![feature(use_extern_macros)]

extern crate cortex_m_rt;
pub extern crate stm32f3x as mcu;

pub mod prelude;
pub mod led;
pub mod btn;
pub mod sys;

pub use mcu::bobbin_bits;
pub use mcu::bobbin_mcu;
pub use mcu::bobbin_hal;
pub use mcu::bobbin_sys;

pub use bobbin_sys::{print, println, abort};
pub use sys::init;

pub type Mcu = mcu::Stm32f3x;
pub type Board = DiscoveryStm32f3;

pub struct DiscoveryStm32f3 {}

impl bobbin_sys::board::Board for DiscoveryStm32f3 {
    fn id(&self) -> &'static str { "discovery-stm32f3" }    
}

fn default_handler(_irqn: i16) {
    bobbin_sys::irq_dispatch::IrqDispatcher::<Mcu>::handle_exception()
}

cortex_m_rt::exception!(*, default_handler);
cortex_m_rt::exception!(SysTick, bobbin_sys::tick::Tick::tick);
cortex_m_rt::exception!(PendSV, bobbin_sys::pend::Pend::pend);