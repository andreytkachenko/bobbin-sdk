
::bobbin_mcu::periph!( FLASH, Flash, FLASH_PERIPH, FlashPeriph, FLASH_OWNED, FLASH_REF_COUNT, 0x40022000, 0x00, 0x00);


// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB1RSTR"), field: Some("FLASHRST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Flash {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb1rstr().flashrst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb1rstr(|r| r.set_flashrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB1ENR"), field: Some("FLASHEN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Flash {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb1enr().flashen() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb1enr(|r| r.set_flashen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB1SMENR"), field: Some("FLASHSMEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Flash {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb1smenr().flashsmen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb1smenr(|r| r.set_flashsmen(value));
        self
    }
}

#[doc="Flash"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FlashPeriph(pub usize);
impl FlashPeriph {
    #[doc="Get the ACR Register."]
    #[inline] pub fn acr_reg(&self) -> ::bobbin_mcu::register::Register<Acr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Acr, 0x0)
    }

    #[doc="Get the *mut pointer for the ACR register."]
    #[inline] pub fn acr_mut(&self) -> *mut Acr { 
        self.acr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ACR register."]
    #[inline] pub fn acr_ptr(&self) -> *const Acr { 
        self.acr_reg().ptr()
    }

    #[doc="Read the ACR register."]
    #[inline] pub fn acr(&self) -> Acr { 
        self.acr_reg().read()
    }

    #[doc="Write the ACR register."]
    #[inline] pub fn write_acr(&self, value: Acr) -> &Self { 
        self.acr_reg().write(value);
        self
    }

    #[doc="Set the ACR register."]
    #[inline] pub fn set_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Self {
        self.acr_reg().set(f);
        self
    }

    #[doc="Modify the ACR register."]
    #[inline] pub fn with_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Self {
        self.acr_reg().with(f);
        self
    }

    #[doc="Get the PDKEYR Register."]
    #[inline] pub fn pdkeyr_reg(&self) -> ::bobbin_mcu::register::Register<Pdkeyr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pdkeyr, 0x4)
    }

    #[doc="Get the *mut pointer for the PDKEYR register."]
    #[inline] pub fn pdkeyr_mut(&self) -> *mut Pdkeyr { 
        self.pdkeyr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PDKEYR register."]
    #[inline] pub fn pdkeyr_ptr(&self) -> *const Pdkeyr { 
        self.pdkeyr_reg().ptr()
    }

    #[doc="Write the PDKEYR register."]
    #[inline] pub fn write_pdkeyr(&self, value: Pdkeyr) -> &Self { 
        self.pdkeyr_reg().write(value);
        self
    }

    #[doc="Set the PDKEYR register."]
    #[inline] pub fn set_pdkeyr<F: FnOnce(Pdkeyr) -> Pdkeyr>(&self, f: F) -> &Self {
        self.pdkeyr_reg().set(f);
        self
    }

    #[doc="Get the KEYR Register."]
    #[inline] pub fn keyr_reg(&self) -> ::bobbin_mcu::register::Register<Keyr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Keyr, 0x8)
    }

    #[doc="Get the *mut pointer for the KEYR register."]
    #[inline] pub fn keyr_mut(&self) -> *mut Keyr { 
        self.keyr_reg().ptr()
    }

    #[doc="Get the *const pointer for the KEYR register."]
    #[inline] pub fn keyr_ptr(&self) -> *const Keyr { 
        self.keyr_reg().ptr()
    }

    #[doc="Write the KEYR register."]
    #[inline] pub fn write_keyr(&self, value: Keyr) -> &Self { 
        self.keyr_reg().write(value);
        self
    }

    #[doc="Set the KEYR register."]
    #[inline] pub fn set_keyr<F: FnOnce(Keyr) -> Keyr>(&self, f: F) -> &Self {
        self.keyr_reg().set(f);
        self
    }

    #[doc="Get the OPTKEYR Register."]
    #[inline] pub fn optkeyr_reg(&self) -> ::bobbin_mcu::register::Register<Optkeyr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Optkeyr, 0xc)
    }

    #[doc="Get the *mut pointer for the OPTKEYR register."]
    #[inline] pub fn optkeyr_mut(&self) -> *mut Optkeyr { 
        self.optkeyr_reg().ptr()
    }

    #[doc="Get the *const pointer for the OPTKEYR register."]
    #[inline] pub fn optkeyr_ptr(&self) -> *const Optkeyr { 
        self.optkeyr_reg().ptr()
    }

    #[doc="Write the OPTKEYR register."]
    #[inline] pub fn write_optkeyr(&self, value: Optkeyr) -> &Self { 
        self.optkeyr_reg().write(value);
        self
    }

    #[doc="Set the OPTKEYR register."]
    #[inline] pub fn set_optkeyr<F: FnOnce(Optkeyr) -> Optkeyr>(&self, f: F) -> &Self {
        self.optkeyr_reg().set(f);
        self
    }

    #[doc="Get the SR Register."]
    #[inline] pub fn sr_reg(&self) -> ::bobbin_mcu::register::Register<Sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr, 0x10)
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        self.sr_reg().read()
    }

    #[doc="Write the SR register."]
    #[inline] pub fn write_sr(&self, value: Sr) -> &Self { 
        self.sr_reg().write(value);
        self
    }

    #[doc="Set the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().set(f);
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().with(f);
        self
    }

    #[doc="Get the CR Register."]
    #[inline] pub fn cr_reg(&self) -> ::bobbin_mcu::register::Register<Cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr, 0x14)
    }

    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        self.cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const Cr { 
        self.cr_reg().ptr()
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        self.cr_reg().read()
    }

    #[doc="Write the CR register."]
    #[inline] pub fn write_cr(&self, value: Cr) -> &Self { 
        self.cr_reg().write(value);
        self
    }

    #[doc="Set the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        self.cr_reg().set(f);
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        self.cr_reg().with(f);
        self
    }

    #[doc="Get the ECCR Register."]
    #[inline] pub fn eccr_reg(&self) -> ::bobbin_mcu::register::Register<Eccr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Eccr, 0x18)
    }

    #[doc="Get the *mut pointer for the ECCR register."]
    #[inline] pub fn eccr_mut(&self) -> *mut Eccr { 
        self.eccr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ECCR register."]
    #[inline] pub fn eccr_ptr(&self) -> *const Eccr { 
        self.eccr_reg().ptr()
    }

    #[doc="Read the ECCR register."]
    #[inline] pub fn eccr(&self) -> Eccr { 
        self.eccr_reg().read()
    }

    #[doc="Write the ECCR register."]
    #[inline] pub fn write_eccr(&self, value: Eccr) -> &Self { 
        self.eccr_reg().write(value);
        self
    }

    #[doc="Set the ECCR register."]
    #[inline] pub fn set_eccr<F: FnOnce(Eccr) -> Eccr>(&self, f: F) -> &Self {
        self.eccr_reg().set(f);
        self
    }

    #[doc="Modify the ECCR register."]
    #[inline] pub fn with_eccr<F: FnOnce(Eccr) -> Eccr>(&self, f: F) -> &Self {
        self.eccr_reg().with(f);
        self
    }

    #[doc="Get the OPTR Register."]
    #[inline] pub fn optr_reg(&self) -> ::bobbin_mcu::register::Register<Optr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Optr, 0x20)
    }

    #[doc="Get the *mut pointer for the OPTR register."]
    #[inline] pub fn optr_mut(&self) -> *mut Optr { 
        self.optr_reg().ptr()
    }

    #[doc="Get the *const pointer for the OPTR register."]
    #[inline] pub fn optr_ptr(&self) -> *const Optr { 
        self.optr_reg().ptr()
    }

    #[doc="Read the OPTR register."]
    #[inline] pub fn optr(&self) -> Optr { 
        self.optr_reg().read()
    }

    #[doc="Write the OPTR register."]
    #[inline] pub fn write_optr(&self, value: Optr) -> &Self { 
        self.optr_reg().write(value);
        self
    }

    #[doc="Set the OPTR register."]
    #[inline] pub fn set_optr<F: FnOnce(Optr) -> Optr>(&self, f: F) -> &Self {
        self.optr_reg().set(f);
        self
    }

    #[doc="Modify the OPTR register."]
    #[inline] pub fn with_optr<F: FnOnce(Optr) -> Optr>(&self, f: F) -> &Self {
        self.optr_reg().with(f);
        self
    }

    #[doc="Get the PCROP1SR Register."]
    #[inline] pub fn pcrop1sr_reg(&self) -> ::bobbin_mcu::register::Register<Pcrop1sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pcrop1sr, 0x24)
    }

    #[doc="Get the *mut pointer for the PCROP1SR register."]
    #[inline] pub fn pcrop1sr_mut(&self) -> *mut Pcrop1sr { 
        self.pcrop1sr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PCROP1SR register."]
    #[inline] pub fn pcrop1sr_ptr(&self) -> *const Pcrop1sr { 
        self.pcrop1sr_reg().ptr()
    }

    #[doc="Read the PCROP1SR register."]
    #[inline] pub fn pcrop1sr(&self) -> Pcrop1sr { 
        self.pcrop1sr_reg().read()
    }

    #[doc="Write the PCROP1SR register."]
    #[inline] pub fn write_pcrop1sr(&self, value: Pcrop1sr) -> &Self { 
        self.pcrop1sr_reg().write(value);
        self
    }

    #[doc="Set the PCROP1SR register."]
    #[inline] pub fn set_pcrop1sr<F: FnOnce(Pcrop1sr) -> Pcrop1sr>(&self, f: F) -> &Self {
        self.pcrop1sr_reg().set(f);
        self
    }

    #[doc="Modify the PCROP1SR register."]
    #[inline] pub fn with_pcrop1sr<F: FnOnce(Pcrop1sr) -> Pcrop1sr>(&self, f: F) -> &Self {
        self.pcrop1sr_reg().with(f);
        self
    }

    #[doc="Get the PCROP1ER Register."]
    #[inline] pub fn pcrop1er_reg(&self) -> ::bobbin_mcu::register::Register<Pcrop1er> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pcrop1er, 0x28)
    }

    #[doc="Get the *mut pointer for the PCROP1ER register."]
    #[inline] pub fn pcrop1er_mut(&self) -> *mut Pcrop1er { 
        self.pcrop1er_reg().ptr()
    }

    #[doc="Get the *const pointer for the PCROP1ER register."]
    #[inline] pub fn pcrop1er_ptr(&self) -> *const Pcrop1er { 
        self.pcrop1er_reg().ptr()
    }

    #[doc="Read the PCROP1ER register."]
    #[inline] pub fn pcrop1er(&self) -> Pcrop1er { 
        self.pcrop1er_reg().read()
    }

    #[doc="Write the PCROP1ER register."]
    #[inline] pub fn write_pcrop1er(&self, value: Pcrop1er) -> &Self { 
        self.pcrop1er_reg().write(value);
        self
    }

    #[doc="Set the PCROP1ER register."]
    #[inline] pub fn set_pcrop1er<F: FnOnce(Pcrop1er) -> Pcrop1er>(&self, f: F) -> &Self {
        self.pcrop1er_reg().set(f);
        self
    }

    #[doc="Modify the PCROP1ER register."]
    #[inline] pub fn with_pcrop1er<F: FnOnce(Pcrop1er) -> Pcrop1er>(&self, f: F) -> &Self {
        self.pcrop1er_reg().with(f);
        self
    }

    #[doc="Get the WRP1AR Register."]
    #[inline] pub fn wrp1ar_reg(&self) -> ::bobbin_mcu::register::Register<Wrp1ar> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Wrp1ar, 0x2c)
    }

    #[doc="Get the *mut pointer for the WRP1AR register."]
    #[inline] pub fn wrp1ar_mut(&self) -> *mut Wrp1ar { 
        self.wrp1ar_reg().ptr()
    }

    #[doc="Get the *const pointer for the WRP1AR register."]
    #[inline] pub fn wrp1ar_ptr(&self) -> *const Wrp1ar { 
        self.wrp1ar_reg().ptr()
    }

    #[doc="Read the WRP1AR register."]
    #[inline] pub fn wrp1ar(&self) -> Wrp1ar { 
        self.wrp1ar_reg().read()
    }

    #[doc="Write the WRP1AR register."]
    #[inline] pub fn write_wrp1ar(&self, value: Wrp1ar) -> &Self { 
        self.wrp1ar_reg().write(value);
        self
    }

    #[doc="Set the WRP1AR register."]
    #[inline] pub fn set_wrp1ar<F: FnOnce(Wrp1ar) -> Wrp1ar>(&self, f: F) -> &Self {
        self.wrp1ar_reg().set(f);
        self
    }

    #[doc="Modify the WRP1AR register."]
    #[inline] pub fn with_wrp1ar<F: FnOnce(Wrp1ar) -> Wrp1ar>(&self, f: F) -> &Self {
        self.wrp1ar_reg().with(f);
        self
    }

    #[doc="Get the WRP1BR Register."]
    #[inline] pub fn wrp1br_reg(&self) -> ::bobbin_mcu::register::Register<Wrp1br> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Wrp1br, 0x30)
    }

    #[doc="Get the *mut pointer for the WRP1BR register."]
    #[inline] pub fn wrp1br_mut(&self) -> *mut Wrp1br { 
        self.wrp1br_reg().ptr()
    }

    #[doc="Get the *const pointer for the WRP1BR register."]
    #[inline] pub fn wrp1br_ptr(&self) -> *const Wrp1br { 
        self.wrp1br_reg().ptr()
    }

    #[doc="Read the WRP1BR register."]
    #[inline] pub fn wrp1br(&self) -> Wrp1br { 
        self.wrp1br_reg().read()
    }

    #[doc="Write the WRP1BR register."]
    #[inline] pub fn write_wrp1br(&self, value: Wrp1br) -> &Self { 
        self.wrp1br_reg().write(value);
        self
    }

    #[doc="Set the WRP1BR register."]
    #[inline] pub fn set_wrp1br<F: FnOnce(Wrp1br) -> Wrp1br>(&self, f: F) -> &Self {
        self.wrp1br_reg().set(f);
        self
    }

    #[doc="Modify the WRP1BR register."]
    #[inline] pub fn with_wrp1br<F: FnOnce(Wrp1br) -> Wrp1br>(&self, f: F) -> &Self {
        self.wrp1br_reg().with(f);
        self
    }

    #[doc="Get the PCROP2SR Register."]
    #[inline] pub fn pcrop2sr_reg(&self) -> ::bobbin_mcu::register::Register<Pcrop2sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pcrop2sr, 0x44)
    }

    #[doc="Get the *mut pointer for the PCROP2SR register."]
    #[inline] pub fn pcrop2sr_mut(&self) -> *mut Pcrop2sr { 
        self.pcrop2sr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PCROP2SR register."]
    #[inline] pub fn pcrop2sr_ptr(&self) -> *const Pcrop2sr { 
        self.pcrop2sr_reg().ptr()
    }

    #[doc="Read the PCROP2SR register."]
    #[inline] pub fn pcrop2sr(&self) -> Pcrop2sr { 
        self.pcrop2sr_reg().read()
    }

    #[doc="Write the PCROP2SR register."]
    #[inline] pub fn write_pcrop2sr(&self, value: Pcrop2sr) -> &Self { 
        self.pcrop2sr_reg().write(value);
        self
    }

    #[doc="Set the PCROP2SR register."]
    #[inline] pub fn set_pcrop2sr<F: FnOnce(Pcrop2sr) -> Pcrop2sr>(&self, f: F) -> &Self {
        self.pcrop2sr_reg().set(f);
        self
    }

    #[doc="Modify the PCROP2SR register."]
    #[inline] pub fn with_pcrop2sr<F: FnOnce(Pcrop2sr) -> Pcrop2sr>(&self, f: F) -> &Self {
        self.pcrop2sr_reg().with(f);
        self
    }

    #[doc="Get the PCROP2ER Register."]
    #[inline] pub fn pcrop2er_reg(&self) -> ::bobbin_mcu::register::Register<Pcrop2er> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pcrop2er, 0x48)
    }

    #[doc="Get the *mut pointer for the PCROP2ER register."]
    #[inline] pub fn pcrop2er_mut(&self) -> *mut Pcrop2er { 
        self.pcrop2er_reg().ptr()
    }

    #[doc="Get the *const pointer for the PCROP2ER register."]
    #[inline] pub fn pcrop2er_ptr(&self) -> *const Pcrop2er { 
        self.pcrop2er_reg().ptr()
    }

    #[doc="Read the PCROP2ER register."]
    #[inline] pub fn pcrop2er(&self) -> Pcrop2er { 
        self.pcrop2er_reg().read()
    }

    #[doc="Write the PCROP2ER register."]
    #[inline] pub fn write_pcrop2er(&self, value: Pcrop2er) -> &Self { 
        self.pcrop2er_reg().write(value);
        self
    }

    #[doc="Set the PCROP2ER register."]
    #[inline] pub fn set_pcrop2er<F: FnOnce(Pcrop2er) -> Pcrop2er>(&self, f: F) -> &Self {
        self.pcrop2er_reg().set(f);
        self
    }

    #[doc="Modify the PCROP2ER register."]
    #[inline] pub fn with_pcrop2er<F: FnOnce(Pcrop2er) -> Pcrop2er>(&self, f: F) -> &Self {
        self.pcrop2er_reg().with(f);
        self
    }

    #[doc="Get the WRP2AR Register."]
    #[inline] pub fn wrp2ar_reg(&self) -> ::bobbin_mcu::register::Register<Wrp2ar> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Wrp2ar, 0x4c)
    }

    #[doc="Get the *mut pointer for the WRP2AR register."]
    #[inline] pub fn wrp2ar_mut(&self) -> *mut Wrp2ar { 
        self.wrp2ar_reg().ptr()
    }

    #[doc="Get the *const pointer for the WRP2AR register."]
    #[inline] pub fn wrp2ar_ptr(&self) -> *const Wrp2ar { 
        self.wrp2ar_reg().ptr()
    }

    #[doc="Read the WRP2AR register."]
    #[inline] pub fn wrp2ar(&self) -> Wrp2ar { 
        self.wrp2ar_reg().read()
    }

    #[doc="Write the WRP2AR register."]
    #[inline] pub fn write_wrp2ar(&self, value: Wrp2ar) -> &Self { 
        self.wrp2ar_reg().write(value);
        self
    }

    #[doc="Set the WRP2AR register."]
    #[inline] pub fn set_wrp2ar<F: FnOnce(Wrp2ar) -> Wrp2ar>(&self, f: F) -> &Self {
        self.wrp2ar_reg().set(f);
        self
    }

    #[doc="Modify the WRP2AR register."]
    #[inline] pub fn with_wrp2ar<F: FnOnce(Wrp2ar) -> Wrp2ar>(&self, f: F) -> &Self {
        self.wrp2ar_reg().with(f);
        self
    }

    #[doc="Get the WRP2BR Register."]
    #[inline] pub fn wrp2br_reg(&self) -> ::bobbin_mcu::register::Register<Wrp2br> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Wrp2br, 0x50)
    }

    #[doc="Get the *mut pointer for the WRP2BR register."]
    #[inline] pub fn wrp2br_mut(&self) -> *mut Wrp2br { 
        self.wrp2br_reg().ptr()
    }

    #[doc="Get the *const pointer for the WRP2BR register."]
    #[inline] pub fn wrp2br_ptr(&self) -> *const Wrp2br { 
        self.wrp2br_reg().ptr()
    }

    #[doc="Read the WRP2BR register."]
    #[inline] pub fn wrp2br(&self) -> Wrp2br { 
        self.wrp2br_reg().read()
    }

    #[doc="Write the WRP2BR register."]
    #[inline] pub fn write_wrp2br(&self, value: Wrp2br) -> &Self { 
        self.wrp2br_reg().write(value);
        self
    }

    #[doc="Set the WRP2BR register."]
    #[inline] pub fn set_wrp2br<F: FnOnce(Wrp2br) -> Wrp2br>(&self, f: F) -> &Self {
        self.wrp2br_reg().set(f);
        self
    }

    #[doc="Modify the WRP2BR register."]
    #[inline] pub fn with_wrp2br<F: FnOnce(Wrp2br) -> Wrp2br>(&self, f: F) -> &Self {
        self.wrp2br_reg().with(f);
        self
    }

}

