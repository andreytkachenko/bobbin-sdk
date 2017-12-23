#[allow(unused_imports)] use bobbin_common::*;

periph!( SDMMC, Sdmmc, _SDMMC, SdioPeriph, 0x40012800);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SDIO Peripheral"]
pub struct SdioPeriph(pub usize); 



impl SdioPeriph {
    #[doc="Get the *mut pointer for the POWER register."]
    #[inline] pub fn power_mut(&self) -> *mut Power { 
        (self.0 + 0x0) as *mut Power
    }

    #[doc="Get the *const pointer for the POWER register."]
    #[inline] pub fn power_ptr(&self) -> *const Power { 
           self.power_mut()
    }

    #[doc="Read the POWER register."]
    #[inline] pub fn power(&self) -> Power { 
        unsafe {
            read_volatile(self.power_ptr())
        }
    }

    #[doc="Write the POWER register."]
    #[inline] pub fn set_power<F: FnOnce(Power) -> Power>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.power_mut(), f(Power(0)));
        }
        self
    }

    #[doc="Modify the POWER register."]
    #[inline] pub fn with_power<F: FnOnce(Power) -> Power>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.power_mut(), f(self.power()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLKCR register."]
    #[inline] pub fn clkcr_mut(&self) -> *mut Clkcr { 
        (self.0 + 0x4) as *mut Clkcr
    }

    #[doc="Get the *const pointer for the CLKCR register."]
    #[inline] pub fn clkcr_ptr(&self) -> *const Clkcr { 
           self.clkcr_mut()
    }

    #[doc="Read the CLKCR register."]
    #[inline] pub fn clkcr(&self) -> Clkcr { 
        unsafe {
            read_volatile(self.clkcr_ptr())
        }
    }

    #[doc="Write the CLKCR register."]
    #[inline] pub fn set_clkcr<F: FnOnce(Clkcr) -> Clkcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clkcr_mut(), f(Clkcr(0)));
        }
        self
    }

    #[doc="Modify the CLKCR register."]
    #[inline] pub fn with_clkcr<F: FnOnce(Clkcr) -> Clkcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clkcr_mut(), f(self.clkcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ARG register."]
    #[inline] pub fn arg_mut(&self) -> *mut Arg { 
        (self.0 + 0x8) as *mut Arg
    }

    #[doc="Get the *const pointer for the ARG register."]
    #[inline] pub fn arg_ptr(&self) -> *const Arg { 
           self.arg_mut()
    }

    #[doc="Read the ARG register."]
    #[inline] pub fn arg(&self) -> Arg { 
        unsafe {
            read_volatile(self.arg_ptr())
        }
    }

    #[doc="Write the ARG register."]
    #[inline] pub fn set_arg<F: FnOnce(Arg) -> Arg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.arg_mut(), f(Arg(0)));
        }
        self
    }

    #[doc="Modify the ARG register."]
    #[inline] pub fn with_arg<F: FnOnce(Arg) -> Arg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.arg_mut(), f(self.arg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CMD register."]
    #[inline] pub fn cmd_mut(&self) -> *mut Cmd { 
        (self.0 + 0xc) as *mut Cmd
    }

    #[doc="Get the *const pointer for the CMD register."]
    #[inline] pub fn cmd_ptr(&self) -> *const Cmd { 
           self.cmd_mut()
    }

    #[doc="Read the CMD register."]
    #[inline] pub fn cmd(&self) -> Cmd { 
        unsafe {
            read_volatile(self.cmd_ptr())
        }
    }

    #[doc="Write the CMD register."]
    #[inline] pub fn set_cmd<F: FnOnce(Cmd) -> Cmd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmd_mut(), f(Cmd(0)));
        }
        self
    }

    #[doc="Modify the CMD register."]
    #[inline] pub fn with_cmd<F: FnOnce(Cmd) -> Cmd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmd_mut(), f(self.cmd()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RESPCMD register."]
    #[inline] pub fn respcmd_mut(&self) -> *mut Respcmd { 
        (self.0 + 0x10) as *mut Respcmd
    }

    #[doc="Get the *const pointer for the RESPCMD register."]
    #[inline] pub fn respcmd_ptr(&self) -> *const Respcmd { 
           self.respcmd_mut()
    }

    #[doc="Read the RESPCMD register."]
    #[inline] pub fn respcmd(&self) -> Respcmd { 
        unsafe {
            read_volatile(self.respcmd_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RESP1 register."]
    #[inline] pub fn resp1_mut(&self) -> *mut Resp1 { 
        (self.0 + 0x14) as *mut Resp1
    }

    #[doc="Get the *const pointer for the RESP1 register."]
    #[inline] pub fn resp1_ptr(&self) -> *const Resp1 { 
           self.resp1_mut()
    }

    #[doc="Read the RESP1 register."]
    #[inline] pub fn resp1(&self) -> Resp1 { 
        unsafe {
            read_volatile(self.resp1_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RESP2 register."]
    #[inline] pub fn resp2_mut(&self) -> *mut Resp2 { 
        (self.0 + 0x18) as *mut Resp2
    }

    #[doc="Get the *const pointer for the RESP2 register."]
    #[inline] pub fn resp2_ptr(&self) -> *const Resp2 { 
           self.resp2_mut()
    }

    #[doc="Read the RESP2 register."]
    #[inline] pub fn resp2(&self) -> Resp2 { 
        unsafe {
            read_volatile(self.resp2_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RESP3 register."]
    #[inline] pub fn resp3_mut(&self) -> *mut Resp3 { 
        (self.0 + 0x1c) as *mut Resp3
    }

    #[doc="Get the *const pointer for the RESP3 register."]
    #[inline] pub fn resp3_ptr(&self) -> *const Resp3 { 
           self.resp3_mut()
    }

    #[doc="Read the RESP3 register."]
    #[inline] pub fn resp3(&self) -> Resp3 { 
        unsafe {
            read_volatile(self.resp3_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RESP4 register."]
    #[inline] pub fn resp4_mut(&self) -> *mut Resp4 { 
        (self.0 + 0x20) as *mut Resp4
    }

    #[doc="Get the *const pointer for the RESP4 register."]
    #[inline] pub fn resp4_ptr(&self) -> *const Resp4 { 
           self.resp4_mut()
    }

    #[doc="Read the RESP4 register."]
    #[inline] pub fn resp4(&self) -> Resp4 { 
        unsafe {
            read_volatile(self.resp4_ptr())
        }
    }

    #[doc="Get the *mut pointer for the DTIMER register."]
    #[inline] pub fn dtimer_mut(&self) -> *mut Dtimer { 
        (self.0 + 0x24) as *mut Dtimer
    }

    #[doc="Get the *const pointer for the DTIMER register."]
    #[inline] pub fn dtimer_ptr(&self) -> *const Dtimer { 
           self.dtimer_mut()
    }

    #[doc="Read the DTIMER register."]
    #[inline] pub fn dtimer(&self) -> Dtimer { 
        unsafe {
            read_volatile(self.dtimer_ptr())
        }
    }

    #[doc="Write the DTIMER register."]
    #[inline] pub fn set_dtimer<F: FnOnce(Dtimer) -> Dtimer>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dtimer_mut(), f(Dtimer(0)));
        }
        self
    }

    #[doc="Modify the DTIMER register."]
    #[inline] pub fn with_dtimer<F: FnOnce(Dtimer) -> Dtimer>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dtimer_mut(), f(self.dtimer()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DLEN register."]
    #[inline] pub fn dlen_mut(&self) -> *mut Dlen { 
        (self.0 + 0x28) as *mut Dlen
    }

    #[doc="Get the *const pointer for the DLEN register."]
    #[inline] pub fn dlen_ptr(&self) -> *const Dlen { 
           self.dlen_mut()
    }

    #[doc="Read the DLEN register."]
    #[inline] pub fn dlen(&self) -> Dlen { 
        unsafe {
            read_volatile(self.dlen_ptr())
        }
    }

    #[doc="Write the DLEN register."]
    #[inline] pub fn set_dlen<F: FnOnce(Dlen) -> Dlen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dlen_mut(), f(Dlen(0)));
        }
        self
    }

    #[doc="Modify the DLEN register."]
    #[inline] pub fn with_dlen<F: FnOnce(Dlen) -> Dlen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dlen_mut(), f(self.dlen()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DCTRL register."]
    #[inline] pub fn dctrl_mut(&self) -> *mut Dctrl { 
        (self.0 + 0x2c) as *mut Dctrl
    }

    #[doc="Get the *const pointer for the DCTRL register."]
    #[inline] pub fn dctrl_ptr(&self) -> *const Dctrl { 
           self.dctrl_mut()
    }

    #[doc="Read the DCTRL register."]
    #[inline] pub fn dctrl(&self) -> Dctrl { 
        unsafe {
            read_volatile(self.dctrl_ptr())
        }
    }

    #[doc="Write the DCTRL register."]
    #[inline] pub fn set_dctrl<F: FnOnce(Dctrl) -> Dctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dctrl_mut(), f(Dctrl(0)));
        }
        self
    }

    #[doc="Modify the DCTRL register."]
    #[inline] pub fn with_dctrl<F: FnOnce(Dctrl) -> Dctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dctrl_mut(), f(self.dctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DCOUNT register."]
    #[inline] pub fn dcount_mut(&self) -> *mut Dcount { 
        (self.0 + 0x30) as *mut Dcount
    }

    #[doc="Get the *const pointer for the DCOUNT register."]
    #[inline] pub fn dcount_ptr(&self) -> *const Dcount { 
           self.dcount_mut()
    }

    #[doc="Read the DCOUNT register."]
    #[inline] pub fn dcount(&self) -> Dcount { 
        unsafe {
            read_volatile(self.dcount_ptr())
        }
    }

    #[doc="Get the *mut pointer for the STA register."]
    #[inline] pub fn sta_mut(&self) -> *mut Sta { 
        (self.0 + 0x34) as *mut Sta
    }

    #[doc="Get the *const pointer for the STA register."]
    #[inline] pub fn sta_ptr(&self) -> *const Sta { 
           self.sta_mut()
    }

    #[doc="Read the STA register."]
    #[inline] pub fn sta(&self) -> Sta { 
        unsafe {
            read_volatile(self.sta_ptr())
        }
    }

    #[doc="Get the *mut pointer for the ICR register."]
    #[inline] pub fn icr_mut(&self) -> *mut Icr { 
        (self.0 + 0x38) as *mut Icr
    }

    #[doc="Get the *const pointer for the ICR register."]
    #[inline] pub fn icr_ptr(&self) -> *const Icr { 
           self.icr_mut()
    }

    #[doc="Read the ICR register."]
    #[inline] pub fn icr(&self) -> Icr { 
        unsafe {
            read_volatile(self.icr_ptr())
        }
    }

    #[doc="Write the ICR register."]
    #[inline] pub fn set_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icr_mut(), f(Icr(0)));
        }
        self
    }

    #[doc="Modify the ICR register."]
    #[inline] pub fn with_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icr_mut(), f(self.icr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MASK register."]
    #[inline] pub fn mask_mut(&self) -> *mut Mask { 
        (self.0 + 0x3c) as *mut Mask
    }

    #[doc="Get the *const pointer for the MASK register."]
    #[inline] pub fn mask_ptr(&self) -> *const Mask { 
           self.mask_mut()
    }

    #[doc="Read the MASK register."]
    #[inline] pub fn mask(&self) -> Mask { 
        unsafe {
            read_volatile(self.mask_ptr())
        }
    }

    #[doc="Write the MASK register."]
    #[inline] pub fn set_mask<F: FnOnce(Mask) -> Mask>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mask_mut(), f(Mask(0)));
        }
        self
    }

    #[doc="Modify the MASK register."]
    #[inline] pub fn with_mask<F: FnOnce(Mask) -> Mask>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mask_mut(), f(self.mask()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FIFOCNT register."]
    #[inline] pub fn fifocnt_mut(&self) -> *mut Fifocnt { 
        (self.0 + 0x48) as *mut Fifocnt
    }

    #[doc="Get the *const pointer for the FIFOCNT register."]
    #[inline] pub fn fifocnt_ptr(&self) -> *const Fifocnt { 
           self.fifocnt_mut()
    }

    #[doc="Read the FIFOCNT register."]
    #[inline] pub fn fifocnt(&self) -> Fifocnt { 
        unsafe {
            read_volatile(self.fifocnt_ptr())
        }
    }

    #[doc="Get the *mut pointer for the FIFO register."]
    #[inline] pub fn fifo_mut(&self) -> *mut Fifo { 
        (self.0 + 0x80) as *mut Fifo
    }

    #[doc="Get the *const pointer for the FIFO register."]
    #[inline] pub fn fifo_ptr(&self) -> *const Fifo { 
           self.fifo_mut()
    }

    #[doc="Read the FIFO register."]
    #[inline] pub fn fifo(&self) -> Fifo { 
        unsafe {
            read_volatile(self.fifo_ptr())
        }
    }

    #[doc="Write the FIFO register."]
    #[inline] pub fn set_fifo<F: FnOnce(Fifo) -> Fifo>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifo_mut(), f(Fifo(0)));
        }
        self
    }

    #[doc="Modify the FIFO register."]
    #[inline] pub fn with_fifo<F: FnOnce(Fifo) -> Fifo>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifo_mut(), f(self.fifo()));
        }
        self
    }

}

