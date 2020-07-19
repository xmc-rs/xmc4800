#[doc = "Reader of register USERCON"]
pub type R = crate::R<u32, super::USERCON>;
#[doc = "Writer for register USERCON"]
pub type W = crate::W<u32, super::USERCON>;
#[doc = "Register USERCON `reset()`'s with value 0"]
impl crate::ResetValue for super::USERCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIP`"]
pub type DIP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIP`"]
pub struct DIP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIP_W<'a> {
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
#[doc = "Address Pins to GPIO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ADDIO_A {
    #[doc = "0: Address Bit is required for addressing memory"]
    VALUE1 = 0,
    #[doc = "1: Address Bit is available for GPIO function"]
    VALUE2 = 1,
}
impl From<ADDIO_A> for u16 {
    #[inline(always)]
    fn from(variant: ADDIO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADDIO`"]
pub type ADDIO_R = crate::R<u16, ADDIO_A>;
impl ADDIO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, ADDIO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADDIO_A::VALUE1),
            1 => Val(ADDIO_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ADDIO_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ADDIO_A::VALUE2
    }
}
#[doc = "Write proxy for field `ADDIO`"]
pub struct ADDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDIO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Address Bit is required for addressing memory"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ADDIO_A::VALUE1)
    }
    #[doc = "Address Bit is available for GPIO function"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ADDIO_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
#[doc = "ADV Pin to GPIO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADVIO_A {
    #[doc = "0: ADV pin is required for controlling memory"]
    VALUE1 = 0,
    #[doc = "1: ADV pin is available for GPIO function"]
    VALUE2 = 1,
}
impl From<ADVIO_A> for bool {
    #[inline(always)]
    fn from(variant: ADVIO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADVIO`"]
pub type ADVIO_R = crate::R<bool, ADVIO_A>;
impl ADVIO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADVIO_A {
        match self.bits {
            false => ADVIO_A::VALUE1,
            true => ADVIO_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ADVIO_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ADVIO_A::VALUE2
    }
}
#[doc = "Write proxy for field `ADVIO`"]
pub struct ADVIO_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADVIO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADV pin is required for controlling memory"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ADVIO_A::VALUE1)
    }
    #[doc = "ADV pin is available for GPIO function"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ADVIO_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Disable Internal Pipelining"]
    #[inline(always)]
    pub fn dip(&self) -> DIP_R {
        DIP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 16:24 - Address Pins to GPIO Mode"]
    #[inline(always)]
    pub fn addio(&self) -> ADDIO_R {
        ADDIO_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 25 - ADV Pin to GPIO Mode"]
    #[inline(always)]
    pub fn advio(&self) -> ADVIO_R {
        ADVIO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable Internal Pipelining"]
    #[inline(always)]
    pub fn dip(&mut self) -> DIP_W {
        DIP_W { w: self }
    }
    #[doc = "Bits 16:24 - Address Pins to GPIO Mode"]
    #[inline(always)]
    pub fn addio(&mut self) -> ADDIO_W {
        ADDIO_W { w: self }
    }
    #[doc = "Bit 25 - ADV Pin to GPIO Mode"]
    #[inline(always)]
    pub fn advio(&mut self) -> ADVIO_W {
        ADVIO_W { w: self }
    }
}
