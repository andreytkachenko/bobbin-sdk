use chip::tc::*;
use hal::pm;
use hal::gclk;
use driver::tc::*;

// Note: actually ticks at 1.024kHz
pub fn delay(ticks: u16) {
    // Use GenClk2 @ 1.024 kHz
    gclk::set_clk(gclk::GenericClock::TCC2_TC3, gclk::GenericClockGen::GClkGen2);
    pm::set_tc_enabled(TC3, true);

    let tc = configure_16bit(TC3, Config {
        prescsync: Prescsync::GCLK,
        runstdby: false,
        prescaler: Prescaler::Div1,
        wavegen: Wavegen::NFRQ,
    });

    tc.set_cc(0, ticks);
    tc.clr_syncrdy();

    tc.set_mc0_enabled(true);
    tc.set_enabled(true);
    
    while tc.intflag().mc0() == 0 {}
    tc.set_enabled(false);
}