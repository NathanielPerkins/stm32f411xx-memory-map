# [ doc = "Reset and clock control" ]
# [ repr ( C ) ]
pub struct Rcc {
    # [ doc = "0x00 - clock control register" ]
    pub cr: Cr,
    # [ doc = "0x04 - PLL configuration register" ]
    pub pllcfgr: Pllcfgr,
    # [ doc = "0x08 - clock configuration register" ]
    pub cfgr: Cfgr,
    # [ doc = "0x0c - clock interrupt register" ]
    pub cir: Cir,
    # [ doc = "0x10 - AHB1 peripheral reset register" ]
    pub ahb1rstr: Ahb1rstr,
    # [ doc = "0x14 - AHB2 peripheral reset register" ]
    pub ahb2rstr: Ahb2rstr,
    _reserved0: [u8; 8usize],
    # [ doc = "0x20 - APB1 peripheral reset register" ]
    pub apb1rstr: Apb1rstr,
    # [ doc = "0x24 - APB2 peripheral reset register" ]
    pub apb2rstr: Apb2rstr,
    _reserved1: [u8; 8usize],
    # [ doc = "0x30 - AHB1 peripheral clock register" ]
    pub ahb1enr: Ahb1enr,
    # [ doc = "0x34 - AHB2 peripheral clock enable register" ]
    pub ahb2enr: Ahb2enr,
    _reserved2: [u8; 8usize],
    # [ doc = "0x40 - APB1 peripheral clock enable register" ]
    pub apb1enr: Apb1enr,
    # [ doc = "0x44 - APB2 peripheral clock enable register" ]
    pub apb2enr: Apb2enr,
    _reserved3: [u8; 8usize],
    # [ doc = "0x50 - AHB1 peripheral clock enable in low power mode register" ]
    pub ahb1lpenr: Ahb1lpenr,
    # [ doc = "0x54 - AHB2 peripheral clock enable in low power mode register" ]
    pub ahb2lpenr: Ahb2lpenr,
    _reserved4: [u8; 8usize],
    # [ doc = "0x60 - APB1 peripheral clock enable in low power mode register" ]
    pub apb1lpenr: Apb1lpenr,
    # [ doc = "0x64 - APB2 peripheral clock enabled in low power mode register" ]
    pub apb2lpenr: Apb2lpenr,
    _reserved5: [u8; 8usize],
    # [ doc = "0x70 - Backup domain control register" ]
    pub bdcr: Bdcr,
    # [ doc = "0x74 - clock control & status register" ]
    pub csr: Csr,
    _reserved6: [u8; 8usize],
    # [ doc = "0x80 - spread spectrum clock generation register" ]
    pub sscgr: Sscgr,
    # [ doc = "0x84 - PLLI2S configuration register" ]
    pub plli2scfgr: Plli2scfgr,
}

# [ repr ( C ) ]
pub struct Cr {
    register: ::volatile_register::RW<u32>,
}

