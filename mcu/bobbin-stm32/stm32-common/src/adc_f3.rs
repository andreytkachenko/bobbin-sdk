
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ADC_F3 Peripheral"]
pub struct AdcPeriph(pub usize); 

pub struct AdcCh { pub periph: AdcPeriph, pub index: usize }

impl AdcPeriph {
    #[doc="Get the ISR Register."]
    #[inline] pub fn isr_reg(&self) -> ::bobbin_mcu::register::Register<Isr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Isr, 0x0)
    }

    #[doc="Get the *mut pointer for the ISR register."]
    #[inline] pub fn isr_mut(&self) -> *mut Isr { 
        self.isr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ISR register."]
    #[inline] pub fn isr_ptr(&self) -> *const Isr { 
        self.isr_reg().ptr()
    }

    #[doc="Read the ISR register."]
    #[inline] pub fn isr(&self) -> Isr { 
        self.isr_reg().read()
    }

    #[doc="Write the ISR register."]
    #[inline] pub fn write_isr(&self, value: Isr) -> &Self { 
        self.isr_reg().write(value);
        self
    }

    #[doc="Set the ISR register."]
    #[inline] pub fn set_isr<F: FnOnce(Isr) -> Isr>(&self, f: F) -> &Self {
        self.isr_reg().set(f);
        self
    }

    #[doc="Modify the ISR register."]
    #[inline] pub fn with_isr<F: FnOnce(Isr) -> Isr>(&self, f: F) -> &Self {
        self.isr_reg().with(f);
        self
    }

    #[doc="Get the IER Register."]
    #[inline] pub fn ier_reg(&self) -> ::bobbin_mcu::register::Register<Ier> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ier, 0x4)
    }

    #[doc="Get the *mut pointer for the IER register."]
    #[inline] pub fn ier_mut(&self) -> *mut Ier { 
        self.ier_reg().ptr()
    }

    #[doc="Get the *const pointer for the IER register."]
    #[inline] pub fn ier_ptr(&self) -> *const Ier { 
        self.ier_reg().ptr()
    }

    #[doc="Read the IER register."]
    #[inline] pub fn ier(&self) -> Ier { 
        self.ier_reg().read()
    }

    #[doc="Write the IER register."]
    #[inline] pub fn write_ier(&self, value: Ier) -> &Self { 
        self.ier_reg().write(value);
        self
    }

    #[doc="Set the IER register."]
    #[inline] pub fn set_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        self.ier_reg().set(f);
        self
    }

    #[doc="Modify the IER register."]
    #[inline] pub fn with_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        self.ier_reg().with(f);
        self
    }

    #[doc="Get the CR Register."]
    #[inline] pub fn cr_reg(&self) -> ::bobbin_mcu::register::Register<Cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr, 0x8)
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

    #[doc="Get the CFGR Register."]
    #[inline] pub fn cfgr_reg(&self) -> ::bobbin_mcu::register::Register<Cfgr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cfgr, 0xc)
    }

    #[doc="Get the *mut pointer for the CFGR register."]
    #[inline] pub fn cfgr_mut(&self) -> *mut Cfgr { 
        self.cfgr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CFGR register."]
    #[inline] pub fn cfgr_ptr(&self) -> *const Cfgr { 
        self.cfgr_reg().ptr()
    }

    #[doc="Read the CFGR register."]
    #[inline] pub fn cfgr(&self) -> Cfgr { 
        self.cfgr_reg().read()
    }

    #[doc="Write the CFGR register."]
    #[inline] pub fn write_cfgr(&self, value: Cfgr) -> &Self { 
        self.cfgr_reg().write(value);
        self
    }

    #[doc="Set the CFGR register."]
    #[inline] pub fn set_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&self, f: F) -> &Self {
        self.cfgr_reg().set(f);
        self
    }

    #[doc="Modify the CFGR register."]
    #[inline] pub fn with_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&self, f: F) -> &Self {
        self.cfgr_reg().with(f);
        self
    }

    #[doc="Get the SMPR1 Register."]
    #[inline] pub fn smpr1_reg(&self) -> ::bobbin_mcu::register::Register<Smpr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Smpr1, 0x14)
    }

    #[doc="Get the *mut pointer for the SMPR1 register."]
    #[inline] pub fn smpr1_mut(&self) -> *mut Smpr1 { 
        self.smpr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the SMPR1 register."]
    #[inline] pub fn smpr1_ptr(&self) -> *const Smpr1 { 
        self.smpr1_reg().ptr()
    }

    #[doc="Read the SMPR1 register."]
    #[inline] pub fn smpr1(&self) -> Smpr1 { 
        self.smpr1_reg().read()
    }

    #[doc="Write the SMPR1 register."]
    #[inline] pub fn write_smpr1(&self, value: Smpr1) -> &Self { 
        self.smpr1_reg().write(value);
        self
    }

    #[doc="Set the SMPR1 register."]
    #[inline] pub fn set_smpr1<F: FnOnce(Smpr1) -> Smpr1>(&self, f: F) -> &Self {
        self.smpr1_reg().set(f);
        self
    }

    #[doc="Modify the SMPR1 register."]
    #[inline] pub fn with_smpr1<F: FnOnce(Smpr1) -> Smpr1>(&self, f: F) -> &Self {
        self.smpr1_reg().with(f);
        self
    }

    #[doc="Get the SMPR2 Register."]
    #[inline] pub fn smpr2_reg(&self) -> ::bobbin_mcu::register::Register<Smpr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Smpr2, 0x18)
    }

    #[doc="Get the *mut pointer for the SMPR2 register."]
    #[inline] pub fn smpr2_mut(&self) -> *mut Smpr2 { 
        self.smpr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the SMPR2 register."]
    #[inline] pub fn smpr2_ptr(&self) -> *const Smpr2 { 
        self.smpr2_reg().ptr()
    }

    #[doc="Read the SMPR2 register."]
    #[inline] pub fn smpr2(&self) -> Smpr2 { 
        self.smpr2_reg().read()
    }

    #[doc="Write the SMPR2 register."]
    #[inline] pub fn write_smpr2(&self, value: Smpr2) -> &Self { 
        self.smpr2_reg().write(value);
        self
    }

    #[doc="Set the SMPR2 register."]
    #[inline] pub fn set_smpr2<F: FnOnce(Smpr2) -> Smpr2>(&self, f: F) -> &Self {
        self.smpr2_reg().set(f);
        self
    }

    #[doc="Modify the SMPR2 register."]
    #[inline] pub fn with_smpr2<F: FnOnce(Smpr2) -> Smpr2>(&self, f: F) -> &Self {
        self.smpr2_reg().with(f);
        self
    }

    #[doc="Get the TR1 Register."]
    #[inline] pub fn tr1_reg(&self) -> ::bobbin_mcu::register::Register<Tr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tr1, 0x20)
    }

    #[doc="Get the *mut pointer for the TR1 register."]
    #[inline] pub fn tr1_mut(&self) -> *mut Tr1 { 
        self.tr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the TR1 register."]
    #[inline] pub fn tr1_ptr(&self) -> *const Tr1 { 
        self.tr1_reg().ptr()
    }

    #[doc="Read the TR1 register."]
    #[inline] pub fn tr1(&self) -> Tr1 { 
        self.tr1_reg().read()
    }

    #[doc="Write the TR1 register."]
    #[inline] pub fn write_tr1(&self, value: Tr1) -> &Self { 
        self.tr1_reg().write(value);
        self
    }

    #[doc="Set the TR1 register."]
    #[inline] pub fn set_tr1<F: FnOnce(Tr1) -> Tr1>(&self, f: F) -> &Self {
        self.tr1_reg().set(f);
        self
    }

    #[doc="Modify the TR1 register."]
    #[inline] pub fn with_tr1<F: FnOnce(Tr1) -> Tr1>(&self, f: F) -> &Self {
        self.tr1_reg().with(f);
        self
    }

    #[doc="Get the TR2 Register."]
    #[inline] pub fn tr2_reg(&self) -> ::bobbin_mcu::register::Register<Tr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tr2, 0x24)
    }

    #[doc="Get the *mut pointer for the TR2 register."]
    #[inline] pub fn tr2_mut(&self) -> *mut Tr2 { 
        self.tr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the TR2 register."]
    #[inline] pub fn tr2_ptr(&self) -> *const Tr2 { 
        self.tr2_reg().ptr()
    }

    #[doc="Read the TR2 register."]
    #[inline] pub fn tr2(&self) -> Tr2 { 
        self.tr2_reg().read()
    }

    #[doc="Write the TR2 register."]
    #[inline] pub fn write_tr2(&self, value: Tr2) -> &Self { 
        self.tr2_reg().write(value);
        self
    }

    #[doc="Set the TR2 register."]
    #[inline] pub fn set_tr2<F: FnOnce(Tr2) -> Tr2>(&self, f: F) -> &Self {
        self.tr2_reg().set(f);
        self
    }

    #[doc="Modify the TR2 register."]
    #[inline] pub fn with_tr2<F: FnOnce(Tr2) -> Tr2>(&self, f: F) -> &Self {
        self.tr2_reg().with(f);
        self
    }

    #[doc="Get the TR3 Register."]
    #[inline] pub fn tr3_reg(&self) -> ::bobbin_mcu::register::Register<Tr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tr3, 0x28)
    }

    #[doc="Get the *mut pointer for the TR3 register."]
    #[inline] pub fn tr3_mut(&self) -> *mut Tr3 { 
        self.tr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the TR3 register."]
    #[inline] pub fn tr3_ptr(&self) -> *const Tr3 { 
        self.tr3_reg().ptr()
    }

    #[doc="Read the TR3 register."]
    #[inline] pub fn tr3(&self) -> Tr3 { 
        self.tr3_reg().read()
    }

    #[doc="Write the TR3 register."]
    #[inline] pub fn write_tr3(&self, value: Tr3) -> &Self { 
        self.tr3_reg().write(value);
        self
    }

    #[doc="Set the TR3 register."]
    #[inline] pub fn set_tr3<F: FnOnce(Tr3) -> Tr3>(&self, f: F) -> &Self {
        self.tr3_reg().set(f);
        self
    }

    #[doc="Modify the TR3 register."]
    #[inline] pub fn with_tr3<F: FnOnce(Tr3) -> Tr3>(&self, f: F) -> &Self {
        self.tr3_reg().with(f);
        self
    }

    #[doc="Get the SQR1 Register."]
    #[inline] pub fn sqr1_reg(&self) -> ::bobbin_mcu::register::Register<Sqr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sqr1, 0x30)
    }

    #[doc="Get the *mut pointer for the SQR1 register."]
    #[inline] pub fn sqr1_mut(&self) -> *mut Sqr1 { 
        self.sqr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the SQR1 register."]
    #[inline] pub fn sqr1_ptr(&self) -> *const Sqr1 { 
        self.sqr1_reg().ptr()
    }

    #[doc="Read the SQR1 register."]
    #[inline] pub fn sqr1(&self) -> Sqr1 { 
        self.sqr1_reg().read()
    }

    #[doc="Write the SQR1 register."]
    #[inline] pub fn write_sqr1(&self, value: Sqr1) -> &Self { 
        self.sqr1_reg().write(value);
        self
    }

    #[doc="Set the SQR1 register."]
    #[inline] pub fn set_sqr1<F: FnOnce(Sqr1) -> Sqr1>(&self, f: F) -> &Self {
        self.sqr1_reg().set(f);
        self
    }

    #[doc="Modify the SQR1 register."]
    #[inline] pub fn with_sqr1<F: FnOnce(Sqr1) -> Sqr1>(&self, f: F) -> &Self {
        self.sqr1_reg().with(f);
        self
    }

    #[doc="Get the SQR2 Register."]
    #[inline] pub fn sqr2_reg(&self) -> ::bobbin_mcu::register::Register<Sqr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sqr2, 0x34)
    }

    #[doc="Get the *mut pointer for the SQR2 register."]
    #[inline] pub fn sqr2_mut(&self) -> *mut Sqr2 { 
        self.sqr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the SQR2 register."]
    #[inline] pub fn sqr2_ptr(&self) -> *const Sqr2 { 
        self.sqr2_reg().ptr()
    }

    #[doc="Read the SQR2 register."]
    #[inline] pub fn sqr2(&self) -> Sqr2 { 
        self.sqr2_reg().read()
    }

    #[doc="Write the SQR2 register."]
    #[inline] pub fn write_sqr2(&self, value: Sqr2) -> &Self { 
        self.sqr2_reg().write(value);
        self
    }

    #[doc="Set the SQR2 register."]
    #[inline] pub fn set_sqr2<F: FnOnce(Sqr2) -> Sqr2>(&self, f: F) -> &Self {
        self.sqr2_reg().set(f);
        self
    }

    #[doc="Modify the SQR2 register."]
    #[inline] pub fn with_sqr2<F: FnOnce(Sqr2) -> Sqr2>(&self, f: F) -> &Self {
        self.sqr2_reg().with(f);
        self
    }

    #[doc="Get the SQR3 Register."]
    #[inline] pub fn sqr3_reg(&self) -> ::bobbin_mcu::register::Register<Sqr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sqr3, 0x38)
    }

    #[doc="Get the *mut pointer for the SQR3 register."]
    #[inline] pub fn sqr3_mut(&self) -> *mut Sqr3 { 
        self.sqr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the SQR3 register."]
    #[inline] pub fn sqr3_ptr(&self) -> *const Sqr3 { 
        self.sqr3_reg().ptr()
    }

    #[doc="Read the SQR3 register."]
    #[inline] pub fn sqr3(&self) -> Sqr3 { 
        self.sqr3_reg().read()
    }

    #[doc="Write the SQR3 register."]
    #[inline] pub fn write_sqr3(&self, value: Sqr3) -> &Self { 
        self.sqr3_reg().write(value);
        self
    }

    #[doc="Set the SQR3 register."]
    #[inline] pub fn set_sqr3<F: FnOnce(Sqr3) -> Sqr3>(&self, f: F) -> &Self {
        self.sqr3_reg().set(f);
        self
    }

    #[doc="Modify the SQR3 register."]
    #[inline] pub fn with_sqr3<F: FnOnce(Sqr3) -> Sqr3>(&self, f: F) -> &Self {
        self.sqr3_reg().with(f);
        self
    }

    #[doc="Get the SQR4 Register."]
    #[inline] pub fn sqr4_reg(&self) -> ::bobbin_mcu::register::Register<Sqr4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sqr4, 0x3c)
    }

    #[doc="Get the *mut pointer for the SQR4 register."]
    #[inline] pub fn sqr4_mut(&self) -> *mut Sqr4 { 
        self.sqr4_reg().ptr()
    }

    #[doc="Get the *const pointer for the SQR4 register."]
    #[inline] pub fn sqr4_ptr(&self) -> *const Sqr4 { 
        self.sqr4_reg().ptr()
    }

    #[doc="Read the SQR4 register."]
    #[inline] pub fn sqr4(&self) -> Sqr4 { 
        self.sqr4_reg().read()
    }

    #[doc="Write the SQR4 register."]
    #[inline] pub fn write_sqr4(&self, value: Sqr4) -> &Self { 
        self.sqr4_reg().write(value);
        self
    }

    #[doc="Set the SQR4 register."]
    #[inline] pub fn set_sqr4<F: FnOnce(Sqr4) -> Sqr4>(&self, f: F) -> &Self {
        self.sqr4_reg().set(f);
        self
    }

    #[doc="Modify the SQR4 register."]
    #[inline] pub fn with_sqr4<F: FnOnce(Sqr4) -> Sqr4>(&self, f: F) -> &Self {
        self.sqr4_reg().with(f);
        self
    }

    #[doc="Get the DR Register."]
    #[inline] pub fn dr_reg(&self) -> ::bobbin_mcu::register::Register<Dr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dr, 0x40)
    }

    #[doc="Get the *mut pointer for the DR register."]
    #[inline] pub fn dr_mut(&self) -> *mut Dr { 
        self.dr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DR register."]
    #[inline] pub fn dr_ptr(&self) -> *const Dr { 
        self.dr_reg().ptr()
    }

    #[doc="Read the DR register."]
    #[inline] pub fn dr(&self) -> Dr { 
        self.dr_reg().read()
    }

    #[doc="Get the JSQR Register."]
    #[inline] pub fn jsqr_reg(&self) -> ::bobbin_mcu::register::Register<Jsqr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Jsqr, 0x4c)
    }

    #[doc="Get the *mut pointer for the JSQR register."]
    #[inline] pub fn jsqr_mut(&self) -> *mut Jsqr { 
        self.jsqr_reg().ptr()
    }

    #[doc="Get the *const pointer for the JSQR register."]
    #[inline] pub fn jsqr_ptr(&self) -> *const Jsqr { 
        self.jsqr_reg().ptr()
    }

    #[doc="Read the JSQR register."]
    #[inline] pub fn jsqr(&self) -> Jsqr { 
        self.jsqr_reg().read()
    }

    #[doc="Write the JSQR register."]
    #[inline] pub fn write_jsqr(&self, value: Jsqr) -> &Self { 
        self.jsqr_reg().write(value);
        self
    }

    #[doc="Set the JSQR register."]
    #[inline] pub fn set_jsqr<F: FnOnce(Jsqr) -> Jsqr>(&self, f: F) -> &Self {
        self.jsqr_reg().set(f);
        self
    }

    #[doc="Modify the JSQR register."]
    #[inline] pub fn with_jsqr<F: FnOnce(Jsqr) -> Jsqr>(&self, f: F) -> &Self {
        self.jsqr_reg().with(f);
        self
    }

    #[doc="Get the OFR1 Register."]
    #[inline] pub fn ofr1_reg(&self) -> ::bobbin_mcu::register::Register<Ofr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ofr1, 0x60)
    }

    #[doc="Get the *mut pointer for the OFR1 register."]
    #[inline] pub fn ofr1_mut(&self) -> *mut Ofr1 { 
        self.ofr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the OFR1 register."]
    #[inline] pub fn ofr1_ptr(&self) -> *const Ofr1 { 
        self.ofr1_reg().ptr()
    }

    #[doc="Read the OFR1 register."]
    #[inline] pub fn ofr1(&self) -> Ofr1 { 
        self.ofr1_reg().read()
    }

    #[doc="Write the OFR1 register."]
    #[inline] pub fn write_ofr1(&self, value: Ofr1) -> &Self { 
        self.ofr1_reg().write(value);
        self
    }

    #[doc="Set the OFR1 register."]
    #[inline] pub fn set_ofr1<F: FnOnce(Ofr1) -> Ofr1>(&self, f: F) -> &Self {
        self.ofr1_reg().set(f);
        self
    }

    #[doc="Modify the OFR1 register."]
    #[inline] pub fn with_ofr1<F: FnOnce(Ofr1) -> Ofr1>(&self, f: F) -> &Self {
        self.ofr1_reg().with(f);
        self
    }

    #[doc="Get the OFR2 Register."]
    #[inline] pub fn ofr2_reg(&self) -> ::bobbin_mcu::register::Register<Ofr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ofr2, 0x64)
    }

    #[doc="Get the *mut pointer for the OFR2 register."]
    #[inline] pub fn ofr2_mut(&self) -> *mut Ofr2 { 
        self.ofr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the OFR2 register."]
    #[inline] pub fn ofr2_ptr(&self) -> *const Ofr2 { 
        self.ofr2_reg().ptr()
    }

    #[doc="Read the OFR2 register."]
    #[inline] pub fn ofr2(&self) -> Ofr2 { 
        self.ofr2_reg().read()
    }

    #[doc="Write the OFR2 register."]
    #[inline] pub fn write_ofr2(&self, value: Ofr2) -> &Self { 
        self.ofr2_reg().write(value);
        self
    }

    #[doc="Set the OFR2 register."]
    #[inline] pub fn set_ofr2<F: FnOnce(Ofr2) -> Ofr2>(&self, f: F) -> &Self {
        self.ofr2_reg().set(f);
        self
    }

    #[doc="Modify the OFR2 register."]
    #[inline] pub fn with_ofr2<F: FnOnce(Ofr2) -> Ofr2>(&self, f: F) -> &Self {
        self.ofr2_reg().with(f);
        self
    }

    #[doc="Get the OFR3 Register."]
    #[inline] pub fn ofr3_reg(&self) -> ::bobbin_mcu::register::Register<Ofr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ofr3, 0x68)
    }

    #[doc="Get the *mut pointer for the OFR3 register."]
    #[inline] pub fn ofr3_mut(&self) -> *mut Ofr3 { 
        self.ofr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the OFR3 register."]
    #[inline] pub fn ofr3_ptr(&self) -> *const Ofr3 { 
        self.ofr3_reg().ptr()
    }

    #[doc="Read the OFR3 register."]
    #[inline] pub fn ofr3(&self) -> Ofr3 { 
        self.ofr3_reg().read()
    }

    #[doc="Write the OFR3 register."]
    #[inline] pub fn write_ofr3(&self, value: Ofr3) -> &Self { 
        self.ofr3_reg().write(value);
        self
    }

    #[doc="Set the OFR3 register."]
    #[inline] pub fn set_ofr3<F: FnOnce(Ofr3) -> Ofr3>(&self, f: F) -> &Self {
        self.ofr3_reg().set(f);
        self
    }

    #[doc="Modify the OFR3 register."]
    #[inline] pub fn with_ofr3<F: FnOnce(Ofr3) -> Ofr3>(&self, f: F) -> &Self {
        self.ofr3_reg().with(f);
        self
    }

    #[doc="Get the OFR4 Register."]
    #[inline] pub fn ofr4_reg(&self) -> ::bobbin_mcu::register::Register<Ofr4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ofr4, 0x6c)
    }

    #[doc="Get the *mut pointer for the OFR4 register."]
    #[inline] pub fn ofr4_mut(&self) -> *mut Ofr4 { 
        self.ofr4_reg().ptr()
    }

    #[doc="Get the *const pointer for the OFR4 register."]
    #[inline] pub fn ofr4_ptr(&self) -> *const Ofr4 { 
        self.ofr4_reg().ptr()
    }

    #[doc="Read the OFR4 register."]
    #[inline] pub fn ofr4(&self) -> Ofr4 { 
        self.ofr4_reg().read()
    }

    #[doc="Write the OFR4 register."]
    #[inline] pub fn write_ofr4(&self, value: Ofr4) -> &Self { 
        self.ofr4_reg().write(value);
        self
    }

    #[doc="Set the OFR4 register."]
    #[inline] pub fn set_ofr4<F: FnOnce(Ofr4) -> Ofr4>(&self, f: F) -> &Self {
        self.ofr4_reg().set(f);
        self
    }

    #[doc="Modify the OFR4 register."]
    #[inline] pub fn with_ofr4<F: FnOnce(Ofr4) -> Ofr4>(&self, f: F) -> &Self {
        self.ofr4_reg().with(f);
        self
    }

    #[doc="Get the JDR1 Register."]
    #[inline] pub fn jdr1_reg(&self) -> ::bobbin_mcu::register::Register<Jdr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Jdr1, 0x80)
    }

    #[doc="Get the *mut pointer for the JDR1 register."]
    #[inline] pub fn jdr1_mut(&self) -> *mut Jdr1 { 
        self.jdr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the JDR1 register."]
    #[inline] pub fn jdr1_ptr(&self) -> *const Jdr1 { 
        self.jdr1_reg().ptr()
    }

    #[doc="Read the JDR1 register."]
    #[inline] pub fn jdr1(&self) -> Jdr1 { 
        self.jdr1_reg().read()
    }

    #[doc="Get the JDR2 Register."]
    #[inline] pub fn jdr2_reg(&self) -> ::bobbin_mcu::register::Register<Jdr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Jdr2, 0x84)
    }

    #[doc="Get the *mut pointer for the JDR2 register."]
    #[inline] pub fn jdr2_mut(&self) -> *mut Jdr2 { 
        self.jdr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the JDR2 register."]
    #[inline] pub fn jdr2_ptr(&self) -> *const Jdr2 { 
        self.jdr2_reg().ptr()
    }

    #[doc="Read the JDR2 register."]
    #[inline] pub fn jdr2(&self) -> Jdr2 { 
        self.jdr2_reg().read()
    }

    #[doc="Get the JDR3 Register."]
    #[inline] pub fn jdr3_reg(&self) -> ::bobbin_mcu::register::Register<Jdr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Jdr3, 0x88)
    }

    #[doc="Get the *mut pointer for the JDR3 register."]
    #[inline] pub fn jdr3_mut(&self) -> *mut Jdr3 { 
        self.jdr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the JDR3 register."]
    #[inline] pub fn jdr3_ptr(&self) -> *const Jdr3 { 
        self.jdr3_reg().ptr()
    }

    #[doc="Read the JDR3 register."]
    #[inline] pub fn jdr3(&self) -> Jdr3 { 
        self.jdr3_reg().read()
    }

    #[doc="Get the JDR4 Register."]
    #[inline] pub fn jdr4_reg(&self) -> ::bobbin_mcu::register::Register<Jdr4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Jdr4, 0x8c)
    }

    #[doc="Get the *mut pointer for the JDR4 register."]
    #[inline] pub fn jdr4_mut(&self) -> *mut Jdr4 { 
        self.jdr4_reg().ptr()
    }

    #[doc="Get the *const pointer for the JDR4 register."]
    #[inline] pub fn jdr4_ptr(&self) -> *const Jdr4 { 
        self.jdr4_reg().ptr()
    }

    #[doc="Read the JDR4 register."]
    #[inline] pub fn jdr4(&self) -> Jdr4 { 
        self.jdr4_reg().read()
    }

    #[doc="Get the AWD2CR Register."]
    #[inline] pub fn awd2cr_reg(&self) -> ::bobbin_mcu::register::Register<Awd2cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Awd2cr, 0xa0)
    }

    #[doc="Get the *mut pointer for the AWD2CR register."]
    #[inline] pub fn awd2cr_mut(&self) -> *mut Awd2cr { 
        self.awd2cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the AWD2CR register."]
    #[inline] pub fn awd2cr_ptr(&self) -> *const Awd2cr { 
        self.awd2cr_reg().ptr()
    }

    #[doc="Read the AWD2CR register."]
    #[inline] pub fn awd2cr(&self) -> Awd2cr { 
        self.awd2cr_reg().read()
    }

    #[doc="Write the AWD2CR register."]
    #[inline] pub fn write_awd2cr(&self, value: Awd2cr) -> &Self { 
        self.awd2cr_reg().write(value);
        self
    }

    #[doc="Set the AWD2CR register."]
    #[inline] pub fn set_awd2cr<F: FnOnce(Awd2cr) -> Awd2cr>(&self, f: F) -> &Self {
        self.awd2cr_reg().set(f);
        self
    }

    #[doc="Modify the AWD2CR register."]
    #[inline] pub fn with_awd2cr<F: FnOnce(Awd2cr) -> Awd2cr>(&self, f: F) -> &Self {
        self.awd2cr_reg().with(f);
        self
    }

    #[doc="Get the AWD3CR Register."]
    #[inline] pub fn awd3cr_reg(&self) -> ::bobbin_mcu::register::Register<Awd3cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Awd3cr, 0xa4)
    }

    #[doc="Get the *mut pointer for the AWD3CR register."]
    #[inline] pub fn awd3cr_mut(&self) -> *mut Awd3cr { 
        self.awd3cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the AWD3CR register."]
    #[inline] pub fn awd3cr_ptr(&self) -> *const Awd3cr { 
        self.awd3cr_reg().ptr()
    }

    #[doc="Read the AWD3CR register."]
    #[inline] pub fn awd3cr(&self) -> Awd3cr { 
        self.awd3cr_reg().read()
    }

    #[doc="Write the AWD3CR register."]
    #[inline] pub fn write_awd3cr(&self, value: Awd3cr) -> &Self { 
        self.awd3cr_reg().write(value);
        self
    }

    #[doc="Set the AWD3CR register."]
    #[inline] pub fn set_awd3cr<F: FnOnce(Awd3cr) -> Awd3cr>(&self, f: F) -> &Self {
        self.awd3cr_reg().set(f);
        self
    }

    #[doc="Modify the AWD3CR register."]
    #[inline] pub fn with_awd3cr<F: FnOnce(Awd3cr) -> Awd3cr>(&self, f: F) -> &Self {
        self.awd3cr_reg().with(f);
        self
    }

    #[doc="Get the DIFSEL Register."]
    #[inline] pub fn difsel_reg(&self) -> ::bobbin_mcu::register::Register<Difsel> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Difsel, 0xb0)
    }

    #[doc="Get the *mut pointer for the DIFSEL register."]
    #[inline] pub fn difsel_mut(&self) -> *mut Difsel { 
        self.difsel_reg().ptr()
    }

    #[doc="Get the *const pointer for the DIFSEL register."]
    #[inline] pub fn difsel_ptr(&self) -> *const Difsel { 
        self.difsel_reg().ptr()
    }

    #[doc="Read the DIFSEL register."]
    #[inline] pub fn difsel(&self) -> Difsel { 
        self.difsel_reg().read()
    }

    #[doc="Write the DIFSEL register."]
    #[inline] pub fn write_difsel(&self, value: Difsel) -> &Self { 
        self.difsel_reg().write(value);
        self
    }

    #[doc="Set the DIFSEL register."]
    #[inline] pub fn set_difsel<F: FnOnce(Difsel) -> Difsel>(&self, f: F) -> &Self {
        self.difsel_reg().set(f);
        self
    }

    #[doc="Modify the DIFSEL register."]
    #[inline] pub fn with_difsel<F: FnOnce(Difsel) -> Difsel>(&self, f: F) -> &Self {
        self.difsel_reg().with(f);
        self
    }

    #[doc="Get the CALFACT Register."]
    #[inline] pub fn calfact_reg(&self) -> ::bobbin_mcu::register::Register<Calfact> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Calfact, 0xb4)
    }

    #[doc="Get the *mut pointer for the CALFACT register."]
    #[inline] pub fn calfact_mut(&self) -> *mut Calfact { 
        self.calfact_reg().ptr()
    }

    #[doc="Get the *const pointer for the CALFACT register."]
    #[inline] pub fn calfact_ptr(&self) -> *const Calfact { 
        self.calfact_reg().ptr()
    }

    #[doc="Read the CALFACT register."]
    #[inline] pub fn calfact(&self) -> Calfact { 
        self.calfact_reg().read()
    }

    #[doc="Write the CALFACT register."]
    #[inline] pub fn write_calfact(&self, value: Calfact) -> &Self { 
        self.calfact_reg().write(value);
        self
    }

    #[doc="Set the CALFACT register."]
    #[inline] pub fn set_calfact<F: FnOnce(Calfact) -> Calfact>(&self, f: F) -> &Self {
        self.calfact_reg().set(f);
        self
    }

    #[doc="Modify the CALFACT register."]
    #[inline] pub fn with_calfact<F: FnOnce(Calfact) -> Calfact>(&self, f: F) -> &Self {
        self.calfact_reg().with(f);
        self
    }

}

