use bobbin_mcu::hz::Hz;
use bobbin_mcu::clock::{Clock, ClockFor, ClockSource};
use bobbin_bits::*;

use ext::rcc::DedicatedClock;

use rcc::RCC;
use flash::FLASH;
use pwr::PWR;
use clock::*;

use ::core::intrinsics::abort;

#[derive(Default)]
pub struct Osc8m {}
impl Clock for Osc8m {
    fn hz() -> Hz { Hz::from_num(8_000_000) }
}

#[derive(Default)]
pub struct Osc32k {}
impl Clock for Osc32k {
    fn hz() -> Hz { Hz::from_num(32768) }
}

#[derive(Default)]
pub struct DynamicClock<OSC: Clock, OSC32: Clock>(OSC, OSC32);

impl<OSC: Clock, OSC32: Clock> DynamicClock<OSC, OSC32> {
    fn pllq(&self) -> Hz {
        let cfgr = RCC.pllcfgr();
        let vco_in = match cfgr.pllsrc() {
            U1::B0 => self.hsi(),
            U1::B1 => self.hse(),
        };
        let vco = (vco_in / cfgr.pllm().into_u32()).normalized() * cfgr.plln().into_u32();
        (vco / cfgr.pllq().into_u32()).normalized()
    }    
}

macro_rules! impl_clock_source {
    ($periph:path, $id:ident, $default:ident) => {
        fn $id(&self) -> Hz {
            match $periph.clock_source() {
                DedicatedClock::Pclk => self.$default(),
                DedicatedClock::Sysclk => self.sysclk(),
                DedicatedClock::Hsi => self.hsi(),
                DedicatedClock::Lse => self.lse(),
            }
        }        
    };
}

impl<OSC: Clock, OSC32: Clock> ClockProvider for DynamicClock<OSC, OSC32> {
    type Osc = OSC;
    type Osc32 = OSC32;
    fn pllclk(&self) -> Hz {
        let cfgr = RCC.pllcfgr();
        let vco_in = match cfgr.pllsrc() {
            U1::B0 => self.lsi(),
            U1::B1 => self.hse(),
        };
        let vco = (vco_in / cfgr.pllm().into_u32()).normalized();
        (vco  * cfgr.plln().into_u32() / (2 * (cfgr.pllp().into_u32() + 1))).normalized()
    }



    fn pll48clk(&self) -> Hz {
        match RCC.dckcfgr2().ck48msel() {
            U1::B0 => self.pllq(),
            U1::B1 => unsafe { abort() },
        }
    }

    fn sysclk(&self) -> Hz {
        match RCC.cfgr().sws() {
            U2::B00 => self.hsi(),
            U2::B01 => self.hse(),
            U2::B10 => self.pllclk(),
            U2::B11 => unsafe { abort() },
        }
    }

    fn hclk(&self) -> Hz {
        let shift = match RCC.cfgr().hpre() {
            U4::B0000 => 0,
            U4::B0001 => 0,
            U4::B0010 => 0,
            U4::B0011 => 0,
            U4::B0100 => 0,
            U4::B0101 => 0,
            U4::B0110 => 0,
            U4::B0111 => 0,
            U4::B1000 => 1,
            U4::B1001 => 2,
            U4::B1010 => 3,
            U4::B1011 => 4,
            // NOTE: Divide by 32 is skipped
            U4::B1100 => 6,
            U4::B1101 => 7,
            U4::B1110 => 8,
            U4::B1111 => 9,
        };
        self.sysclk() >> shift
    }    

    fn systick(&self) -> Hz {
        self.hclk() >> 3
    }

    fn pclk1(&self) -> Hz {
        let shift = match RCC.cfgr().ppre1() {
            U3::B000 => 0,
            U3::B001 => 0,
            U3::B010 => 0,
            U3::B011 => 0,            
            U3::B100 => 1,
            U3::B101 => 2,
            U3::B110 => 3,
            U3::B111 => 4,
        };
        self.hclk() >> shift
    }

    fn tim_pclk1(&self) -> Hz {
        match RCC.cfgr().ppre1() {
            U3::B000 | U3::B001 | U3::B010 | U3::B011 => self.pclk1(),
            _ => self.pclk1() << 1,        
        }
    }

