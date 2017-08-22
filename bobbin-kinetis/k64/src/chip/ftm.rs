#[allow(unused_imports)] use bobbin_common::bits;
pub use kinetis_common::chip::ftm::*;

pub const FTM0: Ftm0 = Periph(0x40038000, Ftm0Id {});
pub const FTM1: Ftm1 = Periph(0x40039000, Ftm1Id {});
pub const FTM2: Ftm2 = Periph(0x4003a000, Ftm2Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Ftm0Id {}
pub type Ftm0 = Periph<Ftm0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Ftm1Id {}
pub type Ftm1 = Periph<Ftm1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Ftm2Id {}
pub type Ftm2 = Periph<Ftm2Id>;

impl super::sig::Signal<super::sig::Ftm0Ch0> for Ftm0Ch0 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch0> for Ftm0Ch0 {}
impl super::sig::Signal<super::sig::Ftm0Ch1> for Ftm0Ch1 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch1> for Ftm0Ch1 {}
impl super::sig::Signal<super::sig::Ftm0Ch2> for Ftm0Ch2 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch2> for Ftm0Ch2 {}
impl super::sig::Signal<super::sig::Ftm0Ch3> for Ftm0Ch3 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch3> for Ftm0Ch3 {}
impl super::sig::Signal<super::sig::Ftm0Ch4> for Ftm0Ch4 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch4> for Ftm0Ch4 {}
impl super::sig::Signal<super::sig::Ftm0Ch5> for Ftm0Ch5 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch5> for Ftm0Ch5 {}
impl super::sig::Signal<super::sig::Ftm0Ch6> for Ftm0Ch6 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch6> for Ftm0Ch6 {}
impl super::sig::Signal<super::sig::Ftm0Ch7> for Ftm0Ch7 {}
impl super::sig::SignalFtm<super::sig::Ftm0Ch7> for Ftm0Ch7 {}

impl super::sig::Signal<super::sig::Ftm1Ch0> for Ftm1Ch0 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch0> for Ftm1Ch0 {}
impl super::sig::Signal<super::sig::Ftm1Ch1> for Ftm1Ch1 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch1> for Ftm1Ch1 {}
impl super::sig::Signal<super::sig::Ftm1Ch2> for Ftm1Ch2 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch2> for Ftm1Ch2 {}
impl super::sig::Signal<super::sig::Ftm1Ch3> for Ftm1Ch3 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch3> for Ftm1Ch3 {}
impl super::sig::Signal<super::sig::Ftm1Ch4> for Ftm1Ch4 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch4> for Ftm1Ch4 {}
impl super::sig::Signal<super::sig::Ftm1Ch5> for Ftm1Ch5 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch5> for Ftm1Ch5 {}
impl super::sig::Signal<super::sig::Ftm1Ch6> for Ftm1Ch6 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch6> for Ftm1Ch6 {}
impl super::sig::Signal<super::sig::Ftm1Ch7> for Ftm1Ch7 {}
impl super::sig::SignalFtm<super::sig::Ftm1Ch7> for Ftm1Ch7 {}

impl super::sig::Signal<super::sig::Ftm2Ch0> for Ftm2Ch0 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch0> for Ftm2Ch0 {}
impl super::sig::Signal<super::sig::Ftm2Ch1> for Ftm2Ch1 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch1> for Ftm2Ch1 {}
impl super::sig::Signal<super::sig::Ftm2Ch2> for Ftm2Ch2 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch2> for Ftm2Ch2 {}
impl super::sig::Signal<super::sig::Ftm2Ch3> for Ftm2Ch3 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch3> for Ftm2Ch3 {}
impl super::sig::Signal<super::sig::Ftm2Ch4> for Ftm2Ch4 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch4> for Ftm2Ch4 {}
impl super::sig::Signal<super::sig::Ftm2Ch5> for Ftm2Ch5 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch5> for Ftm2Ch5 {}
impl super::sig::Signal<super::sig::Ftm2Ch6> for Ftm2Ch6 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch6> for Ftm2Ch6 {}
impl super::sig::Signal<super::sig::Ftm2Ch7> for Ftm2Ch7 {}
impl super::sig::SignalFtm<super::sig::Ftm2Ch7> for Ftm2Ch7 {}


pub const FTM0_CH0: Channel<Ftm0Ch0Id, Ftm0Id> = Channel { periph: FTM0, index: 0, id: Ftm0Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm0Ch0Id {}
pub type Ftm0Ch0 = Channel<Ftm0Ch0Id, Ftm0Id>;

pub const FTM0_CH1: Channel<Ftm0Ch1Id, Ftm0Id> = Channel { periph: FTM0, index: 1, id: Ftm0Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm0Ch1Id {}
pub type Ftm0Ch1 = Channel<Ftm0Ch1Id, Ftm0Id>;

pub const FTM0_CH2: Channel<Ftm0Ch2Id, Ftm0Id> = Channel { periph: FTM0, index: 2, id: Ftm0Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm0Ch2Id {}
pub type Ftm0Ch2 = Channel<Ftm0Ch2Id, Ftm0Id>;

pub const FTM0_CH3: Channel<Ftm0Ch3Id, Ftm0Id> = Channel { periph: FTM0, index: 3, id: Ftm0Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm0Ch3Id {}
pub type Ftm0Ch3 = Channel<Ftm0Ch3Id, Ftm0Id>;

pub const FTM0_CH4: Channel<Ftm0Ch4Id, Ftm0Id> = Channel { periph: FTM0, index: 4, id: Ftm0Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm0Ch4Id {}
pub type Ftm0Ch4 = Channel<Ftm0Ch4Id, Ftm0Id>;

pub const FTM0_CH5: Channel<Ftm0Ch5Id, Ftm0Id> = Channel { periph: FTM0, index: 5, id: Ftm0Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm0Ch5Id {}
pub type Ftm0Ch5 = Channel<Ftm0Ch5Id, Ftm0Id>;

pub const FTM0_CH6: Channel<Ftm0Ch6Id, Ftm0Id> = Channel { periph: FTM0, index: 6, id: Ftm0Ch6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm0Ch6Id {}
pub type Ftm0Ch6 = Channel<Ftm0Ch6Id, Ftm0Id>;

pub const FTM0_CH7: Channel<Ftm0Ch7Id, Ftm0Id> = Channel { periph: FTM0, index: 7, id: Ftm0Ch7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm0Ch7Id {}
pub type Ftm0Ch7 = Channel<Ftm0Ch7Id, Ftm0Id>;

pub const FTM1_CH0: Channel<Ftm1Ch0Id, Ftm1Id> = Channel { periph: FTM1, index: 0, id: Ftm1Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm1Ch0Id {}
pub type Ftm1Ch0 = Channel<Ftm1Ch0Id, Ftm1Id>;

pub const FTM1_CH1: Channel<Ftm1Ch1Id, Ftm1Id> = Channel { periph: FTM1, index: 1, id: Ftm1Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm1Ch1Id {}
pub type Ftm1Ch1 = Channel<Ftm1Ch1Id, Ftm1Id>;

pub const FTM1_CH2: Channel<Ftm1Ch2Id, Ftm1Id> = Channel { periph: FTM1, index: 2, id: Ftm1Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm1Ch2Id {}
pub type Ftm1Ch2 = Channel<Ftm1Ch2Id, Ftm1Id>;

pub const FTM1_CH3: Channel<Ftm1Ch3Id, Ftm1Id> = Channel { periph: FTM1, index: 3, id: Ftm1Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm1Ch3Id {}
pub type Ftm1Ch3 = Channel<Ftm1Ch3Id, Ftm1Id>;

pub const FTM1_CH4: Channel<Ftm1Ch4Id, Ftm1Id> = Channel { periph: FTM1, index: 4, id: Ftm1Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm1Ch4Id {}
pub type Ftm1Ch4 = Channel<Ftm1Ch4Id, Ftm1Id>;

pub const FTM1_CH5: Channel<Ftm1Ch5Id, Ftm1Id> = Channel { periph: FTM1, index: 5, id: Ftm1Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm1Ch5Id {}
pub type Ftm1Ch5 = Channel<Ftm1Ch5Id, Ftm1Id>;

pub const FTM1_CH6: Channel<Ftm1Ch6Id, Ftm1Id> = Channel { periph: FTM1, index: 6, id: Ftm1Ch6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm1Ch6Id {}
pub type Ftm1Ch6 = Channel<Ftm1Ch6Id, Ftm1Id>;

pub const FTM1_CH7: Channel<Ftm1Ch7Id, Ftm1Id> = Channel { periph: FTM1, index: 7, id: Ftm1Ch7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm1Ch7Id {}
pub type Ftm1Ch7 = Channel<Ftm1Ch7Id, Ftm1Id>;

pub const FTM2_CH0: Channel<Ftm2Ch0Id, Ftm2Id> = Channel { periph: FTM2, index: 0, id: Ftm2Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm2Ch0Id {}
pub type Ftm2Ch0 = Channel<Ftm2Ch0Id, Ftm2Id>;

pub const FTM2_CH1: Channel<Ftm2Ch1Id, Ftm2Id> = Channel { periph: FTM2, index: 1, id: Ftm2Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm2Ch1Id {}
pub type Ftm2Ch1 = Channel<Ftm2Ch1Id, Ftm2Id>;

pub const FTM2_CH2: Channel<Ftm2Ch2Id, Ftm2Id> = Channel { periph: FTM2, index: 2, id: Ftm2Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm2Ch2Id {}
pub type Ftm2Ch2 = Channel<Ftm2Ch2Id, Ftm2Id>;

pub const FTM2_CH3: Channel<Ftm2Ch3Id, Ftm2Id> = Channel { periph: FTM2, index: 3, id: Ftm2Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm2Ch3Id {}
pub type Ftm2Ch3 = Channel<Ftm2Ch3Id, Ftm2Id>;

pub const FTM2_CH4: Channel<Ftm2Ch4Id, Ftm2Id> = Channel { periph: FTM2, index: 4, id: Ftm2Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm2Ch4Id {}
pub type Ftm2Ch4 = Channel<Ftm2Ch4Id, Ftm2Id>;

pub const FTM2_CH5: Channel<Ftm2Ch5Id, Ftm2Id> = Channel { periph: FTM2, index: 5, id: Ftm2Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm2Ch5Id {}
pub type Ftm2Ch5 = Channel<Ftm2Ch5Id, Ftm2Id>;

pub const FTM2_CH6: Channel<Ftm2Ch6Id, Ftm2Id> = Channel { periph: FTM2, index: 6, id: Ftm2Ch6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm2Ch6Id {}
pub type Ftm2Ch6 = Channel<Ftm2Ch6Id, Ftm2Id>;

pub const FTM2_CH7: Channel<Ftm2Ch7Id, Ftm2Id> = Channel { periph: FTM2, index: 7, id: Ftm2Ch7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Ftm2Ch7Id {}
pub type Ftm2Ch7 = Channel<Ftm2Ch7Id, Ftm2Id>;

pub trait IrqFtm<T> {
   fn irq_ftm(&self) -> super::irq::Irq<T>;
}

pub trait RegisterFtmHandler {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleFtm {
   fn handle_ftm(&self);
}

impl IrqFtm<super::irq::Ftm0Id> for Ftm0 {
   fn irq_ftm(&self) -> super::irq::IrqFtm0 { super::irq::IRQ_FTM0 }
}

impl RegisterFtmHandler for Ftm0 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(42, Some(wrapper::<F>));
       super::irq::IrqGuard::new(42)
   }
}

impl IrqFtm<super::irq::Ftm1Id> for Ftm1 {
   fn irq_ftm(&self) -> super::irq::IrqFtm1 { super::irq::IRQ_FTM1 }
}

impl RegisterFtmHandler for Ftm1 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(43, Some(wrapper::<F>));
       super::irq::IrqGuard::new(43)
   }
}

impl IrqFtm<super::irq::Ftm2Id> for Ftm2 {
   fn irq_ftm(&self) -> super::irq::IrqFtm2 { super::irq::IRQ_FTM2 }
}

impl RegisterFtmHandler for Ftm2 {
   fn register_ftm_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleFtm>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleFtm>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_ftm() }
       }
       super::irq::set_handler(44, Some(wrapper::<F>));
       super::irq::IrqGuard::new(44)
   }
}