#[doc="Access control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Acr(pub u32);
impl Acr {
    #[doc="Latency"]
    #[inline] pub fn latency(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if LATENCY != 0"]
    #[inline] pub fn test_latency(&self) -> bool {
        self.latency() != 0
    }

    #[doc="Sets the LATENCY field."]
    #[inline] pub fn set_latency<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Prefetch enable"]
    #[inline] pub fn prften(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PRFTEN != 0"]
    #[inline] pub fn test_prften(&self) -> bool {
        self.prften() != 0
    }

    #[doc="Sets the PRFTEN field."]
    #[inline] pub fn set_prften<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Instruction cache enable"]
    #[inline] pub fn icen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ICEN != 0"]
    #[inline] pub fn test_icen(&self) -> bool {
        self.icen() != 0
    }

    #[doc="Sets the ICEN field."]
    #[inline] pub fn set_icen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data cache enable"]
    #[inline] pub fn dcen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DCEN != 0"]
    #[inline] pub fn test_dcen(&self) -> bool {
        self.dcen() != 0
    }

    #[doc="Sets the DCEN field."]
    #[inline] pub fn set_dcen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Instruction cache reset"]
    #[inline] pub fn icrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if ICRST != 0"]
    #[inline] pub fn test_icrst(&self) -> bool {
        self.icrst() != 0
    }

    #[doc="Sets the ICRST field."]
    #[inline] pub fn set_icrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Data cache reset"]
    #[inline] pub fn dcrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DCRST != 0"]
    #[inline] pub fn test_dcrst(&self) -> bool {
        self.dcrst() != 0
    }

    #[doc="Sets the DCRST field."]
    #[inline] pub fn set_dcrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Flash Power-down mode during Low-power run mode"]
    #[inline] pub fn run_pd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if RUN_PD != 0"]
    #[inline] pub fn test_run_pd(&self) -> bool {
        self.run_pd() != 0
    }

    #[doc="Sets the RUN_PD field."]
    #[inline] pub fn set_run_pd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Flash Power-down mode during Low-power sleep mode"]
    #[inline] pub fn sleep_pd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SLEEP_PD != 0"]
    #[inline] pub fn test_sleep_pd(&self) -> bool {
        self.sleep_pd() != 0
    }

    #[doc="Sets the SLEEP_PD field."]
    #[inline] pub fn set_sleep_pd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u32> for Acr {
    #[inline]
    fn from(other: u32) -> Self {
         Acr(other)
    }
}

