#[allow(unused_imports)] use bobbin_common::*;


#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="LPUART Peripheral"]
pub struct LpuartPeriph(pub usize); 


impl LpuartPeriph {
    #[doc="Get the *mut pointer for the VERID register."]
    #[inline] pub fn verid_mut(&self) -> *mut Verid { 
        (self.0 + 0x0) as *mut Verid
    }

    #[doc="Get the *const pointer for the VERID register."]
    #[inline] pub fn verid_ptr(&self) -> *const Verid { 
           self.verid_mut()
    }

    #[doc="Read the VERID register."]
    #[inline] pub fn verid(&self) -> Verid { 
        unsafe {
            read_volatile(self.verid_ptr())
        }
    }

    #[doc="Get the *mut pointer for the PARAM register."]
    #[inline] pub fn param_mut(&self) -> *mut Param { 
        (self.0 + 0x4) as *mut Param
    }

    #[doc="Get the *const pointer for the PARAM register."]
    #[inline] pub fn param_ptr(&self) -> *const Param { 
           self.param_mut()
    }

    #[doc="Read the PARAM register."]
    #[inline] pub fn param(&self) -> Param { 
        unsafe {
            read_volatile(self.param_ptr())
        }
    }

    #[doc="Get the *mut pointer for the GLOBAL register."]
    #[inline] pub fn global_mut(&self) -> *mut Global { 
        (self.0 + 0x8) as *mut Global
    }

    #[doc="Get the *const pointer for the GLOBAL register."]
    #[inline] pub fn global_ptr(&self) -> *const Global { 
           self.global_mut()
    }

    #[doc="Read the GLOBAL register."]
    #[inline] pub fn global(&self) -> Global { 
        unsafe {
            read_volatile(self.global_ptr())
        }
    }

    #[doc="Write the GLOBAL register."]
    #[inline] pub fn set_global<F: FnOnce(Global) -> Global>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.global_mut(), f(Global(0)));
        }
        self
    }

    #[doc="Modify the GLOBAL register."]
    #[inline] pub fn with_global<F: FnOnce(Global) -> Global>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.global_mut(), f(self.global()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PINCFG register."]
    #[inline] pub fn pincfg_mut(&self) -> *mut Pincfg { 
        (self.0 + 0xc) as *mut Pincfg
    }

    #[doc="Get the *const pointer for the PINCFG register."]
    #[inline] pub fn pincfg_ptr(&self) -> *const Pincfg { 
           self.pincfg_mut()
    }

    #[doc="Read the PINCFG register."]
    #[inline] pub fn pincfg(&self) -> Pincfg { 
        unsafe {
            read_volatile(self.pincfg_ptr())
        }
    }

    #[doc="Write the PINCFG register."]
    #[inline] pub fn set_pincfg<F: FnOnce(Pincfg) -> Pincfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pincfg_mut(), f(Pincfg(0)));
        }
        self
    }

    #[doc="Modify the PINCFG register."]
    #[inline] pub fn with_pincfg<F: FnOnce(Pincfg) -> Pincfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pincfg_mut(), f(self.pincfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BAUD register."]
    #[inline] pub fn baud_mut(&self) -> *mut Baud { 
        (self.0 + 0x10) as *mut Baud
    }

    #[doc="Get the *const pointer for the BAUD register."]
    #[inline] pub fn baud_ptr(&self) -> *const Baud { 
           self.baud_mut()
    }

    #[doc="Read the BAUD register."]
    #[inline] pub fn baud(&self) -> Baud { 
        unsafe {
            read_volatile(self.baud_ptr())
        }
    }

    #[doc="Write the BAUD register."]
    #[inline] pub fn set_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.baud_mut(), f(Baud(0)));
        }
        self
    }

    #[doc="Modify the BAUD register."]
    #[inline] pub fn with_baud<F: FnOnce(Baud) -> Baud>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.baud_mut(), f(self.baud()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STAT register."]
    #[inline] pub fn stat_mut(&self) -> *mut Stat { 
        (self.0 + 0x14) as *mut Stat
    }

    #[doc="Get the *const pointer for the STAT register."]
    #[inline] pub fn stat_ptr(&self) -> *const Stat { 
           self.stat_mut()
    }

    #[doc="Read the STAT register."]
    #[inline] pub fn stat(&self) -> Stat { 
        unsafe {
            read_volatile(self.stat_ptr())
        }
    }

    #[doc="Write the STAT register."]
    #[inline] pub fn set_stat<F: FnOnce(Stat) -> Stat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stat_mut(), f(Stat(0)));
        }
        self
    }

    #[doc="Modify the STAT register."]
    #[inline] pub fn with_stat<F: FnOnce(Stat) -> Stat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stat_mut(), f(self.stat()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut(&self) -> *mut Ctrl { 
        (self.0 + 0x18) as *mut Ctrl
    }

    #[doc="Get the *const pointer for the CTRL register."]
    #[inline] pub fn ctrl_ptr(&self) -> *const Ctrl { 
           self.ctrl_mut()
    }

    #[doc="Read the CTRL register."]
    #[inline] pub fn ctrl(&self) -> Ctrl { 
        unsafe {
            read_volatile(self.ctrl_ptr())
        }
    }

    #[doc="Write the CTRL register."]
    #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl_mut(), f(Ctrl(0)));
        }
        self
    }

    #[doc="Modify the CTRL register."]
    #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl_mut(), f(self.ctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DATA register."]
    #[inline] pub fn data_mut(&self) -> *mut Data { 
        (self.0 + 0x1c) as *mut Data
    }

    #[doc="Get the *const pointer for the DATA register."]
    #[inline] pub fn data_ptr(&self) -> *const Data { 
           self.data_mut()
    }

    #[doc="Read the DATA register."]
    #[inline] pub fn data(&self) -> Data { 
        unsafe {
            read_volatile(self.data_ptr())
        }
    }

    #[doc="Write the DATA register."]
    #[inline] pub fn set_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.data_mut(), f(Data(0)));
        }
        self
    }

    #[doc="Modify the DATA register."]
    #[inline] pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.data_mut(), f(self.data()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MATCH register."]
    #[inline] pub fn match_mut(&self) -> *mut Match { 
        (self.0 + 0x20) as *mut Match
    }

    #[doc="Get the *const pointer for the MATCH register."]
    #[inline] pub fn match_ptr(&self) -> *const Match { 
           self.match_mut()
    }

    #[doc="Read the MATCH register."]
    #[inline] pub fn _match(&self) -> Match { 
        unsafe {
            read_volatile(self.match_ptr())
        }
    }

    #[doc="Write the MATCH register."]
    #[inline] pub fn set_match<F: FnOnce(Match) -> Match>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.match_mut(), f(Match(0)));
        }
        self
    }

    #[doc="Modify the MATCH register."]
    #[inline] pub fn with_match<F: FnOnce(Match) -> Match>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.match_mut(), f(self._match()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MODIR register."]
    #[inline] pub fn modir_mut(&self) -> *mut Modir { 
        (self.0 + 0x24) as *mut Modir
    }

    #[doc="Get the *const pointer for the MODIR register."]
    #[inline] pub fn modir_ptr(&self) -> *const Modir { 
           self.modir_mut()
    }

    #[doc="Read the MODIR register."]
    #[inline] pub fn modir(&self) -> Modir { 
        unsafe {
            read_volatile(self.modir_ptr())
        }
    }

    #[doc="Write the MODIR register."]
    #[inline] pub fn set_modir<F: FnOnce(Modir) -> Modir>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.modir_mut(), f(Modir(0)));
        }
        self
    }

    #[doc="Modify the MODIR register."]
    #[inline] pub fn with_modir<F: FnOnce(Modir) -> Modir>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.modir_mut(), f(self.modir()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FIFO register."]
    #[inline] pub fn fifo_mut(&self) -> *mut Fifo { 
        (self.0 + 0x28) as *mut Fifo
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

    #[doc="Get the *mut pointer for the WATER register."]
    #[inline] pub fn water_mut(&self) -> *mut Water { 
        (self.0 + 0x2c) as *mut Water
    }

    #[doc="Get the *const pointer for the WATER register."]
    #[inline] pub fn water_ptr(&self) -> *const Water { 
           self.water_mut()
    }

    #[doc="Read the WATER register."]
    #[inline] pub fn water(&self) -> Water { 
        unsafe {
            read_volatile(self.water_ptr())
        }
    }

    #[doc="Write the WATER register."]
    #[inline] pub fn set_water<F: FnOnce(Water) -> Water>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.water_mut(), f(Water(0)));
        }
        self
    }

    #[doc="Modify the WATER register."]
    #[inline] pub fn with_water<F: FnOnce(Water) -> Water>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.water_mut(), f(self.water()));
        }
        self
    }

}

