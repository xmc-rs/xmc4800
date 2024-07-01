#[doc = "Register `PRSTAT0` reader"]
pub type R = crate::R<PRSTAT0_SPEC>;
#[doc = "VADC Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VADCRS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<VADCRS_A> for bool {
    #[inline(always)]
    fn from(variant: VADCRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VADCRS` reader - VADC Reset Status"]
pub type VADCRS_R = crate::BitReader<VADCRS_A>;
impl VADCRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VADCRS_A {
        match self.bits {
            false => VADCRS_A::VALUE1,
            true => VADCRS_A::VALUE2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VADCRS_A::VALUE1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VADCRS_A::VALUE2
    }
}
#[doc = "DSD Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSDRS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<DSDRS_A> for bool {
    #[inline(always)]
    fn from(variant: DSDRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSDRS` reader - DSD Reset Status"]
pub type DSDRS_R = crate::BitReader<DSDRS_A>;
impl DSDRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DSDRS_A {
        match self.bits {
            false => DSDRS_A::VALUE1,
            true => DSDRS_A::VALUE2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DSDRS_A::VALUE1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DSDRS_A::VALUE2
    }
}
#[doc = "CCU40 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU40RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<CCU40RS_A> for bool {
    #[inline(always)]
    fn from(variant: CCU40RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU40RS` reader - CCU40 Reset Status"]
pub type CCU40RS_R = crate::BitReader<CCU40RS_A>;
impl CCU40RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCU40RS_A {
        match self.bits {
            false => CCU40RS_A::VALUE1,
            true => CCU40RS_A::VALUE2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCU40RS_A::VALUE1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCU40RS_A::VALUE2
    }
}
#[doc = "CCU41 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU41RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<CCU41RS_A> for bool {
    #[inline(always)]
    fn from(variant: CCU41RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU41RS` reader - CCU41 Reset Status"]
pub type CCU41RS_R = crate::BitReader<CCU41RS_A>;
impl CCU41RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCU41RS_A {
        match self.bits {
            false => CCU41RS_A::VALUE1,
            true => CCU41RS_A::VALUE2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCU41RS_A::VALUE1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCU41RS_A::VALUE2
    }
}
#[doc = "CCU42 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU42RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<CCU42RS_A> for bool {
    #[inline(always)]
    fn from(variant: CCU42RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU42RS` reader - CCU42 Reset Status"]
pub type CCU42RS_R = crate::BitReader<CCU42RS_A>;
impl CCU42RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCU42RS_A {
        match self.bits {
            false => CCU42RS_A::VALUE1,
            true => CCU42RS_A::VALUE2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCU42RS_A::VALUE1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCU42RS_A::VALUE2
    }
}
#[doc = "CCU80 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU80RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<CCU80RS_A> for bool {
    #[inline(always)]
    fn from(variant: CCU80RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU80RS` reader - CCU80 Reset Status"]
pub type CCU80RS_R = crate::BitReader<CCU80RS_A>;
impl CCU80RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCU80RS_A {
        match self.bits {
            false => CCU80RS_A::VALUE1,
            true => CCU80RS_A::VALUE2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCU80RS_A::VALUE1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCU80RS_A::VALUE2
    }
}
#[doc = "CCU81 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU81RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<CCU81RS_A> for bool {
    #[inline(always)]
    fn from(variant: CCU81RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU81RS` reader - CCU81 Reset Status"]
pub type CCU81RS_R = crate::BitReader<CCU81RS_A>;
impl CCU81RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCU81RS_A {
        match self.bits {
            false => CCU81RS_A::VALUE1,
            true => CCU81RS_A::VALUE2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCU81RS_A::VALUE1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCU81RS_A::VALUE2
    }
}
#[doc = "POSIF0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POSIF0RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<POSIF0RS_A> for bool {
    #[inline(always)]
    fn from(variant: POSIF0RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSIF0RS` reader - POSIF0 Reset Status"]
pub type POSIF0RS_R = crate::BitReader<POSIF0RS_A>;
impl POSIF0RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POSIF0RS_A {
        match self.bits {
            false => POSIF0RS_A::VALUE1,
            true => POSIF0RS_A::VALUE2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == POSIF0RS_A::VALUE1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == POSIF0RS_A::VALUE2
    }
}
#[doc = "POSIF1 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POSIF1RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<POSIF1RS_A> for bool {
    #[inline(always)]
    fn from(variant: POSIF1RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSIF1RS` reader - POSIF1 Reset Status"]
pub type POSIF1RS_R = crate::BitReader<POSIF1RS_A>;
impl POSIF1RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POSIF1RS_A {
        match self.bits {
            false => POSIF1RS_A::VALUE1,
            true => POSIF1RS_A::VALUE2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == POSIF1RS_A::VALUE1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == POSIF1RS_A::VALUE2
    }
}
#[doc = "USIC0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC0RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<USIC0RS_A> for bool {
    #[inline(always)]
    fn from(variant: USIC0RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC0RS` reader - USIC0 Reset Status"]
pub type USIC0RS_R = crate::BitReader<USIC0RS_A>;
impl USIC0RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USIC0RS_A {
        match self.bits {
            false => USIC0RS_A::VALUE1,
            true => USIC0RS_A::VALUE2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USIC0RS_A::VALUE1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USIC0RS_A::VALUE2
    }
}
#[doc = "ERU1 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERU1RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<ERU1RS_A> for bool {
    #[inline(always)]
    fn from(variant: ERU1RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU1RS` reader - ERU1 Reset Status"]
pub type ERU1RS_R = crate::BitReader<ERU1RS_A>;
impl ERU1RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERU1RS_A {
        match self.bits {
            false => ERU1RS_A::VALUE1,
            true => ERU1RS_A::VALUE2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERU1RS_A::VALUE1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERU1RS_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - VADC Reset Status"]
    #[inline(always)]
    pub fn vadcrs(&self) -> VADCRS_R {
        VADCRS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSD Reset Status"]
    #[inline(always)]
    pub fn dsdrs(&self) -> DSDRS_R {
        DSDRS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CCU40 Reset Status"]
    #[inline(always)]
    pub fn ccu40rs(&self) -> CCU40RS_R {
        CCU40RS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCU41 Reset Status"]
    #[inline(always)]
    pub fn ccu41rs(&self) -> CCU41RS_R {
        CCU41RS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CCU42 Reset Status"]
    #[inline(always)]
    pub fn ccu42rs(&self) -> CCU42RS_R {
        CCU42RS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - CCU80 Reset Status"]
    #[inline(always)]
    pub fn ccu80rs(&self) -> CCU80RS_R {
        CCU80RS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CCU81 Reset Status"]
    #[inline(always)]
    pub fn ccu81rs(&self) -> CCU81RS_R {
        CCU81RS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - POSIF0 Reset Status"]
    #[inline(always)]
    pub fn posif0rs(&self) -> POSIF0RS_R {
        POSIF0RS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - POSIF1 Reset Status"]
    #[inline(always)]
    pub fn posif1rs(&self) -> POSIF1RS_R {
        POSIF1RS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USIC0 Reset Status"]
    #[inline(always)]
    pub fn usic0rs(&self) -> USIC0RS_R {
        USIC0RS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - ERU1 Reset Status"]
    #[inline(always)]
    pub fn eru1rs(&self) -> ERU1RS_R {
        ERU1RS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "RCU Peripheral 0 Reset Status\n\nYou can [`read`](crate::Reg::read) this register and get [`prstat0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRSTAT0_SPEC;
impl crate::RegisterSpec for PRSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prstat0::R`](R) reader structure"]
impl crate::Readable for PRSTAT0_SPEC {}
#[doc = "`reset()` method sets PRSTAT0 to value 0x0001_0f9f"]
impl crate::Resettable for PRSTAT0_SPEC {
    const RESET_VALUE: u32 = 0x0001_0f9f;
}
