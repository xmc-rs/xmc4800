#[doc = "Reader of register OCS"]
pub type R = crate::R<u32, super::OCS>;
#[doc = "Writer for register OCS"]
pub type W = crate::W<u32, super::OCS>;
#[doc = "Register OCS `reset()`'s with value 0"]
impl crate::ResetValue for super::OCS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "OCDS Suspend Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUS_A {
    #[doc = "0: Will not suspend"]
    VALUE1,
    #[doc = "1: Hard suspend: Clock is switched off immediately."]
    VALUE2,
    #[doc = "2: Soft suspend channel 0"]
    VALUE3,
    #[doc = "3: Soft suspend channel 1"]
    VALUE4,
    #[doc = "5: Soft suspend channel 3"]
    VALUE5,
}
impl From<SUS_A> for u8 {
    #[inline(always)]
    fn from(variant: SUS_A) -> Self {
        match variant {
            SUS_A::VALUE1 => 0,
            SUS_A::VALUE2 => 1,
            SUS_A::VALUE3 => 2,
            SUS_A::VALUE4 => 3,
            SUS_A::VALUE5 => 5,
        }
    }
}
#[doc = "Reader of field `SUS`"]
pub type SUS_R = crate::R<u8, SUS_A>;
impl SUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SUS_A::VALUE1),
            1 => Val(SUS_A::VALUE2),
            2 => Val(SUS_A::VALUE3),
            3 => Val(SUS_A::VALUE4),
            5 => Val(SUS_A::VALUE5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SUS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SUS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SUS_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == SUS_A::VALUE5
    }
}
#[doc = "Write proxy for field `SUS`"]
pub struct SUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Will not suspend"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUS_A::VALUE1)
    }
    #[doc = "Hard suspend: Clock is switched off immediately."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUS_A::VALUE2)
    }
    #[doc = "Soft suspend channel 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SUS_A::VALUE3)
    }
    #[doc = "Soft suspend channel 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SUS_A::VALUE4)
    }
    #[doc = "Soft suspend channel 3"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(SUS_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Write proxy for field `SUS_P`"]
pub struct SUS_P_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS_P_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Suspend State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSSTA_A {
    #[doc = "0: Module is not (yet) suspended"]
    VALUE1,
    #[doc = "1: Module is suspended"]
    VALUE2,
}
impl From<SUSSTA_A> for bool {
    #[inline(always)]
    fn from(variant: SUSSTA_A) -> Self {
        match variant {
            SUSSTA_A::VALUE1 => false,
            SUSSTA_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SUSSTA`"]
pub type SUSSTA_R = crate::R<bool, SUSSTA_A>;
impl SUSSTA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSSTA_A {
        match self.bits {
            false => SUSSTA_A::VALUE1,
            true => SUSSTA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SUSSTA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SUSSTA_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 24:27 - OCDS Suspend Control"]
    #[inline(always)]
    pub fn sus(&self) -> SUS_R {
        SUS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Suspend State"]
    #[inline(always)]
    pub fn sussta(&self) -> SUSSTA_R {
        SUSSTA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:27 - OCDS Suspend Control"]
    #[inline(always)]
    pub fn sus(&mut self) -> SUS_W {
        SUS_W { w: self }
    }
    #[doc = "Bit 28 - SUS Write Protection"]
    #[inline(always)]
    pub fn sus_p(&mut self) -> SUS_P_W {
        SUS_P_W { w: self }
    }
}
