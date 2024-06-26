#[doc = "Register `PORT_DESC` reader"]
pub type R = crate::R<PORT_DESC_SPEC>;
#[doc = "Port Configuration\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PORT0_A {
    #[doc = "0: Not implemented"]
    VALUE1 = 0,
    #[doc = "1: Not configured (SII EEPROM)"]
    VALUE2 = 1,
    #[doc = "2: EBUS"]
    VALUE3 = 2,
    #[doc = "3: MII / RMII / RGMII"]
    VALUE4 = 3,
}
impl From<PORT0_A> for u8 {
    #[inline(always)]
    fn from(variant: PORT0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PORT0_A {
    type Ux = u8;
}
impl crate::IsEnum for PORT0_A {}
#[doc = "Field `Port0` reader - Port Configuration"]
pub type PORT0_R = crate::FieldReader<PORT0_A>;
impl PORT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PORT0_A {
        match self.bits {
            0 => PORT0_A::VALUE1,
            1 => PORT0_A::VALUE2,
            2 => PORT0_A::VALUE3,
            3 => PORT0_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Not implemented"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PORT0_A::VALUE1
    }
    #[doc = "Not configured (SII EEPROM)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PORT0_A::VALUE2
    }
    #[doc = "EBUS"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PORT0_A::VALUE3
    }
    #[doc = "MII / RMII / RGMII"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PORT0_A::VALUE4
    }
}
#[doc = "Port Configuration\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PORT1_A {
    #[doc = "0: Not implemented"]
    VALUE1 = 0,
    #[doc = "1: Not configured (SII EEPROM)"]
    VALUE2 = 1,
    #[doc = "2: EBUS"]
    VALUE3 = 2,
    #[doc = "3: MII / RMII / RGMII"]
    VALUE4 = 3,
}
impl From<PORT1_A> for u8 {
    #[inline(always)]
    fn from(variant: PORT1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PORT1_A {
    type Ux = u8;
}
impl crate::IsEnum for PORT1_A {}
#[doc = "Field `Port1` reader - Port Configuration"]
pub type PORT1_R = crate::FieldReader<PORT1_A>;
impl PORT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PORT1_A {
        match self.bits {
            0 => PORT1_A::VALUE1,
            1 => PORT1_A::VALUE2,
            2 => PORT1_A::VALUE3,
            3 => PORT1_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Not implemented"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PORT1_A::VALUE1
    }
    #[doc = "Not configured (SII EEPROM)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PORT1_A::VALUE2
    }
    #[doc = "EBUS"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PORT1_A::VALUE3
    }
    #[doc = "MII / RMII / RGMII"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PORT1_A::VALUE4
    }
}
#[doc = "Port Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PORT2_A {
    #[doc = "0: Not implemented"]
    VALUE1 = 0,
    #[doc = "1: Not configured (SII EEPROM)"]
    VALUE2 = 1,
    #[doc = "2: EBUS"]
    VALUE3 = 2,
    #[doc = "3: MII / RMII / RGMII"]
    VALUE4 = 3,
}
impl From<PORT2_A> for u8 {
    #[inline(always)]
    fn from(variant: PORT2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PORT2_A {
    type Ux = u8;
}
impl crate::IsEnum for PORT2_A {}
#[doc = "Field `Port2` reader - Port Configuration"]
pub type PORT2_R = crate::FieldReader<PORT2_A>;
impl PORT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PORT2_A {
        match self.bits {
            0 => PORT2_A::VALUE1,
            1 => PORT2_A::VALUE2,
            2 => PORT2_A::VALUE3,
            3 => PORT2_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Not implemented"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PORT2_A::VALUE1
    }
    #[doc = "Not configured (SII EEPROM)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PORT2_A::VALUE2
    }
    #[doc = "EBUS"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PORT2_A::VALUE3
    }
    #[doc = "MII / RMII / RGMII"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PORT2_A::VALUE4
    }
}
#[doc = "Port Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PORT3_A {
    #[doc = "0: Not implemented"]
    VALUE1 = 0,
    #[doc = "1: Not configured (SII EEPROM)"]
    VALUE2 = 1,
    #[doc = "2: EBUS"]
    VALUE3 = 2,
    #[doc = "3: MII / RMII / RGMII"]
    VALUE4 = 3,
}
impl From<PORT3_A> for u8 {
    #[inline(always)]
    fn from(variant: PORT3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PORT3_A {
    type Ux = u8;
}
impl crate::IsEnum for PORT3_A {}
#[doc = "Field `Port3` reader - Port Configuration"]
pub type PORT3_R = crate::FieldReader<PORT3_A>;
impl PORT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PORT3_A {
        match self.bits {
            0 => PORT3_A::VALUE1,
            1 => PORT3_A::VALUE2,
            2 => PORT3_A::VALUE3,
            3 => PORT3_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Not implemented"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PORT3_A::VALUE1
    }
    #[doc = "Not configured (SII EEPROM)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PORT3_A::VALUE2
    }
    #[doc = "EBUS"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PORT3_A::VALUE3
    }
    #[doc = "MII / RMII / RGMII"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PORT3_A::VALUE4
    }
}
impl R {
    #[doc = "Bits 0:1 - Port Configuration"]
    #[inline(always)]
    pub fn port0(&self) -> PORT0_R {
        PORT0_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Port Configuration"]
    #[inline(always)]
    pub fn port1(&self) -> PORT1_R {
        PORT1_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Port Configuration"]
    #[inline(always)]
    pub fn port2(&self) -> PORT2_R {
        PORT2_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Port Configuration"]
    #[inline(always)]
    pub fn port3(&self) -> PORT3_R {
        PORT3_R::new((self.bits >> 6) & 3)
    }
}
#[doc = "Port Descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`port_desc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PORT_DESC_SPEC;
impl crate::RegisterSpec for PORT_DESC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`port_desc::R`](R) reader structure"]
impl crate::Readable for PORT_DESC_SPEC {}
#[doc = "`reset()` method sets PORT_DESC to value 0x0f"]
impl crate::Resettable for PORT_DESC_SPEC {
    const RESET_VALUE: u8 = 0x0f;
}