#[doc="interrupt and status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc="JQOVF"]
    #[inline] pub fn jqovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if JQOVF != 0"]
    #[inline] pub fn test_jqovf(&self) -> bool {
        self.jqovf() != 0
    }

    #[doc="Sets the JQOVF field."]
    #[inline] pub fn set_jqovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="AWD3"]
    #[inline] pub fn awd3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if AWD3 != 0"]
    #[inline] pub fn test_awd3(&self) -> bool {
        self.awd3() != 0
    }

    #[doc="Sets the AWD3 field."]
    #[inline] pub fn set_awd3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="AWD2"]
    #[inline] pub fn awd2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if AWD2 != 0"]
    #[inline] pub fn test_awd2(&self) -> bool {
        self.awd2() != 0
    }

    #[doc="Sets the AWD2 field."]
    #[inline] pub fn set_awd2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="AWD1"]
    #[inline] pub fn awd1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if AWD1 != 0"]
    #[inline] pub fn test_awd1(&self) -> bool {
        self.awd1() != 0
    }

    #[doc="Sets the AWD1 field."]
    #[inline] pub fn set_awd1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="JEOS"]
    #[inline] pub fn jeos(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if JEOS != 0"]
    #[inline] pub fn test_jeos(&self) -> bool {
        self.jeos() != 0
    }

    #[doc="Sets the JEOS field."]
    #[inline] pub fn set_jeos<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="JEOC"]
    #[inline] pub fn jeoc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if JEOC != 0"]
    #[inline] pub fn test_jeoc(&self) -> bool {
        self.jeoc() != 0
    }

    #[doc="Sets the JEOC field."]
    #[inline] pub fn set_jeoc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="OVR"]
    #[inline] pub fn ovr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if OVR != 0"]
    #[inline] pub fn test_ovr(&self) -> bool {
        self.ovr() != 0
    }

    #[doc="Sets the OVR field."]
    #[inline] pub fn set_ovr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EOS"]
    #[inline] pub fn eos(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EOS != 0"]
    #[inline] pub fn test_eos(&self) -> bool {
        self.eos() != 0
    }

    #[doc="Sets the EOS field."]
    #[inline] pub fn set_eos<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="EOC"]
    #[inline] pub fn eoc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EOC != 0"]
    #[inline] pub fn test_eoc(&self) -> bool {
        self.eoc() != 0
    }

    #[doc="Sets the EOC field."]
    #[inline] pub fn set_eoc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="EOSMP"]
    #[inline] pub fn eosmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EOSMP != 0"]
    #[inline] pub fn test_eosmp(&self) -> bool {
        self.eosmp() != 0
    }

    #[doc="Sets the EOSMP field."]
    #[inline] pub fn set_eosmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="ADRDY"]
    #[inline] pub fn adrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ADRDY != 0"]
    #[inline] pub fn test_adrdy(&self) -> bool {
        self.adrdy() != 0
    }

    #[doc="Sets the ADRDY field."]
    #[inline] pub fn set_adrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Isr {
    #[inline]
    fn from(other: u32) -> Self {
         Isr(other)
    }
}