#[doc="Version ID Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc="Feature Identification Number"]
    #[inline] pub fn feature(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Feature Identification Number"]
    #[inline] pub fn test_feature(&self) -> bool {
        self.feature() != 0
    }

    #[doc="Feature Identification Number"]
    #[inline] pub fn set_feature<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Minor Version Number"]
    #[inline] pub fn minor(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Minor Version Number"]
    #[inline] pub fn test_minor(&self) -> bool {
        self.minor() != 0
    }

    #[doc="Minor Version Number"]
    #[inline] pub fn set_minor<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Major Version Number"]
    #[inline] pub fn major(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Major Version Number"]
    #[inline] pub fn test_major(&self) -> bool {
        self.major() != 0
    }

    #[doc="Major Version Number"]
    #[inline] pub fn set_major<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Verid {
    #[inline]
    fn from(other: u32) -> Self {
         Verid(other)
    }
}

impl ::core::fmt::Display for Verid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.feature() != 0 { try!(write!(f, " feature=0x{:x}", self.feature()))}
        if self.minor() != 0 { try!(write!(f, " minor=0x{:x}", self.minor()))}
        if self.major() != 0 { try!(write!(f, " major=0x{:x}", self.major()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Parameter Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Param(pub u32);
impl Param {
    #[doc="Transmit FIFO Size"]
    #[inline] pub fn txfifo(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Transmit FIFO Size"]
    #[inline] pub fn test_txfifo(&self) -> bool {
        self.txfifo() != 0
    }

    #[doc="Transmit FIFO Size"]
    #[inline] pub fn set_txfifo<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive FIFO Size"]
    #[inline] pub fn rxfifo(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Receive FIFO Size"]
    #[inline] pub fn test_rxfifo(&self) -> bool {
        self.rxfifo() != 0
    }

    #[doc="Receive FIFO Size"]
    #[inline] pub fn set_rxfifo<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Param {
    #[inline]
    fn from(other: u32) -> Self {
         Param(other)
    }
}

impl ::core::fmt::Display for Param {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Param {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txfifo() != 0 { try!(write!(f, " txfifo=0x{:x}", self.txfifo()))}
        if self.rxfifo() != 0 { try!(write!(f, " rxfifo=0x{:x}", self.rxfifo()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPUART Global Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Global(pub u32);
impl Global {
    #[doc="Software Reset"]
    #[inline] pub fn rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Software Reset"]
    #[inline] pub fn test_rst(&self) -> bool {
        self.rst() != 0
    }

    #[doc="Software Reset"]
    #[inline] pub fn set_rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Global {
    #[inline]
    fn from(other: u32) -> Self {
         Global(other)
    }
}

impl ::core::fmt::Display for Global {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Global {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rst() != 0 { try!(write!(f, " rst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPUART Pin Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pincfg(pub u32);
impl Pincfg {
    #[doc="Trigger Select"]
    #[inline] pub fn trgsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Trigger Select"]
    #[inline] pub fn test_trgsel(&self) -> bool {
        self.trgsel() != 0
    }

    #[doc="Trigger Select"]
    #[inline] pub fn set_trgsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pincfg {
    #[inline]
    fn from(other: u32) -> Self {
         Pincfg(other)
    }
}

impl ::core::fmt::Display for Pincfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pincfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.trgsel() != 0 { try!(write!(f, " trgsel=0x{:x}", self.trgsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPUART Baud Rate Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Baud(pub u32);
impl Baud {
    #[doc="Baud Rate Modulo Divisor."]
    #[inline] pub fn sbr(&self) -> bits::U13 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1fff) as u16) } // [12:0]
    }

    #[doc="Baud Rate Modulo Divisor."]
    #[inline] pub fn test_sbr(&self) -> bool {
        self.sbr() != 0
    }

    #[doc="Baud Rate Modulo Divisor."]
    #[inline] pub fn set_sbr<V: Into<bits::U13>>(mut self, value: V) -> Self {
        let value: bits::U13 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1fff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Stop Bit Number Select"]
    #[inline] pub fn sbns(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Stop Bit Number Select"]
    #[inline] pub fn test_sbns(&self) -> bool {
        self.sbns() != 0
    }

    #[doc="Stop Bit Number Select"]
    #[inline] pub fn set_sbns<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="RX Input Active Edge Interrupt Enable"]
    #[inline] pub fn rxedgie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="RX Input Active Edge Interrupt Enable"]
    #[inline] pub fn test_rxedgie(&self) -> bool {
        self.rxedgie() != 0
    }

    #[doc="RX Input Active Edge Interrupt Enable"]
    #[inline] pub fn set_rxedgie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="LIN Break Detect Interrupt Enable"]
    #[inline] pub fn lbkdie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="LIN Break Detect Interrupt Enable"]
    #[inline] pub fn test_lbkdie(&self) -> bool {
        self.lbkdie() != 0
    }

    #[doc="LIN Break Detect Interrupt Enable"]
    #[inline] pub fn set_lbkdie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Resynchronization Disable"]
    #[inline] pub fn resyncdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Resynchronization Disable"]
    #[inline] pub fn test_resyncdis(&self) -> bool {
        self.resyncdis() != 0
    }

    #[doc="Resynchronization Disable"]
    #[inline] pub fn set_resyncdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Both Edge Sampling"]
    #[inline] pub fn bothedge(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Both Edge Sampling"]
    #[inline] pub fn test_bothedge(&self) -> bool {
        self.bothedge() != 0
    }

    #[doc="Both Edge Sampling"]
    #[inline] pub fn set_bothedge<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Match Configuration"]
    #[inline] pub fn matcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Match Configuration"]
    #[inline] pub fn test_matcfg(&self) -> bool {
        self.matcfg() != 0
    }

    #[doc="Match Configuration"]
    #[inline] pub fn set_matcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Receiver Idle DMA Enable"]
    #[inline] pub fn ridmae(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Receiver Idle DMA Enable"]
    #[inline] pub fn test_ridmae(&self) -> bool {
        self.ridmae() != 0
    }

    #[doc="Receiver Idle DMA Enable"]
    #[inline] pub fn set_ridmae<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Receiver Full DMA Enable"]
    #[inline] pub fn rdmae(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Receiver Full DMA Enable"]
    #[inline] pub fn test_rdmae(&self) -> bool {
        self.rdmae() != 0
    }

    #[doc="Receiver Full DMA Enable"]
    #[inline] pub fn set_rdmae<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Transmitter DMA Enable"]
    #[inline] pub fn tdmae(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Transmitter DMA Enable"]
    #[inline] pub fn test_tdmae(&self) -> bool {
        self.tdmae() != 0
    }

    #[doc="Transmitter DMA Enable"]
    #[inline] pub fn set_tdmae<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Oversampling Ratio"]
    #[inline] pub fn osr(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Oversampling Ratio"]
    #[inline] pub fn test_osr(&self) -> bool {
        self.osr() != 0
    }

    #[doc="Oversampling Ratio"]
    #[inline] pub fn set_osr<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="10-bit Mode select"]
    #[inline] pub fn m10(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="10-bit Mode select"]
    #[inline] pub fn test_m10(&self) -> bool {
        self.m10() != 0
    }

    #[doc="10-bit Mode select"]
    #[inline] pub fn set_m10<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Match Address Mode Enable 2"]
    #[inline] pub fn maen2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Match Address Mode Enable 2"]
    #[inline] pub fn test_maen2(&self) -> bool {
        self.maen2() != 0
    }

    #[doc="Match Address Mode Enable 2"]
    #[inline] pub fn set_maen2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Match Address Mode Enable 1"]
    #[inline] pub fn maen1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Match Address Mode Enable 1"]
    #[inline] pub fn test_maen1(&self) -> bool {
        self.maen1() != 0
    }

    #[doc="Match Address Mode Enable 1"]
    #[inline] pub fn set_maen1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Baud {
    #[inline]
    fn from(other: u32) -> Self {
         Baud(other)
    }
}

