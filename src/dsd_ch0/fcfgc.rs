#[doc = "Reader of register FCFGC"]
pub type R = crate::R<u32, super::FCFGC>;
#[doc = "Writer for register FCFGC"]
pub type W = crate::W<u32, super::FCFGC>;
#[doc = "Register FCFGC `reset()`'s with value 0"]
impl crate::ResetValue for super::FCFGC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CFMDF`"]
pub type CFMDF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFMDF`"]
pub struct CFMDF_W<'a> {
    w: &'a mut W,
}
impl<'a> CFMDF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "CIC Filter (Main Chain) Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFMC_A {
    #[doc = "0: CIC1"]
    VALUE1 = 0,
    #[doc = "1: CIC2"]
    VALUE2 = 1,
    #[doc = "2: CIC3"]
    VALUE3 = 2,
    #[doc = "3: CICF"]
    VALUE4 = 3,
}
impl From<CFMC_A> for u8 {
    #[inline(always)]
    fn from(variant: CFMC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CFMC`"]
pub type CFMC_R = crate::R<u8, CFMC_A>;
impl CFMC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFMC_A {
        match self.bits {
            0 => CFMC_A::VALUE1,
            1 => CFMC_A::VALUE2,
            2 => CFMC_A::VALUE3,
            3 => CFMC_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFMC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFMC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CFMC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CFMC_A::VALUE4
    }
}
#[doc = "Write proxy for field `CFMC`"]
pub struct CFMC_W<'a> {
    w: &'a mut W,
}
impl<'a> CFMC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFMC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CIC1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFMC_A::VALUE1)
    }
    #[doc = "CIC2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFMC_A::VALUE2)
    }
    #[doc = "CIC3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CFMC_A::VALUE3)
    }
    #[doc = "CICF"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CFMC_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "CIC Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFEN_A {
    #[doc = "0: CIC filter disabled and bypassed"]
    VALUE1 = 0,
    #[doc = "1: Enable CIC filter"]
    VALUE2 = 1,
}
impl From<CFEN_A> for bool {
    #[inline(always)]
    fn from(variant: CFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CFEN`"]
pub type CFEN_R = crate::R<bool, CFEN_A>;
impl CFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFEN_A {
        match self.bits {
            false => CFEN_A::VALUE1,
            true => CFEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `CFEN`"]
pub struct CFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CIC filter disabled and bypassed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFEN_A::VALUE1)
    }
    #[doc = "Enable CIC filter"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Service Request Generation Main Chain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRGM_A {
    #[doc = "0: Never, service requests disabled"]
    VALUE1 = 0,
    #[doc = "3: Always, for each new result value"]
    VALUE4 = 3,
}
impl From<SRGM_A> for u8 {
    #[inline(always)]
    fn from(variant: SRGM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRGM`"]
pub type SRGM_R = crate::R<u8, SRGM_A>;
impl SRGM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRGM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRGM_A::VALUE1),
            3 => Val(SRGM_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRGM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SRGM_A::VALUE4
    }
}
#[doc = "Write proxy for field `SRGM`"]
pub struct SRGM_W<'a> {
    w: &'a mut W,
}
impl<'a> SRGM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRGM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Never, service requests disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRGM_A::VALUE1)
    }
    #[doc = "Always, for each new result value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SRGM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `CFMSV`"]
pub type CFMSV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFMSV`"]
pub struct CFMSV_W<'a> {
    w: &'a mut W,
}
impl<'a> CFMSV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CFMDCNT`"]
pub type CFMDCNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - CIC Filter (Main Chain) Decimation Factor"]
    #[inline(always)]
    pub fn cfmdf(&self) -> CFMDF_R {
        CFMDF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - CIC Filter (Main Chain) Configuration"]
    #[inline(always)]
    pub fn cfmc(&self) -> CFMC_R {
        CFMC_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - CIC Filter Enable"]
    #[inline(always)]
    pub fn cfen(&self) -> CFEN_R {
        CFEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Service Request Generation Main Chain"]
    #[inline(always)]
    pub fn srgm(&self) -> SRGM_R {
        SRGM_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:23 - CIC Filter (Main Chain) Start Value"]
    #[inline(always)]
    pub fn cfmsv(&self) -> CFMSV_R {
        CFMSV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CIC Filter (Main Chain) Decimation Counter"]
    #[inline(always)]
    pub fn cfmdcnt(&self) -> CFMDCNT_R {
        CFMDCNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CIC Filter (Main Chain) Decimation Factor"]
    #[inline(always)]
    pub fn cfmdf(&mut self) -> CFMDF_W {
        CFMDF_W { w: self }
    }
    #[doc = "Bits 8:9 - CIC Filter (Main Chain) Configuration"]
    #[inline(always)]
    pub fn cfmc(&mut self) -> CFMC_W {
        CFMC_W { w: self }
    }
    #[doc = "Bit 10 - CIC Filter Enable"]
    #[inline(always)]
    pub fn cfen(&mut self) -> CFEN_W {
        CFEN_W { w: self }
    }
    #[doc = "Bits 14:15 - Service Request Generation Main Chain"]
    #[inline(always)]
    pub fn srgm(&mut self) -> SRGM_W {
        SRGM_W { w: self }
    }
    #[doc = "Bits 16:23 - CIC Filter (Main Chain) Start Value"]
    #[inline(always)]
    pub fn cfmsv(&mut self) -> CFMSV_W {
        CFMSV_W { w: self }
    }
}