#[doc="power control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Power(pub u32);
impl Power {
    #[doc="PWRCTRL"]
    #[inline] pub fn pwrctrl(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if PWRCTRL != 0"]
    #[inline] pub fn test_pwrctrl(&self) -> bool {
        self.pwrctrl() != 0
    }

    #[doc="Sets the PWRCTRL field."]
    #[inline] pub fn set_pwrctrl<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Power {
    #[inline]
    fn from(other: u32) -> Self {
         Power(other)
    }
}

impl ::core::fmt::Display for Power {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Power {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pwrctrl() != 0 { try!(write!(f, " pwrctrl=0x{:x}", self.pwrctrl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SDI clock control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clkcr(pub u32);
impl Clkcr {
    #[doc="HW Flow Control enable"]
    #[inline] pub fn hwfc_en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if HWFC_EN != 0"]
    #[inline] pub fn test_hwfc_en(&self) -> bool {
        self.hwfc_en() != 0
    }

    #[doc="Sets the HWFC_EN field."]
    #[inline] pub fn set_hwfc_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="SDIO_CK dephasing selection bit"]
    #[inline] pub fn negedge(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if NEGEDGE != 0"]
    #[inline] pub fn test_negedge(&self) -> bool {
        self.negedge() != 0
    }

    #[doc="Sets the NEGEDGE field."]
    #[inline] pub fn set_negedge<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Wide bus mode enable bit"]
    #[inline] pub fn widbus(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x3) as u8) } // [12:11]
    }

