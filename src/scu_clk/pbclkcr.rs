#[doc = "Reader of register PBCLKCR"]
pub type R = crate::R<u32, super::PBCLKCR>;
#[doc = "Writer for register PBCLKCR"]
pub type W = crate::W<u32, super::PBCLKCR>;
#[doc = "Register PBCLKCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PBCLKCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PB Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBDIV_A {
    #[doc = "0: fPERIPH = fCPU"]
    VALUE1,
    #[doc = "1: fPERIPH = fCPU / 2"]
    VALUE2,
}
impl From<PBDIV_A> for bool {
    #[inline(always)]
    fn from(variant: PBDIV_A) -> Self {
        match variant {
            PBDIV_A::VALUE1 => false,
            PBDIV_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PBDIV`"]
pub type PBDIV_R = crate::R<bool, PBDIV_A>;
impl PBDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBDIV_A {
        match self.bits {
            false => PBDIV_A::VALUE1,
            true => PBDIV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PBDIV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PBDIV_A::VALUE2
    }
}
#[doc = "Write proxy for field `PBDIV`"]
pub struct PBDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PBDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PBDIV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "fPERIPH = fCPU"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PBDIV_A::VALUE1)
    }
    #[doc = "fPERIPH = fCPU / 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PBDIV_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PB Clock Divider Enable"]
    #[inline(always)]
    pub fn pbdiv(&self) -> PBDIV_R {
        PBDIV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PB Clock Divider Enable"]
    #[inline(always)]
    pub fn pbdiv(&mut self) -> PBDIV_W {
        PBDIV_W { w: self }
    }
}