impl ::core::fmt::Display for Baud {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Baud {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sbr() != 0 { try!(write!(f, " sbr=0x{:x}", self.sbr()))}
        if self.sbns() != 0 { try!(write!(f, " sbns"))}
        if self.rxedgie() != 0 { try!(write!(f, " rxedgie"))}
        if self.lbkdie() != 0 { try!(write!(f, " lbkdie"))}
        if self.resyncdis() != 0 { try!(write!(f, " resyncdis"))}
        if self.bothedge() != 0 { try!(write!(f, " bothedge"))}
        if self.matcfg() != 0 { try!(write!(f, " matcfg=0x{:x}", self.matcfg()))}
        if self.ridmae() != 0 { try!(write!(f, " ridmae"))}
        if self.rdmae() != 0 { try!(write!(f, " rdmae"))}
        if self.tdmae() != 0 { try!(write!(f, " tdmae"))}
        if self.osr() != 0 { try!(write!(f, " osr=0x{:x}", self.osr()))}
        if self.m10() != 0 { try!(write!(f, " m10"))}
        if self.maen2() != 0 { try!(write!(f, " maen2"))}
        if self.maen1() != 0 { try!(write!(f, " maen1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPUART Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc="Match 2 Flag"]
    #[inline] pub fn ma2f(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Match 2 Flag"]
    #[inline] pub fn test_ma2f(&self) -> bool {
        self.ma2f() != 0
    }

    #[doc="Match 2 Flag"]
    #[inline] pub fn set_ma2f<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Match 1 Flag"]
    #[inline] pub fn ma1f(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Match 1 Flag"]
    #[inline] pub fn test_ma1f(&self) -> bool {
        self.ma1f() != 0
    }

    #[doc="Match 1 Flag"]
    #[inline] pub fn set_ma1f<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Parity Error Flag"]
    #[inline] pub fn pf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Parity Error Flag"]
    #[inline] pub fn test_pf(&self) -> bool {
        self.pf() != 0
    }

