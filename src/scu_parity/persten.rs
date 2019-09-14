#[doc = "Reader of register PERSTEN"]
pub type R = crate::R<u32, super::PERSTEN>;
#[doc = "Writer for register PERSTEN"]
pub type W = crate::W<u32, super::PERSTEN>;
#[doc = "Register PERSTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::PERSTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "System Reset Enable upon Parity Error Trap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSEN_A {
    #[doc = "0: Reset request disabled"]
    VALUE1,
    #[doc = "1: Reset request enabled"]
    VALUE2,
}
impl From<RSEN_A> for bool {
    #[inline(always)]
    fn from(variant: RSEN_A) -> Self {
        match variant {
            RSEN_A::VALUE1 => false,
            RSEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RSEN`"]
pub type RSEN_R = crate::R<bool, RSEN_A>;
impl RSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSEN_A {
        match self.bits {
            false => RSEN_A::VALUE1,
            true => RSEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RSEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RSEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `RSEN`"]
pub struct RSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset request disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RSEN_A::VALUE1)
    }
    #[doc = "Reset request enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RSEN_A::VALUE2)
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
    #[doc = "Bit 0 - System Reset Enable upon Parity Error Trap"]
    #[inline(always)]
    pub fn rsen(&self) -> RSEN_R {
        RSEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System Reset Enable upon Parity Error Trap"]
    #[inline(always)]
    pub fn rsen(&mut self) -> RSEN_W {
        RSEN_W { w: self }
    }
}