impl ::core::fmt::Display for Acr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Acr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.latency() != 0 { try!(write!(f, " latency=0x{:x}", self.latency()))}
        if self.prften() != 0 { try!(write!(f, " prften"))}
        if self.icen() != 0 { try!(write!(f, " icen"))}
        if self.dcen() != 0 { try!(write!(f, " dcen"))}
        if self.icrst() != 0 { try!(write!(f, " icrst"))}
        if self.dcrst() != 0 { try!(write!(f, " dcrst"))}
        if self.run_pd() != 0 { try!(write!(f, " run_pd"))}
        if self.sleep_pd() != 0 { try!(write!(f, " sleep_pd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power down key register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdkeyr(pub u32);
impl Pdkeyr {
    #[doc="RUN_PD in FLASH_ACR key"]
    #[inline] pub fn pdkeyr(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if PDKEYR != 0"]
    #[inline] pub fn test_pdkeyr(&self) -> bool {
        self.pdkeyr() != 0
    }

    #[doc="Sets the PDKEYR field."]
    #[inline] pub fn set_pdkeyr<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pdkeyr {
    #[inline]
    fn from(other: u32) -> Self {
         Pdkeyr(other)
    }
}

impl ::core::fmt::Display for Pdkeyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pdkeyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash key register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Keyr(pub u32);
impl Keyr {
    #[doc="KEYR"]
    #[inline] pub fn keyr(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if KEYR != 0"]
    #[inline] pub fn test_keyr(&self) -> bool {
        self.keyr() != 0
    }

    #[doc="Sets the KEYR field."]
    #[inline] pub fn set_keyr<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Keyr {
    #[inline]
    fn from(other: u32) -> Self {
         Keyr(other)
    }
}

impl ::core::fmt::Display for Keyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Keyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Option byte key register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Optkeyr(pub u32);
impl Optkeyr {
    #[doc="Option byte key"]
    #[inline] pub fn optkeyr(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if OPTKEYR != 0"]
    #[inline] pub fn test_optkeyr(&self) -> bool {
        self.optkeyr() != 0
    }

    #[doc="Sets the OPTKEYR field."]
    #[inline] pub fn set_optkeyr<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Optkeyr {
    #[inline]
    fn from(other: u32) -> Self {
         Optkeyr(other)
    }
}