    #[doc="Returns true if WIDBUS != 0"]
    #[inline] pub fn test_widbus(&self) -> bool {
        self.widbus() != 0
    }

    #[doc="Sets the WIDBUS field."]
    #[inline] pub fn set_widbus<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Clock divider bypass enable bit"]
    #[inline] pub fn bypass(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if BYPASS != 0"]
    #[inline] pub fn test_bypass(&self) -> bool {
        self.bypass() != 0
    }

    #[doc="Sets the BYPASS field."]
    #[inline] pub fn set_bypass<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Power saving configuration bit"]
    #[inline] pub fn pwrsav(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PWRSAV != 0"]
    #[inline] pub fn test_pwrsav(&self) -> bool {
        self.pwrsav() != 0
    }

    #[doc="Sets the PWRSAV field."]
    #[inline] pub fn set_pwrsav<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Clock enable bit"]
    #[inline] pub fn clken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CLKEN != 0"]
    #[inline] pub fn test_clken(&self) -> bool {
        self.clken() != 0
    }

    #[doc="Sets the CLKEN field."]
    #[inline] pub fn set_clken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clock divide factor"]
    #[inline] pub fn clkdiv(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CLKDIV != 0"]
    #[inline] pub fn test_clkdiv(&self) -> bool {
        self.clkdiv() != 0
    }

    #[doc="Sets the CLKDIV field."]
    #[inline] pub fn set_clkdiv<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Clkcr {
    #[inline]
    fn from(other: u32) -> Self {
         Clkcr(other)
    }
}

impl ::core::fmt::Display for Clkcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clkcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hwfc_en() != 0 { try!(write!(f, " hwfc_en"))}
        if self.negedge() != 0 { try!(write!(f, " negedge"))}
        if self.widbus() != 0 { try!(write!(f, " widbus=0x{:x}", self.widbus()))}
        if self.bypass() != 0 { try!(write!(f, " bypass"))}
        if self.pwrsav() != 0 { try!(write!(f, " pwrsav"))}
        if self.clken() != 0 { try!(write!(f, " clken"))}
        if self.clkdiv() != 0 { try!(write!(f, " clkdiv=0x{:x}", self.clkdiv()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="argument register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Arg(pub u32);
impl Arg {
    #[doc="Command argument"]
    #[inline] pub fn cmdarg(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CMDARG != 0"]
    #[inline] pub fn test_cmdarg(&self) -> bool {
        self.cmdarg() != 0
    }

    #[doc="Sets the CMDARG field."]
    #[inline] pub fn set_cmdarg<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Arg {
    #[inline]
    fn from(other: u32) -> Self {
         Arg(other)
    }
}

impl ::core::fmt::Display for Arg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Arg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="command register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc="CE-ATA command"]
    #[inline] pub fn ce_atacmd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CE_ATACMD != 0"]
    #[inline] pub fn test_ce_atacmd(&self) -> bool {
        self.ce_atacmd() != 0
    }

    #[doc="Sets the CE_ATACMD field."]
    #[inline] pub fn set_ce_atacmd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="not Interrupt Enable"]
    #[inline] pub fn nien(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if NIEN != 0"]
    #[inline] pub fn test_nien(&self) -> bool {
        self.nien() != 0
    }

    #[doc="Sets the NIEN field."]
    #[inline] pub fn set_nien<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Enable CMD completion"]
    #[inline] pub fn encmdcompl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if ENCMDCOMPL != 0"]
    #[inline] pub fn test_encmdcompl(&self) -> bool {
        self.encmdcompl() != 0
    }

    #[doc="Sets the ENCMDCOMPL field."]
    #[inline] pub fn set_encmdcompl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SD I/O suspend command"]
    #[inline] pub fn sdiosuspend(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SDIOSUSPEND != 0"]
    #[inline] pub fn test_sdiosuspend(&self) -> bool {
        self.sdiosuspend() != 0
    }

    #[doc="Sets the SDIOSUSPEND field."]
    #[inline] pub fn set_sdiosuspend<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Command path state machine (CPSM) Enable bit"]
    #[inline] pub fn cpsmen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CPSMEN != 0"]
    #[inline] pub fn test_cpsmen(&self) -> bool {
        self.cpsmen() != 0
    }

    #[doc="Sets the CPSMEN field."]
    #[inline] pub fn set_cpsmen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="CPSM Waits for ends of data transfer (CmdPend internal signal)"]
    #[inline] pub fn waitpend(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if WAITPEND != 0"]
    #[inline] pub fn test_waitpend(&self) -> bool {
        self.waitpend() != 0
    }

    #[doc="Sets the WAITPEND field."]
    #[inline] pub fn set_waitpend<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="CPSM waits for interrupt request"]
    #[inline] pub fn waitint(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if WAITINT != 0"]
    #[inline] pub fn test_waitint(&self) -> bool {
        self.waitint() != 0
    }

    #[doc="Sets the WAITINT field."]
    #[inline] pub fn set_waitint<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Wait for response bits"]
    #[inline] pub fn waitresp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if WAITRESP != 0"]
    #[inline] pub fn test_waitresp(&self) -> bool {
        self.waitresp() != 0
    }

    #[doc="Sets the WAITRESP field."]
    #[inline] pub fn set_waitresp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Command index"]
    #[inline] pub fn cmdindex(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if CMDINDEX != 0"]
    #[inline] pub fn test_cmdindex(&self) -> bool {
        self.cmdindex() != 0
    }

    #[doc="Sets the CMDINDEX field."]
    #[inline] pub fn set_cmdindex<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cmd {
    #[inline]
    fn from(other: u32) -> Self {
         Cmd(other)
    }
}