    #[doc="Parity Error Flag"]
    #[inline] pub fn set_pf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Framing Error Flag"]
    #[inline] pub fn fe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Framing Error Flag"]
    #[inline] pub fn test_fe(&self) -> bool {
        self.fe() != 0
    }

    #[doc="Framing Error Flag"]
    #[inline] pub fn set_fe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Noise Flag"]
    #[inline] pub fn nf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Noise Flag"]
    #[inline] pub fn test_nf(&self) -> bool {
        self.nf() != 0
    }

    #[doc="Noise Flag"]
    #[inline] pub fn set_nf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Receiver Overrun Flag"]
    #[inline] pub fn or(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Receiver Overrun Flag"]
    #[inline] pub fn test_or(&self) -> bool {
        self.or() != 0
    }

    #[doc="Receiver Overrun Flag"]
    #[inline] pub fn set_or<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Idle Line Flag"]
    #[inline] pub fn idle(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Idle Line Flag"]
    #[inline] pub fn test_idle(&self) -> bool {
        self.idle() != 0
    }

    #[doc="Idle Line Flag"]
    #[inline] pub fn set_idle<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Receive Data Register Full Flag"]
    #[inline] pub fn rdrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Receive Data Register Full Flag"]
    #[inline] pub fn test_rdrf(&self) -> bool {
        self.rdrf() != 0
    }

    #[doc="Receive Data Register Full Flag"]
    #[inline] pub fn set_rdrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Transmission Complete Flag"]
    #[inline] pub fn tc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Transmission Complete Flag"]
    #[inline] pub fn test_tc(&self) -> bool {
        self.tc() != 0
    }

    #[doc="Transmission Complete Flag"]
    #[inline] pub fn set_tc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Transmit Data Register Empty Flag"]
    #[inline] pub fn tdre(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Transmit Data Register Empty Flag"]
    #[inline] pub fn test_tdre(&self) -> bool {
        self.tdre() != 0
    }

    #[doc="Transmit Data Register Empty Flag"]
    #[inline] pub fn set_tdre<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Receiver Active Flag"]
    #[inline] pub fn raf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Receiver Active Flag"]
    #[inline] pub fn test_raf(&self) -> bool {
        self.raf() != 0
    }

    #[doc="Receiver Active Flag"]
    #[inline] pub fn set_raf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="LIN Break Detection Enable"]
    #[inline] pub fn lbkde(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="LIN Break Detection Enable"]
    #[inline] pub fn test_lbkde(&self) -> bool {
        self.lbkde() != 0
    }

    #[doc="LIN Break Detection Enable"]
    #[inline] pub fn set_lbkde<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Break Character Generation Length"]
    #[inline] pub fn brk13(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Break Character Generation Length"]
    #[inline] pub fn test_brk13(&self) -> bool {
        self.brk13() != 0
    }

    #[doc="Break Character Generation Length"]
    #[inline] pub fn set_brk13<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Receive Wake Up Idle Detect"]
    #[inline] pub fn rwuid(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Receive Wake Up Idle Detect"]
    #[inline] pub fn test_rwuid(&self) -> bool {
        self.rwuid() != 0
    }

    #[doc="Receive Wake Up Idle Detect"]
    #[inline] pub fn set_rwuid<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Receive Data Inversion"]
    #[inline] pub fn rxinv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Receive Data Inversion"]
    #[inline] pub fn test_rxinv(&self) -> bool {
        self.rxinv() != 0
    }

    #[doc="Receive Data Inversion"]
    #[inline] pub fn set_rxinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="MSB First"]
    #[inline] pub fn msbf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="MSB First"]
    #[inline] pub fn test_msbf(&self) -> bool {
        self.msbf() != 0
    }

    #[doc="MSB First"]
    #[inline] pub fn set_msbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="RXD Pin Active Edge Interrupt Flag"]
    #[inline] pub fn rxedgif(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="RXD Pin Active Edge Interrupt Flag"]
    #[inline] pub fn test_rxedgif(&self) -> bool {
        self.rxedgif() != 0
    }

    #[doc="RXD Pin Active Edge Interrupt Flag"]
    #[inline] pub fn set_rxedgif<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="LIN Break Detect Interrupt Flag"]
    #[inline] pub fn lbkdif(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="LIN Break Detect Interrupt Flag"]
    #[inline] pub fn test_lbkdif(&self) -> bool {
        self.lbkdif() != 0
    }

    #[doc="LIN Break Detect Interrupt Flag"]
    #[inline] pub fn set_lbkdif<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Stat {
    #[inline]
    fn from(other: u32) -> Self {
         Stat(other)
    }
}

