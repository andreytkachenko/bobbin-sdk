pub const DMAC: Dmac = Periph(0x41004800, DmacId {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DmacId {}
pub type Dmac = Periph<DmacId>;



impl<T> Periph<T> {
  #[inline] pub fn active_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x30) as *const u32
  }
  #[inline] pub fn active_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x30) as *mut u32
  }
  #[inline] pub fn active(&self) -> Active { 
     unsafe {
        Active(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
     }
  }

  #[inline] pub fn baseaddr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x34) as *const u32
  }
  #[inline] pub fn baseaddr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x34) as *mut u32
  }
  #[inline] pub fn baseaddr(&self) -> Baseaddr { 
     unsafe {
        Baseaddr(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
     }
  }
  #[inline] pub fn set_baseaddr(&self, value: Baseaddr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_baseaddr<F: FnOnce(Baseaddr) -> Baseaddr>(&self, f: F) -> &Self {
     let tmp = self.baseaddr();
     self.set_baseaddr(f(tmp))
  }

  #[inline] pub fn busych_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
  #[inline] pub fn busych_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
  #[inline] pub fn busych(&self) -> Busych { 
     unsafe {
        Busych(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }

  #[inline] pub fn chctrla_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x40) as *const u8
  }
  #[inline] pub fn chctrla_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x40) as *mut u8
  }
  #[inline] pub fn chctrla(&self) -> Chctrla { 
     unsafe {
        Chctrla(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u8))
     }
  }
  #[inline] pub fn set_chctrla(&self, value: Chctrla) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_chctrla<F: FnOnce(Chctrla) -> Chctrla>(&self, f: F) -> &Self {
     let tmp = self.chctrla();
     self.set_chctrla(f(tmp))
  }

  #[inline] pub fn chctrlb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x44) as *const u32
  }
  #[inline] pub fn chctrlb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x44) as *mut u32
  }
  #[inline] pub fn chctrlb(&self) -> Chctrlb { 
     unsafe {
        Chctrlb(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
     }
  }
  #[inline] pub fn set_chctrlb(&self, value: Chctrlb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_chctrlb<F: FnOnce(Chctrlb) -> Chctrlb>(&self, f: F) -> &Self {
     let tmp = self.chctrlb();
     self.set_chctrlb(f(tmp))
  }

  #[inline] pub fn chid_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x3f) as *const u8
  }
  #[inline] pub fn chid_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x3f) as *mut u8
  }
  #[inline] pub fn chid(&self) -> Chid { 
     unsafe {
        Chid(::core::ptr::read_volatile(((self.0 as usize) + 0x3f) as *const u8))
     }
  }
  #[inline] pub fn set_chid(&self, value: Chid) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x3f) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_chid<F: FnOnce(Chid) -> Chid>(&self, f: F) -> &Self {
     let tmp = self.chid();
     self.set_chid(f(tmp))
  }

  #[inline] pub fn chintenclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4c) as *const u8
  }
  #[inline] pub fn chintenclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4c) as *mut u8
  }
  #[inline] pub fn chintenclr(&self) -> Chintenclr { 
     unsafe {
        Chintenclr(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u8))
     }
  }
  #[inline] pub fn set_chintenclr(&self, value: Chintenclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_chintenclr<F: FnOnce(Chintenclr) -> Chintenclr>(&self, f: F) -> &Self {
     let tmp = self.chintenclr();
     self.set_chintenclr(f(tmp))
  }

  #[inline] pub fn chintenset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4d) as *const u8
  }
  #[inline] pub fn chintenset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4d) as *mut u8
  }
  #[inline] pub fn chintenset(&self) -> Chintenset { 
     unsafe {
        Chintenset(::core::ptr::read_volatile(((self.0 as usize) + 0x4d) as *const u8))
     }
  }
  #[inline] pub fn set_chintenset(&self, value: Chintenset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4d) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_chintenset<F: FnOnce(Chintenset) -> Chintenset>(&self, f: F) -> &Self {
     let tmp = self.chintenset();
     self.set_chintenset(f(tmp))
  }

  #[inline] pub fn chintflag_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4e) as *const u8
  }
  #[inline] pub fn chintflag_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4e) as *mut u8
  }
  #[inline] pub fn chintflag(&self) -> Chintflag { 
     unsafe {
        Chintflag(::core::ptr::read_volatile(((self.0 as usize) + 0x4e) as *const u8))
     }
  }
  #[inline] pub fn set_chintflag(&self, value: Chintflag) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4e) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_chintflag<F: FnOnce(Chintflag) -> Chintflag>(&self, f: F) -> &Self {
     let tmp = self.chintflag();
     self.set_chintflag(f(tmp))
  }

  #[inline] pub fn chstatus_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4f) as *const u8
  }
  #[inline] pub fn chstatus_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4f) as *mut u8
  }
  #[inline] pub fn chstatus(&self) -> Chstatus { 
     unsafe {
        Chstatus(::core::ptr::read_volatile(((self.0 as usize) + 0x4f) as *const u8))
     }
  }

  #[inline] pub fn crcchksum_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
  #[inline] pub fn crcchksum_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
  #[inline] pub fn crcchksum(&self) -> Crcchksum { 
     unsafe {
        Crcchksum(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  #[inline] pub fn set_crcchksum(&self, value: Crcchksum) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_crcchksum<F: FnOnce(Crcchksum) -> Crcchksum>(&self, f: F) -> &Self {
     let tmp = self.crcchksum();
     self.set_crcchksum(f(tmp))
  }

  #[inline] pub fn crcctrl_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x2) as *const u16
  }
  #[inline] pub fn crcctrl_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x2) as *mut u16
  }
  #[inline] pub fn crcctrl(&self) -> Crcctrl { 
     unsafe {
        Crcctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u16))
     }
  }
  #[inline] pub fn set_crcctrl(&self, value: Crcctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u16, value.0);
     }
     self
  }
  #[inline] pub fn with_crcctrl<F: FnOnce(Crcctrl) -> Crcctrl>(&self, f: F) -> &Self {
     let tmp = self.crcctrl();
     self.set_crcctrl(f(tmp))
  }

  #[inline] pub fn crcdatain_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
  #[inline] pub fn crcdatain_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
  #[inline] pub fn crcdatain(&self) -> Crcdatain { 
     unsafe {
        Crcdatain(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  #[inline] pub fn set_crcdatain(&self, value: Crcdatain) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_crcdatain<F: FnOnce(Crcdatain) -> Crcdatain>(&self, f: F) -> &Self {
     let tmp = self.crcdatain();
     self.set_crcdatain(f(tmp))
  }

  #[inline] pub fn crcstatus_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xc) as *const u8
  }
  #[inline] pub fn crcstatus_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xc) as *mut u8
  }
  #[inline] pub fn crcstatus(&self) -> Crcstatus { 
     unsafe {
        Crcstatus(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u8))
     }
  }
  #[inline] pub fn set_crcstatus(&self, value: Crcstatus) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_crcstatus<F: FnOnce(Crcstatus) -> Crcstatus>(&self, f: F) -> &Self {
     let tmp = self.crcstatus();
     self.set_crcstatus(f(tmp))
  }

  #[inline] pub fn ctrl_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x0) as *const u16
  }
  #[inline] pub fn ctrl_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x0) as *mut u16
  }
  #[inline] pub fn ctrl(&self) -> Ctrl { 
     unsafe {
        Ctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u16))
     }
  }
  #[inline] pub fn set_ctrl(&self, value: Ctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u16, value.0);
     }
     self
  }
  #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
     let tmp = self.ctrl();
     self.set_ctrl(f(tmp))
  }

  #[inline] pub fn dbgctrl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xd) as *const u8
  }
  #[inline] pub fn dbgctrl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xd) as *mut u8
  }
  #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
     unsafe {
        Dbgctrl(::core::ptr::read_volatile(((self.0 as usize) + 0xd) as *const u8))
     }
  }
  #[inline] pub fn set_dbgctrl(&self, value: Dbgctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
     let tmp = self.dbgctrl();
     self.set_dbgctrl(f(tmp))
  }

  #[inline] pub fn intpend_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x20) as *const u16
  }
  #[inline] pub fn intpend_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x20) as *mut u16
  }
  #[inline] pub fn intpend(&self) -> Intpend { 
     unsafe {
        Intpend(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u16))
     }
  }
  #[inline] pub fn set_intpend(&self, value: Intpend) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u16, value.0);
     }
     self
  }
  #[inline] pub fn with_intpend<F: FnOnce(Intpend) -> Intpend>(&self, f: F) -> &Self {
     let tmp = self.intpend();
     self.set_intpend(f(tmp))
  }

  #[inline] pub fn intstatus_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
  #[inline] pub fn intstatus_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
  #[inline] pub fn intstatus(&self) -> Intstatus { 
     unsafe {
        Intstatus(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }

  #[inline] pub fn pendch_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2c) as *const u32
  }
  #[inline] pub fn pendch_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2c) as *mut u32
  }
  #[inline] pub fn pendch(&self) -> Pendch { 
     unsafe {
        Pendch(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
     }
  }

  #[inline] pub fn prictrl0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
  #[inline] pub fn prictrl0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
  #[inline] pub fn prictrl0(&self) -> Prictrl0 { 
     unsafe {
        Prictrl0(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
  #[inline] pub fn set_prictrl0(&self, value: Prictrl0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_prictrl0<F: FnOnce(Prictrl0) -> Prictrl0>(&self, f: F) -> &Self {
     let tmp = self.prictrl0();
     self.set_prictrl0(f(tmp))
  }

  #[inline] pub fn swtrigctrl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
  #[inline] pub fn swtrigctrl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
  #[inline] pub fn swtrigctrl(&self) -> Swtrigctrl { 
     unsafe {
        Swtrigctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  #[inline] pub fn set_swtrigctrl(&self, value: Swtrigctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_swtrigctrl<F: FnOnce(Swtrigctrl) -> Swtrigctrl>(&self, f: F) -> &Self {
     let tmp = self.swtrigctrl();
     self.set_swtrigctrl(f(tmp))
  }

  #[inline] pub fn wrbaddr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x38) as *const u32
  }
  #[inline] pub fn wrbaddr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x38) as *mut u32
  }
  #[inline] pub fn wrbaddr(&self) -> Wrbaddr { 
     unsafe {
        Wrbaddr(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
     }
  }
  #[inline] pub fn set_wrbaddr(&self, value: Wrbaddr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x38) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_wrbaddr<F: FnOnce(Wrbaddr) -> Wrbaddr>(&self, f: F) -> &Self {
     let tmp = self.wrbaddr();
     self.set_wrbaddr(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Active(pub u32);
impl Active {
  #[inline] pub fn lvlex0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_lvlex0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn lvlex1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_lvlex1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn lvlex2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_lvlex2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn lvlex3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_lvlex3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn id(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1f // [12:8]
  }
  #[inline] pub fn set_id(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn abusy(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline] pub fn set_abusy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline] pub fn btcnt(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
  #[inline] pub fn set_btcnt(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Active {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Active {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lvlex0() != 0 { try!(write!(f, " lvlex0"))}
      if self.lvlex1() != 0 { try!(write!(f, " lvlex1"))}
      if self.lvlex2() != 0 { try!(write!(f, " lvlex2"))}
      if self.lvlex3() != 0 { try!(write!(f, " lvlex3"))}
      if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
      if self.abusy() != 0 { try!(write!(f, " abusy"))}
      if self.btcnt() != 0 { try!(write!(f, " btcnt=0x{:x}", self.btcnt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Baseaddr(pub u32);
impl Baseaddr {
  #[inline] pub fn baseaddr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_baseaddr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Baseaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Baseaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Busych(pub u32);
impl Busych {
  #[inline] pub fn busych0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_busych0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn busych1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_busych1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn busych2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_busych2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn busych3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_busych3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn busych4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_busych4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn busych5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_busych5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn busych6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_busych6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn busych7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_busych7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn busych8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_busych8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn busych9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_busych9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn busych10(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_busych10(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn busych11(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_busych11(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

}
impl ::core::fmt::Display for Busych {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Busych {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.busych0() != 0 { try!(write!(f, " busych0"))}
      if self.busych1() != 0 { try!(write!(f, " busych1"))}
      if self.busych2() != 0 { try!(write!(f, " busych2"))}
      if self.busych3() != 0 { try!(write!(f, " busych3"))}
      if self.busych4() != 0 { try!(write!(f, " busych4"))}
      if self.busych5() != 0 { try!(write!(f, " busych5"))}
      if self.busych6() != 0 { try!(write!(f, " busych6"))}
      if self.busych7() != 0 { try!(write!(f, " busych7"))}
      if self.busych8() != 0 { try!(write!(f, " busych8"))}
      if self.busych9() != 0 { try!(write!(f, " busych9"))}
      if self.busych10() != 0 { try!(write!(f, " busych10"))}
      if self.busych11() != 0 { try!(write!(f, " busych11"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Chctrla(pub u8);
impl Chctrla {
  #[inline] pub fn swrst(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_swrst(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn enable(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_enable(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}
impl ::core::fmt::Display for Chctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chctrla {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.enable() != 0 { try!(write!(f, " enable"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Chctrlb(pub u32);
impl Chctrlb {
  #[inline] pub fn evact(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  #[inline] pub fn set_evact(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn evie(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_evie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn evoe(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_evoe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn lvl(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x3 // [6:5]
  }
  #[inline] pub fn set_lvl(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn trigsrc(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3f // [13:8]
  }
  #[inline] pub fn set_trigsrc(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn trigact(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x3 // [23:22]
  }
  #[inline] pub fn set_trigact(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 22);
     self.0 |= value << 22;
     self
  }

  #[inline] pub fn cmd(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x3 // [25:24]
  }
  #[inline] pub fn set_cmd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Chctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chctrlb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.evact() != 0 { try!(write!(f, " evact=0x{:x}", self.evact()))}
      if self.evie() != 0 { try!(write!(f, " evie"))}
      if self.evoe() != 0 { try!(write!(f, " evoe"))}
      if self.lvl() != 0 { try!(write!(f, " lvl=0x{:x}", self.lvl()))}
      if self.trigsrc() != 0 { try!(write!(f, " trigsrc=0x{:x}", self.trigsrc()))}
      if self.trigact() != 0 { try!(write!(f, " trigact=0x{:x}", self.trigact()))}
      if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Chid(pub u8);
impl Chid {
  #[inline] pub fn id(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_id(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Chid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Chintenclr(pub u8);
impl Chintenclr {
  #[inline] pub fn terr(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_terr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn tcmpl(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_tcmpl(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn susp(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_susp(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

}
impl ::core::fmt::Display for Chintenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chintenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.terr() != 0 { try!(write!(f, " terr"))}
      if self.tcmpl() != 0 { try!(write!(f, " tcmpl"))}
      if self.susp() != 0 { try!(write!(f, " susp"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Chintenset(pub u8);
impl Chintenset {
  #[inline] pub fn terr(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_terr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn tcmpl(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_tcmpl(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn susp(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_susp(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

}
impl ::core::fmt::Display for Chintenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chintenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.terr() != 0 { try!(write!(f, " terr"))}
      if self.tcmpl() != 0 { try!(write!(f, " tcmpl"))}
      if self.susp() != 0 { try!(write!(f, " susp"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Chintflag(pub u8);
impl Chintflag {
  #[inline] pub fn terr(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_terr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn tcmpl(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_tcmpl(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn susp(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_susp(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

}
impl ::core::fmt::Display for Chintflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chintflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.terr() != 0 { try!(write!(f, " terr"))}
      if self.tcmpl() != 0 { try!(write!(f, " tcmpl"))}
      if self.susp() != 0 { try!(write!(f, " susp"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Chstatus(pub u8);
impl Chstatus {
  #[inline] pub fn pend(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pend(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn busy(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_busy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn ferr(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_ferr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

}
impl ::core::fmt::Display for Chstatus {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chstatus {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pend() != 0 { try!(write!(f, " pend"))}
      if self.busy() != 0 { try!(write!(f, " busy"))}
      if self.ferr() != 0 { try!(write!(f, " ferr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Crcchksum(pub u32);
impl Crcchksum {
  #[inline] pub fn crcchksum(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_crcchksum(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Crcchksum {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Crcchksum {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Crcctrl(pub u16);
impl Crcctrl {
  #[inline] pub fn crcbeatsize(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x3 // [1:0]
  }
  #[inline] pub fn set_crcbeatsize(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn crcpoly(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x3 // [3:2]
  }
  #[inline] pub fn set_crcpoly(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn crcsrc(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x3f // [13:8]
  }
  #[inline] pub fn set_crcsrc(mut self, value: u16) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Crcctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Crcctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.crcbeatsize() != 0 { try!(write!(f, " crcbeatsize=0x{:x}", self.crcbeatsize()))}
      if self.crcpoly() != 0 { try!(write!(f, " crcpoly=0x{:x}", self.crcpoly()))}
      if self.crcsrc() != 0 { try!(write!(f, " crcsrc=0x{:x}", self.crcsrc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Crcdatain(pub u32);
impl Crcdatain {
  #[inline] pub fn crcdatain(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_crcdatain(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Crcdatain {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Crcdatain {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Crcstatus(pub u8);
impl Crcstatus {
  #[inline] pub fn crcbusy(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_crcbusy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn crczero(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_crczero(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}
impl ::core::fmt::Display for Crcstatus {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Crcstatus {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.crcbusy() != 0 { try!(write!(f, " crcbusy"))}
      if self.crczero() != 0 { try!(write!(f, " crczero"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ctrl(pub u16);
impl Ctrl {
  #[inline] pub fn swrst(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_swrst(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn dmaenable(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_dmaenable(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn crcenable(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_crcenable(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn lvlen0(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_lvlen0(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn lvlen1(&self) -> u16 {
     ((self.0 as u16) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_lvlen1(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn lvlen2(&self) -> u16 {
     ((self.0 as u16) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_lvlen2(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn lvlen3(&self) -> u16 {
     ((self.0 as u16) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_lvlen3(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
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
      if self.swrst() != 0 { try!(write!(f, " swrst"))}
      if self.dmaenable() != 0 { try!(write!(f, " dmaenable"))}
      if self.crcenable() != 0 { try!(write!(f, " crcenable"))}
      if self.lvlen0() != 0 { try!(write!(f, " lvlen0"))}
      if self.lvlen1() != 0 { try!(write!(f, " lvlen1"))}
      if self.lvlen2() != 0 { try!(write!(f, " lvlen2"))}
      if self.lvlen3() != 0 { try!(write!(f, " lvlen3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
  #[inline] pub fn dbgrun(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_dbgrun(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dbgctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbgrun() != 0 { try!(write!(f, " dbgrun"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Intpend(pub u16);
impl Intpend {
  #[inline] pub fn id(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_id(mut self, value: u16) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn terr(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_terr(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn tcmpl(&self) -> u16 {
     ((self.0 as u16) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_tcmpl(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn susp(&self) -> u16 {
     ((self.0 as u16) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_susp(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn ferr(&self) -> u16 {
     ((self.0 as u16) >> 13) & 0x1 // [13]
  }
  #[inline] pub fn set_ferr(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline] pub fn busy(&self) -> u16 {
     ((self.0 as u16) >> 14) & 0x1 // [14]
  }
  #[inline] pub fn set_busy(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline] pub fn pend(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
  #[inline] pub fn set_pend(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

}
impl ::core::fmt::Display for Intpend {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intpend {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
      if self.terr() != 0 { try!(write!(f, " terr"))}
      if self.tcmpl() != 0 { try!(write!(f, " tcmpl"))}
      if self.susp() != 0 { try!(write!(f, " susp"))}
      if self.ferr() != 0 { try!(write!(f, " ferr"))}
      if self.busy() != 0 { try!(write!(f, " busy"))}
      if self.pend() != 0 { try!(write!(f, " pend"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Intstatus(pub u32);
impl Intstatus {
  #[inline] pub fn chint0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_chint0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn chint1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_chint1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn chint2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_chint2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn chint3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_chint3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn chint4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_chint4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn chint5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_chint5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn chint6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_chint6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn chint7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_chint7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn chint8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_chint8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn chint9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_chint9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn chint10(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_chint10(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn chint11(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_chint11(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

}
impl ::core::fmt::Display for Intstatus {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intstatus {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chint0() != 0 { try!(write!(f, " chint0"))}
      if self.chint1() != 0 { try!(write!(f, " chint1"))}
      if self.chint2() != 0 { try!(write!(f, " chint2"))}
      if self.chint3() != 0 { try!(write!(f, " chint3"))}
      if self.chint4() != 0 { try!(write!(f, " chint4"))}
      if self.chint5() != 0 { try!(write!(f, " chint5"))}
      if self.chint6() != 0 { try!(write!(f, " chint6"))}
      if self.chint7() != 0 { try!(write!(f, " chint7"))}
      if self.chint8() != 0 { try!(write!(f, " chint8"))}
      if self.chint9() != 0 { try!(write!(f, " chint9"))}
      if self.chint10() != 0 { try!(write!(f, " chint10"))}
      if self.chint11() != 0 { try!(write!(f, " chint11"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Pendch(pub u32);
impl Pendch {
  #[inline] pub fn pendch0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_pendch0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn pendch1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_pendch1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn pendch2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_pendch2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn pendch3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_pendch3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn pendch4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_pendch4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn pendch5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_pendch5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn pendch6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_pendch6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn pendch7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_pendch7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn pendch8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_pendch8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn pendch9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_pendch9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn pendch10(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_pendch10(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn pendch11(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_pendch11(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

}
impl ::core::fmt::Display for Pendch {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pendch {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pendch0() != 0 { try!(write!(f, " pendch0"))}
      if self.pendch1() != 0 { try!(write!(f, " pendch1"))}
      if self.pendch2() != 0 { try!(write!(f, " pendch2"))}
      if self.pendch3() != 0 { try!(write!(f, " pendch3"))}
      if self.pendch4() != 0 { try!(write!(f, " pendch4"))}
      if self.pendch5() != 0 { try!(write!(f, " pendch5"))}
      if self.pendch6() != 0 { try!(write!(f, " pendch6"))}
      if self.pendch7() != 0 { try!(write!(f, " pendch7"))}
      if self.pendch8() != 0 { try!(write!(f, " pendch8"))}
      if self.pendch9() != 0 { try!(write!(f, " pendch9"))}
      if self.pendch10() != 0 { try!(write!(f, " pendch10"))}
      if self.pendch11() != 0 { try!(write!(f, " pendch11"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Prictrl0(pub u32);
impl Prictrl0 {
  #[inline] pub fn lvlpri0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_lvlpri0(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn rrlvlen0(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_rrlvlen0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn lvlpri1(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  #[inline] pub fn set_lvlpri1(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn rrlvlen1(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline] pub fn set_rrlvlen1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline] pub fn lvlpri2(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  #[inline] pub fn set_lvlpri2(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn rrlvlen2(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  #[inline] pub fn set_rrlvlen2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  #[inline] pub fn lvlpri3(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
  #[inline] pub fn set_lvlpri3(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

  #[inline] pub fn rrlvlen3(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline] pub fn set_rrlvlen3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Prictrl0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prictrl0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lvlpri0() != 0 { try!(write!(f, " lvlpri0=0x{:x}", self.lvlpri0()))}
      if self.rrlvlen0() != 0 { try!(write!(f, " rrlvlen0"))}
      if self.lvlpri1() != 0 { try!(write!(f, " lvlpri1=0x{:x}", self.lvlpri1()))}
      if self.rrlvlen1() != 0 { try!(write!(f, " rrlvlen1"))}
      if self.lvlpri2() != 0 { try!(write!(f, " lvlpri2=0x{:x}", self.lvlpri2()))}
      if self.rrlvlen2() != 0 { try!(write!(f, " rrlvlen2"))}
      if self.lvlpri3() != 0 { try!(write!(f, " lvlpri3=0x{:x}", self.lvlpri3()))}
      if self.rrlvlen3() != 0 { try!(write!(f, " rrlvlen3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Swtrigctrl(pub u32);
impl Swtrigctrl {
  #[inline] pub fn swtrig0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_swtrig0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn swtrig1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_swtrig1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn swtrig2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_swtrig2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn swtrig3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_swtrig3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn swtrig4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_swtrig4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn swtrig5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_swtrig5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn swtrig6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_swtrig6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn swtrig7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_swtrig7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn swtrig8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_swtrig8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn swtrig9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_swtrig9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn swtrig10(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_swtrig10(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn swtrig11(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_swtrig11(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

}
impl ::core::fmt::Display for Swtrigctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Swtrigctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swtrig0() != 0 { try!(write!(f, " swtrig0"))}
      if self.swtrig1() != 0 { try!(write!(f, " swtrig1"))}
      if self.swtrig2() != 0 { try!(write!(f, " swtrig2"))}
      if self.swtrig3() != 0 { try!(write!(f, " swtrig3"))}
      if self.swtrig4() != 0 { try!(write!(f, " swtrig4"))}
      if self.swtrig5() != 0 { try!(write!(f, " swtrig5"))}
      if self.swtrig6() != 0 { try!(write!(f, " swtrig6"))}
      if self.swtrig7() != 0 { try!(write!(f, " swtrig7"))}
      if self.swtrig8() != 0 { try!(write!(f, " swtrig8"))}
      if self.swtrig9() != 0 { try!(write!(f, " swtrig9"))}
      if self.swtrig10() != 0 { try!(write!(f, " swtrig10"))}
      if self.swtrig11() != 0 { try!(write!(f, " swtrig11"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Wrbaddr(pub u32);
impl Wrbaddr {
  #[inline] pub fn wrbaddr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_wrbaddr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Wrbaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wrbaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

pub struct Transfer(pub [u8; 16]);

impl Transfer {
   #[inline] pub fn btctrl(&self) -> Btctrl { 
      unsafe {
         Btctrl(::core::ptr::read_volatile(self.0.as_ptr().offset(0x0) as *const u16))
      }
   }
   #[inline] pub fn set_btctrl(&mut self, value: Btctrl) -> &mut Self {
      unsafe {
         ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x0) as *mut u16, value.0);
      }
      self
  }
   #[inline] pub fn with_btctrl<F: FnOnce(Btctrl) -> Btctrl>(&mut self, f: F) -> &mut Self {
      let tmp = self.btctrl();
      self.set_btctrl(f(tmp))
   }

   #[inline] pub fn btcnt(&self) -> Btcnt { 
      unsafe {
         Btcnt(::core::ptr::read_volatile(self.0.as_ptr().offset(0x2) as *const u16))
      }
   }
   #[inline] pub fn set_btcnt(&mut self, value: Btcnt) -> &mut Self {
      unsafe {
         ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x2) as *mut u16, value.0);
      }
      self
  }
   #[inline] pub fn with_btcnt<F: FnOnce(Btcnt) -> Btcnt>(&mut self, f: F) -> &mut Self {
      let tmp = self.btcnt();
      self.set_btcnt(f(tmp))
   }

   #[inline] pub fn srcaddr(&self) -> Srcaddr { 
      unsafe {
         Srcaddr(::core::ptr::read_volatile(self.0.as_ptr().offset(0x4) as *const u32))
      }
   }
   #[inline] pub fn set_srcaddr(&mut self, value: Srcaddr) -> &mut Self {
      unsafe {
         ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x4) as *mut u32, value.0);
      }
      self
  }
   #[inline] pub fn with_srcaddr<F: FnOnce(Srcaddr) -> Srcaddr>(&mut self, f: F) -> &mut Self {
      let tmp = self.srcaddr();
      self.set_srcaddr(f(tmp))
   }

   #[inline] pub fn dstaddr(&self) -> Dstaddr { 
      unsafe {
         Dstaddr(::core::ptr::read_volatile(self.0.as_ptr().offset(0x8) as *const u32))
      }
   }
   #[inline] pub fn set_dstaddr(&mut self, value: Dstaddr) -> &mut Self {
      unsafe {
         ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x8) as *mut u32, value.0);
      }
      self
  }
   #[inline] pub fn with_dstaddr<F: FnOnce(Dstaddr) -> Dstaddr>(&mut self, f: F) -> &mut Self {
      let tmp = self.dstaddr();
      self.set_dstaddr(f(tmp))
   }

   #[inline] pub fn descaddr(&self) -> Descaddr { 
      unsafe {
         Descaddr(::core::ptr::read_volatile(self.0.as_ptr().offset(0xc) as *const u32))
      }
   }
   #[inline] pub fn set_descaddr(&mut self, value: Descaddr) -> &mut Self {
      unsafe {
         ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0xc) as *mut u32, value.0);
      }
      self
  }
   #[inline] pub fn with_descaddr<F: FnOnce(Descaddr) -> Descaddr>(&mut self, f: F) -> &mut Self {
      let tmp = self.descaddr();
      self.set_descaddr(f(tmp))
   }

}
#[derive(PartialEq, Eq)]
pub struct Btctrl(pub u16);
impl Btctrl {
  #[inline] pub fn valid(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_valid(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn evosel(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x3 // [2:1]
  }
  #[inline] pub fn set_evosel(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn blockact(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x3 // [4:3]
  }
  #[inline] pub fn set_blockact(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn beatsize(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x3 // [9:8]
  }
  #[inline] pub fn set_beatsize(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn srcinc(&self) -> u16 {
     ((self.0 as u16) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_srcinc(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn dstinc(&self) -> u16 {
     ((self.0 as u16) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_dstinc(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn stepsel(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_stepsel(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn stepsize(&self) -> u16 {
     ((self.0 as u16) >> 13) & 0x7 // [15:13]
  }
  #[inline] pub fn set_stepsize(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 13);
     self.0 |= value << 13;
     self
  }

}
impl ::core::fmt::Display for Btctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Btctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.valid() != 0 { try!(write!(f, " valid"))}
      if self.evosel() != 0 { try!(write!(f, " evosel=0x{:x}", self.evosel()))}
      if self.blockact() != 0 { try!(write!(f, " blockact=0x{:x}", self.blockact()))}
      if self.beatsize() != 0 { try!(write!(f, " beatsize=0x{:x}", self.beatsize()))}
      if self.srcinc() != 0 { try!(write!(f, " srcinc"))}
      if self.dstinc() != 0 { try!(write!(f, " dstinc"))}
      if self.stepsel() != 0 { try!(write!(f, " stepsel"))}
      if self.stepsize() != 0 { try!(write!(f, " stepsize=0x{:x}", self.stepsize()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Btcnt(pub u16);
impl Btcnt {
  #[inline] pub fn btcnt(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_btcnt(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Btcnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Btcnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.btcnt() != 0 { try!(write!(f, " btcnt=0x{:x}", self.btcnt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Srcaddr(pub u32);
impl Srcaddr {
  #[inline] pub fn srcaddr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_srcaddr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Srcaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srcaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dstaddr(pub u32);
impl Dstaddr {
  #[inline] pub fn dstaddr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_dstaddr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dstaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dstaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Descaddr(pub u32);
impl Descaddr {
  #[inline] pub fn descaddr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_descaddr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Descaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Descaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
