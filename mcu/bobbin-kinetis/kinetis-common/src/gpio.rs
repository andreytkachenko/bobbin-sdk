
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="GPIO Peripheral"]
pub struct GpioPeriph(pub usize); 

pub struct GpioCh { pub periph: GpioPeriph, pub index: usize }

impl GpioPeriph {
    #[doc="Get the PDOR Register."]
    #[inline] pub fn pdor_reg(&self) -> ::bobbin_mcu::register::Register<Pdor> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pdor, 0x0)
    }

    #[doc="Get the *mut pointer for the PDOR register."]
    #[inline] pub fn pdor_mut(&self) -> *mut Pdor { 
        self.pdor_reg().ptr()
    }

    #[doc="Get the *const pointer for the PDOR register."]
    #[inline] pub fn pdor_ptr(&self) -> *const Pdor { 
        self.pdor_reg().ptr()
    }

    #[doc="Read the PDOR register."]
    #[inline] pub fn pdor(&self) -> Pdor { 
        self.pdor_reg().read()
    }

    #[doc="Write the PDOR register."]
    #[inline] pub fn write_pdor(&self, value: Pdor) -> &Self { 
        self.pdor_reg().write(value);
        self
    }

    #[doc="Set the PDOR register."]
    #[inline] pub fn set_pdor<F: FnOnce(Pdor) -> Pdor>(&self, f: F) -> &Self {
        self.pdor_reg().set(f);
        self
    }

    #[doc="Modify the PDOR register."]
    #[inline] pub fn with_pdor<F: FnOnce(Pdor) -> Pdor>(&self, f: F) -> &Self {
        self.pdor_reg().with(f);
        self
    }

    #[doc="Get the PSOR Register."]
    #[inline] pub fn psor_reg(&self) -> ::bobbin_mcu::register::Register<Psor> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Psor, 0x4)
    }

    #[doc="Get the *mut pointer for the PSOR register."]
    #[inline] pub fn psor_mut(&self) -> *mut Psor { 
        self.psor_reg().ptr()
    }

    #[doc="Get the *const pointer for the PSOR register."]
    #[inline] pub fn psor_ptr(&self) -> *const Psor { 
        self.psor_reg().ptr()
    }

    #[doc="Write the PSOR register."]
    #[inline] pub fn write_psor(&self, value: Psor) -> &Self { 
        self.psor_reg().write(value);
        self
    }

    #[doc="Set the PSOR register."]
    #[inline] pub fn set_psor<F: FnOnce(Psor) -> Psor>(&self, f: F) -> &Self {
        self.psor_reg().set(f);
        self
    }

    #[doc="Get the PCOR Register."]
    #[inline] pub fn pcor_reg(&self) -> ::bobbin_mcu::register::Register<Pcor> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pcor, 0x8)
    }

    #[doc="Get the *mut pointer for the PCOR register."]
    #[inline] pub fn pcor_mut(&self) -> *mut Pcor { 
        self.pcor_reg().ptr()
    }

    #[doc="Get the *const pointer for the PCOR register."]
    #[inline] pub fn pcor_ptr(&self) -> *const Pcor { 
        self.pcor_reg().ptr()
    }

    #[doc="Write the PCOR register."]
    #[inline] pub fn write_pcor(&self, value: Pcor) -> &Self { 
        self.pcor_reg().write(value);
        self
    }

    #[doc="Set the PCOR register."]
    #[inline] pub fn set_pcor<F: FnOnce(Pcor) -> Pcor>(&self, f: F) -> &Self {
        self.pcor_reg().set(f);
        self
    }

    #[doc="Get the PTOR Register."]
    #[inline] pub fn ptor_reg(&self) -> ::bobbin_mcu::register::Register<Ptor> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ptor, 0xc)
    }

    #[doc="Get the *mut pointer for the PTOR register."]
    #[inline] pub fn ptor_mut(&self) -> *mut Ptor { 
        self.ptor_reg().ptr()
    }

    #[doc="Get the *const pointer for the PTOR register."]
    #[inline] pub fn ptor_ptr(&self) -> *const Ptor { 
        self.ptor_reg().ptr()
    }

    #[doc="Write the PTOR register."]
    #[inline] pub fn write_ptor(&self, value: Ptor) -> &Self { 
        self.ptor_reg().write(value);
        self
    }

    #[doc="Set the PTOR register."]
    #[inline] pub fn set_ptor<F: FnOnce(Ptor) -> Ptor>(&self, f: F) -> &Self {
        self.ptor_reg().set(f);
        self
    }

    #[doc="Get the PDIR Register."]
    #[inline] pub fn pdir_reg(&self) -> ::bobbin_mcu::register::Register<Pdir> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pdir, 0x10)
    }

    #[doc="Get the *mut pointer for the PDIR register."]
    #[inline] pub fn pdir_mut(&self) -> *mut Pdir { 
        self.pdir_reg().ptr()
    }

    #[doc="Get the *const pointer for the PDIR register."]
    #[inline] pub fn pdir_ptr(&self) -> *const Pdir { 
        self.pdir_reg().ptr()
    }

    #[doc="Read the PDIR register."]
    #[inline] pub fn pdir(&self) -> Pdir { 
        self.pdir_reg().read()
    }

    #[doc="Get the PDDR Register."]
    #[inline] pub fn pddr_reg(&self) -> ::bobbin_mcu::register::Register<Pddr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pddr, 0x14)
    }

    #[doc="Get the *mut pointer for the PDDR register."]
    #[inline] pub fn pddr_mut(&self) -> *mut Pddr { 
        self.pddr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PDDR register."]
    #[inline] pub fn pddr_ptr(&self) -> *const Pddr { 
        self.pddr_reg().ptr()
    }

    #[doc="Read the PDDR register."]
    #[inline] pub fn pddr(&self) -> Pddr { 
        self.pddr_reg().read()
    }

    #[doc="Write the PDDR register."]
    #[inline] pub fn write_pddr(&self, value: Pddr) -> &Self { 
        self.pddr_reg().write(value);
        self
    }

    #[doc="Set the PDDR register."]
    #[inline] pub fn set_pddr<F: FnOnce(Pddr) -> Pddr>(&self, f: F) -> &Self {
        self.pddr_reg().set(f);
        self
    }

    #[doc="Modify the PDDR register."]
    #[inline] pub fn with_pddr<F: FnOnce(Pddr) -> Pddr>(&self, f: F) -> &Self {
        self.pddr_reg().with(f);
        self
    }

    #[doc="Get the PIDR Register."]
    #[inline] pub fn pidr_reg(&self) -> ::bobbin_mcu::register::Register<Pidr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pidr, 0x18)
    }

    #[doc="Get the *mut pointer for the PIDR register."]
    #[inline] pub fn pidr_mut(&self) -> *mut Pidr { 
        self.pidr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PIDR register."]
    #[inline] pub fn pidr_ptr(&self) -> *const Pidr { 
        self.pidr_reg().ptr()
    }

    #[doc="Read the PIDR register."]
    #[inline] pub fn pidr(&self) -> Pidr { 
        self.pidr_reg().read()
    }

    #[doc="Write the PIDR register."]
    #[inline] pub fn write_pidr(&self, value: Pidr) -> &Self { 
        self.pidr_reg().write(value);
        self
    }

    #[doc="Set the PIDR register."]
    #[inline] pub fn set_pidr<F: FnOnce(Pidr) -> Pidr>(&self, f: F) -> &Self {
        self.pidr_reg().set(f);
        self
    }

    #[doc="Modify the PIDR register."]
    #[inline] pub fn with_pidr<F: FnOnce(Pidr) -> Pidr>(&self, f: F) -> &Self {
        self.pidr_reg().with(f);
        self
    }

}