impl ::core::fmt::Display for Stat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ma2f() != 0 { try!(write!(f, " ma2f"))}
        if self.ma1f() != 0 { try!(write!(f, " ma1f"))}
        if self.pf() != 0 { try!(write!(f, " pf"))}
        if self.fe() != 0 { try!(write!(f, " fe"))}
        if self.nf() != 0 { try!(write!(f, " nf"))}
        if self.or() != 0 { try!(write!(f, " or"))}
        if self.idle() != 0 { try!(write!(f, " idle"))}
        if self.rdrf() != 0 { try!(write!(f, " rdrf"))}
        if self.tc() != 0 { try!(write!(f, " tc"))}
        if self.tdre() != 0 { try!(write!(f, " tdre"))}
        if self.raf() != 0 { try!(write!(f, " raf"))}
        if self.lbkde() != 0 { try!(write!(f, " lbkde"))}
        if self.brk13() != 0 { try!(write!(f, " brk13"))}
        if self.rwuid() != 0 { try!(write!(f, " rwuid"))}
        if self.rxinv() != 0 { try!(write!(f, " rxinv"))}
        if self.msbf() != 0 { try!(write!(f, " msbf"))}
        if self.rxedgif() != 0 { try!(write!(f, " rxedgif"))}
        if self.lbkdif() != 0 { try!(write!(f, " lbkdif"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPUART Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc="Parity Type"]
    #[inline] pub fn pt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Parity Type"]
    #[inline] pub fn test_pt(&self) -> bool {
        self.pt() != 0
    }

    #[doc="Parity Type"]
    #[inline] pub fn set_pt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Parity Enable"]
    #[inline] pub fn pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Parity Enable"]
    #[inline] pub fn test_pe(&self) -> bool {
        self.pe() != 0
    }

    #[doc="Parity Enable"]
    #[inline] pub fn set_pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Idle Line Type Select"]
    #[inline] pub fn ilt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Idle Line Type Select"]
    #[inline] pub fn test_ilt(&self) -> bool {
        self.ilt() != 0
    }

    #[doc="Idle Line Type Select"]
    #[inline] pub fn set_ilt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receiver Wakeup Method Select"]
    #[inline] pub fn wake(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Receiver Wakeup Method Select"]
    #[inline] pub fn test_wake(&self) -> bool {
        self.wake() != 0
    }

    #[doc="Receiver Wakeup Method Select"]
    #[inline] pub fn set_wake<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="9-Bit or 8-Bit Mode Select"]
    #[inline] pub fn m(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="9-Bit or 8-Bit Mode Select"]
    #[inline] pub fn test_m(&self) -> bool {
        self.m() != 0
    }

    #[doc="9-Bit or 8-Bit Mode Select"]
    #[inline] pub fn set_m<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Receiver Source Select"]
    #[inline] pub fn rsrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Receiver Source Select"]
    #[inline] pub fn test_rsrc(&self) -> bool {
        self.rsrc() != 0
    }

    #[doc="Receiver Source Select"]
    #[inline] pub fn set_rsrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Doze Enable"]
    #[inline] pub fn dozeen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Doze Enable"]
    #[inline] pub fn test_dozeen(&self) -> bool {
        self.dozeen() != 0
    }

    #[doc="Doze Enable"]
    #[inline] pub fn set_dozeen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Loop Mode Select"]
    #[inline] pub fn loops(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Loop Mode Select"]
    #[inline] pub fn test_loops(&self) -> bool {
        self.loops() != 0
    }

    #[doc="Loop Mode Select"]
    #[inline] pub fn set_loops<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Idle Configuration"]
    #[inline] pub fn idlecfg(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Idle Configuration"]
    #[inline] pub fn test_idlecfg(&self) -> bool {
        self.idlecfg() != 0
    }

    #[doc="Idle Configuration"]
    #[inline] pub fn set_idlecfg<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="7-Bit Mode Select"]
    #[inline] pub fn m7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="7-Bit Mode Select"]
    #[inline] pub fn test_m7(&self) -> bool {
        self.m7() != 0
    }

    #[doc="7-Bit Mode Select"]
    #[inline] pub fn set_m7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Match 2 Interrupt Enable"]
    #[inline] pub fn ma2ie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Match 2 Interrupt Enable"]
    #[inline] pub fn test_ma2ie(&self) -> bool {
        self.ma2ie() != 0
    }

    #[doc="Match 2 Interrupt Enable"]
    #[inline] pub fn set_ma2ie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Match 1 Interrupt Enable"]
    #[inline] pub fn ma1ie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Match 1 Interrupt Enable"]
    #[inline] pub fn test_ma1ie(&self) -> bool {
        self.ma1ie() != 0
    }

    #[doc="Match 1 Interrupt Enable"]
    #[inline] pub fn set_ma1ie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Send Break"]
    #[inline] pub fn sbk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Send Break"]
    #[inline] pub fn test_sbk(&self) -> bool {
        self.sbk() != 0
    }

    #[doc="Send Break"]
    #[inline] pub fn set_sbk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Receiver Wakeup Control"]
    #[inline] pub fn rwu(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Receiver Wakeup Control"]
    #[inline] pub fn test_rwu(&self) -> bool {
        self.rwu() != 0
    }

    #[doc="Receiver Wakeup Control"]
    #[inline] pub fn set_rwu<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Receiver Enable"]
    #[inline] pub fn re(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Receiver Enable"]
    #[inline] pub fn test_re(&self) -> bool {
        self.re() != 0
    }

    #[doc="Receiver Enable"]
    #[inline] pub fn set_re<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Transmitter Enable"]
    #[inline] pub fn te(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Transmitter Enable"]
    #[inline] pub fn test_te(&self) -> bool {
        self.te() != 0
    }

    #[doc="Transmitter Enable"]
    #[inline] pub fn set_te<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Idle Line Interrupt Enable"]
    #[inline] pub fn ilie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Idle Line Interrupt Enable"]
    #[inline] pub fn test_ilie(&self) -> bool {
        self.ilie() != 0
    }

    #[doc="Idle Line Interrupt Enable"]
    #[inline] pub fn set_ilie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Receiver Interrupt Enable"]
    #[inline] pub fn rie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Receiver Interrupt Enable"]
    #[inline] pub fn test_rie(&self) -> bool {
        self.rie() != 0
    }

    #[doc="Receiver Interrupt Enable"]
    #[inline] pub fn set_rie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Transmission Complete Interrupt Enable for"]
    #[inline] pub fn tcie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Transmission Complete Interrupt Enable for"]
    #[inline] pub fn test_tcie(&self) -> bool {
        self.tcie() != 0
    }

    #[doc="Transmission Complete Interrupt Enable for"]
    #[inline] pub fn set_tcie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Transmit Interrupt Enable"]
    #[inline] pub fn tie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Transmit Interrupt Enable"]
    #[inline] pub fn test_tie(&self) -> bool {
        self.tie() != 0
    }

    #[doc="Transmit Interrupt Enable"]
    #[inline] pub fn set_tie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Parity Error Interrupt Enable"]
    #[inline] pub fn peie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Parity Error Interrupt Enable"]
    #[inline] pub fn test_peie(&self) -> bool {
        self.peie() != 0
    }

    #[doc="Parity Error Interrupt Enable"]
    #[inline] pub fn set_peie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Framing Error Interrupt Enable"]
    #[inline] pub fn feie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Framing Error Interrupt Enable"]
    #[inline] pub fn test_feie(&self) -> bool {
        self.feie() != 0
    }

    #[doc="Framing Error Interrupt Enable"]
    #[inline] pub fn set_feie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Noise Error Interrupt Enable"]
    #[inline] pub fn neie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Noise Error Interrupt Enable"]
    #[inline] pub fn test_neie(&self) -> bool {
        self.neie() != 0
    }

    #[doc="Noise Error Interrupt Enable"]
    #[inline] pub fn set_neie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Overrun Interrupt Enable"]
    #[inline] pub fn orie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Overrun Interrupt Enable"]
    #[inline] pub fn test_orie(&self) -> bool {
        self.orie() != 0
    }

    #[doc="Overrun Interrupt Enable"]
    #[inline] pub fn set_orie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Transmit Data Inversion"]
    #[inline] pub fn txinv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Transmit Data Inversion"]
    #[inline] pub fn test_txinv(&self) -> bool {
        self.txinv() != 0
    }

    #[doc="Transmit Data Inversion"]
    #[inline] pub fn set_txinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="TXD Pin Direction in Single-Wire Mode"]
    #[inline] pub fn txdir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="TXD Pin Direction in Single-Wire Mode"]
    #[inline] pub fn test_txdir(&self) -> bool {
        self.txdir() != 0
    }

    #[doc="TXD Pin Direction in Single-Wire Mode"]
    #[inline] pub fn set_txdir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Receive Bit 9 / Transmit Bit 8"]
    #[inline] pub fn r9t8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Receive Bit 9 / Transmit Bit 8"]
    #[inline] pub fn test_r9t8(&self) -> bool {
        self.r9t8() != 0
    }

    #[doc="Receive Bit 9 / Transmit Bit 8"]
    #[inline] pub fn set_r9t8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Receive Bit 8 / Transmit Bit 9"]
    #[inline] pub fn r8t9(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Receive Bit 8 / Transmit Bit 9"]
    #[inline] pub fn test_r8t9(&self) -> bool {
        self.r8t9() != 0
    }

    #[doc="Receive Bit 8 / Transmit Bit 9"]
    #[inline] pub fn set_r8t9<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Ctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Ctrl(other)
    }
}

