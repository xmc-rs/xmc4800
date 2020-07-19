#[doc = "Reader of register FCFGA"]
pub type R = crate::R<u32, super::FCFGA>;
#[doc = "Writer for register FCFGA"]
pub type W = crate::W<u32, super::FCFGA>;
#[doc = "Register FCFGA `reset()`'s with value 0"]
impl crate::ResetValue for super::FCFGA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CFADF`"]
pub type CFADF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFADF`"]
pub struct CFADF_W<'a> {
    w: &'a mut W,
}
impl<'a> CFADF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "CIC Filter (Auxiliary) Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFAC_A {
    #[doc = "0: CIC1"]
    VALUE1 = 0,
    #[doc = "1: CIC2"]
    VALUE2 = 1,
    #[doc = "2: CIC3"]
    VALUE3 = 2,
    #[doc = "3: CICF"]
    VALUE4 = 3,
}
impl From<CFAC_A> for u8 {
    #[inline(always)]
    fn from(variant: CFAC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CFAC`"]
pub type CFAC_R = crate::R<u8, CFAC_A>;
impl CFAC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFAC_A {
        match self.bits {
            0 => CFAC_A::VALUE1,
            1 => CFAC_A::VALUE2,
            2 => CFAC_A::VALUE3,
            3 => CFAC_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFAC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFAC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CFAC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CFAC_A::VALUE4
    }
}
#[doc = "Write proxy for field `CFAC`"]
pub struct CFAC_W<'a> {
    w: &'a mut W,
}
impl<'a> CFAC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFAC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CIC1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFAC_A::VALUE1)
    }
    #[doc = "CIC2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFAC_A::VALUE2)
    }
    #[doc = "CIC3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CFAC_A::VALUE3)
    }
    #[doc = "CICF"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CFAC_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Service Request Generation Auxiliary Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRGA_A {
    #[doc = "0: Never, service requests disabled"]
    VALUE1 = 0,
    #[doc = "1: Auxiliary filter: As selected by bitfields ESEL and EGT (if integrator enabled)"]
    VALUE2 = 1,
    #[doc = "2: Alternate source: Capturing of a sign delay value to register CGSYNCx (x = 0 - 3)"]
    VALUE3 = 2,
}
impl From<SRGA_A> for u8 {
    #[inline(always)]
    fn from(variant: SRGA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRGA`"]
pub type SRGA_R = crate::R<u8, SRGA_A>;
impl SRGA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRGA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRGA_A::VALUE1),
            1 => Val(SRGA_A::VALUE2),
            2 => Val(SRGA_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRGA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRGA_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SRGA_A::VALUE3
    }
}
#[doc = "Write proxy for field `SRGA`"]
pub struct SRGA_W<'a> {
    w: &'a mut W,
}
impl<'a> SRGA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRGA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Never, service requests disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRGA_A::VALUE1)
    }
    #[doc = "Auxiliary filter: As selected by bitfields ESEL and EGT (if integrator enabled)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRGA_A::VALUE2)
    }
    #[doc = "Alternate source: Capturing of a sign delay value to register CGSYNCx (x = 0 - 3)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SRGA_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Event Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ESEL_A {
    #[doc = "0: Always, for each new result value"]
    VALUE1 = 0,
    #[doc = "1: If result is inside the boundary band"]
    VALUE2 = 1,
    #[doc = "2: If result is outside the boundary band"]
    VALUE3 = 2,
}
impl From<ESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ESEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ESEL`"]
pub type ESEL_R = crate::R<u8, ESEL_A>;
impl ESEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ESEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ESEL_A::VALUE1),
            1 => Val(ESEL_A::VALUE2),
            2 => Val(ESEL_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ESEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ESEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ESEL_A::VALUE3
    }
}
#[doc = "Write proxy for field `ESEL`"]
pub struct ESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ESEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Always, for each new result value"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ESEL_A::VALUE1)
    }
    #[doc = "If result is inside the boundary band"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ESEL_A::VALUE2)
    }
    #[doc = "If result is outside the boundary band"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ESEL_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Event Gating\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EGT_A {
    #[doc = "0: Separate: generate events according to ESEL"]
    VALUE1 = 0,
    #[doc = "1: Coupled: generate events only when the integrator is enabled and after the discard phase defined by bitfield NVALDISWhile the integrator is bypassed, event gating is disabled, i.e. service requests are generated according to bitfield ESEL. The event gating suppresses service requests, result values are still stored in register RESAx."]
    VALUE2 = 1,
}
impl From<EGT_A> for bool {
    #[inline(always)]
    fn from(variant: EGT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EGT`"]
pub type EGT_R = crate::R<bool, EGT_A>;
impl EGT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EGT_A {
        match self.bits {
            false => EGT_A::VALUE1,
            true => EGT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EGT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EGT_A::VALUE2
    }
}
#[doc = "Write proxy for field `EGT`"]
pub struct EGT_W<'a> {
    w: &'a mut W,
}
impl<'a> EGT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EGT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Separate: generate events according to ESEL"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EGT_A::VALUE1)
    }
    #[doc = "Coupled: generate events only when the integrator is enabled and after the discard phase defined by bitfield NVALDISWhile the integrator is bypassed, event gating is disabled, i.e. service requests are generated according to bitfield ESEL. The event gating suppresses service requests, result values are still stored in register RESAx."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EGT_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `CFADCNT`"]
pub type CFADCNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - CIC Filter (Auxiliary) Decimation Factor"]
    #[inline(always)]
    pub fn cfadf(&self) -> CFADF_R {
        CFADF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - CIC Filter (Auxiliary) Configuration"]
    #[inline(always)]
    pub fn cfac(&self) -> CFAC_R {
        CFAC_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Service Request Generation Auxiliary Filter"]
    #[inline(always)]
    pub fn srga(&self) -> SRGA_R {
        SRGA_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Event Select"]
    #[inline(always)]
    pub fn esel(&self) -> ESEL_R {
        ESEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Event Gating"]
    #[inline(always)]
    pub fn egt(&self) -> EGT_R {
        EGT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - CIC Filter (Auxiliary) Decimation Counter"]
    #[inline(always)]
    pub fn cfadcnt(&self) -> CFADCNT_R {
        CFADCNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CIC Filter (Auxiliary) Decimation Factor"]
    #[inline(always)]
    pub fn cfadf(&mut self) -> CFADF_W {
        CFADF_W { w: self }
    }
    #[doc = "Bits 8:9 - CIC Filter (Auxiliary) Configuration"]
    #[inline(always)]
    pub fn cfac(&mut self) -> CFAC_W {
        CFAC_W { w: self }
    }
    #[doc = "Bits 10:11 - Service Request Generation Auxiliary Filter"]
    #[inline(always)]
    pub fn srga(&mut self) -> SRGA_W {
        SRGA_W { w: self }
    }
    #[doc = "Bits 12:13 - Event Select"]
    #[inline(always)]
    pub fn esel(&mut self) -> ESEL_W {
        ESEL_W { w: self }
    }
    #[doc = "Bit 14 - Event Gating"]
    #[inline(always)]
    pub fn egt(&mut self) -> EGT_W {
        EGT_W { w: self }
    }
}