#[doc="Port Data Output Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdor(pub u32);
impl Pdor {
    #[doc="Port Data Output"]
    #[inline] pub fn pdo<I: Into<::bobbin_bits::R32>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PDO != 0"]
    #[inline] pub fn test_pdo<I: Into<::bobbin_bits::R32>>(&self, index: I) -> bool{
        self.pdo(index) != 0
    }

    #[doc="Sets the PDO field."]
    #[inline] pub fn set_pdo<I: Into<::bobbin_bits::R32>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Pdor {
    #[inline]
    fn from(other: u32) -> Self {
         Pdor(other)
    }
}

impl ::core::fmt::Display for Pdor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pdor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pdo(0) != 0 { try!(write!(f, " pdo[0]"))}
        if self.pdo(1) != 0 { try!(write!(f, " pdo[1]"))}
        if self.pdo(2) != 0 { try!(write!(f, " pdo[2]"))}
        if self.pdo(3) != 0 { try!(write!(f, " pdo[3]"))}
        if self.pdo(4) != 0 { try!(write!(f, " pdo[4]"))}
        if self.pdo(5) != 0 { try!(write!(f, " pdo[5]"))}
        if self.pdo(6) != 0 { try!(write!(f, " pdo[6]"))}
        if self.pdo(7) != 0 { try!(write!(f, " pdo[7]"))}
        if self.pdo(8) != 0 { try!(write!(f, " pdo[8]"))}
        if self.pdo(9) != 0 { try!(write!(f, " pdo[9]"))}
        if self.pdo(10) != 0 { try!(write!(f, " pdo[10]"))}
        if self.pdo(11) != 0 { try!(write!(f, " pdo[11]"))}
        if self.pdo(12) != 0 { try!(write!(f, " pdo[12]"))}
        if self.pdo(13) != 0 { try!(write!(f, " pdo[13]"))}
        if self.pdo(14) != 0 { try!(write!(f, " pdo[14]"))}
        if self.pdo(15) != 0 { try!(write!(f, " pdo[15]"))}
        if self.pdo(16) != 0 { try!(write!(f, " pdo[16]"))}
        if self.pdo(17) != 0 { try!(write!(f, " pdo[17]"))}
        if self.pdo(18) != 0 { try!(write!(f, " pdo[18]"))}
        if self.pdo(19) != 0 { try!(write!(f, " pdo[19]"))}
        if self.pdo(20) != 0 { try!(write!(f, " pdo[20]"))}
        if self.pdo(21) != 0 { try!(write!(f, " pdo[21]"))}
        if self.pdo(22) != 0 { try!(write!(f, " pdo[22]"))}
        if self.pdo(23) != 0 { try!(write!(f, " pdo[23]"))}
        if self.pdo(24) != 0 { try!(write!(f, " pdo[24]"))}
        if self.pdo(25) != 0 { try!(write!(f, " pdo[25]"))}
        if self.pdo(26) != 0 { try!(write!(f, " pdo[26]"))}
        if self.pdo(27) != 0 { try!(write!(f, " pdo[27]"))}
        if self.pdo(28) != 0 { try!(write!(f, " pdo[28]"))}
        if self.pdo(29) != 0 { try!(write!(f, " pdo[29]"))}
        if self.pdo(30) != 0 { try!(write!(f, " pdo[30]"))}
        if self.pdo(31) != 0 { try!(write!(f, " pdo[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Set Output Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Psor(pub u32);
impl Psor {
    #[doc="Port Set Output"]
    #[inline] pub fn ptso<I: Into<::bobbin_bits::R32>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PTSO != 0"]
    #[inline] pub fn test_ptso<I: Into<::bobbin_bits::R32>>(&self, index: I) -> bool{
        self.ptso(index) != 0
    }

    #[doc="Sets the PTSO field."]
    #[inline] pub fn set_ptso<I: Into<::bobbin_bits::R32>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Psor {
    #[inline]
    fn from(other: u32) -> Self {
         Psor(other)
    }
}

impl ::core::fmt::Display for Psor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Psor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ptso(0) != 0 { try!(write!(f, " ptso[0]"))}
        if self.ptso(1) != 0 { try!(write!(f, " ptso[1]"))}
        if self.ptso(2) != 0 { try!(write!(f, " ptso[2]"))}
        if self.ptso(3) != 0 { try!(write!(f, " ptso[3]"))}
        if self.ptso(4) != 0 { try!(write!(f, " ptso[4]"))}
        if self.ptso(5) != 0 { try!(write!(f, " ptso[5]"))}
        if self.ptso(6) != 0 { try!(write!(f, " ptso[6]"))}
        if self.ptso(7) != 0 { try!(write!(f, " ptso[7]"))}
        if self.ptso(8) != 0 { try!(write!(f, " ptso[8]"))}
        if self.ptso(9) != 0 { try!(write!(f, " ptso[9]"))}
        if self.ptso(10) != 0 { try!(write!(f, " ptso[10]"))}
        if self.ptso(11) != 0 { try!(write!(f, " ptso[11]"))}
        if self.ptso(12) != 0 { try!(write!(f, " ptso[12]"))}
        if self.ptso(13) != 0 { try!(write!(f, " ptso[13]"))}
        if self.ptso(14) != 0 { try!(write!(f, " ptso[14]"))}
        if self.ptso(15) != 0 { try!(write!(f, " ptso[15]"))}
        if self.ptso(16) != 0 { try!(write!(f, " ptso[16]"))}
        if self.ptso(17) != 0 { try!(write!(f, " ptso[17]"))}
        if self.ptso(18) != 0 { try!(write!(f, " ptso[18]"))}
        if self.ptso(19) != 0 { try!(write!(f, " ptso[19]"))}
        if self.ptso(20) != 0 { try!(write!(f, " ptso[20]"))}
        if self.ptso(21) != 0 { try!(write!(f, " ptso[21]"))}
        if self.ptso(22) != 0 { try!(write!(f, " ptso[22]"))}
        if self.ptso(23) != 0 { try!(write!(f, " ptso[23]"))}
        if self.ptso(24) != 0 { try!(write!(f, " ptso[24]"))}
        if self.ptso(25) != 0 { try!(write!(f, " ptso[25]"))}
        if self.ptso(26) != 0 { try!(write!(f, " ptso[26]"))}
        if self.ptso(27) != 0 { try!(write!(f, " ptso[27]"))}
        if self.ptso(28) != 0 { try!(write!(f, " ptso[28]"))}
        if self.ptso(29) != 0 { try!(write!(f, " ptso[29]"))}
        if self.ptso(30) != 0 { try!(write!(f, " ptso[30]"))}
        if self.ptso(31) != 0 { try!(write!(f, " ptso[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Clear Output Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcor(pub u32);
impl Pcor {
    #[doc="Port Clear Output"]
    #[inline] pub fn ptco<I: Into<::bobbin_bits::R32>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PTCO != 0"]
    #[inline] pub fn test_ptco<I: Into<::bobbin_bits::R32>>(&self, index: I) -> bool{
        self.ptco(index) != 0
    }

    #[doc="Sets the PTCO field."]
    #[inline] pub fn set_ptco<I: Into<::bobbin_bits::R32>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Pcor {
    #[inline]
    fn from(other: u32) -> Self {
         Pcor(other)
    }
}

impl ::core::fmt::Display for Pcor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ptco(0) != 0 { try!(write!(f, " ptco[0]"))}
        if self.ptco(1) != 0 { try!(write!(f, " ptco[1]"))}
        if self.ptco(2) != 0 { try!(write!(f, " ptco[2]"))}
        if self.ptco(3) != 0 { try!(write!(f, " ptco[3]"))}
        if self.ptco(4) != 0 { try!(write!(f, " ptco[4]"))}
        if self.ptco(5) != 0 { try!(write!(f, " ptco[5]"))}
        if self.ptco(6) != 0 { try!(write!(f, " ptco[6]"))}
        if self.ptco(7) != 0 { try!(write!(f, " ptco[7]"))}
        if self.ptco(8) != 0 { try!(write!(f, " ptco[8]"))}
        if self.ptco(9) != 0 { try!(write!(f, " ptco[9]"))}
        if self.ptco(10) != 0 { try!(write!(f, " ptco[10]"))}
        if self.ptco(11) != 0 { try!(write!(f, " ptco[11]"))}
        if self.ptco(12) != 0 { try!(write!(f, " ptco[12]"))}
        if self.ptco(13) != 0 { try!(write!(f, " ptco[13]"))}
        if self.ptco(14) != 0 { try!(write!(f, " ptco[14]"))}
        if self.ptco(15) != 0 { try!(write!(f, " ptco[15]"))}
        if self.ptco(16) != 0 { try!(write!(f, " ptco[16]"))}
        if self.ptco(17) != 0 { try!(write!(f, " ptco[17]"))}
        if self.ptco(18) != 0 { try!(write!(f, " ptco[18]"))}
        if self.ptco(19) != 0 { try!(write!(f, " ptco[19]"))}
        if self.ptco(20) != 0 { try!(write!(f, " ptco[20]"))}
        if self.ptco(21) != 0 { try!(write!(f, " ptco[21]"))}
        if self.ptco(22) != 0 { try!(write!(f, " ptco[22]"))}
        if self.ptco(23) != 0 { try!(write!(f, " ptco[23]"))}
        if self.ptco(24) != 0 { try!(write!(f, " ptco[24]"))}
        if self.ptco(25) != 0 { try!(write!(f, " ptco[25]"))}
        if self.ptco(26) != 0 { try!(write!(f, " ptco[26]"))}
        if self.ptco(27) != 0 { try!(write!(f, " ptco[27]"))}
        if self.ptco(28) != 0 { try!(write!(f, " ptco[28]"))}
        if self.ptco(29) != 0 { try!(write!(f, " ptco[29]"))}
        if self.ptco(30) != 0 { try!(write!(f, " ptco[30]"))}
        if self.ptco(31) != 0 { try!(write!(f, " ptco[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Toggle Output Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptor(pub u32);
impl Ptor {
    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto<I: Into<::bobbin_bits::R32>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PTTO != 0"]
    #[inline] pub fn test_ptto<I: Into<::bobbin_bits::R32>>(&self, index: I) -> bool{
        self.ptto(index) != 0
    }

    #[doc="Sets the PTTO field."]
    #[inline] pub fn set_ptto<I: Into<::bobbin_bits::R32>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Ptor {
    #[inline]
    fn from(other: u32) -> Self {
         Ptor(other)
    }
}

impl ::core::fmt::Display for Ptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ptto(0) != 0 { try!(write!(f, " ptto[0]"))}
        if self.ptto(1) != 0 { try!(write!(f, " ptto[1]"))}
        if self.ptto(2) != 0 { try!(write!(f, " ptto[2]"))}
        if self.ptto(3) != 0 { try!(write!(f, " ptto[3]"))}
        if self.ptto(4) != 0 { try!(write!(f, " ptto[4]"))}
        if self.ptto(5) != 0 { try!(write!(f, " ptto[5]"))}
        if self.ptto(6) != 0 { try!(write!(f, " ptto[6]"))}
        if self.ptto(7) != 0 { try!(write!(f, " ptto[7]"))}
        if self.ptto(8) != 0 { try!(write!(f, " ptto[8]"))}
        if self.ptto(9) != 0 { try!(write!(f, " ptto[9]"))}
        if self.ptto(10) != 0 { try!(write!(f, " ptto[10]"))}
        if self.ptto(11) != 0 { try!(write!(f, " ptto[11]"))}
        if self.ptto(12) != 0 { try!(write!(f, " ptto[12]"))}
        if self.ptto(13) != 0 { try!(write!(f, " ptto[13]"))}
        if self.ptto(14) != 0 { try!(write!(f, " ptto[14]"))}
        if self.ptto(15) != 0 { try!(write!(f, " ptto[15]"))}
        if self.ptto(16) != 0 { try!(write!(f, " ptto[16]"))}
        if self.ptto(17) != 0 { try!(write!(f, " ptto[17]"))}
        if self.ptto(18) != 0 { try!(write!(f, " ptto[18]"))}
        if self.ptto(19) != 0 { try!(write!(f, " ptto[19]"))}
        if self.ptto(20) != 0 { try!(write!(f, " ptto[20]"))}
        if self.ptto(21) != 0 { try!(write!(f, " ptto[21]"))}
        if self.ptto(22) != 0 { try!(write!(f, " ptto[22]"))}
        if self.ptto(23) != 0 { try!(write!(f, " ptto[23]"))}
        if self.ptto(24) != 0 { try!(write!(f, " ptto[24]"))}
        if self.ptto(25) != 0 { try!(write!(f, " ptto[25]"))}
        if self.ptto(26) != 0 { try!(write!(f, " ptto[26]"))}
        if self.ptto(27) != 0 { try!(write!(f, " ptto[27]"))}
        if self.ptto(28) != 0 { try!(write!(f, " ptto[28]"))}
        if self.ptto(29) != 0 { try!(write!(f, " ptto[29]"))}
        if self.ptto(30) != 0 { try!(write!(f, " ptto[30]"))}
        if self.ptto(31) != 0 { try!(write!(f, " ptto[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Data Input Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdir(pub u32);
impl Pdir {
    #[doc="Port Data Input"]
    #[inline] pub fn pdi<I: Into<::bobbin_bits::R32>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PDI != 0"]
    #[inline] pub fn test_pdi<I: Into<::bobbin_bits::R32>>(&self, index: I) -> bool{
        self.pdi(index) != 0
    }

    #[doc="Sets the PDI field."]
    #[inline] pub fn set_pdi<I: Into<::bobbin_bits::R32>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Pdir {
    #[inline]
    fn from(other: u32) -> Self {
         Pdir(other)
    }
}

impl ::core::fmt::Display for Pdir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pdir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pdi(0) != 0 { try!(write!(f, " pdi[0]"))}
        if self.pdi(1) != 0 { try!(write!(f, " pdi[1]"))}
        if self.pdi(2) != 0 { try!(write!(f, " pdi[2]"))}
        if self.pdi(3) != 0 { try!(write!(f, " pdi[3]"))}
        if self.pdi(4) != 0 { try!(write!(f, " pdi[4]"))}
        if self.pdi(5) != 0 { try!(write!(f, " pdi[5]"))}
        if self.pdi(6) != 0 { try!(write!(f, " pdi[6]"))}
        if self.pdi(7) != 0 { try!(write!(f, " pdi[7]"))}
        if self.pdi(8) != 0 { try!(write!(f, " pdi[8]"))}
        if self.pdi(9) != 0 { try!(write!(f, " pdi[9]"))}
        if self.pdi(10) != 0 { try!(write!(f, " pdi[10]"))}
        if self.pdi(11) != 0 { try!(write!(f, " pdi[11]"))}
        if self.pdi(12) != 0 { try!(write!(f, " pdi[12]"))}
        if self.pdi(13) != 0 { try!(write!(f, " pdi[13]"))}
        if self.pdi(14) != 0 { try!(write!(f, " pdi[14]"))}
        if self.pdi(15) != 0 { try!(write!(f, " pdi[15]"))}
        if self.pdi(16) != 0 { try!(write!(f, " pdi[16]"))}
        if self.pdi(17) != 0 { try!(write!(f, " pdi[17]"))}
        if self.pdi(18) != 0 { try!(write!(f, " pdi[18]"))}
        if self.pdi(19) != 0 { try!(write!(f, " pdi[19]"))}
        if self.pdi(20) != 0 { try!(write!(f, " pdi[20]"))}
        if self.pdi(21) != 0 { try!(write!(f, " pdi[21]"))}
        if self.pdi(22) != 0 { try!(write!(f, " pdi[22]"))}
        if self.pdi(23) != 0 { try!(write!(f, " pdi[23]"))}
        if self.pdi(24) != 0 { try!(write!(f, " pdi[24]"))}
        if self.pdi(25) != 0 { try!(write!(f, " pdi[25]"))}
        if self.pdi(26) != 0 { try!(write!(f, " pdi[26]"))}
        if self.pdi(27) != 0 { try!(write!(f, " pdi[27]"))}
        if self.pdi(28) != 0 { try!(write!(f, " pdi[28]"))}
        if self.pdi(29) != 0 { try!(write!(f, " pdi[29]"))}
        if self.pdi(30) != 0 { try!(write!(f, " pdi[30]"))}
        if self.pdi(31) != 0 { try!(write!(f, " pdi[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Data Direction Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pddr(pub u32);
impl Pddr {
    #[doc="Port Data Direction"]
    #[inline] pub fn pdd<I: Into<::bobbin_bits::R32>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PDD != 0"]
    #[inline] pub fn test_pdd<I: Into<::bobbin_bits::R32>>(&self, index: I) -> bool{
        self.pdd(index) != 0
    }

    #[doc="Sets the PDD field."]
    #[inline] pub fn set_pdd<I: Into<::bobbin_bits::R32>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Pddr {
    #[inline]
    fn from(other: u32) -> Self {
         Pddr(other)
    }
}

impl ::core::fmt::Display for Pddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pdd(0) != 0 { try!(write!(f, " pdd[0]"))}
        if self.pdd(1) != 0 { try!(write!(f, " pdd[1]"))}
        if self.pdd(2) != 0 { try!(write!(f, " pdd[2]"))}
        if self.pdd(3) != 0 { try!(write!(f, " pdd[3]"))}
        if self.pdd(4) != 0 { try!(write!(f, " pdd[4]"))}
        if self.pdd(5) != 0 { try!(write!(f, " pdd[5]"))}
        if self.pdd(6) != 0 { try!(write!(f, " pdd[6]"))}
        if self.pdd(7) != 0 { try!(write!(f, " pdd[7]"))}
        if self.pdd(8) != 0 { try!(write!(f, " pdd[8]"))}
        if self.pdd(9) != 0 { try!(write!(f, " pdd[9]"))}
        if self.pdd(10) != 0 { try!(write!(f, " pdd[10]"))}
        if self.pdd(11) != 0 { try!(write!(f, " pdd[11]"))}
        if self.pdd(12) != 0 { try!(write!(f, " pdd[12]"))}
        if self.pdd(13) != 0 { try!(write!(f, " pdd[13]"))}
        if self.pdd(14) != 0 { try!(write!(f, " pdd[14]"))}
        if self.pdd(15) != 0 { try!(write!(f, " pdd[15]"))}
        if self.pdd(16) != 0 { try!(write!(f, " pdd[16]"))}
        if self.pdd(17) != 0 { try!(write!(f, " pdd[17]"))}
        if self.pdd(18) != 0 { try!(write!(f, " pdd[18]"))}
        if self.pdd(19) != 0 { try!(write!(f, " pdd[19]"))}
        if self.pdd(20) != 0 { try!(write!(f, " pdd[20]"))}
        if self.pdd(21) != 0 { try!(write!(f, " pdd[21]"))}
        if self.pdd(22) != 0 { try!(write!(f, " pdd[22]"))}
        if self.pdd(23) != 0 { try!(write!(f, " pdd[23]"))}
        if self.pdd(24) != 0 { try!(write!(f, " pdd[24]"))}
        if self.pdd(25) != 0 { try!(write!(f, " pdd[25]"))}
        if self.pdd(26) != 0 { try!(write!(f, " pdd[26]"))}
        if self.pdd(27) != 0 { try!(write!(f, " pdd[27]"))}
        if self.pdd(28) != 0 { try!(write!(f, " pdd[28]"))}
        if self.pdd(29) != 0 { try!(write!(f, " pdd[29]"))}
        if self.pdd(30) != 0 { try!(write!(f, " pdd[30]"))}
        if self.pdd(31) != 0 { try!(write!(f, " pdd[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Input Disable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pidr(pub u32);
impl Pidr {
    #[doc="Port Input Disable"]
    #[inline] pub fn pid<I: Into<::bobbin_bits::R32>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PID != 0"]
    #[inline] pub fn test_pid<I: Into<::bobbin_bits::R32>>(&self, index: I) -> bool{
        self.pid(index) != 0
    }

    #[doc="Sets the PID field."]
    #[inline] pub fn set_pid<I: Into<::bobbin_bits::R32>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Pidr {
    #[inline]
    fn from(other: u32) -> Self {
         Pidr(other)
    }
}

impl ::core::fmt::Display for Pidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pid(0) != 0 { try!(write!(f, " pid[0]"))}
        if self.pid(1) != 0 { try!(write!(f, " pid[1]"))}
        if self.pid(2) != 0 { try!(write!(f, " pid[2]"))}
        if self.pid(3) != 0 { try!(write!(f, " pid[3]"))}
        if self.pid(4) != 0 { try!(write!(f, " pid[4]"))}
        if self.pid(5) != 0 { try!(write!(f, " pid[5]"))}
        if self.pid(6) != 0 { try!(write!(f, " pid[6]"))}
        if self.pid(7) != 0 { try!(write!(f, " pid[7]"))}
        if self.pid(8) != 0 { try!(write!(f, " pid[8]"))}
        if self.pid(9) != 0 { try!(write!(f, " pid[9]"))}
        if self.pid(10) != 0 { try!(write!(f, " pid[10]"))}
        if self.pid(11) != 0 { try!(write!(f, " pid[11]"))}
        if self.pid(12) != 0 { try!(write!(f, " pid[12]"))}
        if self.pid(13) != 0 { try!(write!(f, " pid[13]"))}
        if self.pid(14) != 0 { try!(write!(f, " pid[14]"))}
        if self.pid(15) != 0 { try!(write!(f, " pid[15]"))}
        if self.pid(16) != 0 { try!(write!(f, " pid[16]"))}
        if self.pid(17) != 0 { try!(write!(f, " pid[17]"))}
        if self.pid(18) != 0 { try!(write!(f, " pid[18]"))}
        if self.pid(19) != 0 { try!(write!(f, " pid[19]"))}
        if self.pid(20) != 0 { try!(write!(f, " pid[20]"))}
        if self.pid(21) != 0 { try!(write!(f, " pid[21]"))}
        if self.pid(22) != 0 { try!(write!(f, " pid[22]"))}
        if self.pid(23) != 0 { try!(write!(f, " pid[23]"))}
        if self.pid(24) != 0 { try!(write!(f, " pid[24]"))}
        if self.pid(25) != 0 { try!(write!(f, " pid[25]"))}
        if self.pid(26) != 0 { try!(write!(f, " pid[26]"))}
        if self.pid(27) != 0 { try!(write!(f, " pid[27]"))}
        if self.pid(28) != 0 { try!(write!(f, " pid[28]"))}
        if self.pid(29) != 0 { try!(write!(f, " pid[29]"))}
        if self.pid(30) != 0 { try!(write!(f, " pid[30]"))}
        if self.pid(31) != 0 { try!(write!(f, " pid[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