impl ::core::fmt::Display for Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jqovf() != 0 { try!(write!(f, " jqovf"))}
        if self.awd3() != 0 { try!(write!(f, " awd3"))}
        if self.awd2() != 0 { try!(write!(f, " awd2"))}
        if self.awd1() != 0 { try!(write!(f, " awd1"))}
        if self.jeos() != 0 { try!(write!(f, " jeos"))}
        if self.jeoc() != 0 { try!(write!(f, " jeoc"))}
        if self.ovr() != 0 { try!(write!(f, " ovr"))}
        if self.eos() != 0 { try!(write!(f, " eos"))}
        if self.eoc() != 0 { try!(write!(f, " eoc"))}
        if self.eosmp() != 0 { try!(write!(f, " eosmp"))}
        if self.adrdy() != 0 { try!(write!(f, " adrdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc="JQOVFIE"]
    #[inline] pub fn jqovfie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if JQOVFIE != 0"]
    #[inline] pub fn test_jqovfie(&self) -> bool {
        self.jqovfie() != 0
    }

    #[doc="Sets the JQOVFIE field."]
    #[inline] pub fn set_jqovfie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="AWD3IE"]
    #[inline] pub fn awd3ie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if AWD3IE != 0"]
    #[inline] pub fn test_awd3ie(&self) -> bool {
        self.awd3ie() != 0
    }

    #[doc="Sets the AWD3IE field."]
    #[inline] pub fn set_awd3ie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="AWD2IE"]
    #[inline] pub fn awd2ie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if AWD2IE != 0"]
    #[inline] pub fn test_awd2ie(&self) -> bool {
        self.awd2ie() != 0
    }

    #[doc="Sets the AWD2IE field."]
    #[inline] pub fn set_awd2ie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="AWD1IE"]
    #[inline] pub fn awd1ie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if AWD1IE != 0"]
    #[inline] pub fn test_awd1ie(&self) -> bool {
        self.awd1ie() != 0
    }

    #[doc="Sets the AWD1IE field."]
    #[inline] pub fn set_awd1ie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="JEOSIE"]
    #[inline] pub fn jeosie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if JEOSIE != 0"]
    #[inline] pub fn test_jeosie(&self) -> bool {
        self.jeosie() != 0
    }

    #[doc="Sets the JEOSIE field."]
    #[inline] pub fn set_jeosie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="JEOCIE"]
    #[inline] pub fn jeocie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if JEOCIE != 0"]
    #[inline] pub fn test_jeocie(&self) -> bool {
        self.jeocie() != 0
    }

    #[doc="Sets the JEOCIE field."]
    #[inline] pub fn set_jeocie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="OVRIE"]
    #[inline] pub fn ovrie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if OVRIE != 0"]
    #[inline] pub fn test_ovrie(&self) -> bool {
        self.ovrie() != 0
    }

    #[doc="Sets the OVRIE field."]
    #[inline] pub fn set_ovrie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EOSIE"]
    #[inline] pub fn eosie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EOSIE != 0"]
    #[inline] pub fn test_eosie(&self) -> bool {
        self.eosie() != 0
    }

    #[doc="Sets the EOSIE field."]
    #[inline] pub fn set_eosie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="EOCIE"]
    #[inline] pub fn eocie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EOCIE != 0"]
    #[inline] pub fn test_eocie(&self) -> bool {
        self.eocie() != 0
    }

    #[doc="Sets the EOCIE field."]
    #[inline] pub fn set_eocie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="EOSMPIE"]
    #[inline] pub fn eosmpie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EOSMPIE != 0"]
    #[inline] pub fn test_eosmpie(&self) -> bool {
        self.eosmpie() != 0
    }

    #[doc="Sets the EOSMPIE field."]
    #[inline] pub fn set_eosmpie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="ADRDYIE"]
    #[inline] pub fn adrdyie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ADRDYIE != 0"]
    #[inline] pub fn test_adrdyie(&self) -> bool {
        self.adrdyie() != 0
    }

    #[doc="Sets the ADRDYIE field."]
    #[inline] pub fn set_adrdyie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ier {
    #[inline]
    fn from(other: u32) -> Self {
         Ier(other)
    }
}

