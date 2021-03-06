
::bobbin_mcu::periph!( SYSCFG, Syscfg, SYSCFG_PERIPH, SyscfgPeriph, SYSCFG_OWNED, SYSCFG_REF_COUNT, 0x40013800, 0x00, 0x03);


#[doc="System configuration controller"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SyscfgPeriph(pub usize);
impl SyscfgPeriph {
    #[doc="Get the MEMRM Register."]
    #[inline] pub fn memrm_reg(&self) -> ::bobbin_mcu::register::Register<Memrm> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Memrm, 0x0)
    }

    #[doc="Get the *mut pointer for the MEMRM register."]
    #[inline] pub fn memrm_mut(&self) -> *mut Memrm { 
        self.memrm_reg().ptr()
    }

    #[doc="Get the *const pointer for the MEMRM register."]
    #[inline] pub fn memrm_ptr(&self) -> *const Memrm { 
        self.memrm_reg().ptr()
    }

    #[doc="Read the MEMRM register."]
    #[inline] pub fn memrm(&self) -> Memrm { 
        self.memrm_reg().read()
    }

    #[doc="Write the MEMRM register."]
    #[inline] pub fn write_memrm(&self, value: Memrm) -> &Self { 
        self.memrm_reg().write(value);
        self
    }

    #[doc="Set the MEMRM register."]
    #[inline] pub fn set_memrm<F: FnOnce(Memrm) -> Memrm>(&self, f: F) -> &Self {
        self.memrm_reg().set(f);
        self
    }

    #[doc="Modify the MEMRM register."]
    #[inline] pub fn with_memrm<F: FnOnce(Memrm) -> Memrm>(&self, f: F) -> &Self {
        self.memrm_reg().with(f);
        self
    }

    #[doc="Get the PMC Register."]
    #[inline] pub fn pmc_reg(&self) -> ::bobbin_mcu::register::Register<Pmc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pmc, 0x4)
    }

    #[doc="Get the *mut pointer for the PMC register."]
    #[inline] pub fn pmc_mut(&self) -> *mut Pmc { 
        self.pmc_reg().ptr()
    }

    #[doc="Get the *const pointer for the PMC register."]
    #[inline] pub fn pmc_ptr(&self) -> *const Pmc { 
        self.pmc_reg().ptr()
    }

    #[doc="Read the PMC register."]
    #[inline] pub fn pmc(&self) -> Pmc { 
        self.pmc_reg().read()
    }

    #[doc="Write the PMC register."]
    #[inline] pub fn write_pmc(&self, value: Pmc) -> &Self { 
        self.pmc_reg().write(value);
        self
    }

    #[doc="Set the PMC register."]
    #[inline] pub fn set_pmc<F: FnOnce(Pmc) -> Pmc>(&self, f: F) -> &Self {
        self.pmc_reg().set(f);
        self
    }

    #[doc="Modify the PMC register."]
    #[inline] pub fn with_pmc<F: FnOnce(Pmc) -> Pmc>(&self, f: F) -> &Self {
        self.pmc_reg().with(f);
        self
    }

    #[doc="Get the EXTICR1 Register."]
    #[inline] pub fn exticr1_reg(&self) -> ::bobbin_mcu::register::Register<Exticr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Exticr1, 0x8)
    }

    #[doc="Get the *mut pointer for the EXTICR1 register."]
    #[inline] pub fn exticr1_mut(&self) -> *mut Exticr1 { 
        self.exticr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the EXTICR1 register."]
    #[inline] pub fn exticr1_ptr(&self) -> *const Exticr1 { 
        self.exticr1_reg().ptr()
    }

    #[doc="Read the EXTICR1 register."]
    #[inline] pub fn exticr1(&self) -> Exticr1 { 
        self.exticr1_reg().read()
    }

    #[doc="Write the EXTICR1 register."]
    #[inline] pub fn write_exticr1(&self, value: Exticr1) -> &Self { 
        self.exticr1_reg().write(value);
        self
    }

    #[doc="Set the EXTICR1 register."]
    #[inline] pub fn set_exticr1<F: FnOnce(Exticr1) -> Exticr1>(&self, f: F) -> &Self {
        self.exticr1_reg().set(f);
        self
    }

    #[doc="Modify the EXTICR1 register."]
    #[inline] pub fn with_exticr1<F: FnOnce(Exticr1) -> Exticr1>(&self, f: F) -> &Self {
        self.exticr1_reg().with(f);
        self
    }

    #[doc="Get the EXTICR2 Register."]
    #[inline] pub fn exticr2_reg(&self) -> ::bobbin_mcu::register::Register<Exticr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Exticr2, 0xc)
    }

    #[doc="Get the *mut pointer for the EXTICR2 register."]
    #[inline] pub fn exticr2_mut(&self) -> *mut Exticr2 { 
        self.exticr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the EXTICR2 register."]
    #[inline] pub fn exticr2_ptr(&self) -> *const Exticr2 { 
        self.exticr2_reg().ptr()
    }

    #[doc="Read the EXTICR2 register."]
    #[inline] pub fn exticr2(&self) -> Exticr2 { 
        self.exticr2_reg().read()
    }

    #[doc="Write the EXTICR2 register."]
    #[inline] pub fn write_exticr2(&self, value: Exticr2) -> &Self { 
        self.exticr2_reg().write(value);
        self
    }

    #[doc="Set the EXTICR2 register."]
    #[inline] pub fn set_exticr2<F: FnOnce(Exticr2) -> Exticr2>(&self, f: F) -> &Self {
        self.exticr2_reg().set(f);
        self
    }

    #[doc="Modify the EXTICR2 register."]
    #[inline] pub fn with_exticr2<F: FnOnce(Exticr2) -> Exticr2>(&self, f: F) -> &Self {
        self.exticr2_reg().with(f);
        self
    }

    #[doc="Get the EXTICR3 Register."]
    #[inline] pub fn exticr3_reg(&self) -> ::bobbin_mcu::register::Register<Exticr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Exticr3, 0x10)
    }

    #[doc="Get the *mut pointer for the EXTICR3 register."]
    #[inline] pub fn exticr3_mut(&self) -> *mut Exticr3 { 
        self.exticr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the EXTICR3 register."]
    #[inline] pub fn exticr3_ptr(&self) -> *const Exticr3 { 
        self.exticr3_reg().ptr()
    }

    #[doc="Read the EXTICR3 register."]
    #[inline] pub fn exticr3(&self) -> Exticr3 { 
        self.exticr3_reg().read()
    }

    #[doc="Write the EXTICR3 register."]
    #[inline] pub fn write_exticr3(&self, value: Exticr3) -> &Self { 
        self.exticr3_reg().write(value);
        self
    }

    #[doc="Set the EXTICR3 register."]
    #[inline] pub fn set_exticr3<F: FnOnce(Exticr3) -> Exticr3>(&self, f: F) -> &Self {
        self.exticr3_reg().set(f);
        self
    }

    #[doc="Modify the EXTICR3 register."]
    #[inline] pub fn with_exticr3<F: FnOnce(Exticr3) -> Exticr3>(&self, f: F) -> &Self {
        self.exticr3_reg().with(f);
        self
    }

    #[doc="Get the EXTICR4 Register."]
    #[inline] pub fn exticr4_reg(&self) -> ::bobbin_mcu::register::Register<Exticr4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Exticr4, 0x14)
    }

    #[doc="Get the *mut pointer for the EXTICR4 register."]
    #[inline] pub fn exticr4_mut(&self) -> *mut Exticr4 { 
        self.exticr4_reg().ptr()
    }

    #[doc="Get the *const pointer for the EXTICR4 register."]
    #[inline] pub fn exticr4_ptr(&self) -> *const Exticr4 { 
        self.exticr4_reg().ptr()
    }

    #[doc="Read the EXTICR4 register."]
    #[inline] pub fn exticr4(&self) -> Exticr4 { 
        self.exticr4_reg().read()
    }

    #[doc="Write the EXTICR4 register."]
    #[inline] pub fn write_exticr4(&self, value: Exticr4) -> &Self { 
        self.exticr4_reg().write(value);
        self
    }

    #[doc="Set the EXTICR4 register."]
    #[inline] pub fn set_exticr4<F: FnOnce(Exticr4) -> Exticr4>(&self, f: F) -> &Self {
        self.exticr4_reg().set(f);
        self
    }

    #[doc="Modify the EXTICR4 register."]
    #[inline] pub fn with_exticr4<F: FnOnce(Exticr4) -> Exticr4>(&self, f: F) -> &Self {
        self.exticr4_reg().with(f);
        self
    }

    #[doc="Get the CMPCR Register."]
    #[inline] pub fn cmpcr_reg(&self) -> ::bobbin_mcu::register::Register<Cmpcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cmpcr, 0x20)
    }

    #[doc="Get the *mut pointer for the CMPCR register."]
    #[inline] pub fn cmpcr_mut(&self) -> *mut Cmpcr { 
        self.cmpcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CMPCR register."]
    #[inline] pub fn cmpcr_ptr(&self) -> *const Cmpcr { 
        self.cmpcr_reg().ptr()
    }

    #[doc="Read the CMPCR register."]
    #[inline] pub fn cmpcr(&self) -> Cmpcr { 
        self.cmpcr_reg().read()
    }

}

