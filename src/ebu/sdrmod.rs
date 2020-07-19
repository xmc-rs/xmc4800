#[doc = "Reader of register SDRMOD"]
pub type R = crate::R<u32, super::SDRMOD>;
#[doc = "Writer for register SDRMOD"]
pub type W = crate::W<u32, super::SDRMOD>;
#[doc = "Register SDRMOD `reset()`'s with value 0x20"]
impl crate::ResetValue for super::SDRMOD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Reader of field `XBA`"]
pub type XBA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XBA`"]
pub struct XBA_W<'a> {
    w: &'a mut W,
}
impl<'a> XBA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `XOPM`"]
pub type XOPM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `XOPM`"]
pub struct XOPM_W<'a> {
    w: &'a mut W,
}
impl<'a> XOPM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `COLDSTART`"]
pub struct COLDSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> COLDSTART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPMODE_A {
    #[doc = "0: Only this value must be written (default after reset)"]
    VALUE1 = 0,
}
impl From<OPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OPMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPMODE`"]
pub type OPMODE_R = crate::R<u8, OPMODE_A>;
impl OPMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPMODE_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OPMODE_A::VALUE1
    }
}
#[doc = "Write proxy for field `OPMODE`"]
pub struct OPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Only this value must be written (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OPMODE_A::VALUE1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 7)) | (((value as u32) & 0x7f) << 7);
        self.w
    }
}
#[doc = "CAS latency\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CASLAT_A {
    #[doc = "2: Two clocks (default after reset)"]
    VALUE1 = 2,
    #[doc = "3: Three clocks"]
    VALUE2 = 3,
}
impl From<CASLAT_A> for u8 {
    #[inline(always)]
    fn from(variant: CASLAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CASLAT`"]
pub type CASLAT_R = crate::R<u8, CASLAT_A>;
impl CASLAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CASLAT_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(CASLAT_A::VALUE1),
            3 => Val(CASLAT_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CASLAT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CASLAT_A::VALUE2
    }
}
#[doc = "Write proxy for field `CASLAT`"]
pub struct CASLAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CASLAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CASLAT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Two clocks (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CASLAT_A::VALUE1)
    }
    #[doc = "Three clocks"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CASLAT_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Burst type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTYP_A {
    #[doc = "0: Only this value should be written (default after reset)"]
    VALUE1 = 0,
}
impl From<BTYP_A> for bool {
    #[inline(always)]
    fn from(variant: BTYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTYP`"]
pub type BTYP_R = crate::R<bool, BTYP_A>;
impl BTYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, BTYP_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(BTYP_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BTYP_A::VALUE1
    }
}
#[doc = "Write proxy for field `BTYP`"]
pub struct BTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> BTYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BTYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only this value should be written (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BTYP_A::VALUE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Burst length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BURSTL_A {
    #[doc = "0: 1 (default after reset)"]
    VALUE1 = 0,
    #[doc = "1: 2"]
    VALUE2 = 1,
    #[doc = "2: 4"]
    VALUE3 = 2,
    #[doc = "3: 8"]
    VALUE4 = 3,
    #[doc = "4: 16"]
    VALUE5 = 4,
}
impl From<BURSTL_A> for u8 {
    #[inline(always)]
    fn from(variant: BURSTL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BURSTL`"]
pub type BURSTL_R = crate::R<u8, BURSTL_A>;
impl BURSTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BURSTL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BURSTL_A::VALUE1),
            1 => Val(BURSTL_A::VALUE2),
            2 => Val(BURSTL_A::VALUE3),
            3 => Val(BURSTL_A::VALUE4),
            4 => Val(BURSTL_A::VALUE5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BURSTL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BURSTL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BURSTL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BURSTL_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == BURSTL_A::VALUE5
    }
}
#[doc = "Write proxy for field `BURSTL`"]
pub struct BURSTL_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURSTL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BURSTL_A::VALUE1)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BURSTL_A::VALUE2)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BURSTL_A::VALUE3)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BURSTL_A::VALUE4)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(BURSTL_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - Extended Operation Bank Select"]
    #[inline(always)]
    pub fn xba(&self) -> XBA_R {
        XBA_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - Extended Operation Mode"]
    #[inline(always)]
    pub fn xopm(&self) -> XOPM_R {
        XOPM_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 7:13 - Operation Mode"]
    #[inline(always)]
    pub fn opmode(&self) -> OPMODE_R {
        OPMODE_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 4:6 - CAS latency"]
    #[inline(always)]
    pub fn caslat(&self) -> CASLAT_R {
        CASLAT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - Burst type"]
    #[inline(always)]
    pub fn btyp(&self) -> BTYP_R {
        BTYP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - Burst length"]
    #[inline(always)]
    pub fn burstl(&self) -> BURSTL_R {
        BURSTL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - Extended Operation Bank Select"]
    #[inline(always)]
    pub fn xba(&mut self) -> XBA_W {
        XBA_W { w: self }
    }
    #[doc = "Bits 16:27 - Extended Operation Mode"]
    #[inline(always)]
    pub fn xopm(&mut self) -> XOPM_W {
        XOPM_W { w: self }
    }
    #[doc = "Bit 15 - SDRAM coldstart"]
    #[inline(always)]
    pub fn coldstart(&mut self) -> COLDSTART_W {
        COLDSTART_W { w: self }
    }
    #[doc = "Bits 7:13 - Operation Mode"]
    #[inline(always)]
    pub fn opmode(&mut self) -> OPMODE_W {
        OPMODE_W { w: self }
    }
    #[doc = "Bits 4:6 - CAS latency"]
    #[inline(always)]
    pub fn caslat(&mut self) -> CASLAT_W {
        CASLAT_W { w: self }
    }
    #[doc = "Bit 3 - Burst type"]
    #[inline(always)]
    pub fn btyp(&mut self) -> BTYP_W {
        BTYP_W { w: self }
    }
    #[doc = "Bits 0:2 - Burst length"]
    #[inline(always)]
    pub fn burstl(&mut self) -> BURSTL_W {
        BURSTL_W { w: self }
    }
}