impl ::core::fmt::Display for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jqovfie() != 0 { try!(write!(f, " jqovfie"))}
        if self.awd3ie() != 0 { try!(write!(f, " awd3ie"))}
        if self.awd2ie() != 0 { try!(write!(f, " awd2ie"))}
        if self.awd1ie() != 0 { try!(write!(f, " awd1ie"))}
        if self.jeosie() != 0 { try!(write!(f, " jeosie"))}
        if self.jeocie() != 0 { try!(write!(f, " jeocie"))}
        if self.ovrie() != 0 { try!(write!(f, " ovrie"))}
        if self.eosie() != 0 { try!(write!(f, " eosie"))}
        if self.eocie() != 0 { try!(write!(f, " eocie"))}
        if self.eosmpie() != 0 { try!(write!(f, " eosmpie"))}
        if self.adrdyie() != 0 { try!(write!(f, " adrdyie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="ADCAL"]
    #[inline] pub fn adcal(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if ADCAL != 0"]
    #[inline] pub fn test_adcal(&self) -> bool {
        self.adcal() != 0
    }

    #[doc="Sets the ADCAL field."]
    #[inline] pub fn set_adcal<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="ADCALDIF"]
    #[inline] pub fn adcaldif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if ADCALDIF != 0"]
    #[inline] pub fn test_adcaldif(&self) -> bool {
        self.adcaldif() != 0
    }

    #[doc="Sets the ADCALDIF field."]
    #[inline] pub fn set_adcaldif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Deep Power Down"]
    #[inline] pub fn deeppwd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if DEEPPWD != 0"]
    #[inline] pub fn test_deeppwd(&self) -> bool {
        self.deeppwd() != 0
    }

    #[doc="Sets the DEEPPWD field."]
    #[inline] pub fn set_deeppwd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="ADVREGEN"]
    #[inline] pub fn advregen(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if ADVREGEN != 0"]
    #[inline] pub fn test_advregen(&self) -> bool {
        self.advregen() != 0
    }

    #[doc="Sets the ADVREGEN field."]
    #[inline] pub fn set_advregen<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="JADSTP"]
    #[inline] pub fn jadstp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if JADSTP != 0"]
    #[inline] pub fn test_jadstp(&self) -> bool {
        self.jadstp() != 0
    }

    #[doc="Sets the JADSTP field."]
    #[inline] pub fn set_jadstp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="ADSTP"]
    #[inline] pub fn adstp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ADSTP != 0"]
    #[inline] pub fn test_adstp(&self) -> bool {
        self.adstp() != 0
    }

    #[doc="Sets the ADSTP field."]
    #[inline] pub fn set_adstp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="JADSTART"]
    #[inline] pub fn jadstart(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if JADSTART != 0"]
    #[inline] pub fn test_jadstart(&self) -> bool {
        self.jadstart() != 0
    }

    #[doc="Sets the JADSTART field."]
    #[inline] pub fn set_jadstart<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="ADSTART"]
    #[inline] pub fn adstart(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ADSTART != 0"]
    #[inline] pub fn test_adstart(&self) -> bool {
        self.adstart() != 0
    }

    #[doc="Sets the ADSTART field."]
    #[inline] pub fn set_adstart<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="ADDIS"]
    #[inline] pub fn addis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ADDIS != 0"]
    #[inline] pub fn test_addis(&self) -> bool {
        self.addis() != 0
    }

    #[doc="Sets the ADDIS field."]
    #[inline] pub fn set_addis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="ADEN"]
    #[inline] pub fn aden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ADEN != 0"]
    #[inline] pub fn test_aden(&self) -> bool {
        self.aden() != 0
    }

    #[doc="Sets the ADEN field."]
    #[inline] pub fn set_aden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
        if self.adcal() != 0 { try!(write!(f, " adcal"))}
        if self.adcaldif() != 0 { try!(write!(f, " adcaldif"))}
        if self.deeppwd() != 0 { try!(write!(f, " deeppwd"))}
        if self.advregen() != 0 { try!(write!(f, " advregen=0x{:x}", self.advregen()))}
        if self.jadstp() != 0 { try!(write!(f, " jadstp"))}
        if self.adstp() != 0 { try!(write!(f, " adstp"))}
        if self.jadstart() != 0 { try!(write!(f, " jadstart"))}
        if self.adstart() != 0 { try!(write!(f, " adstart"))}
        if self.addis() != 0 { try!(write!(f, " addis"))}
        if self.aden() != 0 { try!(write!(f, " aden"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr(pub u32);
impl Cfgr {
    #[doc="AWDCH1CH"]
    #[inline] pub fn awdch1ch(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1f) as u8) } // [30:26]
    }

    #[doc="Returns true if AWDCH1CH != 0"]
    #[inline] pub fn test_awdch1ch(&self) -> bool {
        self.awdch1ch() != 0
    }

    #[doc="Sets the AWDCH1CH field."]
    #[inline] pub fn set_awdch1ch<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="JAUTO"]
    #[inline] pub fn jauto(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if JAUTO != 0"]
    #[inline] pub fn test_jauto(&self) -> bool {
        self.jauto() != 0
    }

    #[doc="Sets the JAUTO field."]
    #[inline] pub fn set_jauto<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="JAWD1EN"]
    #[inline] pub fn jawd1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if JAWD1EN != 0"]
    #[inline] pub fn test_jawd1en(&self) -> bool {
        self.jawd1en() != 0
    }

    #[doc="Sets the JAWD1EN field."]
    #[inline] pub fn set_jawd1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="AWD1EN"]
    #[inline] pub fn awd1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if AWD1EN != 0"]
    #[inline] pub fn test_awd1en(&self) -> bool {
        self.awd1en() != 0
    }

    #[doc="Sets the AWD1EN field."]
    #[inline] pub fn set_awd1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="AWD1SGL"]
    #[inline] pub fn awd1sgl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if AWD1SGL != 0"]
    #[inline] pub fn test_awd1sgl(&self) -> bool {
        self.awd1sgl() != 0
    }

    #[doc="Sets the AWD1SGL field."]
    #[inline] pub fn set_awd1sgl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="JQM"]
    #[inline] pub fn jqm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if JQM != 0"]
    #[inline] pub fn test_jqm(&self) -> bool {
        self.jqm() != 0
    }

    #[doc="Sets the JQM field."]
    #[inline] pub fn set_jqm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="JDISCEN"]
    #[inline] pub fn jdiscen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if JDISCEN != 0"]
    #[inline] pub fn test_jdiscen(&self) -> bool {
        self.jdiscen() != 0
    }

    #[doc="Sets the JDISCEN field."]
    #[inline] pub fn set_jdiscen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="DISCNUM"]
    #[inline] pub fn discnum(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x7) as u8) } // [19:17]
    }

    #[doc="Returns true if DISCNUM != 0"]
    #[inline] pub fn test_discnum(&self) -> bool {
        self.discnum() != 0
    }

    #[doc="Sets the DISCNUM field."]
    #[inline] pub fn set_discnum<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="DISCEN"]
    #[inline] pub fn discen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DISCEN != 0"]
    #[inline] pub fn test_discen(&self) -> bool {
        self.discen() != 0
    }

    #[doc="Sets the DISCEN field."]
    #[inline] pub fn set_discen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="AUTOFF"]
    #[inline] pub fn autoff(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if AUTOFF != 0"]
    #[inline] pub fn test_autoff(&self) -> bool {
        self.autoff() != 0
    }

    #[doc="Sets the AUTOFF field."]
    #[inline] pub fn set_autoff<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="AUTDLY"]
    #[inline] pub fn autdly(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if AUTDLY != 0"]
    #[inline] pub fn test_autdly(&self) -> bool {
        self.autdly() != 0
    }

    #[doc="Sets the AUTDLY field."]
    #[inline] pub fn set_autdly<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="CONT"]
    #[inline] pub fn cont(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CONT != 0"]
    #[inline] pub fn test_cont(&self) -> bool {
        self.cont() != 0
    }

    #[doc="Sets the CONT field."]
    #[inline] pub fn set_cont<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="OVRMOD"]
    #[inline] pub fn ovrmod(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if OVRMOD != 0"]
    #[inline] pub fn test_ovrmod(&self) -> bool {
        self.ovrmod() != 0
    }

    #[doc="Sets the OVRMOD field."]
    #[inline] pub fn set_ovrmod<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTEN"]
    #[inline] pub fn exten(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if EXTEN != 0"]
    #[inline] pub fn test_exten(&self) -> bool {
        self.exten() != 0
    }

    #[doc="Sets the EXTEN field."]
    #[inline] pub fn set_exten<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="EXTSEL"]
    #[inline] pub fn extsel(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0xf) as u8) } // [9:6]
    }

    #[doc="Returns true if EXTSEL != 0"]
    #[inline] pub fn test_extsel(&self) -> bool {
        self.extsel() != 0
    }

    #[doc="Sets the EXTSEL field."]
    #[inline] pub fn set_extsel<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="ALIGN"]
    #[inline] pub fn align(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ALIGN != 0"]
    #[inline] pub fn test_align(&self) -> bool {
        self.align() != 0
    }

    #[doc="Sets the ALIGN field."]
    #[inline] pub fn set_align<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="RES"]
    #[inline] pub fn res(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if RES != 0"]
    #[inline] pub fn test_res(&self) -> bool {
        self.res() != 0
    }

    #[doc="Sets the RES field."]
    #[inline] pub fn set_res<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="DMACFG"]
    #[inline] pub fn dmacfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DMACFG != 0"]
    #[inline] pub fn test_dmacfg(&self) -> bool {
        self.dmacfg() != 0
    }

    #[doc="Sets the DMACFG field."]
    #[inline] pub fn set_dmacfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DMAEN"]
    #[inline] pub fn dmaen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DMAEN != 0"]
    #[inline] pub fn test_dmaen(&self) -> bool {
        self.dmaen() != 0
    }

    #[doc="Sets the DMAEN field."]
    #[inline] pub fn set_dmaen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cfgr {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgr(other)
    }
}