impl ::core::fmt::Display for Cmd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ce_atacmd() != 0 { try!(write!(f, " ce_atacmd"))}
        if self.nien() != 0 { try!(write!(f, " nien"))}
        if self.encmdcompl() != 0 { try!(write!(f, " encmdcompl"))}
        if self.sdiosuspend() != 0 { try!(write!(f, " sdiosuspend"))}
        if self.cpsmen() != 0 { try!(write!(f, " cpsmen"))}
        if self.waitpend() != 0 { try!(write!(f, " waitpend"))}
        if self.waitint() != 0 { try!(write!(f, " waitint"))}
        if self.waitresp() != 0 { try!(write!(f, " waitresp=0x{:x}", self.waitresp()))}
        if self.cmdindex() != 0 { try!(write!(f, " cmdindex=0x{:x}", self.cmdindex()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="command response register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Respcmd(pub u32);
impl Respcmd {
    #[doc="Response command index"]
    #[inline] pub fn respcmd(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if RESPCMD != 0"]
    #[inline] pub fn test_respcmd(&self) -> bool {
        self.respcmd() != 0
    }

    #[doc="Sets the RESPCMD field."]
    #[inline] pub fn set_respcmd<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Respcmd {
    #[inline]
    fn from(other: u32) -> Self {
         Respcmd(other)
    }
}

impl ::core::fmt::Display for Respcmd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Respcmd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.respcmd() != 0 { try!(write!(f, " respcmd=0x{:x}", self.respcmd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="response 1..4 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Resp1(pub u32);
impl Resp1 {
    #[doc="see Table 132"]
    #[inline] pub fn cardstatus1(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CARDSTATUS1 != 0"]
    #[inline] pub fn test_cardstatus1(&self) -> bool {
        self.cardstatus1() != 0
    }

    #[doc="Sets the CARDSTATUS1 field."]
    #[inline] pub fn set_cardstatus1<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Resp1 {
    #[inline]
    fn from(other: u32) -> Self {
         Resp1(other)
    }
}

impl ::core::fmt::Display for Resp1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Resp1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="response 1..4 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Resp2(pub u32);
impl Resp2 {
    #[doc="see Table 132"]
    #[inline] pub fn cardstatus2(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CARDSTATUS2 != 0"]
    #[inline] pub fn test_cardstatus2(&self) -> bool {
        self.cardstatus2() != 0
    }

    #[doc="Sets the CARDSTATUS2 field."]
    #[inline] pub fn set_cardstatus2<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Resp2 {
    #[inline]
    fn from(other: u32) -> Self {
         Resp2(other)
    }
}

impl ::core::fmt::Display for Resp2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Resp2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="response 1..4 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Resp3(pub u32);
impl Resp3 {
    #[doc="see Table 132"]
    #[inline] pub fn cardstatus3(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CARDSTATUS3 != 0"]
    #[inline] pub fn test_cardstatus3(&self) -> bool {
        self.cardstatus3() != 0
    }

    #[doc="Sets the CARDSTATUS3 field."]
    #[inline] pub fn set_cardstatus3<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Resp3 {
    #[inline]
    fn from(other: u32) -> Self {
         Resp3(other)
    }
}

impl ::core::fmt::Display for Resp3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Resp3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="response 1..4 register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Resp4(pub u32);
impl Resp4 {
    #[doc="see Table 132"]
    #[inline] pub fn cardstatus4(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CARDSTATUS4 != 0"]
    #[inline] pub fn test_cardstatus4(&self) -> bool {
        self.cardstatus4() != 0
    }

    #[doc="Sets the CARDSTATUS4 field."]
    #[inline] pub fn set_cardstatus4<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Resp4 {
    #[inline]
    fn from(other: u32) -> Self {
         Resp4(other)
    }
}

impl ::core::fmt::Display for Resp4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Resp4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data timer register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dtimer(pub u32);
impl Dtimer {
    #[doc="Data timeout period"]
    #[inline] pub fn datatime(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATATIME != 0"]
    #[inline] pub fn test_datatime(&self) -> bool {
        self.datatime() != 0
    }

    #[doc="Sets the DATATIME field."]
    #[inline] pub fn set_datatime<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dtimer {
    #[inline]
    fn from(other: u32) -> Self {
         Dtimer(other)
    }
}

impl ::core::fmt::Display for Dtimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dtimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data length register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dlen(pub u32);
impl Dlen {
    #[doc="Data length value"]
    #[inline] pub fn datalength(&self) -> bits::U25 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ffffff) as u32) } // [24:0]
    }

    #[doc="Returns true if DATALENGTH != 0"]
    #[inline] pub fn test_datalength(&self) -> bool {
        self.datalength() != 0
    }

    #[doc="Sets the DATALENGTH field."]
    #[inline] pub fn set_datalength<V: Into<bits::U25>>(mut self, value: V) -> Self {
        let value: bits::U25 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dlen {
    #[inline]
    fn from(other: u32) -> Self {
         Dlen(other)
    }
}

impl ::core::fmt::Display for Dlen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dlen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datalength() != 0 { try!(write!(f, " datalength=0x{:x}", self.datalength()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dctrl(pub u32);
impl Dctrl {
    #[doc="SD I/O enable functions"]
    #[inline] pub fn sdioen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SDIOEN != 0"]
    #[inline] pub fn test_sdioen(&self) -> bool {
        self.sdioen() != 0
    }

    #[doc="Sets the SDIOEN field."]
    #[inline] pub fn set_sdioen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Read wait mode"]
    #[inline] pub fn rwmod(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if RWMOD != 0"]
    #[inline] pub fn test_rwmod(&self) -> bool {
        self.rwmod() != 0
    }

    #[doc="Sets the RWMOD field."]
    #[inline] pub fn set_rwmod<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Read wait stop"]
    #[inline] pub fn rwstop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RWSTOP != 0"]
    #[inline] pub fn test_rwstop(&self) -> bool {
        self.rwstop() != 0
    }

    #[doc="Sets the RWSTOP field."]
    #[inline] pub fn set_rwstop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Read wait start"]
    #[inline] pub fn rwstart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RWSTART != 0"]
    #[inline] pub fn test_rwstart(&self) -> bool {
        self.rwstart() != 0
    }

    #[doc="Sets the RWSTART field."]
    #[inline] pub fn set_rwstart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Data block size"]
    #[inline] pub fn dblocksize(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if DBLOCKSIZE != 0"]
    #[inline] pub fn test_dblocksize(&self) -> bool {
        self.dblocksize() != 0
    }

    #[doc="Sets the DBLOCKSIZE field."]
    #[inline] pub fn set_dblocksize<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="DMA enable bit"]
    #[inline] pub fn dmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DMAEN != 0"]
    #[inline] pub fn test_dmaen(&self) -> bool {
        self.dmaen() != 0
    }

    #[doc="Sets the DMAEN field."]
    #[inline] pub fn set_dmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Data transfer mode selection 1: Stream or SDIO multibyte data transfer"]
    #[inline] pub fn dtmode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DTMODE != 0"]
    #[inline] pub fn test_dtmode(&self) -> bool {
        self.dtmode() != 0
    }

    #[doc="Sets the DTMODE field."]
    #[inline] pub fn set_dtmode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Data transfer direction selection"]
    #[inline] pub fn dtdir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DTDIR != 0"]
    #[inline] pub fn test_dtdir(&self) -> bool {
        self.dtdir() != 0
    }

    #[doc="Sets the DTDIR field."]
    #[inline] pub fn set_dtdir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DTEN"]
    #[inline] pub fn dten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DTEN != 0"]
    #[inline] pub fn test_dten(&self) -> bool {
        self.dten() != 0
    }

    #[doc="Sets the DTEN field."]
    #[inline] pub fn set_dten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Dctrl(other)
    }
}