impl ::core::fmt::Display for Optkeyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Optkeyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="End of operation"]
    #[inline] pub fn eop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EOP != 0"]
    #[inline] pub fn test_eop(&self) -> bool {
        self.eop() != 0
    }

    #[doc="Sets the EOP field."]
    #[inline] pub fn set_eop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Operation error"]
    #[inline] pub fn operr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OPERR != 0"]
    #[inline] pub fn test_operr(&self) -> bool {
        self.operr() != 0
    }

    #[doc="Sets the OPERR field."]
    #[inline] pub fn set_operr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Programming error"]
    #[inline] pub fn progerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PROGERR != 0"]
    #[inline] pub fn test_progerr(&self) -> bool {
        self.progerr() != 0
    }

    #[doc="Sets the PROGERR field."]
    #[inline] pub fn set_progerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Write protected error"]
    #[inline] pub fn wrperr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WRPERR != 0"]
    #[inline] pub fn test_wrperr(&self) -> bool {
        self.wrperr() != 0
    }

    #[doc="Sets the WRPERR field."]
    #[inline] pub fn set_wrperr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Programming alignment error"]
    #[inline] pub fn pgaerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PGAERR != 0"]
    #[inline] pub fn test_pgaerr(&self) -> bool {
        self.pgaerr() != 0
    }

    #[doc="Sets the PGAERR field."]
    #[inline] pub fn set_pgaerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Size error"]
    #[inline] pub fn sizerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SIZERR != 0"]
    #[inline] pub fn test_sizerr(&self) -> bool {
        self.sizerr() != 0
    }

    #[doc="Sets the SIZERR field."]
    #[inline] pub fn set_sizerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Programming sequence error"]
    #[inline] pub fn pgserr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PGSERR != 0"]
    #[inline] pub fn test_pgserr(&self) -> bool {
        self.pgserr() != 0
    }

    #[doc="Sets the PGSERR field."]
    #[inline] pub fn set_pgserr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Fast programming data miss error"]
    #[inline] pub fn miserr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if MISERR != 0"]
    #[inline] pub fn test_miserr(&self) -> bool {
        self.miserr() != 0
    }

    #[doc="Sets the MISERR field."]
    #[inline] pub fn set_miserr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Fast programming error"]
    #[inline] pub fn fasterr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FASTERR != 0"]
    #[inline] pub fn test_fasterr(&self) -> bool {
        self.fasterr() != 0
    }

    #[doc="Sets the FASTERR field."]
    #[inline] pub fn set_fasterr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="PCROP read error"]
    #[inline] pub fn rderr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RDERR != 0"]
    #[inline] pub fn test_rderr(&self) -> bool {
        self.rderr() != 0
    }

    #[doc="Sets the RDERR field."]
    #[inline] pub fn set_rderr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Option validity error"]
    #[inline] pub fn optverr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OPTVERR != 0"]
    #[inline] pub fn test_optverr(&self) -> bool {
        self.optverr() != 0
    }

    #[doc="Sets the OPTVERR field."]
    #[inline] pub fn set_optverr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Busy"]
    #[inline] pub fn bsy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if BSY != 0"]
    #[inline] pub fn test_bsy(&self) -> bool {
        self.bsy() != 0
    }

    #[doc="Sets the BSY field."]
    #[inline] pub fn set_bsy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Sr {
    #[inline]
    fn from(other: u32) -> Self {
         Sr(other)
    }
}