    fn pclk2(&self) -> Hz {
        let shift = match RCC.cfgr().ppre2() {
            U3::B000 => 0,
            U3::B001 => 0,
            U3::B010 => 0,
            U3::B011 => 0,            
            U3::B100 => 1,
            U3::B101 => 2,
            U3::B110 => 3,
            U3::B111 => 4,
        };
        self.hclk() >> shift
    }

    fn tim_pclk2(&self) -> Hz {
        match RCC.cfgr().ppre2() {
            U3::B000 | U3::B001 | U3::B010 | U3::B011 => self.pclk2(),
            _ => self.pclk2() << 1,
        }
    }    

    impl_clock_source!(::usart::USART1, usart1, pclk2);
    impl_clock_source!(::usart::USART2, usart2, pclk1);
    impl_clock_source!(::usart::USART3, usart3, pclk1);
    impl_clock_source!(::usart::UART4, uart4, pclk1);
    impl_clock_source!(::usart::UART5, uart5, pclk1);
    impl_clock_source!(::usart::USART6, usart6, pclk2);
    impl_clock_source!(::usart::UART7, uart7, pclk1);
    impl_clock_source!(::usart::UART8, uart8, pclk1);

    impl_clock_source!(::i2c::I2C1, i2c1, pclk1);
    impl_clock_source!(::i2c::I2C2, i2c2, pclk1);
    impl_clock_source!(::i2c::I2C3, i2c3, pclk1);
    
}

impl<CP> ClockFor<::systick::Systick> for Clocks<CP> where CP: ClockProvider {
    fn clock_for(&self, _: ::systick::Systick) -> Hz { self.systick() }
}

pub fn enable_pll_hse_bypass_mode(m: u32, n: u32, p: u32, q: u32) {
    let rcc = RCC;
    let flash = FLASH;
    let pwr = PWR;
    
    // Enable internal high-speed oscillator.
    rcc.with_cr(|r| r.set_hsion(1));

    // Wait for HSI Ready
    while rcc.cr().hsirdy() == 0 {}

    // Select HSI as SYSCLK source. 
    rcc.with_cfgr(|r| r.set_sw(0b00));
    while rcc.cfgr().sws() != 0b00 {}

    // Enable external high-speed oscillator 8MHz.
    // rcc.with_cr(|r| r.set_hseon(1));

    // Enable external source
    rcc.with_cr(|r| r.set_hseon(1).set_hsebyp(1));
    
    // Wait for HSE Ready
    while rcc.cr().hserdy() == 0 {}

    pwr.with_csr1(|r| r.set_vosrdy(1));

    // Set prescalers for AHB, ADC, ABP1, ABP2

    // HPRE = HPRE_DIV_NONE
    // PPRE1 = PPRE_DIV_4
    // PPRE2 = PPRE_DIV_2
    rcc.with_cfgr(|r|
        r.set_hpre(0)
         .set_ppre1(0b101)
         .set_ppre2(0b100));

    rcc.with_pllcfgr(|r|
        r.set_pllsrc(1)
            .set_pllm(m)
            .set_plln(n)
            .set_pllp(p)
            .set_pllq(q)
    );

    // Enable PLL oscillator and wait for it to stabilize.
    rcc.with_cr(|r| r.set_pllon(1));


    // Enable Overdrive

    rcc.with_apb1enr(|r| r.set_pwren(1));

    pwr.with_cr1(|r| r.set_oden(1));
    while !pwr.csr1().test_odrdy() {asm!("nop")}

    pwr.with_cr1(|r| r.set_odswen(1));
    while !pwr.csr1().test_odswrdy() {asm!("nop")}

    // Wait for PLL Ready
    while rcc.cr().pllrdy() == 0 {asm!("nop")}

    // Configure flash settings.

    flash.with_acr(|r| 
        r.set_icen(1)
         .set_dcen(1)
         .set_latency(5));
    
    // Select PLL as SYSCLK source.

    rcc.with_cfgr(|r| r.set_sw(0b10));
    while rcc.cfgr().sws() != 0b10 {asm!("nop")}

    // Disable internal high-speed oscillator.        
    // rcc.with_cr(|r| r.set_hsion(0));
}