#[doc="memory remap register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Memrm(pub u32);
impl Memrm {
    #[doc="MEM_MODE"]
    #[inline] pub fn mem_mode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if MEM_MODE != 0"]
    #[inline] pub fn test_mem_mode(&self) -> bool {
        self.mem_mode() != 0
    }

    #[doc="Sets the MEM_MODE field."]
    #[inline] pub fn set_mem_mode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Memrm {
    #[inline]
    fn from(other: u32) -> Self {
         Memrm(other)
    }
}

impl ::core::fmt::Display for Memrm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Memrm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mem_mode() != 0 { try!(write!(f, " mem_mode=0x{:x}", self.mem_mode()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="peripheral mode configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pmc(pub u32);
impl Pmc {
    #[doc="Ethernet PHY interface selection"]
    #[inline] pub fn mii_rmii_sel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if MII_RMII_SEL != 0"]
    #[inline] pub fn test_mii_rmii_sel(&self) -> bool {
        self.mii_rmii_sel() != 0
    }

    #[doc="Sets the MII_RMII_SEL field."]
    #[inline] pub fn set_mii_rmii_sel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

}

impl From<u32> for Pmc {
    #[inline]
    fn from(other: u32) -> Self {
         Pmc(other)
    }
}

impl ::core::fmt::Display for Pmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mii_rmii_sel() != 0 { try!(write!(f, " mii_rmii_sel"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="external interrupt configuration register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr1(pub u32);
impl Exticr1 {
    #[doc="EXTI x configuration (x = 0 to 3)"]
    #[inline] pub fn exti3(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if EXTI3 != 0"]
    #[inline] pub fn test_exti3(&self) -> bool {
        self.exti3() != 0
    }

    #[doc="Sets the EXTI3 field."]
    #[inline] pub fn set_exti3<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTI x configuration (x = 0 to 3)"]
    #[inline] pub fn exti2(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if EXTI2 != 0"]
    #[inline] pub fn test_exti2(&self) -> bool {
        self.exti2() != 0
    }

    #[doc="Sets the EXTI2 field."]
    #[inline] pub fn set_exti2<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI x configuration (x = 0 to 3)"]
    #[inline] pub fn exti1(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if EXTI1 != 0"]
    #[inline] pub fn test_exti1(&self) -> bool {
        self.exti1() != 0
    }

    #[doc="Sets the EXTI1 field."]
    #[inline] pub fn set_exti1<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI x configuration (x = 0 to 3)"]
    #[inline] pub fn exti0(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EXTI0 != 0"]
    #[inline] pub fn test_exti0(&self) -> bool {
        self.exti0() != 0
    }

    #[doc="Sets the EXTI0 field."]
    #[inline] pub fn set_exti0<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Exticr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr1(other)
    }
}