impl ::core::fmt::Display for Dctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sdioen() != 0 { try!(write!(f, " sdioen"))}
        if self.rwmod() != 0 { try!(write!(f, " rwmod"))}
        if self.rwstop() != 0 { try!(write!(f, " rwstop"))}
        if self.rwstart() != 0 { try!(write!(f, " rwstart"))}
        if self.dblocksize() != 0 { try!(write!(f, " dblocksize=0x{:x}", self.dblocksize()))}
        if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
        if self.dtmode() != 0 { try!(write!(f, " dtmode"))}
        if self.dtdir() != 0 { try!(write!(f, " dtdir"))}
        if self.dten() != 0 { try!(write!(f, " dten"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data counter register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcount(pub u32);
impl Dcount {
    #[doc="Data count value"]
    #[inline] pub fn datacount(&self) -> bits::U25 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ffffff) as u32) } // [24:0]
    }

    #[doc="Returns true if DATACOUNT != 0"]
    #[inline] pub fn test_datacount(&self) -> bool {
        self.datacount() != 0
    }

    #[doc="Sets the DATACOUNT field."]
    #[inline] pub fn set_datacount<V: Into<bits::U25>>(mut self, value: V) -> Self {
        let value: bits::U25 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dcount {
    #[inline]
    fn from(other: u32) -> Self {
         Dcount(other)
    }
}

impl ::core::fmt::Display for Dcount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datacount() != 0 { try!(write!(f, " datacount=0x{:x}", self.datacount()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sta(pub u32);
impl Sta {
    #[doc="CE-ATA command completion signal received for CMD61"]
    #[inline] pub fn ceataend(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if CEATAEND != 0"]
    #[inline] pub fn test_ceataend(&self) -> bool {
        self.ceataend() != 0
    }

    #[doc="Sets the CEATAEND field."]
    #[inline] pub fn set_ceataend<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="SDIO interrupt received"]
    #[inline] pub fn sdioit(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if SDIOIT != 0"]
    #[inline] pub fn test_sdioit(&self) -> bool {
        self.sdioit() != 0
    }

    #[doc="Sets the SDIOIT field."]
    #[inline] pub fn set_sdioit<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Data available in receive FIFO"]
    #[inline] pub fn rxdavl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if RXDAVL != 0"]
    #[inline] pub fn test_rxdavl(&self) -> bool {
        self.rxdavl() != 0
    }

    #[doc="Sets the RXDAVL field."]
    #[inline] pub fn set_rxdavl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Data available in transmit FIFO"]
    #[inline] pub fn txdavl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if TXDAVL != 0"]
    #[inline] pub fn test_txdavl(&self) -> bool {
        self.txdavl() != 0
    }

    #[doc="Sets the TXDAVL field."]
    #[inline] pub fn set_txdavl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Receive FIFO empty"]
    #[inline] pub fn rxfifoe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if RXFIFOE != 0"]
    #[inline] pub fn test_rxfifoe(&self) -> bool {
        self.rxfifoe() != 0
    }

    #[doc="Sets the RXFIFOE field."]
    #[inline] pub fn set_rxfifoe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Transmit FIFO empty"]
    #[inline] pub fn txfifoe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TXFIFOE != 0"]
    #[inline] pub fn test_txfifoe(&self) -> bool {
        self.txfifoe() != 0
    }

    #[doc="Sets the TXFIFOE field."]
    #[inline] pub fn set_txfifoe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Receive FIFO full"]
    #[inline] pub fn rxfifof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if RXFIFOF != 0"]
    #[inline] pub fn test_rxfifof(&self) -> bool {
        self.rxfifof() != 0
    }

    #[doc="Sets the RXFIFOF field."]
    #[inline] pub fn set_rxfifof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Transmit FIFO full"]
    #[inline] pub fn txfifof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TXFIFOF != 0"]
    #[inline] pub fn test_txfifof(&self) -> bool {
        self.txfifof() != 0
    }

    #[doc="Sets the TXFIFOF field."]
    #[inline] pub fn set_txfifof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Receive FIFO half full: there are at least 8 words in the FIFO"]
    #[inline] pub fn rxfifohf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if RXFIFOHF != 0"]
    #[inline] pub fn test_rxfifohf(&self) -> bool {
        self.rxfifohf() != 0
    }

    #[doc="Sets the RXFIFOHF field."]
    #[inline] pub fn set_rxfifohf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Transmit FIFO half empty: at least 8 words can be written into the FIFO"]
    #[inline] pub fn txfifohe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TXFIFOHE != 0"]
    #[inline] pub fn test_txfifohe(&self) -> bool {
        self.txfifohe() != 0
    }

    #[doc="Sets the TXFIFOHE field."]
    #[inline] pub fn set_txfifohe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Data receive in progress"]
    #[inline] pub fn rxact(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if RXACT != 0"]
    #[inline] pub fn test_rxact(&self) -> bool {
        self.rxact() != 0
    }

    #[doc="Sets the RXACT field."]
    #[inline] pub fn set_rxact<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Data transmit in progress"]
    #[inline] pub fn txact(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TXACT != 0"]
    #[inline] pub fn test_txact(&self) -> bool {
        self.txact() != 0
    }

    #[doc="Sets the TXACT field."]
    #[inline] pub fn set_txact<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Command transfer in progress"]
    #[inline] pub fn cmdact(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CMDACT != 0"]
    #[inline] pub fn test_cmdact(&self) -> bool {
        self.cmdact() != 0
    }

    #[doc="Sets the CMDACT field."]
    #[inline] pub fn set_cmdact<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Data block sent/received (CRC check passed)"]
    #[inline] pub fn dbckend(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DBCKEND != 0"]
    #[inline] pub fn test_dbckend(&self) -> bool {
        self.dbckend() != 0
    }

    #[doc="Sets the DBCKEND field."]
    #[inline] pub fn set_dbckend<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Start bit not detected on all data signals in wide bus mode"]
    #[inline] pub fn stbiterr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if STBITERR != 0"]
    #[inline] pub fn test_stbiterr(&self) -> bool {
        self.stbiterr() != 0
    }

    #[doc="Sets the STBITERR field."]
    #[inline] pub fn set_stbiterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data end (data counter, SDIDCOUNT, is zero)"]
    #[inline] pub fn dataend(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DATAEND != 0"]
    #[inline] pub fn test_dataend(&self) -> bool {
        self.dataend() != 0
    }

    #[doc="Sets the DATAEND field."]
    #[inline] pub fn set_dataend<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Command sent (no response required)"]
    #[inline] pub fn cmdsent(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CMDSENT != 0"]
    #[inline] pub fn test_cmdsent(&self) -> bool {
        self.cmdsent() != 0
    }

    #[doc="Sets the CMDSENT field."]
    #[inline] pub fn set_cmdsent<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Command response received (CRC check passed)"]
    #[inline] pub fn cmdrend(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CMDREND != 0"]
    #[inline] pub fn test_cmdrend(&self) -> bool {
        self.cmdrend() != 0
    }

    #[doc="Sets the CMDREND field."]
    #[inline] pub fn set_cmdrend<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Received FIFO overrun error"]
    #[inline] pub fn rxoverr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXOVERR != 0"]
    #[inline] pub fn test_rxoverr(&self) -> bool {
        self.rxoverr() != 0
    }

    #[doc="Sets the RXOVERR field."]
    #[inline] pub fn set_rxoverr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transmit FIFO underrun error"]
    #[inline] pub fn txunderr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TXUNDERR != 0"]
    #[inline] pub fn test_txunderr(&self) -> bool {
        self.txunderr() != 0
    }

    #[doc="Sets the TXUNDERR field."]
    #[inline] pub fn set_txunderr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data timeout"]
    #[inline] pub fn dtimeout(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DTIMEOUT != 0"]
    #[inline] pub fn test_dtimeout(&self) -> bool {
        self.dtimeout() != 0
    }

    #[doc="Sets the DTIMEOUT field."]
    #[inline] pub fn set_dtimeout<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Command response timeout"]
    #[inline] pub fn ctimeout(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CTIMEOUT != 0"]
    #[inline] pub fn test_ctimeout(&self) -> bool {
        self.ctimeout() != 0
    }

    #[doc="Sets the CTIMEOUT field."]
    #[inline] pub fn set_ctimeout<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Data block sent/received (CRC check failed)"]
    #[inline] pub fn dcrcfail(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DCRCFAIL != 0"]
    #[inline] pub fn test_dcrcfail(&self) -> bool {
        self.dcrcfail() != 0
    }

    #[doc="Sets the DCRCFAIL field."]
    #[inline] pub fn set_dcrcfail<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Command response received (CRC check failed)"]
    #[inline] pub fn ccrcfail(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CCRCFAIL != 0"]
    #[inline] pub fn test_ccrcfail(&self) -> bool {
        self.ccrcfail() != 0
    }

    #[doc="Sets the CCRCFAIL field."]
    #[inline] pub fn set_ccrcfail<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sta {
    #[inline]
    fn from(other: u32) -> Self {
         Sta(other)
    }
}