pub fn enable_pll_hse_mode(m: u32, n: u32, p: u32, q: u32) {
    let rcc = RCC;
    let flash = FLASH;
    let pwr = PWR;

    // Enable internal high-speed oscillator.
    rcc.with_cr(|r| r.set_hsion(1));

    // Wait for HSI Ready
    while rcc.cr().hsirdy() == 0 {asm!("nop")}

    // Select HSI as SYSCLK source. 
    rcc.with_cfgr(|r| r.set_sw(0b00));
    while rcc.cfgr().sws() != 0b00 {}

    // Enable external high-speed oscillator 8MHz.
    // rcc.with_cr(|r| r.set_hseon(1));

    // Enable external source
    rcc.with_cr(|r| r.set_hseon(1).set_hsebyp(0));
    
    // Wait for HSE Ready
    while rcc.cr().hserdy() == 0 {asm!("nop")}

    pwr.with_csr1(|r| r.set_vosrdy(1));

    // Set prescalers for AHB, ADC, ABP1, ABP2

    // HPRE  = HPRE_DIV_NONE
    // PPRE1 = PPRE_DIV_4
    // PPRE2 = PPRE_DIV_2
    rcc.with_cfgr(|r| 
        r.set_hpre(0)
         .set_ppre1(0b101)
         .set_ppre2(0b100));
    
    rcc.with_pllcfgr(|r|
        r.set_pllsrc(1)
            .set_pllm(m)
            .set_plln(n)
            .set_pllp(p)
            .set_pllq(q)
    );

    // Enable PLL oscillator and wait for it to stabilize.
    rcc.with_cr(|r| r.set_pllon(1));
    
    // Wait for PLL Ready
    while rcc.cr().pllrdy() == 0 {asm!("nop")}

    // Configure flash settings.

    flash.with_acr(|r| 
        r.set_icen(1)
         .set_dcen(1)
         .set_latency(5));
    
    // Select PLL as SYSCLK source.

    rcc.with_cfgr(|r| r.set_sw(0b10));
    while rcc.cfgr().sws() != 0b10 {asm!("nop")}
    
    // // Disable internal high-speed oscillator.        
    // rcc.with_cr(|r| r.set_hsion(0));
}

pub fn enable_pll_hsi_mode() {
    let rcc = RCC;
    let flash = FLASH;
    let pwr = PWR;

    // Enable internal high-speed oscillator.
    rcc.with_cr(|r| r.set_hsion(1));

    // Wait for HSI Ready
    while rcc.cr().hsirdy() == 0 {asm!("nop")}

    // // Select HSI as SYSCLK source. 
    // rcc.with_cfgr(|r| r.set_sw(0b00));
    // while RCC.cfgr().sws() != 0b00 {}

    pwr.with_csr1(|r| r.set_vosrdy(1));

    // Set prescalers for AHB, ADC, ABP1, ABP2

    // HPRE = HPRE_DIV_NONE
    // PPRE1 = PPRE_DIV_4
    // PPRE2 = PPRE_DIV_2
    rcc.with_cfgr(|r| 
        r.set_hpre(0)
         .set_ppre1(0b101)
         .set_ppre2(0b100));

    // Configure PLL
    // PLLSRC = HSE
    // M = 8
    // N = 336
    // P = 2
    // Q = 7
    // R = 0
    
    rcc.with_pllcfgr(|r|
        r.set_pllsrc(0)
            .set_pllm(8)
            .set_plln(360)
            .set_pllp(2)
            .set_pllq(8)
    );

    // Enable PLL oscillator and wait for it to stabilize.
    rcc.with_cr(|r| r.set_pllon(1));
    
    // Wait for PLL Ready
    while rcc.cr().pllrdy() == 0 {asm!("nop")}

    // // Configure flash settings.

    flash.with_acr(|r| 
        r.set_icen(1)
         .set_dcen(1)
         .set_latency(5));
    
    // // Select PLL as SYSCLK source.

    rcc.with_cfgr(|r| r.set_sw(0b10));
    while rcc.cfgr().sws() != 0b10 {asm!("nop")}
}