impl ::core::fmt::Display for Exticr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti3() != 0 { try!(write!(f, " exti3=0x{:x}", self.exti3()))}
        if self.exti2() != 0 { try!(write!(f, " exti2=0x{:x}", self.exti2()))}
        if self.exti1() != 0 { try!(write!(f, " exti1=0x{:x}", self.exti1()))}
        if self.exti0() != 0 { try!(write!(f, " exti0=0x{:x}", self.exti0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="external interrupt configuration register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr2(pub u32);
impl Exticr2 {
    #[doc="EXTI x configuration (x = 4 to 7)"]
    #[inline] pub fn exti7(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if EXTI7 != 0"]
    #[inline] pub fn test_exti7(&self) -> bool {
        self.exti7() != 0
    }

    #[doc="Sets the EXTI7 field."]
    #[inline] pub fn set_exti7<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTI x configuration (x = 4 to 7)"]
    #[inline] pub fn exti6(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if EXTI6 != 0"]
    #[inline] pub fn test_exti6(&self) -> bool {
        self.exti6() != 0
    }

    #[doc="Sets the EXTI6 field."]
    #[inline] pub fn set_exti6<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI x configuration (x = 4 to 7)"]
    #[inline] pub fn exti5(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if EXTI5 != 0"]
    #[inline] pub fn test_exti5(&self) -> bool {
        self.exti5() != 0
    }

    #[doc="Sets the EXTI5 field."]
    #[inline] pub fn set_exti5<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI x configuration (x = 4 to 7)"]
    #[inline] pub fn exti4(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EXTI4 != 0"]
    #[inline] pub fn test_exti4(&self) -> bool {
        self.exti4() != 0
    }

    #[doc="Sets the EXTI4 field."]
    #[inline] pub fn set_exti4<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Exticr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr2(other)
    }
}