impl ::core::fmt::Display for Sta {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sta {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ceataend() != 0 { try!(write!(f, " ceataend"))}
        if self.sdioit() != 0 { try!(write!(f, " sdioit"))}
        if self.rxdavl() != 0 { try!(write!(f, " rxdavl"))}
        if self.txdavl() != 0 { try!(write!(f, " txdavl"))}
        if self.rxfifoe() != 0 { try!(write!(f, " rxfifoe"))}
        if self.txfifoe() != 0 { try!(write!(f, " txfifoe"))}
        if self.rxfifof() != 0 { try!(write!(f, " rxfifof"))}
        if self.txfifof() != 0 { try!(write!(f, " txfifof"))}
        if self.rxfifohf() != 0 { try!(write!(f, " rxfifohf"))}
        if self.txfifohe() != 0 { try!(write!(f, " txfifohe"))}
        if self.rxact() != 0 { try!(write!(f, " rxact"))}
        if self.txact() != 0 { try!(write!(f, " txact"))}
        if self.cmdact() != 0 { try!(write!(f, " cmdact"))}
        if self.dbckend() != 0 { try!(write!(f, " dbckend"))}
        if self.stbiterr() != 0 { try!(write!(f, " stbiterr"))}
        if self.dataend() != 0 { try!(write!(f, " dataend"))}
        if self.cmdsent() != 0 { try!(write!(f, " cmdsent"))}
        if self.cmdrend() != 0 { try!(write!(f, " cmdrend"))}
        if self.rxoverr() != 0 { try!(write!(f, " rxoverr"))}
        if self.txunderr() != 0 { try!(write!(f, " txunderr"))}
        if self.dtimeout() != 0 { try!(write!(f, " dtimeout"))}
        if self.ctimeout() != 0 { try!(write!(f, " ctimeout"))}
        if self.dcrcfail() != 0 { try!(write!(f, " dcrcfail"))}
        if self.ccrcfail() != 0 { try!(write!(f, " ccrcfail"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc="CEATAEND flag clear bit"]
    #[inline] pub fn ceataendc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if CEATAENDC != 0"]
    #[inline] pub fn test_ceataendc(&self) -> bool {
        self.ceataendc() != 0
    }

    #[doc="Sets the CEATAENDC field."]
    #[inline] pub fn set_ceataendc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="SDIOIT flag clear bit"]
    #[inline] pub fn sdioitc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if SDIOITC != 0"]
    #[inline] pub fn test_sdioitc(&self) -> bool {
        self.sdioitc() != 0
    }

    #[doc="Sets the SDIOITC field."]
    #[inline] pub fn set_sdioitc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="DBCKEND flag clear bit"]
    #[inline] pub fn dbckendc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DBCKENDC != 0"]
    #[inline] pub fn test_dbckendc(&self) -> bool {
        self.dbckendc() != 0
    }

    #[doc="Sets the DBCKENDC field."]
    #[inline] pub fn set_dbckendc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="STBITERR flag clear bit"]
    #[inline] pub fn stbiterrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if STBITERRC != 0"]
    #[inline] pub fn test_stbiterrc(&self) -> bool {
        self.stbiterrc() != 0
    }

    #[doc="Sets the STBITERRC field."]
    #[inline] pub fn set_stbiterrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="DATAEND flag clear bit"]
    #[inline] pub fn dataendc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DATAENDC != 0"]
    #[inline] pub fn test_dataendc(&self) -> bool {
        self.dataendc() != 0
    }

    #[doc="Sets the DATAENDC field."]
    #[inline] pub fn set_dataendc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="CMDSENT flag clear bit"]
    #[inline] pub fn cmdsentc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CMDSENTC != 0"]
    #[inline] pub fn test_cmdsentc(&self) -> bool {
        self.cmdsentc() != 0
    }

    #[doc="Sets the CMDSENTC field."]
    #[inline] pub fn set_cmdsentc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="CMDREND flag clear bit"]
    #[inline] pub fn cmdrendc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CMDRENDC != 0"]
    #[inline] pub fn test_cmdrendc(&self) -> bool {
        self.cmdrendc() != 0
    }

    #[doc="Sets the CMDRENDC field."]
    #[inline] pub fn set_cmdrendc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="RXOVERR flag clear bit"]
    #[inline] pub fn rxoverrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXOVERRC != 0"]
    #[inline] pub fn test_rxoverrc(&self) -> bool {
        self.rxoverrc() != 0
    }

    #[doc="Sets the RXOVERRC field."]
    #[inline] pub fn set_rxoverrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TXUNDERR flag clear bit"]
    #[inline] pub fn txunderrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TXUNDERRC != 0"]
    #[inline] pub fn test_txunderrc(&self) -> bool {
        self.txunderrc() != 0
    }

    #[doc="Sets the TXUNDERRC field."]
    #[inline] pub fn set_txunderrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="DTIMEOUT flag clear bit"]
    #[inline] pub fn dtimeoutc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DTIMEOUTC != 0"]
    #[inline] pub fn test_dtimeoutc(&self) -> bool {
        self.dtimeoutc() != 0
    }

    #[doc="Sets the DTIMEOUTC field."]
    #[inline] pub fn set_dtimeoutc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="CTIMEOUT flag clear bit"]
    #[inline] pub fn ctimeoutc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CTIMEOUTC != 0"]
    #[inline] pub fn test_ctimeoutc(&self) -> bool {
        self.ctimeoutc() != 0
    }

    #[doc="Sets the CTIMEOUTC field."]
    #[inline] pub fn set_ctimeoutc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DCRCFAIL flag clear bit"]
    #[inline] pub fn dcrcfailc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DCRCFAILC != 0"]
    #[inline] pub fn test_dcrcfailc(&self) -> bool {
        self.dcrcfailc() != 0
    }

    #[doc="Sets the DCRCFAILC field."]
    #[inline] pub fn set_dcrcfailc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="CCRCFAIL flag clear bit"]
    #[inline] pub fn ccrcfailc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CCRCFAILC != 0"]
    #[inline] pub fn test_ccrcfailc(&self) -> bool {
        self.ccrcfailc() != 0
    }

    #[doc="Sets the CCRCFAILC field."]
    #[inline] pub fn set_ccrcfailc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Icr {
    #[inline]
    fn from(other: u32) -> Self {
         Icr(other)
    }
}

