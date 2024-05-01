#[doc = "Register `PRSTAT0` reader"]
pub type R = crate::R<Prstat0Spec>;
#[doc = "VADC Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vadcrs {
    #[doc = "0: Reset de-asserted"]
    Value1 = 0,
    #[doc = "1: Reset asserted"]
    Value2 = 1,
}
impl From<Vadcrs> for bool {
    #[inline(always)]
    fn from(variant: Vadcrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VADCRS` reader - VADC Reset Status"]
pub type VadcrsR = crate::BitReader<Vadcrs>;
impl VadcrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vadcrs {
        match self.bits {
            false => Vadcrs::Value1,
            true => Vadcrs::Value2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vadcrs::Value1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vadcrs::Value2
    }
}
#[doc = "DSD Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsdrs {
    #[doc = "0: Reset de-asserted"]
    Value1 = 0,
    #[doc = "1: Reset asserted"]
    Value2 = 1,
}
impl From<Dsdrs> for bool {
    #[inline(always)]
    fn from(variant: Dsdrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSDRS` reader - DSD Reset Status"]
pub type DsdrsR = crate::BitReader<Dsdrs>;
impl DsdrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsdrs {
        match self.bits {
            false => Dsdrs::Value1,
            true => Dsdrs::Value2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dsdrs::Value1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dsdrs::Value2
    }
}
#[doc = "CCU40 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu40rs {
    #[doc = "0: Reset de-asserted"]
    Value1 = 0,
    #[doc = "1: Reset asserted"]
    Value2 = 1,
}
impl From<Ccu40rs> for bool {
    #[inline(always)]
    fn from(variant: Ccu40rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU40RS` reader - CCU40 Reset Status"]
pub type Ccu40rsR = crate::BitReader<Ccu40rs>;
impl Ccu40rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu40rs {
        match self.bits {
            false => Ccu40rs::Value1,
            true => Ccu40rs::Value2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ccu40rs::Value1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ccu40rs::Value2
    }
}
#[doc = "CCU41 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu41rs {
    #[doc = "0: Reset de-asserted"]
    Value1 = 0,
    #[doc = "1: Reset asserted"]
    Value2 = 1,
}
impl From<Ccu41rs> for bool {
    #[inline(always)]
    fn from(variant: Ccu41rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU41RS` reader - CCU41 Reset Status"]
pub type Ccu41rsR = crate::BitReader<Ccu41rs>;
impl Ccu41rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu41rs {
        match self.bits {
            false => Ccu41rs::Value1,
            true => Ccu41rs::Value2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ccu41rs::Value1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ccu41rs::Value2
    }
}
#[doc = "CCU42 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu42rs {
    #[doc = "0: Reset de-asserted"]
    Value1 = 0,
    #[doc = "1: Reset asserted"]
    Value2 = 1,
}
impl From<Ccu42rs> for bool {
    #[inline(always)]
    fn from(variant: Ccu42rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU42RS` reader - CCU42 Reset Status"]
pub type Ccu42rsR = crate::BitReader<Ccu42rs>;
impl Ccu42rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu42rs {
        match self.bits {
            false => Ccu42rs::Value1,
            true => Ccu42rs::Value2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ccu42rs::Value1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ccu42rs::Value2
    }
}
#[doc = "CCU80 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu80rs {
    #[doc = "0: Reset de-asserted"]
    Value1 = 0,
    #[doc = "1: Reset asserted"]
    Value2 = 1,
}
impl From<Ccu80rs> for bool {
    #[inline(always)]
    fn from(variant: Ccu80rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU80RS` reader - CCU80 Reset Status"]
pub type Ccu80rsR = crate::BitReader<Ccu80rs>;
impl Ccu80rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu80rs {
        match self.bits {
            false => Ccu80rs::Value1,
            true => Ccu80rs::Value2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ccu80rs::Value1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ccu80rs::Value2
    }
}
#[doc = "CCU81 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu81rs {
    #[doc = "0: Reset de-asserted"]
    Value1 = 0,
    #[doc = "1: Reset asserted"]
    Value2 = 1,
}
impl From<Ccu81rs> for bool {
    #[inline(always)]
    fn from(variant: Ccu81rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU81RS` reader - CCU81 Reset Status"]
pub type Ccu81rsR = crate::BitReader<Ccu81rs>;
impl Ccu81rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu81rs {
        match self.bits {
            false => Ccu81rs::Value1,
            true => Ccu81rs::Value2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ccu81rs::Value1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ccu81rs::Value2
    }
}
#[doc = "POSIF0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posif0rs {
    #[doc = "0: Reset de-asserted"]
    Value1 = 0,
    #[doc = "1: Reset asserted"]
    Value2 = 1,
}
impl From<Posif0rs> for bool {
    #[inline(always)]
    fn from(variant: Posif0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSIF0RS` reader - POSIF0 Reset Status"]
pub type Posif0rsR = crate::BitReader<Posif0rs>;
impl Posif0rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Posif0rs {
        match self.bits {
            false => Posif0rs::Value1,
            true => Posif0rs::Value2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Posif0rs::Value1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Posif0rs::Value2
    }
}
#[doc = "POSIF1 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posif1rs {
    #[doc = "0: Reset de-asserted"]
    Value1 = 0,
    #[doc = "1: Reset asserted"]
    Value2 = 1,
}
impl From<Posif1rs> for bool {
    #[inline(always)]
    fn from(variant: Posif1rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSIF1RS` reader - POSIF1 Reset Status"]
pub type Posif1rsR = crate::BitReader<Posif1rs>;
impl Posif1rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Posif1rs {
        match self.bits {
            false => Posif1rs::Value1,
            true => Posif1rs::Value2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Posif1rs::Value1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Posif1rs::Value2
    }
}
#[doc = "USIC0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usic0rs {
    #[doc = "0: Reset de-asserted"]
    Value1 = 0,
    #[doc = "1: Reset asserted"]
    Value2 = 1,
}
impl From<Usic0rs> for bool {
    #[inline(always)]
    fn from(variant: Usic0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC0RS` reader - USIC0 Reset Status"]
pub type Usic0rsR = crate::BitReader<Usic0rs>;
impl Usic0rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usic0rs {
        match self.bits {
            false => Usic0rs::Value1,
            true => Usic0rs::Value2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Usic0rs::Value1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Usic0rs::Value2
    }
}
#[doc = "ERU1 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eru1rs {
    #[doc = "0: Reset de-asserted"]
    Value1 = 0,
    #[doc = "1: Reset asserted"]
    Value2 = 1,
}
impl From<Eru1rs> for bool {
    #[inline(always)]
    fn from(variant: Eru1rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU1RS` reader - ERU1 Reset Status"]
pub type Eru1rsR = crate::BitReader<Eru1rs>;
impl Eru1rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eru1rs {
        match self.bits {
            false => Eru1rs::Value1,
            true => Eru1rs::Value2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Eru1rs::Value1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Eru1rs::Value2
    }
}
impl R {
    #[doc = "Bit 0 - VADC Reset Status"]
    #[inline(always)]
    pub fn vadcrs(&self) -> VadcrsR {
        VadcrsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSD Reset Status"]
    #[inline(always)]
    pub fn dsdrs(&self) -> DsdrsR {
        DsdrsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CCU40 Reset Status"]
    #[inline(always)]
    pub fn ccu40rs(&self) -> Ccu40rsR {
        Ccu40rsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCU41 Reset Status"]
    #[inline(always)]
    pub fn ccu41rs(&self) -> Ccu41rsR {
        Ccu41rsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CCU42 Reset Status"]
    #[inline(always)]
    pub fn ccu42rs(&self) -> Ccu42rsR {
        Ccu42rsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - CCU80 Reset Status"]
    #[inline(always)]
    pub fn ccu80rs(&self) -> Ccu80rsR {
        Ccu80rsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CCU81 Reset Status"]
    #[inline(always)]
    pub fn ccu81rs(&self) -> Ccu81rsR {
        Ccu81rsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - POSIF0 Reset Status"]
    #[inline(always)]
    pub fn posif0rs(&self) -> Posif0rsR {
        Posif0rsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - POSIF1 Reset Status"]
    #[inline(always)]
    pub fn posif1rs(&self) -> Posif1rsR {
        Posif1rsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USIC0 Reset Status"]
    #[inline(always)]
    pub fn usic0rs(&self) -> Usic0rsR {
        Usic0rsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - ERU1 Reset Status"]
    #[inline(always)]
    pub fn eru1rs(&self) -> Eru1rsR {
        Eru1rsR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "RCU Peripheral 0 Reset Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstat0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstat0Spec;
impl crate::RegisterSpec for Prstat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prstat0::R`](R) reader structure"]
impl crate::Readable for Prstat0Spec {}
#[doc = "`reset()` method sets PRSTAT0 to value 0x0001_0f9f"]
impl crate::Resettable for Prstat0Spec {
    const RESET_VALUE: u32 = 0x0001_0f9f;
}
