pub use mcu::bobbin_common::console::*;
use common::periph::IntoPeriph;
use common::configure::Configure;
use mcu::pin::*;
use mcu::sercom::*;
use mcu::gclk;

pub const SERCOM: Sercom0 = SERCOM0;
pub const SERCOM_TX: Pa10 = PA10;
pub const SERCOM_RX: Pa11 = PA11;

pub fn init() {
    SERCOM.gate_enable();
    SERCOM_RX.port().gate_enable();
    SERCOM_TX.port().gate_enable();
    // Set GCLK_GEN0 as source for SERCOM

    gclk::GCLK.set_clkctrl(|r| r
        .set_id(0x14 + 0) // ID corresponds to SERCOM
        .set_gen(0x0)
        .set_clken(1)
    );
    // Wait for synchronization
    while gclk::GCLK.status().syncbusy() != 0 {}

    // Set Pin Configuration
    SERCOM_TX.connect_to(SERCOM);
    SERCOM_RX.connect_to(SERCOM);

    SERCOM
        .set_config(|c| c
            .set_mode_usart_int()
            .set_baud(63018)
            .set_txpo(1)
            .set_rxpo(3)
        )
        .set_enabled(true);

    set_console(Console::new(SERCOM.into_periph()));
}

impl ::FeatherM0 {
    pub fn console(&self) -> Console {
        Console::new(SERCOM.into_periph())
    }
}