impl ::core::fmt::Display for Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ceataendc() != 0 { try!(write!(f, " ceataendc"))}
        if self.sdioitc() != 0 { try!(write!(f, " sdioitc"))}
        if self.dbckendc() != 0 { try!(write!(f, " dbckendc"))}
        if self.stbiterrc() != 0 { try!(write!(f, " stbiterrc"))}
        if self.dataendc() != 0 { try!(write!(f, " dataendc"))}
        if self.cmdsentc() != 0 { try!(write!(f, " cmdsentc"))}
        if self.cmdrendc() != 0 { try!(write!(f, " cmdrendc"))}
        if self.rxoverrc() != 0 { try!(write!(f, " rxoverrc"))}
        if self.txunderrc() != 0 { try!(write!(f, " txunderrc"))}
        if self.dtimeoutc() != 0 { try!(write!(f, " dtimeoutc"))}
        if self.ctimeoutc() != 0 { try!(write!(f, " ctimeoutc"))}
        if self.dcrcfailc() != 0 { try!(write!(f, " dcrcfailc"))}
        if self.ccrcfailc() != 0 { try!(write!(f, " ccrcfailc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mask register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mask(pub u32);
impl Mask {
    #[doc="CE-ATA command completion signal received interrupt enable"]
    #[inline] pub fn ceataendie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if CEATAENDIE != 0"]
    #[inline] pub fn test_ceataendie(&self) -> bool {
        self.ceataendie() != 0
    }

    #[doc="Sets the CEATAENDIE field."]
    #[inline] pub fn set_ceataendie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="SDIO mode interrupt received interrupt enable"]
    #[inline] pub fn sdioitie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if SDIOITIE != 0"]
    #[inline] pub fn test_sdioitie(&self) -> bool {
        self.sdioitie() != 0
    }

    #[doc="Sets the SDIOITIE field."]
    #[inline] pub fn set_sdioitie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Data available in Rx FIFO interrupt enable"]
    #[inline] pub fn rxdavlie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if RXDAVLIE != 0"]
    #[inline] pub fn test_rxdavlie(&self) -> bool {
        self.rxdavlie() != 0
    }

    #[doc="Sets the RXDAVLIE field."]
    #[inline] pub fn set_rxdavlie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Data available in Tx FIFO interrupt enable"]
    #[inline] pub fn txdavlie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if TXDAVLIE != 0"]
    #[inline] pub fn test_txdavlie(&self) -> bool {
        self.txdavlie() != 0
    }

    #[doc="Sets the TXDAVLIE field."]
    #[inline] pub fn set_txdavlie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Rx FIFO empty interrupt enable"]
    #[inline] pub fn rxfifoeie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if RXFIFOEIE != 0"]
    #[inline] pub fn test_rxfifoeie(&self) -> bool {
        self.rxfifoeie() != 0
    }

    #[doc="Sets the RXFIFOEIE field."]
    #[inline] pub fn set_rxfifoeie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Tx FIFO empty interrupt enable"]
    #[inline] pub fn txfifoeie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TXFIFOEIE != 0"]
    #[inline] pub fn test_txfifoeie(&self) -> bool {
        self.txfifoeie() != 0
    }

    #[doc="Sets the TXFIFOEIE field."]
    #[inline] pub fn set_txfifoeie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Rx FIFO full interrupt enable"]
    #[inline] pub fn rxfifofie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if RXFIFOFIE != 0"]
    #[inline] pub fn test_rxfifofie(&self) -> bool {
        self.rxfifofie() != 0
    }

    #[doc="Sets the RXFIFOFIE field."]
    #[inline] pub fn set_rxfifofie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Tx FIFO full interrupt enable"]
    #[inline] pub fn txfifofie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TXFIFOFIE != 0"]
    #[inline] pub fn test_txfifofie(&self) -> bool {
        self.txfifofie() != 0
    }

    #[doc="Sets the TXFIFOFIE field."]
    #[inline] pub fn set_txfifofie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Rx FIFO half full interrupt enable"]
    #[inline] pub fn rxfifohfie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if RXFIFOHFIE != 0"]
    #[inline] pub fn test_rxfifohfie(&self) -> bool {
        self.rxfifohfie() != 0
    }

    #[doc="Sets the RXFIFOHFIE field."]
    #[inline] pub fn set_rxfifohfie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Tx FIFO half empty interrupt enable"]
    #[inline] pub fn txfifoheie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TXFIFOHEIE != 0"]
    #[inline] pub fn test_txfifoheie(&self) -> bool {
        self.txfifoheie() != 0
    }

    #[doc="Sets the TXFIFOHEIE field."]
    #[inline] pub fn set_txfifoheie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Data receive acting interrupt enable"]
    #[inline] pub fn rxactie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if RXACTIE != 0"]
    #[inline] pub fn test_rxactie(&self) -> bool {
        self.rxactie() != 0
    }

    #[doc="Sets the RXACTIE field."]
    #[inline] pub fn set_rxactie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Data transmit acting interrupt enable"]
    #[inline] pub fn txactie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TXACTIE != 0"]
    #[inline] pub fn test_txactie(&self) -> bool {
        self.txactie() != 0
    }

    #[doc="Sets the TXACTIE field."]
    #[inline] pub fn set_txactie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Command acting interrupt enable"]
    #[inline] pub fn cmdactie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CMDACTIE != 0"]
    #[inline] pub fn test_cmdactie(&self) -> bool {
        self.cmdactie() != 0
    }

    #[doc="Sets the CMDACTIE field."]
    #[inline] pub fn set_cmdactie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Data block end interrupt enable"]
    #[inline] pub fn dbckendie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DBCKENDIE != 0"]
    #[inline] pub fn test_dbckendie(&self) -> bool {
        self.dbckendie() != 0
    }

    #[doc="Sets the DBCKENDIE field."]
    #[inline] pub fn set_dbckendie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Start bit error interrupt enable"]
    #[inline] pub fn stbiterrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if STBITERRIE != 0"]
    #[inline] pub fn test_stbiterrie(&self) -> bool {
        self.stbiterrie() != 0
    }

    #[doc="Sets the STBITERRIE field."]
    #[inline] pub fn set_stbiterrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data end interrupt enable"]
    #[inline] pub fn dataendie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DATAENDIE != 0"]
    #[inline] pub fn test_dataendie(&self) -> bool {
        self.dataendie() != 0
    }

    #[doc="Sets the DATAENDIE field."]
    #[inline] pub fn set_dataendie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Command sent interrupt enable"]
    #[inline] pub fn cmdsentie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CMDSENTIE != 0"]
    #[inline] pub fn test_cmdsentie(&self) -> bool {
        self.cmdsentie() != 0
    }

    #[doc="Sets the CMDSENTIE field."]
    #[inline] pub fn set_cmdsentie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Command response received interrupt enable"]
    #[inline] pub fn cmdrendie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CMDRENDIE != 0"]
    #[inline] pub fn test_cmdrendie(&self) -> bool {
        self.cmdrendie() != 0
    }

    #[doc="Sets the CMDRENDIE field."]
    #[inline] pub fn set_cmdrendie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Rx FIFO overrun error interrupt enable"]
    #[inline] pub fn rxoverrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXOVERRIE != 0"]
    #[inline] pub fn test_rxoverrie(&self) -> bool {
        self.rxoverrie() != 0
    }

    #[doc="Sets the RXOVERRIE field."]
    #[inline] pub fn set_rxoverrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Tx FIFO underrun error interrupt enable"]
    #[inline] pub fn txunderrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TXUNDERRIE != 0"]
    #[inline] pub fn test_txunderrie(&self) -> bool {
        self.txunderrie() != 0
    }

    #[doc="Sets the TXUNDERRIE field."]
    #[inline] pub fn set_txunderrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data timeout interrupt enable"]
    #[inline] pub fn dtimeoutie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DTIMEOUTIE != 0"]
    #[inline] pub fn test_dtimeoutie(&self) -> bool {
        self.dtimeoutie() != 0
    }

    #[doc="Sets the DTIMEOUTIE field."]
    #[inline] pub fn set_dtimeoutie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Command timeout interrupt enable"]
    #[inline] pub fn ctimeoutie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CTIMEOUTIE != 0"]
    #[inline] pub fn test_ctimeoutie(&self) -> bool {
        self.ctimeoutie() != 0
    }

    #[doc="Sets the CTIMEOUTIE field."]
    #[inline] pub fn set_ctimeoutie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Data CRC fail interrupt enable"]
    #[inline] pub fn dcrcfailie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DCRCFAILIE != 0"]
    #[inline] pub fn test_dcrcfailie(&self) -> bool {
        self.dcrcfailie() != 0
    }

    #[doc="Sets the DCRCFAILIE field."]
    #[inline] pub fn set_dcrcfailie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Command CRC fail interrupt enable"]
    #[inline] pub fn ccrcfailie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CCRCFAILIE != 0"]
    #[inline] pub fn test_ccrcfailie(&self) -> bool {
        self.ccrcfailie() != 0
    }

    #[doc="Sets the CCRCFAILIE field."]
    #[inline] pub fn set_ccrcfailie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mask {
    #[inline]
    fn from(other: u32) -> Self {
         Mask(other)
    }
}