impl Cr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CrR, &'w mut CrW) -> &'w mut CrW
    {
        let bits = self.register.read();
        let r = CrR { bits: bits };
        let mut w = CrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CrR {
        CrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CrW) -> &mut CrW
    {
        let mut w = CrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CrR {
    bits: u32,
}

impl CrR {
    # [ doc = "Bit 27 - PLLI2S clock ready flag" ]
    pub fn plli2srdy(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - PLLI2S enable" ]
    pub fn plli2son(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Main PLL (PLL) clock ready flag" ]
    pub fn pllrdy(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Main PLL (PLL) enable" ]
    pub fn pllon(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Clock security system enable" ]
    pub fn csson(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - HSE clock bypass" ]
    pub fn hsebyp(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - HSE clock ready flag" ]
    pub fn hserdy(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - HSE clock enable" ]
    pub fn hseon(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:15 - Internal high-speed clock calibration" ]
    pub fn hsical(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 3:7 - Internal high-speed clock trimming" ]
    pub fn hsitrim(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 1 - Internal high-speed clock ready flag" ]
    pub fn hsirdy(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Internal high-speed clock enable" ]
    pub fn hsion(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CrW {
    bits: u32,
}

impl CrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CrW { bits: 131u32 }
    }
    # [ doc = "Bit 26 - PLLI2S enable" ]
    pub fn plli2son(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - Main PLL (PLL) enable" ]
    pub fn pllon(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Clock security system enable" ]
    pub fn csson(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - HSE clock bypass" ]
    pub fn hsebyp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - HSE clock enable" ]
    pub fn hseon(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 3:7 - Internal high-speed clock trimming" ]
    pub fn hsitrim(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 0 - Internal high-speed clock enable" ]
    pub fn hsion(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Pllcfgr {
    register: ::volatile_register::RW<u32>,
}

impl Pllcfgr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PllcfgrR, &'w mut PllcfgrW) -> &'w mut PllcfgrW
    {
        let bits = self.register.read();
        let r = PllcfgrR { bits: bits };
        let mut w = PllcfgrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PllcfgrR {
        PllcfgrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PllcfgrW) -> &mut PllcfgrW
    {
        let mut w = PllcfgrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PllcfgrR {
    bits: u32,
}

impl PllcfgrR {
    # [ doc = "Bit 27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ]
    pub fn pllq3(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ]
    pub fn pllq2(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ]
    pub fn pllq1(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ]
    pub fn pllq0(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source" ]
    pub fn pllsrc(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Main PLL (PLL) division factor for main system clock" ]
    pub fn pllp1(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Main PLL (PLL) division factor for main system clock" ]
    pub fn pllp0(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Main PLL (PLL) multiplication factor for VCO" ]
    pub fn plln8(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Main PLL (PLL) multiplication factor for VCO" ]
    pub fn plln7(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Main PLL (PLL) multiplication factor for VCO" ]
    pub fn plln6(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Main PLL (PLL) multiplication factor for VCO" ]
    pub fn plln5(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Main PLL (PLL) multiplication factor for VCO" ]
    pub fn plln4(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Main PLL (PLL) multiplication factor for VCO" ]
    pub fn plln3(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Main PLL (PLL) multiplication factor for VCO" ]
    pub fn plln2(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Main PLL (PLL) multiplication factor for VCO" ]
    pub fn plln1(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Main PLL (PLL) multiplication factor for VCO" ]
    pub fn plln0(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
    pub fn pllm5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
    pub fn pllm4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
    pub fn pllm3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
    pub fn pllm2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
    pub fn pllm1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
    pub fn pllm0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PllcfgrW {
    bits: u32,
}

impl PllcfgrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PllcfgrW { bits: 603992080u32 }
    }
    # [ doc = "Bit 27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ]
    pub fn pllq3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ]
    pub fn pllq2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ]
    pub fn pllq1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ]
    pub fn pllq0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source" ]
    pub fn pllsrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Main PLL (PLL) division factor for main system clock" ]
    pub fn pllp1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Main PLL (PLL) division factor for main system clock" ]
    pub fn pllp0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Main PLL (PLL) multiplication factor for VCO" ]
    pub fn plln8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Main PLL (PLL) multiplication factor for VCO" ]
    pub fn plln7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Main PLL (PLL) multiplication factor for VCO" ]
    pub fn plln6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Main PLL (PLL) multiplication factor for VCO" ]
    pub fn plln5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Main PLL (PLL) multiplication factor for VCO" ]
    pub fn plln4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Main PLL (PLL) multiplication factor for VCO" ]
    pub fn plln3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Main PLL (PLL) multiplication factor for VCO" ]
    pub fn plln2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Main PLL (PLL) multiplication factor for VCO" ]
    pub fn plln1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Main PLL (PLL) multiplication factor for VCO" ]
    pub fn plln0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
    pub fn pllm5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
    pub fn pllm4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
    pub fn pllm3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
    pub fn pllm2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
    pub fn pllm1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
    pub fn pllm0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Cfgr {
    register: ::volatile_register::RW<u32>,
}

impl Cfgr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CfgrR, &'w mut CfgrW) -> &'w mut CfgrW
    {
        let bits = self.register.read();
        let r = CfgrR { bits: bits };
        let mut w = CfgrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CfgrR {
        CfgrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CfgrW) -> &mut CfgrW
    {
        let mut w = CfgrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CfgrR {
    bits: u32,
}

impl CfgrR {
    # [ doc = "Bits 30:31 - Microcontroller clock output 2" ]
    pub fn mco2(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 30u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 27:29 - MCO2 prescaler" ]
    pub fn mco2pre(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 27u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:26 - MCO1 prescaler" ]
    pub fn mco1pre(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 23 - I2S clock selection" ]
    pub fn i2ssrc(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 21:22 - Microcontroller clock output 1" ]
    pub fn mco1(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 21u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:20 - HSE division factor for RTC clock" ]
    pub fn rtcpre(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 13:15 - APB high-speed prescaler (APB2)" ]
    pub fn ppre2(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 10:12 - APB Low speed prescaler (APB1)" ]
    pub fn ppre1(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - AHB prescaler" ]
    pub fn hpre(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 3 - System clock switch status" ]
    pub fn sws1(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - System clock switch status" ]
    pub fn sws0(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - System clock switch" ]
    pub fn sw1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - System clock switch" ]
    pub fn sw0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CfgrW {
    bits: u32,
}

impl CfgrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CfgrW { bits: 0u32 }
    }
    # [ doc = "Bits 30:31 - Microcontroller clock output 2" ]
    pub fn mco2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 30u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 27:29 - MCO2 prescaler" ]
    pub fn mco2pre(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 27u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:26 - MCO1 prescaler" ]
    pub fn mco1pre(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 23 - I2S clock selection" ]
    pub fn i2ssrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 21:22 - Microcontroller clock output 1" ]
    pub fn mco1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 21u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:20 - HSE division factor for RTC clock" ]
    pub fn rtcpre(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 13:15 - APB high-speed prescaler (APB2)" ]
    pub fn ppre2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 10:12 - APB Low speed prescaler (APB1)" ]
    pub fn ppre1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - AHB prescaler" ]
    pub fn hpre(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 1 - System clock switch" ]
    pub fn sw1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - System clock switch" ]
    pub fn sw0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Cir {
    register: ::volatile_register::RW<u32>,
}

impl Cir {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CirR, &'w mut CirW) -> &'w mut CirW
    {
        let bits = self.register.read();
        let r = CirR { bits: bits };
        let mut w = CirW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CirR {
        CirR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CirW) -> &mut CirW
    {
        let mut w = CirW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CirR {
    bits: u32,
}

impl CirR {
    # [ doc = "Bit 13 - PLLI2S ready interrupt enable" ]
    pub fn plli2srdyie(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Main PLL (PLL) ready interrupt enable" ]
    pub fn pllrdyie(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - HSE ready interrupt enable" ]
    pub fn hserdyie(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - HSI ready interrupt enable" ]
    pub fn hsirdyie(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - LSE ready interrupt enable" ]
    pub fn lserdyie(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - LSI ready interrupt enable" ]
    pub fn lsirdyie(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Clock security system interrupt flag" ]
    pub fn cssf(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - PLLI2S ready interrupt flag" ]
    pub fn plli2srdyf(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Main PLL (PLL) ready interrupt flag" ]
    pub fn pllrdyf(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - HSE ready interrupt flag" ]
    pub fn hserdyf(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - HSI ready interrupt flag" ]
    pub fn hsirdyf(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - LSE ready interrupt flag" ]
    pub fn lserdyf(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - LSI ready interrupt flag" ]
    pub fn lsirdyf(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CirW {
    bits: u32,
}

impl CirW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CirW { bits: 0u32 }
    }
    # [ doc = "Bit 23 - Clock security system interrupt clear" ]
    pub fn cssc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - PLLI2S ready interrupt clear" ]
    pub fn plli2srdyc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Main PLL(PLL) ready interrupt clear" ]
    pub fn pllrdyc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - HSE ready interrupt clear" ]
    pub fn hserdyc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - HSI ready interrupt clear" ]
    pub fn hsirdyc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - LSE ready interrupt clear" ]
    pub fn lserdyc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - LSI ready interrupt clear" ]
    pub fn lsirdyc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - PLLI2S ready interrupt enable" ]
    pub fn plli2srdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Main PLL (PLL) ready interrupt enable" ]
    pub fn pllrdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - HSE ready interrupt enable" ]
    pub fn hserdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - HSI ready interrupt enable" ]
    pub fn hsirdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - LSE ready interrupt enable" ]
    pub fn lserdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - LSI ready interrupt enable" ]
    pub fn lsirdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Ahb1rstr {
    register: ::volatile_register::RW<u32>,
}

impl Ahb1rstr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ahb1rstrR, &'w mut Ahb1rstrW) -> &'w mut Ahb1rstrW
    {
        let bits = self.register.read();
        let r = Ahb1rstrR { bits: bits };
        let mut w = Ahb1rstrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ahb1rstrR {
        Ahb1rstrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ahb1rstrW) -> &mut Ahb1rstrW
    {
        let mut w = Ahb1rstrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb1rstrR {
    bits: u32,
}

impl Ahb1rstrR {
    # [ doc = "Bit 22 - DMA2 reset" ]
    pub fn dma2rst(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - DMA2 reset" ]
    pub fn dma1rst(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - CRC reset" ]
    pub fn crcrst(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - IO port H reset" ]
    pub fn gpiohrst(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - IO port E reset" ]
    pub fn gpioerst(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - IO port D reset" ]
    pub fn gpiodrst(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - IO port C reset" ]
    pub fn gpiocrst(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - IO port B reset" ]
    pub fn gpiobrst(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - IO port A reset" ]
    pub fn gpioarst(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb1rstrW {
    bits: u32,
}

impl Ahb1rstrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ahb1rstrW { bits: 0u32 }
    }
    # [ doc = "Bit 22 - DMA2 reset" ]
    pub fn dma2rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - DMA2 reset" ]
    pub fn dma1rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - CRC reset" ]
    pub fn crcrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - IO port H reset" ]
    pub fn gpiohrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - IO port E reset" ]
    pub fn gpioerst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - IO port D reset" ]
    pub fn gpiodrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - IO port C reset" ]
    pub fn gpiocrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - IO port B reset" ]
    pub fn gpiobrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - IO port A reset" ]
    pub fn gpioarst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Ahb2rstr {
    register: ::volatile_register::RW<u32>,
}

impl Ahb2rstr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ahb2rstrR, &'w mut Ahb2rstrW) -> &'w mut Ahb2rstrW
    {
        let bits = self.register.read();
        let r = Ahb2rstrR { bits: bits };
        let mut w = Ahb2rstrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ahb2rstrR {
        Ahb2rstrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ahb2rstrW) -> &mut Ahb2rstrW
    {
        let mut w = Ahb2rstrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb2rstrR {
    bits: u32,
}

impl Ahb2rstrR {
    # [ doc = "Bit 7 - USB OTG FS module reset" ]
    pub fn otgfsrst(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb2rstrW {
    bits: u32,
}

impl Ahb2rstrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ahb2rstrW { bits: 0u32 }
    }
    # [ doc = "Bit 7 - USB OTG FS module reset" ]
    pub fn otgfsrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Apb1rstr {
    register: ::volatile_register::RW<u32>,
}

impl Apb1rstr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Apb1rstrR, &'w mut Apb1rstrW) -> &'w mut Apb1rstrW
    {
        let bits = self.register.read();
        let r = Apb1rstrR { bits: bits };
        let mut w = Apb1rstrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Apb1rstrR {
        Apb1rstrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb1rstrW) -> &mut Apb1rstrW
    {
        let mut w = Apb1rstrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1rstrR {
    bits: u32,
}

impl Apb1rstrR {
    # [ doc = "Bit 28 - Power interface reset" ]
    pub fn pwrrst(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - I2C3 reset" ]
    pub fn i2c3rst(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - I2C 2 reset" ]
    pub fn i2c2rst(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - I2C 1 reset" ]
    pub fn i2c1rst(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - USART 2 reset" ]
    pub fn uart2rst(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - SPI 3 reset" ]
    pub fn spi3rst(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - SPI 2 reset" ]
    pub fn spi2rst(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Window watchdog reset" ]
    pub fn wwdgrst(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - TIM5 reset" ]
    pub fn tim5rst(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - TIM4 reset" ]
    pub fn tim4rst(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - TIM3 reset" ]
    pub fn tim3rst(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - TIM2 reset" ]
    pub fn tim2rst(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1rstrW {
    bits: u32,
}

impl Apb1rstrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Apb1rstrW { bits: 0u32 }
    }
    # [ doc = "Bit 28 - Power interface reset" ]
    pub fn pwrrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - I2C3 reset" ]
    pub fn i2c3rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - I2C 2 reset" ]
    pub fn i2c2rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - I2C 1 reset" ]
    pub fn i2c1rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - USART 2 reset" ]
    pub fn uart2rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - SPI 3 reset" ]
    pub fn spi3rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - SPI 2 reset" ]
    pub fn spi2rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Window watchdog reset" ]
    pub fn wwdgrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - TIM5 reset" ]
    pub fn tim5rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - TIM4 reset" ]
    pub fn tim4rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - TIM3 reset" ]
    pub fn tim3rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - TIM2 reset" ]
    pub fn tim2rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Apb2rstr {
    register: ::volatile_register::RW<u32>,
}

impl Apb2rstr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Apb2rstrR, &'w mut Apb2rstrW) -> &'w mut Apb2rstrW
    {
        let bits = self.register.read();
        let r = Apb2rstrR { bits: bits };
        let mut w = Apb2rstrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Apb2rstrR {
        Apb2rstrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb2rstrW) -> &mut Apb2rstrW
    {
        let mut w = Apb2rstrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb2rstrR {
    bits: u32,
}

impl Apb2rstrR {
    # [ doc = "Bit 18 - TIM11 reset" ]
    pub fn tim11rst(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - TIM10 reset" ]
    pub fn tim10rst(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - TIM9 reset" ]
    pub fn tim9rst(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - System configuration controller reset" ]
    pub fn syscfgrst(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - SPI 1 reset" ]
    pub fn spi1rst(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - SDIO reset" ]
    pub fn sdiorst(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - ADC interface reset (common to all ADCs)" ]
    pub fn adcrst(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - USART6 reset" ]
    pub fn usart6rst(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - USART1 reset" ]
    pub fn usart1rst(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - TIM1 reset" ]
    pub fn tim1rst(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb2rstrW {
    bits: u32,
}

impl Apb2rstrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Apb2rstrW { bits: 0u32 }
    }
    # [ doc = "Bit 18 - TIM11 reset" ]
    pub fn tim11rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - TIM10 reset" ]
    pub fn tim10rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - TIM9 reset" ]
    pub fn tim9rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - System configuration controller reset" ]
    pub fn syscfgrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - SPI 1 reset" ]
    pub fn spi1rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - SDIO reset" ]
    pub fn sdiorst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - ADC interface reset (common to all ADCs)" ]
    pub fn adcrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - USART6 reset" ]
    pub fn usart6rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - USART1 reset" ]
    pub fn usart1rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - TIM1 reset" ]
    pub fn tim1rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Ahb1enr {
    register: ::volatile_register::RW<u32>,
}

impl Ahb1enr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ahb1enrR, &'w mut Ahb1enrW) -> &'w mut Ahb1enrW
    {
        let bits = self.register.read();
        let r = Ahb1enrR { bits: bits };
        let mut w = Ahb1enrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ahb1enrR {
        Ahb1enrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ahb1enrW) -> &mut Ahb1enrW
    {
        let mut w = Ahb1enrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb1enrR {
    bits: u32,
}

impl Ahb1enrR {
    # [ doc = "Bit 22 - DMA2 clock enable" ]
    pub fn dma2en(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - DMA1 clock enable" ]
    pub fn dma1en(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - CRC clock enable" ]
    pub fn crcen(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - IO port H clock enable" ]
    pub fn gpiohen(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - IO port E clock enable" ]
    pub fn gpioeen(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - IO port D clock enable" ]
    pub fn gpioden(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - IO port C clock enable" ]
    pub fn gpiocen(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - IO port B clock enable" ]
    pub fn gpioben(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - IO port A clock enable" ]
    pub fn gpioaen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb1enrW {
    bits: u32,
}

impl Ahb1enrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ahb1enrW { bits: 1048576u32 }
    }
    # [ doc = "Bit 22 - DMA2 clock enable" ]
    pub fn dma2en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - DMA1 clock enable" ]
    pub fn dma1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - CRC clock enable" ]
    pub fn crcen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - IO port H clock enable" ]
    pub fn gpiohen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - IO port E clock enable" ]
    pub fn gpioeen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - IO port D clock enable" ]
    pub fn gpioden(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - IO port C clock enable" ]
    pub fn gpiocen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - IO port B clock enable" ]
    pub fn gpioben(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - IO port A clock enable" ]
    pub fn gpioaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Ahb2enr {
    register: ::volatile_register::RW<u32>,
}

impl Ahb2enr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ahb2enrR, &'w mut Ahb2enrW) -> &'w mut Ahb2enrW
    {
        let bits = self.register.read();
        let r = Ahb2enrR { bits: bits };
        let mut w = Ahb2enrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ahb2enrR {
        Ahb2enrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ahb2enrW) -> &mut Ahb2enrW
    {
        let mut w = Ahb2enrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb2enrR {
    bits: u32,
}

impl Ahb2enrR {
    # [ doc = "Bit 7 - USB OTG FS clock enable" ]
    pub fn otgfsen(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb2enrW {
    bits: u32,
}

impl Ahb2enrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ahb2enrW { bits: 0u32 }
    }
    # [ doc = "Bit 7 - USB OTG FS clock enable" ]
    pub fn otgfsen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Apb1enr {
    register: ::volatile_register::RW<u32>,
}

impl Apb1enr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Apb1enrR, &'w mut Apb1enrW) -> &'w mut Apb1enrW
    {
        let bits = self.register.read();
        let r = Apb1enrR { bits: bits };
        let mut w = Apb1enrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Apb1enrR {
        Apb1enrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb1enrW) -> &mut Apb1enrW
    {
        let mut w = Apb1enrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1enrR {
    bits: u32,
}

impl Apb1enrR {
    # [ doc = "Bit 28 - Power interface clock enable" ]
    pub fn pwren(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - I2C3 clock enable" ]
    pub fn i2c3en(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - I2C2 clock enable" ]
    pub fn i2c2en(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - I2C1 clock enable" ]
    pub fn i2c1en(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - USART 2 clock enable" ]
    pub fn usart2en(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - SPI3 clock enable" ]
    pub fn spi3en(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - SPI2 clock enable" ]
    pub fn spi2en(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Window watchdog clock enable" ]
    pub fn wwdgen(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - TIM5 clock enable" ]
    pub fn tim5en(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - TIM4 clock enable" ]
    pub fn tim4en(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - TIM3 clock enable" ]
    pub fn tim3en(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - TIM2 clock enable" ]
    pub fn tim2en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1enrW {
    bits: u32,
}

impl Apb1enrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Apb1enrW { bits: 0u32 }
    }
    # [ doc = "Bit 28 - Power interface clock enable" ]
    pub fn pwren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - I2C3 clock enable" ]
    pub fn i2c3en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - I2C2 clock enable" ]
    pub fn i2c2en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - I2C1 clock enable" ]
    pub fn i2c1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - USART 2 clock enable" ]
    pub fn usart2en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - SPI3 clock enable" ]
    pub fn spi3en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - SPI2 clock enable" ]
    pub fn spi2en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Window watchdog clock enable" ]
    pub fn wwdgen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - TIM5 clock enable" ]
    pub fn tim5en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - TIM4 clock enable" ]
    pub fn tim4en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - TIM3 clock enable" ]
    pub fn tim3en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - TIM2 clock enable" ]
    pub fn tim2en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Apb2enr {
    register: ::volatile_register::RW<u32>,
}

impl Apb2enr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Apb2enrR, &'w mut Apb2enrW) -> &'w mut Apb2enrW
    {
        let bits = self.register.read();
        let r = Apb2enrR { bits: bits };
        let mut w = Apb2enrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Apb2enrR {
        Apb2enrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb2enrW) -> &mut Apb2enrW
    {
        let mut w = Apb2enrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb2enrR {
    bits: u32,
}

impl Apb2enrR {
    # [ doc = "Bit 0 - TIM1 clock enable" ]
    pub fn tim1en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - USART1 clock enable" ]
    pub fn usart1en(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - USART6 clock enable" ]
    pub fn usart6en(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - ADC1 clock enable" ]
    pub fn adc1en(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - SDIO clock enable" ]
    pub fn sdioen(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - SPI1 clock enable" ]
    pub fn spi1en(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - SPI4 clock enable" ]
    pub fn spi4en(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - System configuration controller clock enable" ]
    pub fn syscfgen(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - TIM9 clock enable" ]
    pub fn tim9en(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - TIM10 clock enable" ]
    pub fn tim10en(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - TIM11 clock enable" ]
    pub fn tim11en(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb2enrW {
    bits: u32,
}

impl Apb2enrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Apb2enrW { bits: 0u32 }
    }
    # [ doc = "Bit 0 - TIM1 clock enable" ]
    pub fn tim1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - USART1 clock enable" ]
    pub fn usart1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - USART6 clock enable" ]
    pub fn usart6en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - ADC1 clock enable" ]
    pub fn adc1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - SDIO clock enable" ]
    pub fn sdioen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - SPI1 clock enable" ]
    pub fn spi1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - SPI4 clock enable" ]
    pub fn spi4en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - System configuration controller clock enable" ]
    pub fn syscfgen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - TIM9 clock enable" ]
    pub fn tim9en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - TIM10 clock enable" ]
    pub fn tim10en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - TIM11 clock enable" ]
    pub fn tim11en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Ahb1lpenr {
    register: ::volatile_register::RW<u32>,
}

impl Ahb1lpenr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ahb1lpenrR, &'w mut Ahb1lpenrW) -> &'w mut Ahb1lpenrW
    {
        let bits = self.register.read();
        let r = Ahb1lpenrR { bits: bits };
        let mut w = Ahb1lpenrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ahb1lpenrR {
        Ahb1lpenrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ahb1lpenrW) -> &mut Ahb1lpenrW
    {
        let mut w = Ahb1lpenrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb1lpenrR {
    bits: u32,
}

impl Ahb1lpenrR {
    # [ doc = "Bit 22 - DMA2 clock enable during Sleep mode" ]
    pub fn dma2lpen(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - DMA1 clock enable during Sleep mode" ]
    pub fn dma1lpen(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - SRAM 1interface clock enable during Sleep mode" ]
    pub fn sram1lpen(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - Flash interface clock enable during Sleep mode" ]
    pub fn flitflpen(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - CRC clock enable during Sleep mode" ]
    pub fn crclpen(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - IO port H clock enable during Sleep mode" ]
    pub fn gpiohlpen(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - IO port E clock enable during Sleep mode" ]
    pub fn gpioelpen(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - IO port D clock enable during Sleep mode" ]
    pub fn gpiodlpen(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - IO port C clock enable during Sleep mode" ]
    pub fn gpioclpen(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - IO port B clock enable during Sleep mode" ]
    pub fn gpioblpen(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - IO port A clock enable during sleep mode" ]
    pub fn gpioalpen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb1lpenrW {
    bits: u32,
}

impl Ahb1lpenrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ahb1lpenrW { bits: 2120716799u32 }
    }
    # [ doc = "Bit 22 - DMA2 clock enable during Sleep mode" ]
    pub fn dma2lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - DMA1 clock enable during Sleep mode" ]
    pub fn dma1lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - SRAM 1interface clock enable during Sleep mode" ]
    pub fn sram1lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - Flash interface clock enable during Sleep mode" ]
    pub fn flitflpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - CRC clock enable during Sleep mode" ]
    pub fn crclpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - IO port H clock enable during Sleep mode" ]
    pub fn gpiohlpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - IO port E clock enable during Sleep mode" ]
    pub fn gpioelpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - IO port D clock enable during Sleep mode" ]
    pub fn gpiodlpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - IO port C clock enable during Sleep mode" ]
    pub fn gpioclpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - IO port B clock enable during Sleep mode" ]
    pub fn gpioblpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - IO port A clock enable during sleep mode" ]
    pub fn gpioalpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Ahb2lpenr {
    register: ::volatile_register::RW<u32>,
}

impl Ahb2lpenr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ahb2lpenrR, &'w mut Ahb2lpenrW) -> &'w mut Ahb2lpenrW
    {
        let bits = self.register.read();
        let r = Ahb2lpenrR { bits: bits };
        let mut w = Ahb2lpenrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ahb2lpenrR {
        Ahb2lpenrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ahb2lpenrW) -> &mut Ahb2lpenrW
    {
        let mut w = Ahb2lpenrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb2lpenrR {
    bits: u32,
}

impl Ahb2lpenrR {
    # [ doc = "Bit 7 - USB OTG FS clock enable during Sleep mode" ]
    pub fn otgfslpen(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb2lpenrW {
    bits: u32,
}

impl Ahb2lpenrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ahb2lpenrW { bits: 241u32 }
    }
    # [ doc = "Bit 7 - USB OTG FS clock enable during Sleep mode" ]
    pub fn otgfslpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Apb1lpenr {
    register: ::volatile_register::RW<u32>,
}

impl Apb1lpenr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Apb1lpenrR, &'w mut Apb1lpenrW) -> &'w mut Apb1lpenrW
    {
        let bits = self.register.read();
        let r = Apb1lpenrR { bits: bits };
        let mut w = Apb1lpenrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Apb1lpenrR {
        Apb1lpenrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb1lpenrW) -> &mut Apb1lpenrW
    {
        let mut w = Apb1lpenrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1lpenrR {
    bits: u32,
}

impl Apb1lpenrR {
    # [ doc = "Bit 28 - Power interface clock enable during Sleep mode" ]
    pub fn pwrlpen(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - I2C3 clock enable during Sleep mode" ]
    pub fn i2c3lpen(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - I2C2 clock enable during Sleep mode" ]
    pub fn i2c2lpen(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - I2C1 clock enable during Sleep mode" ]
    pub fn i2c1lpen(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - USART2 clock enable during Sleep mode" ]
    pub fn usart2lpen(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - SPI3 clock enable during Sleep mode" ]
    pub fn spi3lpen(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - SPI2 clock enable during Sleep mode" ]
    pub fn spi2lpen(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Window watchdog clock enable during Sleep mode" ]
    pub fn wwdglpen(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - TIM5 clock enable during Sleep mode" ]
    pub fn tim5lpen(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - TIM4 clock enable during Sleep mode" ]
    pub fn tim4lpen(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - TIM3 clock enable during Sleep mode" ]
    pub fn tim3lpen(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - TIM2 clock enable during Sleep mode" ]
    pub fn tim2lpen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1lpenrW {
    bits: u32,
}

impl Apb1lpenrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Apb1lpenrW { bits: 922667519u32 }
    }
    # [ doc = "Bit 28 - Power interface clock enable during Sleep mode" ]
    pub fn pwrlpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - I2C3 clock enable during Sleep mode" ]
    pub fn i2c3lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - I2C2 clock enable during Sleep mode" ]
    pub fn i2c2lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - I2C1 clock enable during Sleep mode" ]
    pub fn i2c1lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - USART2 clock enable during Sleep mode" ]
    pub fn usart2lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - SPI3 clock enable during Sleep mode" ]
    pub fn spi3lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - SPI2 clock enable during Sleep mode" ]
    pub fn spi2lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Window watchdog clock enable during Sleep mode" ]
    pub fn wwdglpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - TIM5 clock enable during Sleep mode" ]
    pub fn tim5lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - TIM4 clock enable during Sleep mode" ]
    pub fn tim4lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - TIM3 clock enable during Sleep mode" ]
    pub fn tim3lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - TIM2 clock enable during Sleep mode" ]
    pub fn tim2lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Apb2lpenr {
    register: ::volatile_register::RW<u32>,
}

impl Apb2lpenr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Apb2lpenrR, &'w mut Apb2lpenrW) -> &'w mut Apb2lpenrW
    {
        let bits = self.register.read();
        let r = Apb2lpenrR { bits: bits };
        let mut w = Apb2lpenrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Apb2lpenrR {
        Apb2lpenrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb2lpenrW) -> &mut Apb2lpenrW
    {
        let mut w = Apb2lpenrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb2lpenrR {
    bits: u32,
}

impl Apb2lpenrR {
    # [ doc = "Bit 0 - TIM1 clock enable during Sleep mode" ]
    pub fn tim1lpen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - USART1 clock enable during Sleep mode" ]
    pub fn usart1lpen(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - USART6 clock enable during Sleep mode" ]
    pub fn usart6lpen(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - ADC1 clock enable during Sleep mode" ]
    pub fn adc1lpen(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - SDIO clock enable during Sleep mode" ]
    pub fn sdiolpen(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - SPI 1 clock enable during Sleep mode" ]
    pub fn spi1lpen(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - SPI4 clock enable during Sleep mode" ]
    pub fn spi4lpen(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - System configuration controller clock enable during Sleep mode" ]
    pub fn syscfglpen(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - TIM9 clock enable during sleep mode" ]
    pub fn tim9lpen(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - TIM10 clock enable during Sleep mode" ]
    pub fn tim10lpen(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - TIM11 clock enable during Sleep mode" ]
    pub fn tim11lpen(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb2lpenrW {
    bits: u32,
}

impl Apb2lpenrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Apb2lpenrW { bits: 483123u32 }
    }
    # [ doc = "Bit 0 - TIM1 clock enable during Sleep mode" ]
    pub fn tim1lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - USART1 clock enable during Sleep mode" ]
    pub fn usart1lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - USART6 clock enable during Sleep mode" ]
    pub fn usart6lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - ADC1 clock enable during Sleep mode" ]
    pub fn adc1lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - SDIO clock enable during Sleep mode" ]
    pub fn sdiolpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - SPI 1 clock enable during Sleep mode" ]
    pub fn spi1lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - SPI4 clock enable during Sleep mode" ]
    pub fn spi4lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - System configuration controller clock enable during Sleep mode" ]
    pub fn syscfglpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - TIM9 clock enable during sleep mode" ]
    pub fn tim9lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - TIM10 clock enable during Sleep mode" ]
    pub fn tim10lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - TIM11 clock enable during Sleep mode" ]
    pub fn tim11lpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Bdcr {
    register: ::volatile_register::RW<u32>,
}

impl Bdcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BdcrR, &'w mut BdcrW) -> &'w mut BdcrW
    {
        let bits = self.register.read();
        let r = BdcrR { bits: bits };
        let mut w = BdcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BdcrR {
        BdcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BdcrW) -> &mut BdcrW
    {
        let mut w = BdcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BdcrR {
    bits: u32,
}

impl BdcrR {
    # [ doc = "Bit 16 - Backup domain software reset" ]
    pub fn bdrst(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - RTC clock enable" ]
    pub fn rtcen(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - RTC clock source selection" ]
    pub fn rtcsel1(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - RTC clock source selection" ]
    pub fn rtcsel0(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - External low-speed oscillator bypass" ]
    pub fn lsebyp(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - External low-speed oscillator ready" ]
    pub fn lserdy(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - External low-speed oscillator enable" ]
    pub fn lseon(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BdcrW {
    bits: u32,
}

impl BdcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BdcrW { bits: 0u32 }
    }
    # [ doc = "Bit 16 - Backup domain software reset" ]
    pub fn bdrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - RTC clock enable" ]
    pub fn rtcen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - RTC clock source selection" ]
    pub fn rtcsel1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - RTC clock source selection" ]
    pub fn rtcsel0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - External low-speed oscillator bypass" ]
    pub fn lsebyp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - External low-speed oscillator enable" ]
    pub fn lseon(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Csr {
    register: ::volatile_register::RW<u32>,
}

impl Csr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CsrR, &'w mut CsrW) -> &'w mut CsrW
    {
        let bits = self.register.read();
        let r = CsrR { bits: bits };
        let mut w = CsrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CsrR {
        CsrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CsrW) -> &mut CsrW
    {
        let mut w = CsrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CsrR {
    bits: u32,
}

impl CsrR {
    # [ doc = "Bit 31 - Low-power reset flag" ]
    pub fn lpwrrstf(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Window watchdog reset flag" ]
    pub fn wwdgrstf(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - Independent watchdog reset flag" ]
    pub fn wdgrstf(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 28 - Software reset flag" ]
    pub fn sftrstf(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 27 - POR/PDR reset flag" ]
    pub fn porrstf(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - PIN reset flag" ]
    pub fn padrstf(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - BOR reset flag" ]
    pub fn borrstf(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Remove reset flag" ]
    pub fn rmvf(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Internal low-speed oscillator ready" ]
    pub fn lsirdy(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Internal low-speed oscillator enable" ]
    pub fn lsion(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CsrW {
    bits: u32,
}

impl CsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CsrW { bits: 234881024u32 }
    }
    # [ doc = "Bit 31 - Low-power reset flag" ]
    pub fn lpwrrstf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Window watchdog reset flag" ]
    pub fn wwdgrstf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Independent watchdog reset flag" ]
    pub fn wdgrstf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Software reset flag" ]
    pub fn sftrstf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - POR/PDR reset flag" ]
    pub fn porrstf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - PIN reset flag" ]
    pub fn padrstf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - BOR reset flag" ]
    pub fn borrstf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - Remove reset flag" ]
    pub fn rmvf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Internal low-speed oscillator enable" ]
    pub fn lsion(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Sscgr {
    register: ::volatile_register::RW<u32>,
}

impl Sscgr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&SscgrR, &'w mut SscgrW) -> &'w mut SscgrW
    {
        let bits = self.register.read();
        let r = SscgrR { bits: bits };
        let mut w = SscgrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> SscgrR {
        SscgrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut SscgrW) -> &mut SscgrW
    {
        let mut w = SscgrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SscgrR {
    bits: u32,
}

impl SscgrR {
    # [ doc = "Bit 31 - Spread spectrum modulation enable" ]
    pub fn sscgen(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Spread Select" ]
    pub fn spreadsel(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 13:27 - Incrementation step" ]
    pub fn incstep(&self) -> u16 {
        const MASK: u32 = 32767;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:12 - Modulation period" ]
    pub fn modper(&self) -> u16 {
        const MASK: u32 = 8191;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SscgrW {
    bits: u32,
}

impl SscgrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        SscgrW { bits: 0u32 }
    }
    # [ doc = "Bit 31 - Spread spectrum modulation enable" ]
    pub fn sscgen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Spread Select" ]
    pub fn spreadsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 13:27 - Incrementation step" ]
    pub fn incstep(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u16 = 32767;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:12 - Modulation period" ]
    pub fn modper(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 8191;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Plli2scfgr {
    register: ::volatile_register::RW<u32>,
}

impl Plli2scfgr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Plli2scfgrR, &'w mut Plli2scfgrW) -> &'w mut Plli2scfgrW
    {
        let bits = self.register.read();
        let r = Plli2scfgrR { bits: bits };
        let mut w = Plli2scfgrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Plli2scfgrR {
        Plli2scfgrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Plli2scfgrW) -> &mut Plli2scfgrW
    {
        let mut w = Plli2scfgrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Plli2scfgrR {
    bits: u32,
}

impl Plli2scfgrR {
    # [ doc = "Bits 28:30 - PLLI2S division factor for I2S clocks" ]
    pub fn plli2srx(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 6:14 - PLLI2S multiplication factor for VCO" ]
    pub fn plli2snx(&self) -> u16 {
        const MASK: u32 = 511;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Plli2scfgrW {
    bits: u32,
}

impl Plli2scfgrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Plli2scfgrW { bits: 536883200u32 }
    }
    # [ doc = "Bits 28:30 - PLLI2S division factor for I2S clocks" ]
    pub fn plli2srx(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 6:14 - PLLI2S multiplication factor for VCO" ]
    pub fn plli2snx(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u16 = 511;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