impl ::core::fmt::Display for Ctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pt() != 0 { try!(write!(f, " pt"))}
        if self.pe() != 0 { try!(write!(f, " pe"))}
        if self.ilt() != 0 { try!(write!(f, " ilt"))}
        if self.wake() != 0 { try!(write!(f, " wake"))}
        if self.m() != 0 { try!(write!(f, " m"))}
        if self.rsrc() != 0 { try!(write!(f, " rsrc"))}
        if self.dozeen() != 0 { try!(write!(f, " dozeen"))}
        if self.loops() != 0 { try!(write!(f, " loops"))}
        if self.idlecfg() != 0 { try!(write!(f, " idlecfg=0x{:x}", self.idlecfg()))}
        if self.m7() != 0 { try!(write!(f, " m7"))}
        if self.ma2ie() != 0 { try!(write!(f, " ma2ie"))}
        if self.ma1ie() != 0 { try!(write!(f, " ma1ie"))}
        if self.sbk() != 0 { try!(write!(f, " sbk"))}
        if self.rwu() != 0 { try!(write!(f, " rwu"))}
        if self.re() != 0 { try!(write!(f, " re"))}
        if self.te() != 0 { try!(write!(f, " te"))}
        if self.ilie() != 0 { try!(write!(f, " ilie"))}
        if self.rie() != 0 { try!(write!(f, " rie"))}
        if self.tcie() != 0 { try!(write!(f, " tcie"))}
        if self.tie() != 0 { try!(write!(f, " tie"))}
        if self.peie() != 0 { try!(write!(f, " peie"))}
        if self.feie() != 0 { try!(write!(f, " feie"))}
        if self.neie() != 0 { try!(write!(f, " neie"))}
        if self.orie() != 0 { try!(write!(f, " orie"))}
        if self.txinv() != 0 { try!(write!(f, " txinv"))}
        if self.txdir() != 0 { try!(write!(f, " txdir"))}
        if self.r9t8() != 0 { try!(write!(f, " r9t8"))}
        if self.r8t9() != 0 { try!(write!(f, " r8t9"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPUART Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Data(pub u32);
impl Data {
    #[doc="RT"]
    #[inline] pub fn rt(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="RT"]
    #[inline] pub fn test_rt(&self) -> bool {
        self.rt() != 0
    }

    #[doc="RT"]
    #[inline] pub fn set_rt<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Idle Line"]
    #[inline] pub fn idline(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Idle Line"]
    #[inline] pub fn test_idline(&self) -> bool {
        self.idline() != 0
    }

    #[doc="Idle Line"]
    #[inline] pub fn set_idline<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Receive Buffer Empty"]
    #[inline] pub fn rxempt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Receive Buffer Empty"]
    #[inline] pub fn test_rxempt(&self) -> bool {
        self.rxempt() != 0
    }

    #[doc="Receive Buffer Empty"]
    #[inline] pub fn set_rxempt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Frame Error / Transmit Special Character"]
    #[inline] pub fn fretsc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Frame Error / Transmit Special Character"]
    #[inline] pub fn test_fretsc(&self) -> bool {
        self.fretsc() != 0
    }

    #[doc="Frame Error / Transmit Special Character"]
    #[inline] pub fn set_fretsc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="PARITYE"]
    #[inline] pub fn paritye(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="PARITYE"]
    #[inline] pub fn test_paritye(&self) -> bool {
        self.paritye() != 0
    }

    #[doc="PARITYE"]
    #[inline] pub fn set_paritye<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="NOISY"]
    #[inline] pub fn noisy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="NOISY"]
    #[inline] pub fn test_noisy(&self) -> bool {
        self.noisy() != 0
    }

    #[doc="NOISY"]
    #[inline] pub fn set_noisy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Data {
    #[inline]
    fn from(other: u32) -> Self {
         Data(other)
    }
}