impl ::core::fmt::Display for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.eop() != 0 { try!(write!(f, " eop"))}
        if self.operr() != 0 { try!(write!(f, " operr"))}
        if self.progerr() != 0 { try!(write!(f, " progerr"))}
        if self.wrperr() != 0 { try!(write!(f, " wrperr"))}
        if self.pgaerr() != 0 { try!(write!(f, " pgaerr"))}
        if self.sizerr() != 0 { try!(write!(f, " sizerr"))}
        if self.pgserr() != 0 { try!(write!(f, " pgserr"))}
        if self.miserr() != 0 { try!(write!(f, " miserr"))}
        if self.fasterr() != 0 { try!(write!(f, " fasterr"))}
        if self.rderr() != 0 { try!(write!(f, " rderr"))}
        if self.optverr() != 0 { try!(write!(f, " optverr"))}
        if self.bsy() != 0 { try!(write!(f, " bsy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Programming"]
    #[inline] pub fn pg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PG != 0"]
    #[inline] pub fn test_pg(&self) -> bool {
        self.pg() != 0
    }

    #[doc="Sets the PG field."]
    #[inline] pub fn set_pg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Page erase"]
    #[inline] pub fn per(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PER != 0"]
    #[inline] pub fn test_per(&self) -> bool {
        self.per() != 0
    }

    #[doc="Sets the PER field."]
    #[inline] pub fn set_per<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Bank 1 Mass erase"]
    #[inline] pub fn mer1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MER1 != 0"]
    #[inline] pub fn test_mer1(&self) -> bool {
        self.mer1() != 0
    }

    #[doc="Sets the MER1 field."]
    #[inline] pub fn set_mer1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Page number"]
    #[inline] pub fn pnb(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0xff) as u8) } // [10:3]
    }

    #[doc="Returns true if PNB != 0"]
    #[inline] pub fn test_pnb(&self) -> bool {
        self.pnb() != 0
    }

    #[doc="Sets the PNB field."]
    #[inline] pub fn set_pnb<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Bank erase"]
    #[inline] pub fn bker(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if BKER != 0"]
    #[inline] pub fn test_bker(&self) -> bool {
        self.bker() != 0
    }

    #[doc="Sets the BKER field."]
    #[inline] pub fn set_bker<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Bank 2 Mass erase"]
    #[inline] pub fn mer2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if MER2 != 0"]
    #[inline] pub fn test_mer2(&self) -> bool {
        self.mer2() != 0
    }

    #[doc="Sets the MER2 field."]
    #[inline] pub fn set_mer2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Start"]
    #[inline] pub fn start(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if START != 0"]
    #[inline] pub fn test_start(&self) -> bool {
        self.start() != 0
    }

    #[doc="Sets the START field."]
    #[inline] pub fn set_start<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Options modification start"]
    #[inline] pub fn optstrt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if OPTSTRT != 0"]
    #[inline] pub fn test_optstrt(&self) -> bool {
        self.optstrt() != 0
    }

    #[doc="Sets the OPTSTRT field."]
    #[inline] pub fn set_optstrt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Fast programming"]
    #[inline] pub fn fstpg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if FSTPG != 0"]
    #[inline] pub fn test_fstpg(&self) -> bool {
        self.fstpg() != 0
    }

    #[doc="Sets the FSTPG field."]
    #[inline] pub fn set_fstpg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="End of operation interrupt enable"]
    #[inline] pub fn eopie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if EOPIE != 0"]
    #[inline] pub fn test_eopie(&self) -> bool {
        self.eopie() != 0
    }

    #[doc="Sets the EOPIE field."]
    #[inline] pub fn set_eopie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn errie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if ERRIE != 0"]
    #[inline] pub fn test_errie(&self) -> bool {
        self.errie() != 0
    }

    #[doc="Sets the ERRIE field."]
    #[inline] pub fn set_errie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="PCROP read error interrupt enable"]
    #[inline] pub fn rderrie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if RDERRIE != 0"]
    #[inline] pub fn test_rderrie(&self) -> bool {
        self.rderrie() != 0
    }

    #[doc="Sets the RDERRIE field."]
    #[inline] pub fn set_rderrie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Force the option byte loading"]
    #[inline] pub fn obl_launch(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if OBL_LAUNCH != 0"]
    #[inline] pub fn test_obl_launch(&self) -> bool {
        self.obl_launch() != 0
    }

    #[doc="Sets the OBL_LAUNCH field."]
    #[inline] pub fn set_obl_launch<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Options Lock"]
    #[inline] pub fn optlock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if OPTLOCK != 0"]
    #[inline] pub fn test_optlock(&self) -> bool {
        self.optlock() != 0
    }

    #[doc="Sets the OPTLOCK field."]
    #[inline] pub fn set_optlock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="FLASH_CR Lock"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
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
        if self.pg() != 0 { try!(write!(f, " pg"))}
        if self.per() != 0 { try!(write!(f, " per"))}
        if self.mer1() != 0 { try!(write!(f, " mer1"))}
        if self.pnb() != 0 { try!(write!(f, " pnb=0x{:x}", self.pnb()))}
        if self.bker() != 0 { try!(write!(f, " bker"))}
        if self.mer2() != 0 { try!(write!(f, " mer2"))}
        if self.start() != 0 { try!(write!(f, " start"))}
        if self.optstrt() != 0 { try!(write!(f, " optstrt"))}
        if self.fstpg() != 0 { try!(write!(f, " fstpg"))}
        if self.eopie() != 0 { try!(write!(f, " eopie"))}
        if self.errie() != 0 { try!(write!(f, " errie"))}
        if self.rderrie() != 0 { try!(write!(f, " rderrie"))}
        if self.obl_launch() != 0 { try!(write!(f, " obl_launch"))}
        if self.optlock() != 0 { try!(write!(f, " optlock"))}
        if self.lock() != 0 { try!(write!(f, " lock"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash ECC register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Eccr(pub u32);
impl Eccr {
    #[doc="ECC fail address"]
    #[inline] pub fn addr_ecc(&self) -> ::bobbin_bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if ADDR_ECC != 0"]
    #[inline] pub fn test_addr_ecc(&self) -> bool {
        self.addr_ecc() != 0
    }

    #[doc="Sets the ADDR_ECC field."]
    #[inline] pub fn set_addr_ecc<V: Into<::bobbin_bits::U19>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="ECC fail bank"]
    #[inline] pub fn bk_ecc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if BK_ECC != 0"]
    #[inline] pub fn test_bk_ecc(&self) -> bool {
        self.bk_ecc() != 0
    }

    #[doc="Sets the BK_ECC field."]
    #[inline] pub fn set_bk_ecc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="System Flash ECC fail"]
    #[inline] pub fn sysf_ecc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if SYSF_ECC != 0"]
    #[inline] pub fn test_sysf_ecc(&self) -> bool {
        self.sysf_ecc() != 0
    }

    #[doc="Sets the SYSF_ECC field."]
    #[inline] pub fn set_sysf_ecc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="ECC correction interrupt enable"]
    #[inline] pub fn eccie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if ECCIE != 0"]
    #[inline] pub fn test_eccie(&self) -> bool {
        self.eccie() != 0
    }

    #[doc="Sets the ECCIE field."]
    #[inline] pub fn set_eccie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="ECC correction"]
    #[inline] pub fn eccc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if ECCC != 0"]
    #[inline] pub fn test_eccc(&self) -> bool {
        self.eccc() != 0
    }

    #[doc="Sets the ECCC field."]
    #[inline] pub fn set_eccc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="ECC detection"]
    #[inline] pub fn eccd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if ECCD != 0"]
    #[inline] pub fn test_eccd(&self) -> bool {
        self.eccd() != 0
    }

    #[doc="Sets the ECCD field."]
    #[inline] pub fn set_eccd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Eccr {
    #[inline]
    fn from(other: u32) -> Self {
         Eccr(other)
    }
}

