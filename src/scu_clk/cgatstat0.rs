#[doc = "Register `CGATSTAT0` reader"]
pub type R = crate::R<Cgatstat0Spec>;
#[doc = "VADC Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vadc {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Vadc> for bool {
    #[inline(always)]
    fn from(variant: Vadc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VADC` reader - VADC Gating Status"]
pub type VadcR = crate::BitReader<Vadc>;
impl VadcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vadc {
        match self.bits {
            false => Vadc::Value1,
            true => Vadc::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vadc::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vadc::Value2
    }
}
#[doc = "DSD Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsd {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Dsd> for bool {
    #[inline(always)]
    fn from(variant: Dsd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSD` reader - DSD Gating Status"]
pub type DsdR = crate::BitReader<Dsd>;
impl DsdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsd {
        match self.bits {
            false => Dsd::Value1,
            true => Dsd::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dsd::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dsd::Value2
    }
}
#[doc = "CCU40 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu40 {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Ccu40> for bool {
    #[inline(always)]
    fn from(variant: Ccu40) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU40` reader - CCU40 Gating Status"]
pub type Ccu40R = crate::BitReader<Ccu40>;
impl Ccu40R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu40 {
        match self.bits {
            false => Ccu40::Value1,
            true => Ccu40::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ccu40::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ccu40::Value2
    }
}
#[doc = "CCU41 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu41 {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Ccu41> for bool {
    #[inline(always)]
    fn from(variant: Ccu41) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU41` reader - CCU41 Gating Status"]
pub type Ccu41R = crate::BitReader<Ccu41>;
impl Ccu41R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu41 {
        match self.bits {
            false => Ccu41::Value1,
            true => Ccu41::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ccu41::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ccu41::Value2
    }
}
#[doc = "CCU42 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu42 {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Ccu42> for bool {
    #[inline(always)]
    fn from(variant: Ccu42) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU42` reader - CCU42 Gating Status"]
pub type Ccu42R = crate::BitReader<Ccu42>;
impl Ccu42R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu42 {
        match self.bits {
            false => Ccu42::Value1,
            true => Ccu42::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ccu42::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ccu42::Value2
    }
}
#[doc = "CCU80 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu80 {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Ccu80> for bool {
    #[inline(always)]
    fn from(variant: Ccu80) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU80` reader - CCU80 Gating Status"]
pub type Ccu80R = crate::BitReader<Ccu80>;
impl Ccu80R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu80 {
        match self.bits {
            false => Ccu80::Value1,
            true => Ccu80::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ccu80::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ccu80::Value2
    }
}
#[doc = "CCU81 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu81 {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Ccu81> for bool {
    #[inline(always)]
    fn from(variant: Ccu81) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU81` reader - CCU81 Gating Status"]
pub type Ccu81R = crate::BitReader<Ccu81>;
impl Ccu81R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccu81 {
        match self.bits {
            false => Ccu81::Value1,
            true => Ccu81::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ccu81::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ccu81::Value2
    }
}
#[doc = "POSIF0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posif0 {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Posif0> for bool {
    #[inline(always)]
    fn from(variant: Posif0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSIF0` reader - POSIF0 Gating Status"]
pub type Posif0R = crate::BitReader<Posif0>;
impl Posif0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Posif0 {
        match self.bits {
            false => Posif0::Value1,
            true => Posif0::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Posif0::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Posif0::Value2
    }
}
#[doc = "POSIF1 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posif1 {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Posif1> for bool {
    #[inline(always)]
    fn from(variant: Posif1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSIF1` reader - POSIF1 Gating Status"]
pub type Posif1R = crate::BitReader<Posif1>;
impl Posif1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Posif1 {
        match self.bits {
            false => Posif1::Value1,
            true => Posif1::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Posif1::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Posif1::Value2
    }
}
#[doc = "USIC0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usic0 {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Usic0> for bool {
    #[inline(always)]
    fn from(variant: Usic0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC0` reader - USIC0 Gating Status"]
pub type Usic0R = crate::BitReader<Usic0>;
impl Usic0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usic0 {
        match self.bits {
            false => Usic0::Value1,
            true => Usic0::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Usic0::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Usic0::Value2
    }
}
#[doc = "ERU1 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eru1 {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Eru1> for bool {
    #[inline(always)]
    fn from(variant: Eru1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU1` reader - ERU1 Gating Status"]
pub type Eru1R = crate::BitReader<Eru1>;
impl Eru1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eru1 {
        match self.bits {
            false => Eru1::Value1,
            true => Eru1::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Eru1::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Eru1::Value2
    }
}
impl R {
    #[doc = "Bit 0 - VADC Gating Status"]
    #[inline(always)]
    pub fn vadc(&self) -> VadcR {
        VadcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSD Gating Status"]
    #[inline(always)]
    pub fn dsd(&self) -> DsdR {
        DsdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CCU40 Gating Status"]
    #[inline(always)]
    pub fn ccu40(&self) -> Ccu40R {
        Ccu40R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCU41 Gating Status"]
    #[inline(always)]
    pub fn ccu41(&self) -> Ccu41R {
        Ccu41R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CCU42 Gating Status"]
    #[inline(always)]
    pub fn ccu42(&self) -> Ccu42R {
        Ccu42R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - CCU80 Gating Status"]
    #[inline(always)]
    pub fn ccu80(&self) -> Ccu80R {
        Ccu80R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CCU81 Gating Status"]
    #[inline(always)]
    pub fn ccu81(&self) -> Ccu81R {
        Ccu81R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - POSIF0 Gating Status"]
    #[inline(always)]
    pub fn posif0(&self) -> Posif0R {
        Posif0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - POSIF1 Gating Status"]
    #[inline(always)]
    pub fn posif1(&self) -> Posif1R {
        Posif1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USIC0 Gating Status"]
    #[inline(always)]
    pub fn usic0(&self) -> Usic0R {
        Usic0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - ERU1 Gating Status"]
    #[inline(always)]
    pub fn eru1(&self) -> Eru1R {
        Eru1R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Peripheral 0 Clock Gating Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgatstat0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cgatstat0Spec;
impl crate::RegisterSpec for Cgatstat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgatstat0::R`](R) reader structure"]
impl crate::Readable for Cgatstat0Spec {}
#[doc = "`reset()` method sets CGATSTAT0 to value 0"]
impl crate::Resettable for Cgatstat0Spec {
    const RESET_VALUE: u32 = 0;
}