impl ::core::fmt::Display for Data {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Data {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rt() != 0 { try!(write!(f, " rt=0x{:x}", self.rt()))}
        if self.idline() != 0 { try!(write!(f, " idline"))}
        if self.rxempt() != 0 { try!(write!(f, " rxempt"))}
        if self.fretsc() != 0 { try!(write!(f, " fretsc"))}
        if self.paritye() != 0 { try!(write!(f, " paritye"))}
        if self.noisy() != 0 { try!(write!(f, " noisy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPUART Match Address Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Match(pub u32);
impl Match {
    #[doc="Match Address 1"]
    #[inline] pub fn ma1(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Match Address 1"]
    #[inline] pub fn test_ma1(&self) -> bool {
        self.ma1() != 0
    }

    #[doc="Match Address 1"]
    #[inline] pub fn set_ma1<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Match Address 2"]
    #[inline] pub fn ma2(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3ff) as u16) } // [25:16]
    }

    #[doc="Match Address 2"]
    #[inline] pub fn test_ma2(&self) -> bool {
        self.ma2() != 0
    }

    #[doc="Match Address 2"]
    #[inline] pub fn set_ma2<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Match {
    #[inline]
    fn from(other: u32) -> Self {
         Match(other)
    }
}

impl ::core::fmt::Display for Match {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Match {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ma1() != 0 { try!(write!(f, " ma1=0x{:x}", self.ma1()))}
        if self.ma2() != 0 { try!(write!(f, " ma2=0x{:x}", self.ma2()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPUART Modem IrDA Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Modir(pub u32);
impl Modir {
    #[doc="Transmitter clear-to-send enable"]
    #[inline] pub fn txctse(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Transmitter clear-to-send enable"]
    #[inline] pub fn test_txctse(&self) -> bool {
        self.txctse() != 0
    }

    #[doc="Transmitter clear-to-send enable"]
    #[inline] pub fn set_txctse<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmitter request-to-send enable"]
    #[inline] pub fn txrtse(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Transmitter request-to-send enable"]
    #[inline] pub fn test_txrtse(&self) -> bool {
        self.txrtse() != 0
    }

    #[doc="Transmitter request-to-send enable"]
    #[inline] pub fn set_txrtse<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Transmitter request-to-send polarity"]
    #[inline] pub fn txrtspol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Transmitter request-to-send polarity"]
    #[inline] pub fn test_txrtspol(&self) -> bool {
        self.txrtspol() != 0
    }

    #[doc="Transmitter request-to-send polarity"]
    #[inline] pub fn set_txrtspol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receiver request-to-send enable"]
    #[inline] pub fn rxrtse(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Receiver request-to-send enable"]
    #[inline] pub fn test_rxrtse(&self) -> bool {
        self.rxrtse() != 0
    }

    #[doc="Receiver request-to-send enable"]
    #[inline] pub fn set_rxrtse<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmit CTS Configuration"]
    #[inline] pub fn txctsc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Transmit CTS Configuration"]
    #[inline] pub fn test_txctsc(&self) -> bool {
        self.txctsc() != 0
    }

    #[doc="Transmit CTS Configuration"]
    #[inline] pub fn set_txctsc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmit CTS Source"]
    #[inline] pub fn txctssrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Transmit CTS Source"]
    #[inline] pub fn test_txctssrc(&self) -> bool {
        self.txctssrc() != 0
    }

    #[doc="Transmit CTS Source"]
    #[inline] pub fn set_txctssrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Receive RTS Configuration"]
    #[inline] pub fn rtswater(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Receive RTS Configuration"]
    #[inline] pub fn test_rtswater(&self) -> bool {
        self.rtswater() != 0
    }

    #[doc="Receive RTS Configuration"]
    #[inline] pub fn set_rtswater<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Transmitter narrow pulse"]
    #[inline] pub fn tnp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Transmitter narrow pulse"]
    #[inline] pub fn test_tnp(&self) -> bool {
        self.tnp() != 0
    }

    #[doc="Transmitter narrow pulse"]
    #[inline] pub fn set_tnp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Infrared enable"]
    #[inline] pub fn iren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Infrared enable"]
    #[inline] pub fn test_iren(&self) -> bool {
        self.iren() != 0
    }

    #[doc="Infrared enable"]
    #[inline] pub fn set_iren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

}

impl From<u32> for Modir {
    #[inline]
    fn from(other: u32) -> Self {
         Modir(other)
    }
}