impl ::core::fmt::Display for Eccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Eccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr_ecc() != 0 { try!(write!(f, " addr_ecc=0x{:x}", self.addr_ecc()))}
        if self.bk_ecc() != 0 { try!(write!(f, " bk_ecc"))}
        if self.sysf_ecc() != 0 { try!(write!(f, " sysf_ecc"))}
        if self.eccie() != 0 { try!(write!(f, " eccie"))}
        if self.eccc() != 0 { try!(write!(f, " eccc"))}
        if self.eccd() != 0 { try!(write!(f, " eccd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash option register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Optr(pub u32);
impl Optr {
    #[doc="Read protection level"]
    #[inline] pub fn rdp(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if RDP != 0"]
    #[inline] pub fn test_rdp(&self) -> bool {
        self.rdp() != 0
    }

    #[doc="Sets the RDP field."]
    #[inline] pub fn set_rdp<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="BOR reset Level"]
    #[inline] pub fn bor_lev(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if BOR_LEV != 0"]
    #[inline] pub fn test_bor_lev(&self) -> bool {
        self.bor_lev() != 0
    }

    #[doc="Sets the BOR_LEV field."]
    #[inline] pub fn set_bor_lev<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="nRST_STOP"]
    #[inline] pub fn nrst_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if nRST_STOP != 0"]
    #[inline] pub fn test_nrst_stop(&self) -> bool {
        self.nrst_stop() != 0
    }

    #[doc="Sets the nRST_STOP field."]
    #[inline] pub fn set_nrst_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="nRST_STDBY"]
    #[inline] pub fn nrst_stdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if nRST_STDBY != 0"]
    #[inline] pub fn test_nrst_stdby(&self) -> bool {
        self.nrst_stdby() != 0
    }

    #[doc="Sets the nRST_STDBY field."]
    #[inline] pub fn set_nrst_stdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Independent watchdog selection"]
    #[inline] pub fn idwg_sw(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if IDWG_SW != 0"]
    #[inline] pub fn test_idwg_sw(&self) -> bool {
        self.idwg_sw() != 0
    }

    #[doc="Sets the IDWG_SW field."]
    #[inline] pub fn set_idwg_sw<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Independent watchdog counter freeze in Stop mode"]
    #[inline] pub fn iwdg_stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if IWDG_STOP != 0"]
    #[inline] pub fn test_iwdg_stop(&self) -> bool {
        self.iwdg_stop() != 0
    }

    #[doc="Sets the IWDG_STOP field."]
    #[inline] pub fn set_iwdg_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Independent watchdog counter freeze in Standby mode"]
    #[inline] pub fn iwdg_stdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if IWDG_STDBY != 0"]
    #[inline] pub fn test_iwdg_stdby(&self) -> bool {
        self.iwdg_stdby() != 0
    }

    #[doc="Sets the IWDG_STDBY field."]
    #[inline] pub fn set_iwdg_stdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Window watchdog selection"]
    #[inline] pub fn wwdg_sw(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if WWDG_SW != 0"]
    #[inline] pub fn test_wwdg_sw(&self) -> bool {
        self.wwdg_sw() != 0
    }

    #[doc="Sets the WWDG_SW field."]
    #[inline] pub fn set_wwdg_sw<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Dual-bank boot"]
    #[inline] pub fn bfb2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if BFB2 != 0"]
    #[inline] pub fn test_bfb2(&self) -> bool {
        self.bfb2() != 0
    }

    #[doc="Sets the BFB2 field."]
    #[inline] pub fn set_bfb2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Dual-Bank on 512 KB or 256 KB Flash memory devices"]
    #[inline] pub fn dualbank(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if DUALBANK != 0"]
    #[inline] pub fn test_dualbank(&self) -> bool {
        self.dualbank() != 0
    }

    #[doc="Sets the DUALBANK field."]
    #[inline] pub fn set_dualbank<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Boot configuration"]
    #[inline] pub fn nboot1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if nBOOT1 != 0"]
    #[inline] pub fn test_nboot1(&self) -> bool {
        self.nboot1() != 0
    }

    #[doc="Sets the nBOOT1 field."]
    #[inline] pub fn set_nboot1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="SRAM2 parity check enable"]
    #[inline] pub fn sram2_pe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if SRAM2_PE != 0"]
    #[inline] pub fn test_sram2_pe(&self) -> bool {
        self.sram2_pe() != 0
    }

    #[doc="Sets the SRAM2_PE field."]
    #[inline] pub fn set_sram2_pe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="SRAM2 Erase when system reset"]
    #[inline] pub fn sram2_rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if SRAM2_RST != 0"]
    #[inline] pub fn test_sram2_rst(&self) -> bool {
        self.sram2_rst() != 0
    }

    #[doc="Sets the SRAM2_RST field."]
    #[inline] pub fn set_sram2_rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

}