impl ::core::fmt::Display for Mask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ceataendie() != 0 { try!(write!(f, " ceataendie"))}
        if self.sdioitie() != 0 { try!(write!(f, " sdioitie"))}
        if self.rxdavlie() != 0 { try!(write!(f, " rxdavlie"))}
        if self.txdavlie() != 0 { try!(write!(f, " txdavlie"))}
        if self.rxfifoeie() != 0 { try!(write!(f, " rxfifoeie"))}
        if self.txfifoeie() != 0 { try!(write!(f, " txfifoeie"))}
        if self.rxfifofie() != 0 { try!(write!(f, " rxfifofie"))}
        if self.txfifofie() != 0 { try!(write!(f, " txfifofie"))}
        if self.rxfifohfie() != 0 { try!(write!(f, " rxfifohfie"))}
        if self.txfifoheie() != 0 { try!(write!(f, " txfifoheie"))}
        if self.rxactie() != 0 { try!(write!(f, " rxactie"))}
        if self.txactie() != 0 { try!(write!(f, " txactie"))}
        if self.cmdactie() != 0 { try!(write!(f, " cmdactie"))}
        if self.dbckendie() != 0 { try!(write!(f, " dbckendie"))}
        if self.stbiterrie() != 0 { try!(write!(f, " stbiterrie"))}
        if self.dataendie() != 0 { try!(write!(f, " dataendie"))}
        if self.cmdsentie() != 0 { try!(write!(f, " cmdsentie"))}
        if self.cmdrendie() != 0 { try!(write!(f, " cmdrendie"))}
        if self.rxoverrie() != 0 { try!(write!(f, " rxoverrie"))}
        if self.txunderrie() != 0 { try!(write!(f, " txunderrie"))}
        if self.dtimeoutie() != 0 { try!(write!(f, " dtimeoutie"))}
        if self.ctimeoutie() != 0 { try!(write!(f, " ctimeoutie"))}
        if self.dcrcfailie() != 0 { try!(write!(f, " dcrcfailie"))}
        if self.ccrcfailie() != 0 { try!(write!(f, " ccrcfailie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FIFO counter register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifocnt(pub u32);
impl Fifocnt {
    #[doc="Remaining number of words to be written to or read from the FIFO"]
    #[inline] pub fn fifocount(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if FIFOCOUNT != 0"]
    #[inline] pub fn test_fifocount(&self) -> bool {
        self.fifocount() != 0
    }

    #[doc="Sets the FIFOCOUNT field."]
    #[inline] pub fn set_fifocount<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fifocnt {
    #[inline]
    fn from(other: u32) -> Self {
         Fifocnt(other)
    }
}

impl ::core::fmt::Display for Fifocnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fifocnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fifocount() != 0 { try!(write!(f, " fifocount=0x{:x}", self.fifocount()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data FIFO register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifo(pub u32);
impl Fifo {
    #[doc="Receive and transmit FIFO data"]
    #[inline] pub fn fifodata(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if FIFODATA != 0"]
    #[inline] pub fn test_fifodata(&self) -> bool {
        self.fifodata() != 0
    }

    #[doc="Sets the FIFODATA field."]
    #[inline] pub fn set_fifodata<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fifo {
    #[inline]
    fn from(other: u32) -> Self {
         Fifo(other)
    }
}

impl ::core::fmt::Display for Fifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}