impl ::core::fmt::Display for Cfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awdch1ch() != 0 { try!(write!(f, " awdch1ch=0x{:x}", self.awdch1ch()))}
        if self.jauto() != 0 { try!(write!(f, " jauto"))}
        if self.jawd1en() != 0 { try!(write!(f, " jawd1en"))}
        if self.awd1en() != 0 { try!(write!(f, " awd1en"))}
        if self.awd1sgl() != 0 { try!(write!(f, " awd1sgl"))}
        if self.jqm() != 0 { try!(write!(f, " jqm"))}
        if self.jdiscen() != 0 { try!(write!(f, " jdiscen"))}
        if self.discnum() != 0 { try!(write!(f, " discnum=0x{:x}", self.discnum()))}
        if self.discen() != 0 { try!(write!(f, " discen"))}
        if self.autoff() != 0 { try!(write!(f, " autoff"))}
        if self.autdly() != 0 { try!(write!(f, " autdly"))}
        if self.cont() != 0 { try!(write!(f, " cont"))}
        if self.ovrmod() != 0 { try!(write!(f, " ovrmod"))}
        if self.exten() != 0 { try!(write!(f, " exten=0x{:x}", self.exten()))}
        if self.extsel() != 0 { try!(write!(f, " extsel=0x{:x}", self.extsel()))}
        if self.align() != 0 { try!(write!(f, " align"))}
        if self.res() != 0 { try!(write!(f, " res=0x{:x}", self.res()))}
        if self.dmacfg() != 0 { try!(write!(f, " dmacfg"))}
        if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="sample time register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Smpr1(pub u32);
impl Smpr1 {
    #[doc="SMP9"]
    #[inline] pub fn smp9(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x7) as u8) } // [29:27]
    }

    #[doc="Returns true if SMP9 != 0"]
    #[inline] pub fn test_smp9(&self) -> bool {
        self.smp9() != 0
    }

    #[doc="Sets the SMP9 field."]
    #[inline] pub fn set_smp9<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="SMP8"]
    #[inline] pub fn smp8(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if SMP8 != 0"]
    #[inline] pub fn test_smp8(&self) -> bool {
        self.smp8() != 0
    }

    #[doc="Sets the SMP8 field."]
    #[inline] pub fn set_smp8<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="SMP7"]
    #[inline] pub fn smp7(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x7) as u8) } // [23:21]
    }

    #[doc="Returns true if SMP7 != 0"]
    #[inline] pub fn test_smp7(&self) -> bool {
        self.smp7() != 0
    }

    #[doc="Sets the SMP7 field."]
    #[inline] pub fn set_smp7<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="SMP6"]
    #[inline] pub fn smp6(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x7) as u8) } // [20:18]
    }

    #[doc="Returns true if SMP6 != 0"]
    #[inline] pub fn test_smp6(&self) -> bool {
        self.smp6() != 0
    }

    #[doc="Sets the SMP6 field."]
    #[inline] pub fn set_smp6<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="SMP5"]
    #[inline] pub fn smp5(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x7) as u8) } // [17:15]
    }

    #[doc="Returns true if SMP5 != 0"]
    #[inline] pub fn test_smp5(&self) -> bool {
        self.smp5() != 0
    }

    #[doc="Sets the SMP5 field."]
    #[inline] pub fn set_smp5<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="SMP4"]
    #[inline] pub fn smp4(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if SMP4 != 0"]
    #[inline] pub fn test_smp4(&self) -> bool {
        self.smp4() != 0
    }

    #[doc="Sets the SMP4 field."]
    #[inline] pub fn set_smp4<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SMP3"]
    #[inline] pub fn smp3(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x7) as u8) } // [11:9]
    }

    #[doc="Returns true if SMP3 != 0"]
    #[inline] pub fn test_smp3(&self) -> bool {
        self.smp3() != 0
    }

    #[doc="Sets the SMP3 field."]
    #[inline] pub fn set_smp3<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="SMP2"]
    #[inline] pub fn smp2(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x7) as u8) } // [8:6]
    }

    #[doc="Returns true if SMP2 != 0"]
    #[inline] pub fn test_smp2(&self) -> bool {
        self.smp2() != 0
    }

    #[doc="Sets the SMP2 field."]
    #[inline] pub fn set_smp2<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SMP1"]
    #[inline] pub fn smp1(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if SMP1 != 0"]
    #[inline] pub fn test_smp1(&self) -> bool {
        self.smp1() != 0
    }

    #[doc="Sets the SMP1 field."]
    #[inline] pub fn set_smp1<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Smpr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Smpr1(other)
    }
}

