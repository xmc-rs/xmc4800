#[doc = "Reader of register HFSR"]
pub type R = crate::R<u32, super::HFSR>;
#[doc = "Writer for register HFSR"]
pub type W = crate::W<u32, super::HFSR>;
#[doc = "Register HFSR `reset()`'s with value 0"]
impl crate::ResetValue for super::HFSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "BusFault on vector table read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VECTTBL_A {
    #[doc = "0: no BusFault on vector table read"]
    VALUE1 = 0,
    #[doc = "1: BusFault on vector table read"]
    VALUE2 = 1,
}
impl From<VECTTBL_A> for bool {
    #[inline(always)]
    fn from(variant: VECTTBL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VECTTBL`"]
pub type VECTTBL_R = crate::R<bool, VECTTBL_A>;
impl VECTTBL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VECTTBL_A {
        match self.bits {
            false => VECTTBL_A::VALUE1,
            true => VECTTBL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VECTTBL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VECTTBL_A::VALUE2
    }
}
#[doc = "Write proxy for field `VECTTBL`"]
pub struct VECTTBL_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTTBL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VECTTBL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no BusFault on vector table read"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VECTTBL_A::VALUE1)
    }
    #[doc = "BusFault on vector table read"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VECTTBL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Forced HardFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCED_A {
    #[doc = "0: no forced HardFault"]
    VALUE1 = 0,
    #[doc = "1: forced HardFault."]
    VALUE2 = 1,
}
impl From<FORCED_A> for bool {
    #[inline(always)]
    fn from(variant: FORCED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FORCED`"]
pub type FORCED_R = crate::R<bool, FORCED_A>;
impl FORCED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCED_A {
        match self.bits {
            false => FORCED_A::VALUE1,
            true => FORCED_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FORCED_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FORCED_A::VALUE2
    }
}
#[doc = "Write proxy for field `FORCED`"]
pub struct FORCED_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no forced HardFault"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FORCED_A::VALUE1)
    }
    #[doc = "forced HardFault."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FORCED_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `DEBUGEVT`"]
pub type DEBUGEVT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEBUGEVT`"]
pub struct DEBUGEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUGEVT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - BusFault on vector table read"]
    #[inline(always)]
    pub fn vecttbl(&self) -> VECTTBL_R {
        VECTTBL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Forced HardFault"]
    #[inline(always)]
    pub fn forced(&self) -> FORCED_R {
        FORCED_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Reserved for Debug use"]
    #[inline(always)]
    pub fn debugevt(&self) -> DEBUGEVT_R {
        DEBUGEVT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - BusFault on vector table read"]
    #[inline(always)]
    pub fn vecttbl(&mut self) -> VECTTBL_W {
        VECTTBL_W { w: self }
    }
    #[doc = "Bit 30 - Forced HardFault"]
    #[inline(always)]
    pub fn forced(&mut self) -> FORCED_W {
        FORCED_W { w: self }
    }
    #[doc = "Bit 31 - Reserved for Debug use"]
    #[inline(always)]
    pub fn debugevt(&mut self) -> DEBUGEVT_W {
        DEBUGEVT_W { w: self }
    }
}