impl ::core::fmt::Display for Modir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Modir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txctse() != 0 { try!(write!(f, " txctse"))}
        if self.txrtse() != 0 { try!(write!(f, " txrtse"))}
        if self.txrtspol() != 0 { try!(write!(f, " txrtspol"))}
        if self.rxrtse() != 0 { try!(write!(f, " rxrtse"))}
        if self.txctsc() != 0 { try!(write!(f, " txctsc"))}
        if self.txctssrc() != 0 { try!(write!(f, " txctssrc"))}
        if self.rtswater() != 0 { try!(write!(f, " rtswater=0x{:x}", self.rtswater()))}
        if self.tnp() != 0 { try!(write!(f, " tnp=0x{:x}", self.tnp()))}
        if self.iren() != 0 { try!(write!(f, " iren"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPUART FIFO Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifo(pub u32);
impl Fifo {
    #[doc="Receive FIFO. Buffer Depth"]
    #[inline] pub fn rxfifosize(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Receive FIFO. Buffer Depth"]
    #[inline] pub fn test_rxfifosize(&self) -> bool {
        self.rxfifosize() != 0
    }

    #[doc="Receive FIFO. Buffer Depth"]
    #[inline] pub fn set_rxfifosize<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive FIFO Enable"]
    #[inline] pub fn rxfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Receive FIFO Enable"]
    #[inline] pub fn test_rxfe(&self) -> bool {
        self.rxfe() != 0
    }

    #[doc="Receive FIFO Enable"]
    #[inline] pub fn set_rxfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmit FIFO. Buffer Depth"]
    #[inline] pub fn txfifosize(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Transmit FIFO. Buffer Depth"]
    #[inline] pub fn test_txfifosize(&self) -> bool {
        self.txfifosize() != 0
    }

    #[doc="Transmit FIFO. Buffer Depth"]
    #[inline] pub fn set_txfifosize<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmit FIFO Enable"]
    #[inline] pub fn txfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Transmit FIFO Enable"]
    #[inline] pub fn test_txfe(&self) -> bool {
        self.txfe() != 0
    }

    #[doc="Transmit FIFO Enable"]
    #[inline] pub fn set_txfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Receive FIFO Underflow Interrupt Enable"]
    #[inline] pub fn rxufe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Receive FIFO Underflow Interrupt Enable"]
    #[inline] pub fn test_rxufe(&self) -> bool {
        self.rxufe() != 0
    }

    #[doc="Receive FIFO Underflow Interrupt Enable"]
    #[inline] pub fn set_rxufe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Transmit FIFO Overflow Interrupt Enable"]
    #[inline] pub fn txofe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Transmit FIFO Overflow Interrupt Enable"]
    #[inline] pub fn test_txofe(&self) -> bool {
        self.txofe() != 0
    }

    #[doc="Transmit FIFO Overflow Interrupt Enable"]
    #[inline] pub fn set_txofe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Receiver Idle Empty Enable"]
    #[inline] pub fn rxiden(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x7) as u8) } // [12:10]
    }

    #[doc="Receiver Idle Empty Enable"]
    #[inline] pub fn test_rxiden(&self) -> bool {
        self.rxiden() != 0
    }

    #[doc="Receiver Idle Empty Enable"]
    #[inline] pub fn set_rxiden<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Receive FIFO/Buffer Flush"]
    #[inline] pub fn rxflush(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Receive FIFO/Buffer Flush"]
    #[inline] pub fn test_rxflush(&self) -> bool {
        self.rxflush() != 0
    }

    #[doc="Receive FIFO/Buffer Flush"]
    #[inline] pub fn set_rxflush<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Transmit FIFO/Buffer Flush"]
    #[inline] pub fn txflush(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Transmit FIFO/Buffer Flush"]
    #[inline] pub fn test_txflush(&self) -> bool {
        self.txflush() != 0
    }

    #[doc="Transmit FIFO/Buffer Flush"]
    #[inline] pub fn set_txflush<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Receiver Buffer Underflow Flag"]
    #[inline] pub fn rxuf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Receiver Buffer Underflow Flag"]
    #[inline] pub fn test_rxuf(&self) -> bool {
        self.rxuf() != 0
    }

    #[doc="Receiver Buffer Underflow Flag"]
    #[inline] pub fn set_rxuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Transmitter Buffer Overflow Flag"]
    #[inline] pub fn txof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Transmitter Buffer Overflow Flag"]
    #[inline] pub fn test_txof(&self) -> bool {
        self.txof() != 0
    }

    #[doc="Transmitter Buffer Overflow Flag"]
    #[inline] pub fn set_txof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Receive Buffer/FIFO Empty"]
    #[inline] pub fn rxempt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Receive Buffer/FIFO Empty"]
    #[inline] pub fn test_rxempt(&self) -> bool {
        self.rxempt() != 0
    }

    #[doc="Receive Buffer/FIFO Empty"]
    #[inline] pub fn set_rxempt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Transmit Buffer/FIFO Empty"]
    #[inline] pub fn txempt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Transmit Buffer/FIFO Empty"]
    #[inline] pub fn test_txempt(&self) -> bool {
        self.txempt() != 0
    }

    #[doc="Transmit Buffer/FIFO Empty"]
    #[inline] pub fn set_txempt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
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
        if self.rxfifosize() != 0 { try!(write!(f, " rxfifosize=0x{:x}", self.rxfifosize()))}
        if self.rxfe() != 0 { try!(write!(f, " rxfe"))}
        if self.txfifosize() != 0 { try!(write!(f, " txfifosize=0x{:x}", self.txfifosize()))}
        if self.txfe() != 0 { try!(write!(f, " txfe"))}
        if self.rxufe() != 0 { try!(write!(f, " rxufe"))}
        if self.txofe() != 0 { try!(write!(f, " txofe"))}
        if self.rxiden() != 0 { try!(write!(f, " rxiden=0x{:x}", self.rxiden()))}
        if self.rxflush() != 0 { try!(write!(f, " rxflush"))}
        if self.txflush() != 0 { try!(write!(f, " txflush"))}
        if self.rxuf() != 0 { try!(write!(f, " rxuf"))}
        if self.txof() != 0 { try!(write!(f, " txof"))}
        if self.rxempt() != 0 { try!(write!(f, " rxempt"))}
        if self.txempt() != 0 { try!(write!(f, " txempt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LPUART Watermark Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Water(pub u32);
impl Water {
    #[doc="Transmit Watermark"]
    #[inline] pub fn txwater(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Transmit Watermark"]
    #[inline] pub fn test_txwater(&self) -> bool {
        self.txwater() != 0
    }

    #[doc="Transmit Watermark"]
    #[inline] pub fn set_txwater<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Counter"]
    #[inline] pub fn txcount(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Transmit Counter"]
    #[inline] pub fn test_txcount(&self) -> bool {
        self.txcount() != 0
    }

    #[doc="Transmit Counter"]
    #[inline] pub fn set_txcount<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Receive Watermark"]
    #[inline] pub fn rxwater(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Receive Watermark"]
    #[inline] pub fn test_rxwater(&self) -> bool {
        self.rxwater() != 0
    }

    #[doc="Receive Watermark"]
    #[inline] pub fn set_rxwater<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Receive Counter"]
    #[inline] pub fn rxcount(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Receive Counter"]
    #[inline] pub fn test_rxcount(&self) -> bool {
        self.rxcount() != 0
    }

    #[doc="Receive Counter"]
    #[inline] pub fn set_rxcount<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Water {
    #[inline]
    fn from(other: u32) -> Self {
         Water(other)
    }
}

impl ::core::fmt::Display for Water {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Water {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txwater() != 0 { try!(write!(f, " txwater=0x{:x}", self.txwater()))}
        if self.txcount() != 0 { try!(write!(f, " txcount=0x{:x}", self.txcount()))}
        if self.rxwater() != 0 { try!(write!(f, " rxwater=0x{:x}", self.rxwater()))}
        if self.rxcount() != 0 { try!(write!(f, " rxcount=0x{:x}", self.rxcount()))}
        try!(write!(f, "]"));
        Ok(())
    }
}


