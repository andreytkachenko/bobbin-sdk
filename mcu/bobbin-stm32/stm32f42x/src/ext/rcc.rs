use bobbin_bits::{U1, U2};
use bobbin_mcu::clock::ClockSource;

#[repr(u8)]
pub enum DedicatedClock {
    Pclk = 0b00,
    Sysclk = 0b01,
    Hsi = 0b10,
    Lse = 0b11,
}

#[repr(u8)]
pub enum TimClock {
    Pclk = 0b0,
    Pll = 0b1,
}

impl From<U1> for TimClock {
    fn from(other: U1) -> TimClock {
        match other {
            U1::B0 => TimClock::Pclk,
            U1::B1 => TimClock::Pll,
        }
    }
}

impl From<TimClock> for U1 {
    fn from(other: TimClock) -> U1 {
        match other {
            TimClock::Pclk => U1::B0,
            TimClock::Pll => U1::B1,
        }
    }
}


#[repr(u8)]
pub enum I2cClock {
    Hsi = 0b0,
    Sysclk = 0b1,
}

impl From<U1> for I2cClock {
    fn from(other: U1) -> I2cClock {
        match other {
            U1::B0 => I2cClock::Hsi,
            U1::B1 => I2cClock::Sysclk,
        }
    }
}
impl From<I2cClock> for U1 {
    fn from(other: I2cClock) -> U1 {
        match other {
            I2cClock::Hsi => U1::B0,
            I2cClock::Sysclk => U1::B1,
        }
    }
}

#[repr(u8)]
pub enum UsartClock {
    Pclk = 0b00,
    Sysclk = 0b01,
    Lse = 0b10,
    Hsi = 0b11,
}

impl From<U2> for UsartClock {
    fn from(other: U2) -> UsartClock {
        match other {
            U2::B00 => UsartClock::Pclk,
            U2::B01 => UsartClock::Sysclk,
            U2::B10 => UsartClock::Lse,
            U2::B11 => UsartClock::Hsi,
        }
    }
}
impl From<UsartClock> for U2 {
    fn from(other: UsartClock) -> U2 {
        match other {
            UsartClock::Pclk => U2::B00,
            UsartClock::Sysclk => U2::B01,
            UsartClock::Lse => U2::B10,
            UsartClock::Hsi => U2::B11,
        }
    }
}


impl From<U2> for DedicatedClock {
    fn from(other: U2) -> DedicatedClock {
        match other {
            U2::B00 => DedicatedClock::Pclk,
            U2::B01 => DedicatedClock::Sysclk,
            U2::B10 => DedicatedClock::Hsi,
            U2::B11 => DedicatedClock::Lse,
        }
    }
}
impl From<DedicatedClock> for U2 {
    fn from(other: DedicatedClock) -> U2 {
        match other {
            DedicatedClock::Pclk => U2::B00,
            DedicatedClock::Sysclk => U2::B01,
            DedicatedClock::Hsi => U2::B10,
            DedicatedClock::Lse => U2::B11,
        }
    }
}

macro_rules! impl_clocksource {
    ($periph:path, $getter:ident, $setter:ident) => {
        impl ClockSource<DedicatedClock> for $periph {
            fn clock_source(&self) -> DedicatedClock {
                // ::rcc::RCC.dckcfgr().$getter().into()
                unimplemented!()
            }
            fn set_clock_source(&self, clk: DedicatedClock) -> &Self {
                // ::rcc::RCC.with_dckcfgr(|r| r.$setter(clk));
                // self
                unimplemented!()
            }
        }        
    };
}

impl_clocksource!(::usart::Usart1, usart1sel, set_usart1sel);
impl_clocksource!(::usart::Usart2, usart2sel, set_usart2sel);
impl_clocksource!(::usart::Usart3, usart3sel, set_usart3sel);
impl_clocksource!(::usart::Uart4, uart4sel, set_uart4sel);
impl_clocksource!(::usart::Uart5, uart5sel, set_uart5sel);
impl_clocksource!(::usart::Usart6, usart6sel, set_usart6sel);
impl_clocksource!(::usart::Uart7, uart7sel, set_uart7sel);
impl_clocksource!(::usart::Uart8, uart8sel, set_uart8sel);

impl_clocksource!(::i2c::I2c1, i2c1sel, set_i2c1sel);
impl_clocksource!(::i2c::I2c2, i2c2sel, set_i2c2sel);
impl_clocksource!(::i2c::I2c3, i2c3sel, set_i2c3sel);

