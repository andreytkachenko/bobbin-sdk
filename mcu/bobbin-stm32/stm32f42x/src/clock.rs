use ::bobbin_mcu::clock::{Clock, ClockFor};
use ::bobbin_mcu::hz::Hz;

#[derive(Default)]
pub struct Clocks<CP: ClockProvider> { provider: CP }

impl<CP: ClockProvider> ::core::ops::Deref for Clocks<CP> {
    type Target = CP;
    fn deref(&self) -> &CP { &self.provider }
}


// Define Global Clocks

#[derive(Default)]
pub struct Hsi {}
impl Clock for Hsi { fn hz() -> Hz { Hz::from_num(16000000) } }

#[derive(Default)]
pub struct Lsi {}
impl Clock for Lsi { fn hz() -> Hz { Hz::from_num(32000) } }


pub trait ClockProvider : Default {
    type Osc: Clock;
    type Osc32: Clock;
    fn osc(&self) -> Hz { Self::Osc::hz() }
    fn osc32(&self) -> Hz { Self::Osc32::hz() }
    fn hsi(&self) -> Hz { Hz::from_num(16000000) }
    fn hse(&self) -> Hz { self.osc() }
    fn lsi(&self) -> Hz { Hz::from_num(32000) }
    fn lse(&self) -> Hz { self.osc32() }
    fn pllclk(&self) -> Hz { unimplemented!() }
    fn pll48clk(&self) -> Hz { unimplemented!() }
    fn sysclk(&self) -> Hz { unimplemented!() }
    fn i2s(&self) -> Hz { unimplemented!() }
    fn otg_hs_scl(&self) -> Hz { unimplemented!() }
    fn hclk(&self) -> Hz { unimplemented!() }
    fn systick(&self) -> Hz { unimplemented!() }
    fn fclk(&self) -> Hz { unimplemented!() }
    fn pclk1(&self) -> Hz { unimplemented!() }
    fn pclk2(&self) -> Hz { unimplemented!() }
    fn tim_pclk1(&self) -> Hz { unimplemented!() }
    fn tim_pclk2(&self) -> Hz { unimplemented!() }
    fn rtc(&self) -> Hz { unimplemented!() }
    fn sdmmc(&self) -> Hz { unimplemented!() }
    fn hdmi_cec(&self) -> Hz { unimplemented!() }
    fn spdif(&self) -> Hz { unimplemented!() }
    fn sai1(&self) -> Hz { unimplemented!() }
    fn sai2(&self) -> Hz { unimplemented!() }
    fn eth_mactx(&self) -> Hz { unimplemented!() }
    fn eth_macrx(&self) -> Hz { unimplemented!() }
    fn eth_macrmii(&self) -> Hz { unimplemented!() }
    fn usart1(&self) -> Hz { unimplemented!() }
    fn usart2(&self) -> Hz { unimplemented!() }
    fn usart3(&self) -> Hz { unimplemented!() }
    fn uart4(&self) -> Hz { unimplemented!() }
    fn uart5(&self) -> Hz { unimplemented!() }
    fn usart6(&self) -> Hz { unimplemented!() }
    fn uart7(&self) -> Hz { unimplemented!() }
    fn uart8(&self) -> Hz { unimplemented!() }
    fn i2c1(&self) -> Hz { unimplemented!() }
    fn i2c2(&self) -> Hz { unimplemented!() }
    fn i2c3(&self) -> Hz { unimplemented!() }
}