impl ::core::fmt::Display for Exticr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti7() != 0 { try!(write!(f, " exti7=0x{:x}", self.exti7()))}
        if self.exti6() != 0 { try!(write!(f, " exti6=0x{:x}", self.exti6()))}
        if self.exti5() != 0 { try!(write!(f, " exti5=0x{:x}", self.exti5()))}
        if self.exti4() != 0 { try!(write!(f, " exti4=0x{:x}", self.exti4()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="external interrupt configuration register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr3(pub u32);
impl Exticr3 {
    #[doc="EXTI x configuration (x = 8 to 11)"]
    #[inline] pub fn exti11(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if EXTI11 != 0"]
    #[inline] pub fn test_exti11(&self) -> bool {
        self.exti11() != 0
    }

    #[doc="Sets the EXTI11 field."]
    #[inline] pub fn set_exti11<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTI10"]
    #[inline] pub fn exti10(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if EXTI10 != 0"]
    #[inline] pub fn test_exti10(&self) -> bool {
        self.exti10() != 0
    }

    #[doc="Sets the EXTI10 field."]
    #[inline] pub fn set_exti10<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI x configuration (x = 8 to 11)"]
    #[inline] pub fn exti9(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if EXTI9 != 0"]
    #[inline] pub fn test_exti9(&self) -> bool {
        self.exti9() != 0
    }

    #[doc="Sets the EXTI9 field."]
    #[inline] pub fn set_exti9<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI x configuration (x = 8 to 11)"]
    #[inline] pub fn exti8(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EXTI8 != 0"]
    #[inline] pub fn test_exti8(&self) -> bool {
        self.exti8() != 0
    }

    #[doc="Sets the EXTI8 field."]
    #[inline] pub fn set_exti8<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Exticr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr3(other)
    }
}

impl ::core::fmt::Display for Exticr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti11() != 0 { try!(write!(f, " exti11=0x{:x}", self.exti11()))}
        if self.exti10() != 0 { try!(write!(f, " exti10=0x{:x}", self.exti10()))}
        if self.exti9() != 0 { try!(write!(f, " exti9=0x{:x}", self.exti9()))}
        if self.exti8() != 0 { try!(write!(f, " exti8=0x{:x}", self.exti8()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="external interrupt configuration register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr4(pub u32);
impl Exticr4 {
    #[doc="EXTI x configuration (x = 12 to 15)"]
    #[inline] pub fn exti15(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if EXTI15 != 0"]
    #[inline] pub fn test_exti15(&self) -> bool {
        self.exti15() != 0
    }

    #[doc="Sets the EXTI15 field."]
    #[inline] pub fn set_exti15<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTI x configuration (x = 12 to 15)"]
    #[inline] pub fn exti14(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if EXTI14 != 0"]
    #[inline] pub fn test_exti14(&self) -> bool {
        self.exti14() != 0
    }

    #[doc="Sets the EXTI14 field."]
    #[inline] pub fn set_exti14<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI x configuration (x = 12 to 15)"]
    #[inline] pub fn exti13(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if EXTI13 != 0"]
    #[inline] pub fn test_exti13(&self) -> bool {
        self.exti13() != 0
    }

    #[doc="Sets the EXTI13 field."]
    #[inline] pub fn set_exti13<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI x configuration (x = 12 to 15)"]
    #[inline] pub fn exti12(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EXTI12 != 0"]
    #[inline] pub fn test_exti12(&self) -> bool {
        self.exti12() != 0
    }

    #[doc="Sets the EXTI12 field."]
    #[inline] pub fn set_exti12<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Exticr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr4(other)
    }
}

impl ::core::fmt::Display for Exticr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti15() != 0 { try!(write!(f, " exti15=0x{:x}", self.exti15()))}
        if self.exti14() != 0 { try!(write!(f, " exti14=0x{:x}", self.exti14()))}
        if self.exti13() != 0 { try!(write!(f, " exti13=0x{:x}", self.exti13()))}
        if self.exti12() != 0 { try!(write!(f, " exti12=0x{:x}", self.exti12()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compensation cell control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmpcr(pub u32);
impl Cmpcr {
    #[doc="READY"]
    #[inline] pub fn ready(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if READY != 0"]
    #[inline] pub fn test_ready(&self) -> bool {
        self.ready() != 0
    }

    #[doc="Sets the READY field."]
    #[inline] pub fn set_ready<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Compensation cell power-down"]
    #[inline] pub fn cmp_pd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMP_PD != 0"]
    #[inline] pub fn test_cmp_pd(&self) -> bool {
        self.cmp_pd() != 0
    }

    #[doc="Sets the CMP_PD field."]
    #[inline] pub fn set_cmp_pd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cmpcr {
    #[inline]
    fn from(other: u32) -> Self {
         Cmpcr(other)
    }
}

impl ::core::fmt::Display for Cmpcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmpcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ready() != 0 { try!(write!(f, " ready"))}
        if self.cmp_pd() != 0 { try!(write!(f, " cmp_pd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