impl ::core::fmt::Display for Smpr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Smpr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.smp9() != 0 { try!(write!(f, " smp9=0x{:x}", self.smp9()))}
        if self.smp8() != 0 { try!(write!(f, " smp8=0x{:x}", self.smp8()))}
        if self.smp7() != 0 { try!(write!(f, " smp7=0x{:x}", self.smp7()))}
        if self.smp6() != 0 { try!(write!(f, " smp6=0x{:x}", self.smp6()))}
        if self.smp5() != 0 { try!(write!(f, " smp5=0x{:x}", self.smp5()))}
        if self.smp4() != 0 { try!(write!(f, " smp4=0x{:x}", self.smp4()))}
        if self.smp3() != 0 { try!(write!(f, " smp3=0x{:x}", self.smp3()))}
        if self.smp2() != 0 { try!(write!(f, " smp2=0x{:x}", self.smp2()))}
        if self.smp1() != 0 { try!(write!(f, " smp1=0x{:x}", self.smp1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="sample time register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Smpr2(pub u32);
impl Smpr2 {
    #[doc="SMP18"]
    #[inline] pub fn smp18(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if SMP18 != 0"]
    #[inline] pub fn test_smp18(&self) -> bool {
        self.smp18() != 0
    }

    #[doc="Sets the SMP18 field."]
    #[inline] pub fn set_smp18<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="SMP17"]
    #[inline] pub fn smp17(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x7) as u8) } // [23:21]
    }

    #[doc="Returns true if SMP17 != 0"]
    #[inline] pub fn test_smp17(&self) -> bool {
        self.smp17() != 0
    }

    #[doc="Sets the SMP17 field."]
    #[inline] pub fn set_smp17<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="SMP16"]
    #[inline] pub fn smp16(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x7) as u8) } // [20:18]
    }

    #[doc="Returns true if SMP16 != 0"]
    #[inline] pub fn test_smp16(&self) -> bool {
        self.smp16() != 0
    }

    #[doc="Sets the SMP16 field."]
    #[inline] pub fn set_smp16<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="SMP15"]
    #[inline] pub fn smp15(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x7) as u8) } // [17:15]
    }

    #[doc="Returns true if SMP15 != 0"]
    #[inline] pub fn test_smp15(&self) -> bool {
        self.smp15() != 0
    }

    #[doc="Sets the SMP15 field."]
    #[inline] pub fn set_smp15<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="SMP14"]
    #[inline] pub fn smp14(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if SMP14 != 0"]
    #[inline] pub fn test_smp14(&self) -> bool {
        self.smp14() != 0
    }

    #[doc="Sets the SMP14 field."]
    #[inline] pub fn set_smp14<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SMP13"]
    #[inline] pub fn smp13(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x7) as u8) } // [11:9]
    }

    #[doc="Returns true if SMP13 != 0"]
    #[inline] pub fn test_smp13(&self) -> bool {
        self.smp13() != 0
    }

    #[doc="Sets the SMP13 field."]
    #[inline] pub fn set_smp13<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="SMP12"]
    #[inline] pub fn smp12(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x7) as u8) } // [8:6]
    }

    #[doc="Returns true if SMP12 != 0"]
    #[inline] pub fn test_smp12(&self) -> bool {
        self.smp12() != 0
    }

    #[doc="Sets the SMP12 field."]
    #[inline] pub fn set_smp12<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SMP11"]
    #[inline] pub fn smp11(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if SMP11 != 0"]
    #[inline] pub fn test_smp11(&self) -> bool {
        self.smp11() != 0
    }

    #[doc="Sets the SMP11 field."]
    #[inline] pub fn set_smp11<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="SMP10"]
    #[inline] pub fn smp10(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if SMP10 != 0"]
    #[inline] pub fn test_smp10(&self) -> bool {
        self.smp10() != 0
    }

    #[doc="Sets the SMP10 field."]
    #[inline] pub fn set_smp10<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Smpr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Smpr2(other)
    }
}

impl ::core::fmt::Display for Smpr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Smpr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.smp18() != 0 { try!(write!(f, " smp18=0x{:x}", self.smp18()))}
        if self.smp17() != 0 { try!(write!(f, " smp17=0x{:x}", self.smp17()))}
        if self.smp16() != 0 { try!(write!(f, " smp16=0x{:x}", self.smp16()))}
        if self.smp15() != 0 { try!(write!(f, " smp15=0x{:x}", self.smp15()))}
        if self.smp14() != 0 { try!(write!(f, " smp14=0x{:x}", self.smp14()))}
        if self.smp13() != 0 { try!(write!(f, " smp13=0x{:x}", self.smp13()))}
        if self.smp12() != 0 { try!(write!(f, " smp12=0x{:x}", self.smp12()))}
        if self.smp11() != 0 { try!(write!(f, " smp11=0x{:x}", self.smp11()))}
        if self.smp10() != 0 { try!(write!(f, " smp10=0x{:x}", self.smp10()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="watchdog threshold register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tr1(pub u32);
impl Tr1 {
    #[doc="HT1"]
    #[inline] pub fn ht1(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xfff) as u16) } // [27:16]
    }

    #[doc="Returns true if HT1 != 0"]
    #[inline] pub fn test_ht1(&self) -> bool {
        self.ht1() != 0
    }

    #[doc="Sets the HT1 field."]
    #[inline] pub fn set_ht1<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="LT1"]
    #[inline] pub fn lt1(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if LT1 != 0"]
    #[inline] pub fn test_lt1(&self) -> bool {
        self.lt1() != 0
    }

    #[doc="Sets the LT1 field."]
    #[inline] pub fn set_lt1<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Tr1(other)
    }
}

impl ::core::fmt::Display for Tr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ht1() != 0 { try!(write!(f, " ht1=0x{:x}", self.ht1()))}
        if self.lt1() != 0 { try!(write!(f, " lt1=0x{:x}", self.lt1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="watchdog threshold register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tr2(pub u32);
impl Tr2 {
    #[doc="HT2"]
    #[inline] pub fn ht2(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if HT2 != 0"]
    #[inline] pub fn test_ht2(&self) -> bool {
        self.ht2() != 0
    }

    #[doc="Sets the HT2 field."]
    #[inline] pub fn set_ht2<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="LT2"]
    #[inline] pub fn lt2(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if LT2 != 0"]
    #[inline] pub fn test_lt2(&self) -> bool {
        self.lt2() != 0
    }

    #[doc="Sets the LT2 field."]
    #[inline] pub fn set_lt2<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Tr2(other)
    }
}

impl ::core::fmt::Display for Tr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ht2() != 0 { try!(write!(f, " ht2=0x{:x}", self.ht2()))}
        if self.lt2() != 0 { try!(write!(f, " lt2=0x{:x}", self.lt2()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="watchdog threshold register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tr3(pub u32);
impl Tr3 {
    #[doc="HT3"]
    #[inline] pub fn ht3(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if HT3 != 0"]
    #[inline] pub fn test_ht3(&self) -> bool {
        self.ht3() != 0
    }

    #[doc="Sets the HT3 field."]
    #[inline] pub fn set_ht3<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="LT3"]
    #[inline] pub fn lt3(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if LT3 != 0"]
    #[inline] pub fn test_lt3(&self) -> bool {
        self.lt3() != 0
    }

    #[doc="Sets the LT3 field."]
    #[inline] pub fn set_lt3<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Tr3(other)
    }
}

impl ::core::fmt::Display for Tr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ht3() != 0 { try!(write!(f, " ht3=0x{:x}", self.ht3()))}
        if self.lt3() != 0 { try!(write!(f, " lt3=0x{:x}", self.lt3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="regular sequence register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sqr1(pub u32);
impl Sqr1 {
    #[doc="SQ4"]
    #[inline] pub fn sq4(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if SQ4 != 0"]
    #[inline] pub fn test_sq4(&self) -> bool {
        self.sq4() != 0
    }

    #[doc="Sets the SQ4 field."]
    #[inline] pub fn set_sq4<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="SQ3"]
    #[inline] pub fn sq3(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1f) as u8) } // [22:18]
    }

    #[doc="Returns true if SQ3 != 0"]
    #[inline] pub fn test_sq3(&self) -> bool {
        self.sq3() != 0
    }

    #[doc="Sets the SQ3 field."]
    #[inline] pub fn set_sq3<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="SQ2"]
    #[inline] pub fn sq2(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1f) as u8) } // [16:12]
    }

    #[doc="Returns true if SQ2 != 0"]
    #[inline] pub fn test_sq2(&self) -> bool {
        self.sq2() != 0
    }

    #[doc="Sets the SQ2 field."]
    #[inline] pub fn set_sq2<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SQ1"]
    #[inline] pub fn sq1(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1f) as u8) } // [10:6]
    }

    #[doc="Returns true if SQ1 != 0"]
    #[inline] pub fn test_sq1(&self) -> bool {
        self.sq1() != 0
    }

    #[doc="Sets the SQ1 field."]
    #[inline] pub fn set_sq1<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="L"]
    #[inline] pub fn l(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if L != 0"]
    #[inline] pub fn test_l(&self) -> bool {
        self.l() != 0
    }

    #[doc="Sets the L field."]
    #[inline] pub fn set_l<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sqr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Sqr1(other)
    }
}

impl ::core::fmt::Display for Sqr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sqr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sq4() != 0 { try!(write!(f, " sq4=0x{:x}", self.sq4()))}
        if self.sq3() != 0 { try!(write!(f, " sq3=0x{:x}", self.sq3()))}
        if self.sq2() != 0 { try!(write!(f, " sq2=0x{:x}", self.sq2()))}
        if self.sq1() != 0 { try!(write!(f, " sq1=0x{:x}", self.sq1()))}
        if self.l() != 0 { try!(write!(f, " l=0x{:x}", self.l()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="regular sequence register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sqr2(pub u32);
impl Sqr2 {
    #[doc="SQ9"]
    #[inline] pub fn sq9(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if SQ9 != 0"]
    #[inline] pub fn test_sq9(&self) -> bool {
        self.sq9() != 0
    }

    #[doc="Sets the SQ9 field."]
    #[inline] pub fn set_sq9<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="SQ8"]
    #[inline] pub fn sq8(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1f) as u8) } // [22:18]
    }

    #[doc="Returns true if SQ8 != 0"]
    #[inline] pub fn test_sq8(&self) -> bool {
        self.sq8() != 0
    }

    #[doc="Sets the SQ8 field."]
    #[inline] pub fn set_sq8<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="SQ7"]
    #[inline] pub fn sq7(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1f) as u8) } // [16:12]
    }

    #[doc="Returns true if SQ7 != 0"]
    #[inline] pub fn test_sq7(&self) -> bool {
        self.sq7() != 0
    }

    #[doc="Sets the SQ7 field."]
    #[inline] pub fn set_sq7<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SQ6"]
    #[inline] pub fn sq6(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1f) as u8) } // [10:6]
    }

    #[doc="Returns true if SQ6 != 0"]
    #[inline] pub fn test_sq6(&self) -> bool {
        self.sq6() != 0
    }

    #[doc="Sets the SQ6 field."]
    #[inline] pub fn set_sq6<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SQ5"]
    #[inline] pub fn sq5(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if SQ5 != 0"]
    #[inline] pub fn test_sq5(&self) -> bool {
        self.sq5() != 0
    }

    #[doc="Sets the SQ5 field."]
    #[inline] pub fn set_sq5<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sqr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Sqr2(other)
    }
}

