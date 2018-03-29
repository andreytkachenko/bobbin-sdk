
#[allow(unused_imports)] use bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="CRC Peripheral"]
pub struct CrcPeriph(pub usize); 

impl CrcPeriph {
    #[doc="Get the *mut pointer for the DR register."]
    #[inline] pub fn dr_mut(&self) -> *mut Dr { 
        (self.0 + 0x0) as *mut Dr
    }

    #[doc="Get the *const pointer for the DR register."]
    #[inline] pub fn dr_ptr(&self) -> *const Dr { 
           self.dr_mut()
    }

    #[doc="Read the DR register."]
    #[inline] pub fn dr(&self) -> Dr { 
        unsafe {
            read_volatile(self.dr_ptr())
        }
    }

    #[doc="Write the DR register."]
    #[inline] pub fn set_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dr_mut(), f(Dr(0)));
        }
        self
    }

    #[doc="Modify the DR register."]
    #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dr_mut(), f(self.dr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IDR register."]
    #[inline] pub fn idr_mut(&self) -> *mut Idr { 
        (self.0 + 0x4) as *mut Idr
    }

    #[doc="Get the *const pointer for the IDR register."]
    #[inline] pub fn idr_ptr(&self) -> *const Idr { 
           self.idr_mut()
    }

    #[doc="Read the IDR register."]
    #[inline] pub fn idr(&self) -> Idr { 
        unsafe {
            read_volatile(self.idr_ptr())
        }
    }

    #[doc="Write the IDR register."]
    #[inline] pub fn set_idr<F: FnOnce(Idr) -> Idr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.idr_mut(), f(Idr(0)));
        }
        self
    }

    #[doc="Modify the IDR register."]
    #[inline] pub fn with_idr<F: FnOnce(Idr) -> Idr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.idr_mut(), f(self.idr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        (self.0 + 0x8) as *mut Cr
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const Cr { 
           self.cr_mut()
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        unsafe {
            read_volatile(self.cr_ptr())
        }
    }

    #[doc="Write the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr_mut(), f(Cr(0)));
        }
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr_mut(), f(self.cr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INIT register."]
    #[inline] pub fn init_mut(&self) -> *mut Init { 
        (self.0 + 0x10) as *mut Init
    }

    #[doc="Get the *const pointer for the INIT register."]
    #[inline] pub fn init_ptr(&self) -> *const Init { 
           self.init_mut()
    }

    #[doc="Read the INIT register."]
    #[inline] pub fn init(&self) -> Init { 
        unsafe {
            read_volatile(self.init_ptr())
        }
    }

    #[doc="Write the INIT register."]
    #[inline] pub fn set_init<F: FnOnce(Init) -> Init>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.init_mut(), f(Init(0)));
        }
        self
    }

    #[doc="Modify the INIT register."]
    #[inline] pub fn with_init<F: FnOnce(Init) -> Init>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.init_mut(), f(self.init()));
        }
        self
    }

    #[doc="Get the *mut pointer for the POL register."]
    #[inline] pub fn pol_mut(&self) -> *mut Pol { 
        (self.0 + 0x14) as *mut Pol
    }

    #[doc="Get the *const pointer for the POL register."]
    #[inline] pub fn pol_ptr(&self) -> *const Pol { 
           self.pol_mut()
    }

    #[doc="Read the POL register."]
    #[inline] pub fn pol(&self) -> Pol { 
        unsafe {
            read_volatile(self.pol_ptr())
        }
    }

    #[doc="Write the POL register."]
    #[inline] pub fn set_pol<F: FnOnce(Pol) -> Pol>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pol_mut(), f(Pol(0)));
        }
        self
    }

    #[doc="Modify the POL register."]
    #[inline] pub fn with_pol<F: FnOnce(Pol) -> Pol>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pol_mut(), f(self.pol()));
        }
        self
    }

}

#[doc="Data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc="Data register bits"]
    #[inline] pub fn dr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DR != 0"]
    #[inline] pub fn test_dr(&self) -> bool {
        self.dr() != 0
    }

    #[doc="Sets the DR field."]
    #[inline] pub fn set_dr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dr {
    #[inline]
    fn from(other: u32) -> Self {
         Dr(other)
    }
}

impl ::core::fmt::Display for Dr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Independent data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Idr(pub u32);
impl Idr {
    #[doc="General-purpose 8-bit data register bits"]
    #[inline] pub fn idr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if IDR != 0"]
    #[inline] pub fn test_idr(&self) -> bool {
        self.idr() != 0
    }

    #[doc="Sets the IDR field."]
    #[inline] pub fn set_idr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Idr {
    #[inline]
    fn from(other: u32) -> Self {
         Idr(other)
    }
}

impl ::core::fmt::Display for Idr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Idr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.idr() != 0 { try!(write!(f, " idr=0x{:x}", self.idr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Reverse output data"]
    #[inline] pub fn rev_out(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if REV_OUT != 0"]
    #[inline] pub fn test_rev_out(&self) -> bool {
        self.rev_out() != 0
    }

    #[doc="Sets the REV_OUT field."]
    #[inline] pub fn set_rev_out<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Reverse input data"]
    #[inline] pub fn rev_in(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if REV_IN != 0"]
    #[inline] pub fn test_rev_in(&self) -> bool {
        self.rev_in() != 0
    }

    #[doc="Sets the REV_IN field."]
    #[inline] pub fn set_rev_in<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Polynomial size"]
    #[inline] pub fn polysize(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if POLYSIZE != 0"]
    #[inline] pub fn test_polysize(&self) -> bool {
        self.polysize() != 0
    }

    #[doc="Sets the POLYSIZE field."]
    #[inline] pub fn set_polysize<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="RESET bit"]
    #[inline] pub fn _reset(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RESET != 0"]
    #[inline] pub fn test_reset(&self) -> bool {
        self._reset() != 0
    }

    #[doc="Sets the RESET field."]
    #[inline] pub fn set_reset<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr {
    #[inline]
    fn from(other: u32) -> Self {
         Cr(other)
    }
}

impl ::core::fmt::Display for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rev_out() != 0 { try!(write!(f, " rev_out"))}
        if self.rev_in() != 0 { try!(write!(f, " rev_in=0x{:x}", self.rev_in()))}
        if self.polysize() != 0 { try!(write!(f, " polysize=0x{:x}", self.polysize()))}
        if self._reset() != 0 { try!(write!(f, " _reset"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Initial CRC value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Init(pub u32);
impl Init {
    #[doc="Programmable initial CRC value"]
    #[inline] pub fn init(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if INIT != 0"]
    #[inline] pub fn test_init(&self) -> bool {
        self.init() != 0
    }

    #[doc="Sets the INIT field."]
    #[inline] pub fn set_init<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Init {
    #[inline]
    fn from(other: u32) -> Self {
         Init(other)
    }
}

impl ::core::fmt::Display for Init {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Init {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="polynomial"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pol(pub u32);
impl Pol {
    #[doc="Programmable polynomial"]
    #[inline] pub fn pol(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if POL != 0"]
    #[inline] pub fn test_pol(&self) -> bool {
        self.pol() != 0
    }

    #[doc="Sets the POL field."]
    #[inline] pub fn set_pol<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pol {
    #[inline]
    fn from(other: u32) -> Self {
         Pol(other)
    }
}

impl ::core::fmt::Display for Pol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}
