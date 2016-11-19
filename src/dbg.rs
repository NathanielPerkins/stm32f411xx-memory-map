# [ doc = "Debug support" ]
# [ repr ( C ) ]
pub struct Dbg {
    # [ doc = "0x00 - IDCODE" ]
    pub dbgmcu_idcode: DbgmcuIdcode,
    # [ doc = "0x04 - Control Register" ]
    pub dbgmcu_cr: DbgmcuCr,
    # [ doc = "0x08 - Debug MCU APB1 Freeze registe" ]
    pub dbgmcu_apb1_fz: DbgmcuApb1Fz,
    # [ doc = "0x0c - Debug MCU APB2 Freeze registe" ]
    pub dbgmcu_apb2_fz: DbgmcuApb2Fz,
}

# [ repr ( C ) ]
pub struct DbgmcuIdcode {
    register: ::volatile_register::RO<u32>,
}

impl DbgmcuIdcode {
    pub fn read(&self) -> DbgmcuIdcodeR {
        DbgmcuIdcodeR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DbgmcuIdcodeR {
    bits: u32,
}

impl DbgmcuIdcodeR {
    # [ doc = "Bits 0:11 - DEV_ID" ]
    pub fn dev_id(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - REV_ID" ]
    pub fn rev_id(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DbgmcuIdcodeW {
    bits: u32,
}

impl DbgmcuIdcodeW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DbgmcuIdcodeW { bits: 268461073u32 }
    }
    # [ doc = "Bits 0:11 - DEV_ID" ]
    pub fn dev_id(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - REV_ID" ]
    pub fn rev_id(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct DbgmcuCr {
    register: ::volatile_register::RW<u32>,
}

impl DbgmcuCr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DbgmcuCrR, &'w mut DbgmcuCrW) -> &'w mut DbgmcuCrW
    {
        let bits = self.register.read();
        let r = DbgmcuCrR { bits: bits };
        let mut w = DbgmcuCrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DbgmcuCrR {
        DbgmcuCrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DbgmcuCrW) -> &mut DbgmcuCrW
    {
        let mut w = DbgmcuCrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DbgmcuCrR {
    bits: u32,
}

impl DbgmcuCrR {
    # [ doc = "Bit 0 - DBG_SLEEP" ]
    pub fn dbg_sleep(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - DBG_STOP" ]
    pub fn dbg_stop(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - DBG_STANDBY" ]
    pub fn dbg_standby(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - TRACE_IOEN" ]
    pub fn trace_ioen(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 6:7 - TRACE_MODE" ]
    pub fn trace_mode(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DbgmcuCrW {
    bits: u32,
}

impl DbgmcuCrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DbgmcuCrW { bits: 0u32 }
    }
    # [ doc = "Bit 0 - DBG_SLEEP" ]
    pub fn dbg_sleep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - DBG_STOP" ]
    pub fn dbg_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - DBG_STANDBY" ]
    pub fn dbg_standby(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - TRACE_IOEN" ]
    pub fn trace_ioen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 6:7 - TRACE_MODE" ]
    pub fn trace_mode(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct DbgmcuApb1Fz {
    register: ::volatile_register::RW<u32>,
}

impl DbgmcuApb1Fz {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DbgmcuApb1FzR, &'w mut DbgmcuApb1FzW) -> &'w mut DbgmcuApb1FzW
    {
        let bits = self.register.read();
        let r = DbgmcuApb1FzR { bits: bits };
        let mut w = DbgmcuApb1FzW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DbgmcuApb1FzR {
        DbgmcuApb1FzR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DbgmcuApb1FzW) -> &mut DbgmcuApb1FzW
    {
        let mut w = DbgmcuApb1FzW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DbgmcuApb1FzR {
    bits: u32,
}

impl DbgmcuApb1FzR {
    # [ doc = "Bit 0 - DBG_TIM2_STOP" ]
    pub fn dbg_tim2_stop(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - DBG_TIM3 _STOP" ]
    pub fn dbg_tim3_stop(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - DBG_TIM4_STOP" ]
    pub fn dbg_tim4_stop(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - DBG_TIM5_STOP" ]
    pub fn dbg_tim5_stop(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - RTC stopped when Core is halted" ]
    pub fn dbg_rtc_stop(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - DBG_WWDG_STOP" ]
    pub fn dbg_wwdg_stop(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - DBG_IWDEG_STOP" ]
    pub fn dbg_iwdeg_stop(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - DBG_J2C1_SMBUS_TIMEOUT" ]
    pub fn dbg_i2c1_smbus_timeout(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - DBG_J2C2_SMBUS_TIMEOUT" ]
    pub fn dbg_i2c2_smbus_timeout(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - DBG_J2C3SMBUS_TIMEOUT" ]
    pub fn dbg_i2c3smbus_timeout(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DbgmcuApb1FzW {
    bits: u32,
}

impl DbgmcuApb1FzW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DbgmcuApb1FzW { bits: 0u32 }
    }
    # [ doc = "Bit 0 - DBG_TIM2_STOP" ]
    pub fn dbg_tim2_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - DBG_TIM3 _STOP" ]
    pub fn dbg_tim3_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - DBG_TIM4_STOP" ]
    pub fn dbg_tim4_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - DBG_TIM5_STOP" ]
    pub fn dbg_tim5_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - RTC stopped when Core is halted" ]
    pub fn dbg_rtc_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - DBG_WWDG_STOP" ]
    pub fn dbg_wwdg_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - DBG_IWDEG_STOP" ]
    pub fn dbg_iwdeg_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - DBG_J2C1_SMBUS_TIMEOUT" ]
    pub fn dbg_i2c1_smbus_timeout(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - DBG_J2C2_SMBUS_TIMEOUT" ]
    pub fn dbg_i2c2_smbus_timeout(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - DBG_J2C3SMBUS_TIMEOUT" ]
    pub fn dbg_i2c3smbus_timeout(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct DbgmcuApb2Fz {
    register: ::volatile_register::RW<u32>,
}

impl DbgmcuApb2Fz {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DbgmcuApb2FzR, &'w mut DbgmcuApb2FzW) -> &'w mut DbgmcuApb2FzW
    {
        let bits = self.register.read();
        let r = DbgmcuApb2FzR { bits: bits };
        let mut w = DbgmcuApb2FzW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DbgmcuApb2FzR {
        DbgmcuApb2FzR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DbgmcuApb2FzW) -> &mut DbgmcuApb2FzW
    {
        let mut w = DbgmcuApb2FzW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DbgmcuApb2FzR {
    bits: u32,
}

impl DbgmcuApb2FzR {
    # [ doc = "Bit 0 - TIM1 counter stopped when core is halted" ]
    pub fn dbg_tim1_stop(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - TIM9 counter stopped when core is halted" ]
    pub fn dbg_tim9_stop(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - TIM10 counter stopped when core is halted" ]
    pub fn dbg_tim10_stop(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - TIM11 counter stopped when core is halted" ]
    pub fn dbg_tim11_stop(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DbgmcuApb2FzW {
    bits: u32,
}

impl DbgmcuApb2FzW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DbgmcuApb2FzW { bits: 0u32 }
    }
    # [ doc = "Bit 0 - TIM1 counter stopped when core is halted" ]
    pub fn dbg_tim1_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - TIM9 counter stopped when core is halted" ]
    pub fn dbg_tim9_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - TIM10 counter stopped when core is halted" ]
    pub fn dbg_tim10_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - TIM11 counter stopped when core is halted" ]
    pub fn dbg_tim11_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}