impl From<u32> for Optr {
    #[inline]
    fn from(other: u32) -> Self {
         Optr(other)
    }
}

impl ::core::fmt::Display for Optr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Optr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rdp() != 0 { try!(write!(f, " rdp=0x{:x}", self.rdp()))}
        if self.bor_lev() != 0 { try!(write!(f, " bor_lev=0x{:x}", self.bor_lev()))}
        if self.nrst_stop() != 0 { try!(write!(f, " nrst_stop"))}
        if self.nrst_stdby() != 0 { try!(write!(f, " nrst_stdby"))}
        if self.idwg_sw() != 0 { try!(write!(f, " idwg_sw"))}
        if self.iwdg_stop() != 0 { try!(write!(f, " iwdg_stop"))}
        if self.iwdg_stdby() != 0 { try!(write!(f, " iwdg_stdby"))}
        if self.wwdg_sw() != 0 { try!(write!(f, " wwdg_sw"))}
        if self.bfb2() != 0 { try!(write!(f, " bfb2"))}
        if self.dualbank() != 0 { try!(write!(f, " dualbank"))}
        if self.nboot1() != 0 { try!(write!(f, " nboot1"))}
        if self.sram2_pe() != 0 { try!(write!(f, " sram2_pe"))}
        if self.sram2_rst() != 0 { try!(write!(f, " sram2_rst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Bank 1 PCROP Start address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcrop1sr(pub u32);
impl Pcrop1sr {
    #[doc="Bank 1 PCROP area start offset"]
    #[inline] pub fn pcrop1_strt(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if PCROP1_STRT != 0"]
    #[inline] pub fn test_pcrop1_strt(&self) -> bool {
        self.pcrop1_strt() != 0
    }

    #[doc="Sets the PCROP1_STRT field."]
    #[inline] pub fn set_pcrop1_strt<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pcrop1sr {
    #[inline]
    fn from(other: u32) -> Self {
         Pcrop1sr(other)
    }
}

impl ::core::fmt::Display for Pcrop1sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcrop1sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcrop1_strt() != 0 { try!(write!(f, " pcrop1_strt=0x{:x}", self.pcrop1_strt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Bank 1 PCROP End address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcrop1er(pub u32);
impl Pcrop1er {
    #[doc="Bank 1 PCROP area end offset"]
    #[inline] pub fn pcrop1_end(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if PCROP1_END != 0"]
    #[inline] pub fn test_pcrop1_end(&self) -> bool {
        self.pcrop1_end() != 0
    }

    #[doc="Sets the PCROP1_END field."]
    #[inline] pub fn set_pcrop1_end<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="PCROP area preserved when RDP level decreased"]
    #[inline] pub fn pcrop_rdp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PCROP_RDP != 0"]
    #[inline] pub fn test_pcrop_rdp(&self) -> bool {
        self.pcrop_rdp() != 0
    }

    #[doc="Sets the PCROP_RDP field."]
    #[inline] pub fn set_pcrop_rdp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Pcrop1er {
    #[inline]
    fn from(other: u32) -> Self {
         Pcrop1er(other)
    }
}

impl ::core::fmt::Display for Pcrop1er {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcrop1er {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcrop1_end() != 0 { try!(write!(f, " pcrop1_end=0x{:x}", self.pcrop1_end()))}
        if self.pcrop_rdp() != 0 { try!(write!(f, " pcrop_rdp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Bank 1 WRP area A address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wrp1ar(pub u32);
impl Wrp1ar {
    #[doc="Bank 1 WRP first area ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â¦ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã¢â‚¬Å“AÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ start offset"]
    #[inline] pub fn wrp1a_strt(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if WRP1A_STRT != 0"]
    #[inline] pub fn test_wrp1a_strt(&self) -> bool {
        self.wrp1a_strt() != 0
    }

    #[doc="Sets the WRP1A_STRT field."]
    #[inline] pub fn set_wrp1a_strt<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Bank 1 WRP first area A end offset"]
    #[inline] pub fn wrp1a_end(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if WRP1A_END != 0"]
    #[inline] pub fn test_wrp1a_end(&self) -> bool {
        self.wrp1a_end() != 0
    }

    #[doc="Sets the WRP1A_END field."]
    #[inline] pub fn set_wrp1a_end<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Wrp1ar {
    #[inline]
    fn from(other: u32) -> Self {
         Wrp1ar(other)
    }
}

impl ::core::fmt::Display for Wrp1ar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wrp1ar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wrp1a_strt() != 0 { try!(write!(f, " wrp1a_strt=0x{:x}", self.wrp1a_strt()))}
        if self.wrp1a_end() != 0 { try!(write!(f, " wrp1a_end=0x{:x}", self.wrp1a_end()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Bank 1 WRP area B address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wrp1br(pub u32);
impl Wrp1br {
    #[doc="Bank 1 WRP second area B end offset"]
    #[inline] pub fn wrp1b_strt(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if WRP1B_STRT != 0"]
    #[inline] pub fn test_wrp1b_strt(&self) -> bool {
        self.wrp1b_strt() != 0
    }

    #[doc="Sets the WRP1B_STRT field."]
    #[inline] pub fn set_wrp1b_strt<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Bank 1 WRP second area B start offset"]
    #[inline] pub fn wrp1b_end(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if WRP1B_END != 0"]
    #[inline] pub fn test_wrp1b_end(&self) -> bool {
        self.wrp1b_end() != 0
    }

    #[doc="Sets the WRP1B_END field."]
    #[inline] pub fn set_wrp1b_end<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Wrp1br {
    #[inline]
    fn from(other: u32) -> Self {
         Wrp1br(other)
    }
}

impl ::core::fmt::Display for Wrp1br {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wrp1br {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wrp1b_strt() != 0 { try!(write!(f, " wrp1b_strt=0x{:x}", self.wrp1b_strt()))}
        if self.wrp1b_end() != 0 { try!(write!(f, " wrp1b_end=0x{:x}", self.wrp1b_end()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Bank 2 PCROP Start address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcrop2sr(pub u32);
impl Pcrop2sr {
    #[doc="Bank 2 PCROP area start offset"]
    #[inline] pub fn pcrop2_strt(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if PCROP2_STRT != 0"]
    #[inline] pub fn test_pcrop2_strt(&self) -> bool {
        self.pcrop2_strt() != 0
    }

    #[doc="Sets the PCROP2_STRT field."]
    #[inline] pub fn set_pcrop2_strt<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pcrop2sr {
    #[inline]
    fn from(other: u32) -> Self {
         Pcrop2sr(other)
    }
}

impl ::core::fmt::Display for Pcrop2sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcrop2sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcrop2_strt() != 0 { try!(write!(f, " pcrop2_strt=0x{:x}", self.pcrop2_strt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Bank 2 PCROP End address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcrop2er(pub u32);
impl Pcrop2er {
    #[doc="Bank 2 PCROP area end offset"]
    #[inline] pub fn pcrop2_end(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if PCROP2_END != 0"]
    #[inline] pub fn test_pcrop2_end(&self) -> bool {
        self.pcrop2_end() != 0
    }

    #[doc="Sets the PCROP2_END field."]
    #[inline] pub fn set_pcrop2_end<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pcrop2er {
    #[inline]
    fn from(other: u32) -> Self {
         Pcrop2er(other)
    }
}

impl ::core::fmt::Display for Pcrop2er {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcrop2er {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pcrop2_end() != 0 { try!(write!(f, " pcrop2_end=0x{:x}", self.pcrop2_end()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Bank 2 WRP area A address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wrp2ar(pub u32);
impl Wrp2ar {
    #[doc="Bank 2 WRP first area A start offset"]
    #[inline] pub fn wrp2a_strt(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if WRP2A_STRT != 0"]
    #[inline] pub fn test_wrp2a_strt(&self) -> bool {
        self.wrp2a_strt() != 0
    }

    #[doc="Sets the WRP2A_STRT field."]
    #[inline] pub fn set_wrp2a_strt<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Bank 2 WRP first area A end offset"]
    #[inline] pub fn wrp2a_end(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if WRP2A_END != 0"]
    #[inline] pub fn test_wrp2a_end(&self) -> bool {
        self.wrp2a_end() != 0
    }

    #[doc="Sets the WRP2A_END field."]
    #[inline] pub fn set_wrp2a_end<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Wrp2ar {
    #[inline]
    fn from(other: u32) -> Self {
         Wrp2ar(other)
    }
}

impl ::core::fmt::Display for Wrp2ar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wrp2ar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wrp2a_strt() != 0 { try!(write!(f, " wrp2a_strt=0x{:x}", self.wrp2a_strt()))}
        if self.wrp2a_end() != 0 { try!(write!(f, " wrp2a_end=0x{:x}", self.wrp2a_end()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Bank 2 WRP area B address register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wrp2br(pub u32);
impl Wrp2br {
    #[doc="Bank 2 WRP second area B start offset"]
    #[inline] pub fn wrp2b_strt(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if WRP2B_STRT != 0"]
    #[inline] pub fn test_wrp2b_strt(&self) -> bool {
        self.wrp2b_strt() != 0
    }

    #[doc="Sets the WRP2B_STRT field."]
    #[inline] pub fn set_wrp2b_strt<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Bank 2 WRP second area B end offset"]
    #[inline] pub fn wrp2b_end(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if WRP2B_END != 0"]
    #[inline] pub fn test_wrp2b_end(&self) -> bool {
        self.wrp2b_end() != 0
    }

    #[doc="Sets the WRP2B_END field."]
    #[inline] pub fn set_wrp2b_end<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Wrp2br {
    #[inline]
    fn from(other: u32) -> Self {
         Wrp2br(other)
    }
}

impl ::core::fmt::Display for Wrp2br {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wrp2br {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wrp2b_strt() != 0 { try!(write!(f, " wrp2b_strt=0x{:x}", self.wrp2b_strt()))}
        if self.wrp2b_end() != 0 { try!(write!(f, " wrp2b_end=0x{:x}", self.wrp2b_end()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