impl ::core::fmt::Display for Sqr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sqr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sq9() != 0 { try!(write!(f, " sq9=0x{:x}", self.sq9()))}
        if self.sq8() != 0 { try!(write!(f, " sq8=0x{:x}", self.sq8()))}
        if self.sq7() != 0 { try!(write!(f, " sq7=0x{:x}", self.sq7()))}
        if self.sq6() != 0 { try!(write!(f, " sq6=0x{:x}", self.sq6()))}
        if self.sq5() != 0 { try!(write!(f, " sq5=0x{:x}", self.sq5()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="regular sequence register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sqr3(pub u32);
impl Sqr3 {
    #[doc="SQ14"]
    #[inline] pub fn sq14(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if SQ14 != 0"]
    #[inline] pub fn test_sq14(&self) -> bool {
        self.sq14() != 0
    }

    #[doc="Sets the SQ14 field."]
    #[inline] pub fn set_sq14<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="SQ13"]
    #[inline] pub fn sq13(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1f) as u8) } // [22:18]
    }

    #[doc="Returns true if SQ13 != 0"]
    #[inline] pub fn test_sq13(&self) -> bool {
        self.sq13() != 0
    }

    #[doc="Sets the SQ13 field."]
    #[inline] pub fn set_sq13<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="SQ12"]
    #[inline] pub fn sq12(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1f) as u8) } // [16:12]
    }

    #[doc="Returns true if SQ12 != 0"]
    #[inline] pub fn test_sq12(&self) -> bool {
        self.sq12() != 0
    }

    #[doc="Sets the SQ12 field."]
    #[inline] pub fn set_sq12<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SQ11"]
    #[inline] pub fn sq11(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1f) as u8) } // [10:6]
    }

    #[doc="Returns true if SQ11 != 0"]
    #[inline] pub fn test_sq11(&self) -> bool {
        self.sq11() != 0
    }

    #[doc="Sets the SQ11 field."]
    #[inline] pub fn set_sq11<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SQ10"]
    #[inline] pub fn sq10(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if SQ10 != 0"]
    #[inline] pub fn test_sq10(&self) -> bool {
        self.sq10() != 0
    }

    #[doc="Sets the SQ10 field."]
    #[inline] pub fn set_sq10<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sqr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Sqr3(other)
    }
}

impl ::core::fmt::Display for Sqr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sqr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sq14() != 0 { try!(write!(f, " sq14=0x{:x}", self.sq14()))}
        if self.sq13() != 0 { try!(write!(f, " sq13=0x{:x}", self.sq13()))}
        if self.sq12() != 0 { try!(write!(f, " sq12=0x{:x}", self.sq12()))}
        if self.sq11() != 0 { try!(write!(f, " sq11=0x{:x}", self.sq11()))}
        if self.sq10() != 0 { try!(write!(f, " sq10=0x{:x}", self.sq10()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="regular sequence register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sqr4(pub u32);
impl Sqr4 {
    #[doc="SQ16"]
    #[inline] pub fn sq16(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1f) as u8) } // [10:6]
    }

    #[doc="Returns true if SQ16 != 0"]
    #[inline] pub fn test_sq16(&self) -> bool {
        self.sq16() != 0
    }

    #[doc="Sets the SQ16 field."]
    #[inline] pub fn set_sq16<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SQ15"]
    #[inline] pub fn sq15(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if SQ15 != 0"]
    #[inline] pub fn test_sq15(&self) -> bool {
        self.sq15() != 0
    }

    #[doc="Sets the SQ15 field."]
    #[inline] pub fn set_sq15<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sqr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Sqr4(other)
    }
}

impl ::core::fmt::Display for Sqr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sqr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sq16() != 0 { try!(write!(f, " sq16=0x{:x}", self.sq16()))}
        if self.sq15() != 0 { try!(write!(f, " sq15=0x{:x}", self.sq15()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc="DATA"]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DATA (16 bit)"]
    #[inline] pub fn data_16(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if DATA_16 != 0"]
    #[inline] pub fn test_data_16(&self) -> bool {
        self.data_16() != 0
    }

    #[doc="Sets the DATA_16 field."]
    #[inline] pub fn set_data_16<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DATA (12 bit)"]
    #[inline] pub fn data_12(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if DATA_12 != 0"]
    #[inline] pub fn test_data_12(&self) -> bool {
        self.data_12() != 0
    }

    #[doc="Sets the DATA_12 field."]
    #[inline] pub fn set_data_12<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DATA (10 bit)"]
    #[inline] pub fn data_10(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if DATA_10 != 0"]
    #[inline] pub fn test_data_10(&self) -> bool {
        self.data_10() != 0
    }

    #[doc="Sets the DATA_10 field."]
    #[inline] pub fn set_data_10<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DATA (8 bit)"]
    #[inline] pub fn data_8(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA_8 != 0"]
    #[inline] pub fn test_data_8(&self) -> bool {
        self.data_8() != 0
    }

    #[doc="Sets the DATA_8 field."]
    #[inline] pub fn set_data_8<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DATA (6 bit)"]
    #[inline] pub fn data_6(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if DATA_6 != 0"]
    #[inline] pub fn test_data_6(&self) -> bool {
        self.data_6() != 0
    }

    #[doc="Sets the DATA_6 field."]
    #[inline] pub fn set_data_6<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
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
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        if self.data_16() != 0 { try!(write!(f, " data_16=0x{:x}", self.data_16()))}
        if self.data_12() != 0 { try!(write!(f, " data_12=0x{:x}", self.data_12()))}
        if self.data_10() != 0 { try!(write!(f, " data_10=0x{:x}", self.data_10()))}
        if self.data_8() != 0 { try!(write!(f, " data_8=0x{:x}", self.data_8()))}
        if self.data_6() != 0 { try!(write!(f, " data_6=0x{:x}", self.data_6()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="injected sequence register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Jsqr(pub u32);
impl Jsqr {
    #[doc="JSQ4"]
    #[inline] pub fn jsq4(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1f) as u8) } // [30:26]
    }

    #[doc="Returns true if JSQ4 != 0"]
    #[inline] pub fn test_jsq4(&self) -> bool {
        self.jsq4() != 0
    }

    #[doc="Sets the JSQ4 field."]
    #[inline] pub fn set_jsq4<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="JSQ3"]
    #[inline] pub fn jsq3(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1f) as u8) } // [24:20]
    }

    #[doc="Returns true if JSQ3 != 0"]
    #[inline] pub fn test_jsq3(&self) -> bool {
        self.jsq3() != 0
    }

    #[doc="Sets the JSQ3 field."]
    #[inline] pub fn set_jsq3<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="JSQ2"]
    #[inline] pub fn jsq2(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1f) as u8) } // [18:14]
    }

    #[doc="Returns true if JSQ2 != 0"]
    #[inline] pub fn test_jsq2(&self) -> bool {
        self.jsq2() != 0
    }

    #[doc="Sets the JSQ2 field."]
    #[inline] pub fn set_jsq2<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="JSQ1"]
    #[inline] pub fn jsq1(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Returns true if JSQ1 != 0"]
    #[inline] pub fn test_jsq1(&self) -> bool {
        self.jsq1() != 0
    }

    #[doc="Sets the JSQ1 field."]
    #[inline] pub fn set_jsq1<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="JEXTEN"]
    #[inline] pub fn jexten(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if JEXTEN != 0"]
    #[inline] pub fn test_jexten(&self) -> bool {
        self.jexten() != 0
    }

    #[doc="Sets the JEXTEN field."]
    #[inline] pub fn set_jexten<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="JEXTSEL"]
    #[inline] pub fn jextsel(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0xf) as u8) } // [5:2]
    }

    #[doc="Returns true if JEXTSEL != 0"]
    #[inline] pub fn test_jextsel(&self) -> bool {
        self.jextsel() != 0
    }

    #[doc="Sets the JEXTSEL field."]
    #[inline] pub fn set_jextsel<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="JL"]
    #[inline] pub fn jl(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if JL != 0"]
    #[inline] pub fn test_jl(&self) -> bool {
        self.jl() != 0
    }

    #[doc="Sets the JL field."]
    #[inline] pub fn set_jl<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Jsqr {
    #[inline]
    fn from(other: u32) -> Self {
         Jsqr(other)
    }
}

impl ::core::fmt::Display for Jsqr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Jsqr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jsq4() != 0 { try!(write!(f, " jsq4=0x{:x}", self.jsq4()))}
        if self.jsq3() != 0 { try!(write!(f, " jsq3=0x{:x}", self.jsq3()))}
        if self.jsq2() != 0 { try!(write!(f, " jsq2=0x{:x}", self.jsq2()))}
        if self.jsq1() != 0 { try!(write!(f, " jsq1=0x{:x}", self.jsq1()))}
        if self.jexten() != 0 { try!(write!(f, " jexten=0x{:x}", self.jexten()))}
        if self.jextsel() != 0 { try!(write!(f, " jextsel=0x{:x}", self.jextsel()))}
        if self.jl() != 0 { try!(write!(f, " jl=0x{:x}", self.jl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="offset register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ofr1(pub u32);
impl Ofr1 {
    #[doc="OFFSET1_EN"]
    #[inline] pub fn offset1_en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if OFFSET1_EN != 0"]
    #[inline] pub fn test_offset1_en(&self) -> bool {
        self.offset1_en() != 0
    }

    #[doc="Sets the OFFSET1_EN field."]
    #[inline] pub fn set_offset1_en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="OFFSET1_CH"]
    #[inline] pub fn offset1_ch(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1f) as u8) } // [30:26]
    }

    #[doc="Returns true if OFFSET1_CH != 0"]
    #[inline] pub fn test_offset1_ch(&self) -> bool {
        self.offset1_ch() != 0
    }

    #[doc="Sets the OFFSET1_CH field."]
    #[inline] pub fn set_offset1_ch<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="OFFSET1"]
    #[inline] pub fn offset1(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if OFFSET1 != 0"]
    #[inline] pub fn test_offset1(&self) -> bool {
        self.offset1() != 0
    }

    #[doc="Sets the OFFSET1 field."]
    #[inline] pub fn set_offset1<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ofr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Ofr1(other)
    }
}

