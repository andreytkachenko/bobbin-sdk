pub use ::stm32_common::tim_bas::*;

::bobbin_mcu::periph!( TIM6, Tim6, TIM6_PERIPH, TimBasPeriph, TIM6_OWNED, TIM6_REF_COUNT, 0x40001000, 0x00, 0x1c);
::bobbin_mcu::periph!( TIM7, Tim7, TIM7_PERIPH, TimBasPeriph, TIM7_OWNED, TIM7_REF_COUNT, 0x40001400, 0x01, 0x1d);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("TIM6RST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Tim6 {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1rstr().tim6rst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_tim6rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("TIM6EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Tim6 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1enr().tim6en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_tim6en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1LPENR"), field: Some("TIM6LPEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Tim6 {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1lpenr().tim6lpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1lpenr(|r| r.set_tim6lpen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("TIM7RST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Tim7 {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1rstr().tim7rst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_tim7rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("TIM7EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Tim7 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1enr().tim7en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_tim7en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1LPENR"), field: Some("TIM7LPEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Tim7 {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1lpenr().tim7lpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1lpenr(|r| r.set_tim7lpen(value));
        self
    }
}

