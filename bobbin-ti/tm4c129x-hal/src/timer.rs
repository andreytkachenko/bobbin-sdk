pub use chip::timer::*;
pub use sysctl::SysctlEnabled;

pub enum Dir {
    Down = 0,
    Up = 1,
}

pub enum Mode {
    OneShot = 1,
    Periodic = 2,
    Capture = 3,
}

pub trait TimerExt {
    fn delay(&self, value: u32) -> &Self;
}

impl<T> TimerExt for Periph<T> {    
    fn delay(&self, value: u32) -> &Self {
        // disable timer a        
        self.with_ctl(|r| r.set_ten(0, 0));
        // set 32 bit mode
        self.set_cfg(Cfg(0x0));
        // set timer a mode = one-shot
        self.set_tmr(0, Tmr(0).set_tmr(0x01));        
        // set timer a load register
        self.set_tilr(0, Tilr(0).set_tilr(value));
        // clear timeout interrupt
        self.set_icr(Icr(0).set_ttocint(0, 1));
        // enable timer a
        self.with_ctl(|r| r.set_ten(0, 1));        
        // wait for timer a timeout
        while self.ris().ttoris(0) == 0 {}
        // clear timeout interrupt
        self.set_icr(Icr(0).set_ttocint(0, 1));
        self
    }
}

pub trait TimerChExt {
    fn tmr(&self) -> Tmr;
    fn set_tmr(&self, Tmr) -> &Self;
    fn with_tmr<F: FnOnce(Tmr) -> Tmr>(&self, F) -> &Self;
    fn enabled(&self) -> bool;
    fn set_enabled(&self, value: bool) -> &Self;
    fn reload(&self) -> u32;
    fn set_reload(&self, value: u32) -> &Self;
    fn compare(&self) -> u32;
    fn set_compare(&self, value: u32) -> &Self;
    fn prescale(&self) -> u8;
    fn set_prescale(&self, value: u8) -> &Self;
    fn value(&self) -> u32;
    fn set_value(&self, value: u32) -> &Self;
    fn match_dmaen(&self) -> bool;
    fn set_match_dmaen(&self, value: bool) -> &Self;
    fn timeout_flag(&self) -> bool;
    fn clr_timeout_flag(&self) -> &Self;
    fn match_flag(&self) -> bool;
    fn clr_match_flag(&self) -> &Self;
    fn dma_done_flag(&self) -> bool;
    fn clr_dma_done_flag(&self) -> &Self;
}

impl<P, T> TimerChExt for Channel<P, T> {
    fn tmr(&self) -> Tmr {
        self.periph.tmr(self.index)
    }
    fn set_tmr(&self, value: Tmr) -> &Self {
        self.periph.set_tmr(self.index, value);
        self
    }
    fn with_tmr<F: FnOnce(Tmr) -> Tmr>(&self, f: F) -> &Self {
        self.periph.with_tmr(self.index, f);
        self
    }
    fn enabled(&self) -> bool {
        self.periph.ctl().ten(self.index) != 0
    }
    fn set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.periph.with_ctl(|r| r.set_ten(self.index, value));
        self
    }
    fn reload(&self) -> u32 {
        self.periph.tilr(self.index).tilr()
    }
    fn set_reload(&self, value: u32) -> &Self {
        self.periph.set_tilr(self.index, Tilr(value));
        self
    }

    fn prescale(&self) -> u8 {
        self.periph.tpr(self.index).tpsr() as u8
    }

    fn set_prescale(&self, value: u8) -> &Self {
        self.periph.set_tpr(self.index, Tpr(0).set_tpsr(value as u32));
        self
    }

    fn compare(&self) -> u32 {
        self.periph.tmtchr(self.index).tmtchr()
    }
    fn set_compare(&self, value: u32) -> &Self {
        self.periph.set_tmtchr(self.index, Tmtchr(value));
        self
    }    
    fn value(&self) -> u32 {
        self.periph.tv(self.index).tv()
    }
    fn set_value(&self, value: u32) -> &Self {
        self.periph.set_tv(self.index, Tv(value));
        self
    }        
    fn match_dmaen(&self) -> bool {
        self.periph.dmaev().tmdmaen(self.index) != 0
    }
    fn set_match_dmaen(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.periph.with_dmaev(|r| r.set_tmdmaen(self.index, value));
        self
    }
    #[inline]
    fn timeout_flag(&self) -> bool {
        self.periph.ris().ttoris(self.index) != 0
    }
    #[inline]
    fn clr_timeout_flag(&self) -> &Self {
        self.periph.set_icr(Icr(0).set_ttocint(self.index, 1));
        self
    }
    #[inline]
    fn match_flag(&self) -> bool {
        match self.index {
            0 => self.periph.ris().tamris() != 0,
            1 => self.periph.ris().tbmris() != 0,
            _ => unimplemented!(),
        }
    }
    #[inline]
    fn clr_match_flag(&self) -> &Self {
        match self.index {
            0 => self.periph.set_icr(Icr(0).set_tamcint(1)),
            1 => self.periph.set_icr(Icr(0).set_tbmcint(1)),
            _ => unimplemented!(),
        };
        self
    }   
    fn dma_done_flag(&self) -> bool {
        self.periph.ris().dmaris(self.index) != 0
    }
    fn clr_dma_done_flag(&self) -> &Self {
        self.periph.set_icr(Icr(0).set_dmaint(self.index, 1));
        self
    }       
}