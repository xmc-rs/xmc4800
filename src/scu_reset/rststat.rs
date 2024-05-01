#[doc = "Register `RSTSTAT` reader"]
pub type R = crate::R<RststatSpec>;
#[doc = "Reset Status Information\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rststat {
    #[doc = "1: PORST reset"]
    Value1 = 1,
    #[doc = "2: SWD reset"]
    Value2 = 2,
    #[doc = "4: PV reset"]
    Value3 = 4,
    #[doc = "8: CPU system reset"]
    Value4 = 8,
    #[doc = "16: CPU lockup reset"]
    Value5 = 16,
    #[doc = "32: WDT reset"]
    Value6 = 32,
    #[doc = "128: Parity Error reset"]
    Value8 = 128,
}
impl From<Rststat> for u8 {
    #[inline(always)]
    fn from(variant: Rststat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rststat {
    type Ux = u8;
}
impl crate::IsEnum for Rststat {}
#[doc = "Field `RSTSTAT` reader - Reset Status Information"]
pub type RststatR = crate::FieldReader<Rststat>;
impl RststatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rststat> {
        match self.bits {
            1 => Some(Rststat::Value1),
            2 => Some(Rststat::Value2),
            4 => Some(Rststat::Value3),
            8 => Some(Rststat::Value4),
            16 => Some(Rststat::Value5),
            32 => Some(Rststat::Value6),
            128 => Some(Rststat::Value8),
            _ => None,
        }
    }
    #[doc = "PORST reset"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rststat::Value1
    }
    #[doc = "SWD reset"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rststat::Value2
    }
    #[doc = "PV reset"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rststat::Value3
    }
    #[doc = "CPU system reset"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rststat::Value4
    }
    #[doc = "CPU lockup reset"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Rststat::Value5
    }
    #[doc = "WDT reset"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Rststat::Value6
    }
    #[doc = "Parity Error reset"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Rststat::Value8
    }
}
#[doc = "Hibernate Wake-up Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hibwk {
    #[doc = "0: No Wake-up"]
    Value1 = 0,
    #[doc = "1: Wake-up event"]
    Value2 = 1,
}
impl From<Hibwk> for bool {
    #[inline(always)]
    fn from(variant: Hibwk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBWK` reader - Hibernate Wake-up Status"]
pub type HibwkR = crate::BitReader<Hibwk>;
impl HibwkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hibwk {
        match self.bits {
            false => Hibwk::Value1,
            true => Hibwk::Value2,
        }
    }
    #[doc = "No Wake-up"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hibwk::Value1
    }
    #[doc = "Wake-up event"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hibwk::Value2
    }
}
#[doc = "Hibernate Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hibrs {
    #[doc = "0: Reset de-asserted"]
    Value1 = 0,
    #[doc = "1: Reset asserted"]
    Value2 = 1,
}
impl From<Hibrs> for bool {
    #[inline(always)]
    fn from(variant: Hibrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBRS` reader - Hibernate Reset Status"]
pub type HibrsR = crate::BitReader<Hibrs>;
impl HibrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hibrs {
        match self.bits {
            false => Hibrs::Value1,
            true => Hibrs::Value2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hibrs::Value1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hibrs::Value2
    }
}
#[doc = "Enable Lockup Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcken {
    #[doc = "0: Reset by Lockup disabled"]
    Value1 = 0,
    #[doc = "1: Reset by Lockup enabled"]
    Value2 = 1,
}
impl From<Lcken> for bool {
    #[inline(always)]
    fn from(variant: Lcken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCKEN` reader - Enable Lockup Status"]
pub type LckenR = crate::BitReader<Lcken>;
impl LckenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcken {
        match self.bits {
            false => Lcken::Value1,
            true => Lcken::Value2,
        }
    }
    #[doc = "Reset by Lockup disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lcken::Value1
    }
    #[doc = "Reset by Lockup enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lcken::Value2
    }
}
#[doc = "ECAT0 Reset Status Information\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecat0rs {
    #[doc = "0: Reset did not occur"]
    Value1 = 0,
    #[doc = "1: Reset occurred"]
    Value2 = 1,
}
impl From<Ecat0rs> for bool {
    #[inline(always)]
    fn from(variant: Ecat0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECAT0RS` reader - ECAT0 Reset Status Information"]
pub type Ecat0rsR = crate::BitReader<Ecat0rs>;
impl Ecat0rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecat0rs {
        match self.bits {
            false => Ecat0rs::Value1,
            true => Ecat0rs::Value2,
        }
    }
    #[doc = "Reset did not occur"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ecat0rs::Value1
    }
    #[doc = "Reset occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ecat0rs::Value2
    }
}
impl R {
    #[doc = "Bits 0:7 - Reset Status Information"]
    #[inline(always)]
    pub fn rststat(&self) -> RststatR {
        RststatR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Hibernate Wake-up Status"]
    #[inline(always)]
    pub fn hibwk(&self) -> HibwkR {
        HibwkR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Hibernate Reset Status"]
    #[inline(always)]
    pub fn hibrs(&self) -> HibrsR {
        HibrsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Lockup Status"]
    #[inline(always)]
    pub fn lcken(&self) -> LckenR {
        LckenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - ECAT0 Reset Status Information"]
    #[inline(always)]
    pub fn ecat0rs(&self) -> Ecat0rsR {
        Ecat0rsR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "RCU Reset Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rststat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RststatSpec;
impl crate::RegisterSpec for RststatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rststat::R`](R) reader structure"]
impl crate::Readable for RststatSpec {}
#[doc = "`reset()` method sets RSTSTAT to value 0"]
impl crate::Resettable for RststatSpec {
    const RESET_VALUE: u32 = 0;
}
