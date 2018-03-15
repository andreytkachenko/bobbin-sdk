#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::sim::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( SIM, Sim, SIM_PERIPH, SimPeriph, 0x40047000, 0x00);

impl En for super::uart::Uart4 {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc1().uart4() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc1(|r| r.set_uart4(value)); }
}

impl En for super::uart::Uart5 {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc1().uart5() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc1(|r| r.set_uart5(value)); }
}

impl En for super::spi::Spi2 {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc3().spi2() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc3(|r| r.set_spi2(value)); }
}

impl En for super::i2c::I2c0 {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc4().i2c0() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc4(|r| r.set_i2c0(value)); }
}

impl En for super::i2c::I2c1 {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc4().i2c1() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc4(|r| r.set_i2c1(value)); }
}

impl En for super::uart::Uart0 {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc4().uart0() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc4(|r| r.set_uart0(value)); }
}

impl En for super::uart::Uart1 {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc4().uart1() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc4(|r| r.set_uart1(value)); }
}

impl En for super::uart::Uart2 {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc4().uart2() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc4(|r| r.set_uart2(value)); }
}

impl En for super::uart::Uart3 {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc4().uart3() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc4(|r| r.set_uart3(value)); }
}

impl En for super::port::Porta {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc5().porta() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc5(|r| r.set_porta(value)); }
}

impl En for super::port::Portb {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc5().portb() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc5(|r| r.set_portb(value)); }
}

impl En for super::port::Portc {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc5().portc() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc5(|r| r.set_portc(value)); }
}

impl En for super::port::Portd {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc5().portd() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc5(|r| r.set_portd(value)); }
}

impl En for super::port::Porte {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc5().porte() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc5(|r| r.set_porte(value)); }
}

impl En for super::spi::Spi0 {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc6().spi0() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc6(|r| r.set_spi0(value)); }
}

impl En for super::spi::Spi1 {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc6().spi1() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc6(|r| r.set_spi1(value)); }
}

impl En for super::pit::Pit {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc6().pit() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc6(|r| r.set_pit(value)); }
}

impl En for super::ftm::Ftm0 {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc6().ftm0() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc6(|r| r.set_ftm0(value)); }
}

impl En for super::ftm::Ftm1 {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc6().ftm1() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc6(|r| r.set_ftm1(value)); }
}

impl En for super::ftm::Ftm2 {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc6().ftm2() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc6(|r| r.set_ftm2(value)); }
}

impl En for super::edma::Dma {
    #[inline] fn en(&self) -> bits::U1 { SIM.scgc7().dma() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { SIM.with_scgc7(|r| r.set_dma(value)); }
}

