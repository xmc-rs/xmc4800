#[doc = "Reader of register GLOBCFG"]
pub type R = crate::R<u32, super::GLOBCFG>;
#[doc = "Writer for register GLOBCFG"]
pub type W = crate::W<u32, super::GLOBCFG>;
#[doc = "Register GLOBCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::GLOBCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Modulator Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCSEL_A {
    #[doc = "0: Internal clock off, no source selected"]
    VALUE1,
    #[doc = "1: fDSD"]
    VALUE2,
}
impl From<MCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MCSEL_A) -> Self {
        match variant {
            MCSEL_A::VALUE1 => 0,
            MCSEL_A::VALUE2 => 1,
        }
    }
}
#[doc = "Reader of field `MCSEL`"]
pub type MCSEL_R = crate::R<u8, MCSEL_A>;
impl MCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MCSEL_A::VALUE1),
            1 => Val(MCSEL_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MCSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCSEL_A::VALUE2
    }
}
#[doc = "Write proxy for field `MCSEL`"]
pub struct MCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal clock off, no source selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCSEL_A::VALUE1)
    }
    #[doc = "fDSD"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCSEL_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Modulator Clock Select"]
    #[inline(always)]
    pub fn mcsel(&self) -> MCSEL_R {
        MCSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Modulator Clock Select"]
    #[inline(always)]
    pub fn mcsel(&mut self) -> MCSEL_W {
        MCSEL_W { w: self }
    }
}