impl ::core::fmt::Display for Ofr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ofr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.offset1_en() != 0 { try!(write!(f, " offset1_en"))}
        if self.offset1_ch() != 0 { try!(write!(f, " offset1_ch=0x{:x}", self.offset1_ch()))}
        if self.offset1() != 0 { try!(write!(f, " offset1=0x{:x}", self.offset1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="offset register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ofr2(pub u32);
impl Ofr2 {
    #[doc="OFFSET2_EN"]
    #[inline] pub fn offset2_en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if OFFSET2_EN != 0"]
    #[inline] pub fn test_offset2_en(&self) -> bool {
        self.offset2_en() != 0
    }

    #[doc="Sets the OFFSET2_EN field."]
    #[inline] pub fn set_offset2_en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="OFFSET2_CH"]
    #[inline] pub fn offset2_ch(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1f) as u8) } // [30:26]
    }

    #[doc="Returns true if OFFSET2_CH != 0"]
    #[inline] pub fn test_offset2_ch(&self) -> bool {
        self.offset2_ch() != 0
    }

    #[doc="Sets the OFFSET2_CH field."]
    #[inline] pub fn set_offset2_ch<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="OFFSET2"]
    #[inline] pub fn offset2(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if OFFSET2 != 0"]
    #[inline] pub fn test_offset2(&self) -> bool {
        self.offset2() != 0
    }

    #[doc="Sets the OFFSET2 field."]
    #[inline] pub fn set_offset2<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ofr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Ofr2(other)
    }
}

impl ::core::fmt::Display for Ofr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ofr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.offset2_en() != 0 { try!(write!(f, " offset2_en"))}
        if self.offset2_ch() != 0 { try!(write!(f, " offset2_ch=0x{:x}", self.offset2_ch()))}
        if self.offset2() != 0 { try!(write!(f, " offset2=0x{:x}", self.offset2()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="offset register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ofr3(pub u32);
impl Ofr3 {
    #[doc="OFFSET3_EN"]
    #[inline] pub fn offset3_en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if OFFSET3_EN != 0"]
    #[inline] pub fn test_offset3_en(&self) -> bool {
        self.offset3_en() != 0
    }

    #[doc="Sets the OFFSET3_EN field."]
    #[inline] pub fn set_offset3_en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="OFFSET3_CH"]
    #[inline] pub fn offset3_ch(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1f) as u8) } // [30:26]
    }

    #[doc="Returns true if OFFSET3_CH != 0"]
    #[inline] pub fn test_offset3_ch(&self) -> bool {
        self.offset3_ch() != 0
    }

    #[doc="Sets the OFFSET3_CH field."]
    #[inline] pub fn set_offset3_ch<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="OFFSET3"]
    #[inline] pub fn offset3(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if OFFSET3 != 0"]
    #[inline] pub fn test_offset3(&self) -> bool {
        self.offset3() != 0
    }

    #[doc="Sets the OFFSET3 field."]
    #[inline] pub fn set_offset3<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ofr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Ofr3(other)
    }
}

impl ::core::fmt::Display for Ofr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ofr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.offset3_en() != 0 { try!(write!(f, " offset3_en"))}
        if self.offset3_ch() != 0 { try!(write!(f, " offset3_ch=0x{:x}", self.offset3_ch()))}
        if self.offset3() != 0 { try!(write!(f, " offset3=0x{:x}", self.offset3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="offset register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ofr4(pub u32);
impl Ofr4 {
    #[doc="OFFSET4_EN"]
    #[inline] pub fn offset4_en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if OFFSET4_EN != 0"]
    #[inline] pub fn test_offset4_en(&self) -> bool {
        self.offset4_en() != 0
    }

    #[doc="Sets the OFFSET4_EN field."]
    #[inline] pub fn set_offset4_en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="OFFSET4_CH"]
    #[inline] pub fn offset4_ch(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1f) as u8) } // [30:26]
    }

    #[doc="Returns true if OFFSET4_CH != 0"]
    #[inline] pub fn test_offset4_ch(&self) -> bool {
        self.offset4_ch() != 0
    }

    #[doc="Sets the OFFSET4_CH field."]
    #[inline] pub fn set_offset4_ch<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="OFFSET4"]
    #[inline] pub fn offset4(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if OFFSET4 != 0"]
    #[inline] pub fn test_offset4(&self) -> bool {
        self.offset4() != 0
    }

    #[doc="Sets the OFFSET4 field."]
    #[inline] pub fn set_offset4<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ofr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Ofr4(other)
    }
}

impl ::core::fmt::Display for Ofr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ofr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.offset4_en() != 0 { try!(write!(f, " offset4_en"))}
        if self.offset4_ch() != 0 { try!(write!(f, " offset4_ch=0x{:x}", self.offset4_ch()))}
        if self.offset4() != 0 { try!(write!(f, " offset4=0x{:x}", self.offset4()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="injected data register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Jdr1(pub u32);
impl Jdr1 {
    #[doc="JDATA1"]
    #[inline] pub fn jdata1(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if JDATA1 != 0"]
    #[inline] pub fn test_jdata1(&self) -> bool {
        self.jdata1() != 0
    }

    #[doc="Sets the JDATA1 field."]
    #[inline] pub fn set_jdata1<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Jdr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Jdr1(other)
    }
}

impl ::core::fmt::Display for Jdr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Jdr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jdata1() != 0 { try!(write!(f, " jdata1=0x{:x}", self.jdata1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="injected data register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Jdr2(pub u32);
impl Jdr2 {
    #[doc="JDATA2"]
    #[inline] pub fn jdata2(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if JDATA2 != 0"]
    #[inline] pub fn test_jdata2(&self) -> bool {
        self.jdata2() != 0
    }

    #[doc="Sets the JDATA2 field."]
    #[inline] pub fn set_jdata2<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Jdr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Jdr2(other)
    }
}

impl ::core::fmt::Display for Jdr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Jdr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jdata2() != 0 { try!(write!(f, " jdata2=0x{:x}", self.jdata2()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="injected data register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Jdr3(pub u32);
impl Jdr3 {
    #[doc="JDATA3"]
    #[inline] pub fn jdata3(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if JDATA3 != 0"]
    #[inline] pub fn test_jdata3(&self) -> bool {
        self.jdata3() != 0
    }

    #[doc="Sets the JDATA3 field."]
    #[inline] pub fn set_jdata3<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Jdr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Jdr3(other)
    }
}

impl ::core::fmt::Display for Jdr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Jdr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jdata3() != 0 { try!(write!(f, " jdata3=0x{:x}", self.jdata3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="injected data register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Jdr4(pub u32);
impl Jdr4 {
    #[doc="JDATA4"]
    #[inline] pub fn jdata4(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if JDATA4 != 0"]
    #[inline] pub fn test_jdata4(&self) -> bool {
        self.jdata4() != 0
    }

    #[doc="Sets the JDATA4 field."]
    #[inline] pub fn set_jdata4<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Jdr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Jdr4(other)
    }
}

impl ::core::fmt::Display for Jdr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Jdr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jdata4() != 0 { try!(write!(f, " jdata4=0x{:x}", self.jdata4()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Analog Watchdog 2 Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Awd2cr(pub u32);
impl Awd2cr {
    #[doc="AWD2CH"]
    #[inline] pub fn awd2ch(&self) -> ::bobbin_bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3ffff) as u32) } // [18:1]
    }

    #[doc="Returns true if AWD2CH != 0"]
    #[inline] pub fn test_awd2ch(&self) -> bool {
        self.awd2ch() != 0
    }

    #[doc="Sets the AWD2CH field."]
    #[inline] pub fn set_awd2ch<V: Into<::bobbin_bits::U18>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Awd2cr {
    #[inline]
    fn from(other: u32) -> Self {
         Awd2cr(other)
    }
}

impl ::core::fmt::Display for Awd2cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Awd2cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awd2ch() != 0 { try!(write!(f, " awd2ch=0x{:x}", self.awd2ch()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Analog Watchdog 3 Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Awd3cr(pub u32);
impl Awd3cr {
    #[doc="AWD3CH"]
    #[inline] pub fn awd3ch(&self) -> ::bobbin_bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3ffff) as u32) } // [18:1]
    }

    #[doc="Returns true if AWD3CH != 0"]
    #[inline] pub fn test_awd3ch(&self) -> bool {
        self.awd3ch() != 0
    }

    #[doc="Sets the AWD3CH field."]
    #[inline] pub fn set_awd3ch<V: Into<::bobbin_bits::U18>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Awd3cr {
    #[inline]
    fn from(other: u32) -> Self {
         Awd3cr(other)
    }
}

impl ::core::fmt::Display for Awd3cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Awd3cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.awd3ch() != 0 { try!(write!(f, " awd3ch=0x{:x}", self.awd3ch()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Differential Mode Selection Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Difsel(pub u32);
impl Difsel {
    #[doc="Differential mode for channels 15 to 1"]
    #[inline] pub fn difsel_1_15(&self) -> ::bobbin_bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7fff) as u16) } // [15:1]
    }

    #[doc="Returns true if DIFSEL_1_15 != 0"]
    #[inline] pub fn test_difsel_1_15(&self) -> bool {
        self.difsel_1_15() != 0
    }

    #[doc="Sets the DIFSEL_1_15 field."]
    #[inline] pub fn set_difsel_1_15<V: Into<::bobbin_bits::U15>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U15 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7fff << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Differential mode for channels 18 to 16"]
    #[inline] pub fn difsel_16_18(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if DIFSEL_16_18 != 0"]
    #[inline] pub fn test_difsel_16_18(&self) -> bool {
        self.difsel_16_18() != 0
    }

    #[doc="Sets the DIFSEL_16_18 field."]
    #[inline] pub fn set_difsel_16_18<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Difsel {
    #[inline]
    fn from(other: u32) -> Self {
         Difsel(other)
    }
}

impl ::core::fmt::Display for Difsel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Difsel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.difsel_1_15() != 0 { try!(write!(f, " difsel_1_15=0x{:x}", self.difsel_1_15()))}
        if self.difsel_16_18() != 0 { try!(write!(f, " difsel_16_18=0x{:x}", self.difsel_16_18()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Calibration Factors"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Calfact(pub u32);
impl Calfact {
    #[doc="CALFACT_D"]
    #[inline] pub fn calfact_d(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7f) as u8) } // [22:16]
    }

    #[doc="Returns true if CALFACT_D != 0"]
    #[inline] pub fn test_calfact_d(&self) -> bool {
        self.calfact_d() != 0
    }

    #[doc="Sets the CALFACT_D field."]
    #[inline] pub fn set_calfact_d<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="CALFACT_S"]
    #[inline] pub fn calfact_s(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if CALFACT_S != 0"]
    #[inline] pub fn test_calfact_s(&self) -> bool {
        self.calfact_s() != 0
    }

    #[doc="Sets the CALFACT_S field."]
    #[inline] pub fn set_calfact_s<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Calfact {
    #[inline]
    fn from(other: u32) -> Self {
         Calfact(other)
    }
}

impl ::core::fmt::Display for Calfact {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Calfact {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.calfact_d() != 0 { try!(write!(f, " calfact_d=0x{:x}", self.calfact_d()))}
        if self.calfact_s() != 0 { try!(write!(f, " calfact_s=0x{:x}", self.calfact_s()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

