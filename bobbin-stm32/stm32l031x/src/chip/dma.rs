#[allow(unused_imports)] use bobbin_common::bits;
pub use stm32_common::chip::dma_f3::*;

pub const DMA1: Dma1 = Periph(0x40020000, Dma1Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Dma1Id {}
pub type Dma1 = Periph<Dma1Id>;



pub const DMA1_CH1: Channel<Dma1Ch1Id, Dma1Id> = Channel { periph: DMA1, index: 0, id: Dma1Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Ch1Id {}
pub type Dma1Ch1 = Channel<Dma1Ch1Id, Dma1Id>;

pub const DMA1_CH2: Channel<Dma1Ch2Id, Dma1Id> = Channel { periph: DMA1, index: 1, id: Dma1Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Ch2Id {}
pub type Dma1Ch2 = Channel<Dma1Ch2Id, Dma1Id>;

pub const DMA1_CH3: Channel<Dma1Ch3Id, Dma1Id> = Channel { periph: DMA1, index: 2, id: Dma1Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Ch3Id {}
pub type Dma1Ch3 = Channel<Dma1Ch3Id, Dma1Id>;

pub const DMA1_CH4: Channel<Dma1Ch4Id, Dma1Id> = Channel { periph: DMA1, index: 3, id: Dma1Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Ch4Id {}
pub type Dma1Ch4 = Channel<Dma1Ch4Id, Dma1Id>;

pub const DMA1_CH5: Channel<Dma1Ch5Id, Dma1Id> = Channel { periph: DMA1, index: 4, id: Dma1Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Ch5Id {}
pub type Dma1Ch5 = Channel<Dma1Ch5Id, Dma1Id>;

pub const DMA1_CH6: Channel<Dma1Ch6Id, Dma1Id> = Channel { periph: DMA1, index: 5, id: Dma1Ch6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Ch6Id {}
pub type Dma1Ch6 = Channel<Dma1Ch6Id, Dma1Id>;

pub const DMA1_CH7: Channel<Dma1Ch7Id, Dma1Id> = Channel { periph: DMA1, index: 6, id: Dma1Ch7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Dma1Ch7Id {}
pub type Dma1Ch7 = Channel<Dma1Ch7Id, Dma1Id>;

