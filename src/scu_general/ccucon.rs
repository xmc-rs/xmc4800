#[doc = "Reader of register CCUCON"]
pub type R = crate::R<u32, super::CCUCON>;
#[doc = "Writer for register CCUCON"]
pub type W = crate::W<u32, super::CCUCON>;
#[doc = "Register CCUCON `reset()`'s with value 0"]
impl crate::ResetValue for super::CCUCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Global Start Control CCU40\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GSC40_A {
    #[doc = "0: Disable"]
    VALUE1,
    #[doc = "1: Enable"]
    VALUE2,
}
impl From<GSC40_A> for bool {
    #[inline(always)]
    fn from(variant: GSC40_A) -> Self {
        match variant {
            GSC40_A::VALUE1 => false,
            GSC40_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `GSC40`"]
pub type GSC40_R = crate::R<bool, GSC40_A>;
impl GSC40_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GSC40_A {
        match self.bits {
            false => GSC40_A::VALUE1,
            true => GSC40_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GSC40_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GSC40_A::VALUE2
    }
}
#[doc = "Write proxy for field `GSC40`"]
pub struct GSC40_W<'a> {
    w: &'a mut W,
}
impl<'a> GSC40_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GSC40_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GSC40_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GSC40_A::VALUE2)
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
#[doc = "Global Start Control CCU41\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GSC41_A {
    #[doc = "0: Disable"]
    VALUE1,
    #[doc = "1: Enable"]
    VALUE2,
}
impl From<GSC41_A> for bool {
    #[inline(always)]
    fn from(variant: GSC41_A) -> Self {
        match variant {
            GSC41_A::VALUE1 => false,
            GSC41_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `GSC41`"]
pub type GSC41_R = crate::R<bool, GSC41_A>;
impl GSC41_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GSC41_A {
        match self.bits {
            false => GSC41_A::VALUE1,
            true => GSC41_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GSC41_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GSC41_A::VALUE2
    }
}
#[doc = "Write proxy for field `GSC41`"]
pub struct GSC41_W<'a> {
    w: &'a mut W,
}
impl<'a> GSC41_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GSC41_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GSC41_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GSC41_A::VALUE2)
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
#[doc = "Global Start Control CCU42\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GSC42_A {
    #[doc = "0: Disable"]
    VALUE1,
    #[doc = "1: Enable"]
    VALUE2,
}
impl From<GSC42_A> for bool {
    #[inline(always)]
    fn from(variant: GSC42_A) -> Self {
        match variant {
            GSC42_A::VALUE1 => false,
            GSC42_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `GSC42`"]
pub type GSC42_R = crate::R<bool, GSC42_A>;
impl GSC42_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GSC42_A {
        match self.bits {
            false => GSC42_A::VALUE1,
            true => GSC42_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GSC42_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GSC42_A::VALUE2
    }
}
#[doc = "Write proxy for field `GSC42`"]
pub struct GSC42_W<'a> {
    w: &'a mut W,
}
impl<'a> GSC42_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GSC42_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GSC42_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GSC42_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Global Start Control CCU43\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GSC43_A {
    #[doc = "0: Disable"]
    VALUE1,
    #[doc = "1: Enable"]
    VALUE2,
}
impl From<GSC43_A> for bool {
    #[inline(always)]
    fn from(variant: GSC43_A) -> Self {
        match variant {
            GSC43_A::VALUE1 => false,
            GSC43_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `GSC43`"]
pub type GSC43_R = crate::R<bool, GSC43_A>;
impl GSC43_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GSC43_A {
        match self.bits {
            false => GSC43_A::VALUE1,
            true => GSC43_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GSC43_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GSC43_A::VALUE2
    }
}
#[doc = "Write proxy for field `GSC43`"]
pub struct GSC43_W<'a> {
    w: &'a mut W,
}
impl<'a> GSC43_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GSC43_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GSC43_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GSC43_A::VALUE2)
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
#[doc = "Global Start Control CCU80\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GSC80_A {
    #[doc = "0: Disable"]
    VALUE1,
    #[doc = "1: Enable"]
    VALUE2,
}
impl From<GSC80_A> for bool {
    #[inline(always)]
    fn from(variant: GSC80_A) -> Self {
        match variant {
            GSC80_A::VALUE1 => false,
            GSC80_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `GSC80`"]
pub type GSC80_R = crate::R<bool, GSC80_A>;
impl GSC80_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GSC80_A {
        match self.bits {
            false => GSC80_A::VALUE1,
            true => GSC80_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GSC80_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GSC80_A::VALUE2
    }
}
#[doc = "Write proxy for field `GSC80`"]
pub struct GSC80_W<'a> {
    w: &'a mut W,
}
impl<'a> GSC80_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GSC80_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GSC80_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GSC80_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Global Start Control CCU81\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GSC81_A {
    #[doc = "0: Disable"]
    VALUE1,
    #[doc = "1: Enable"]
    VALUE2,
}
impl From<GSC81_A> for bool {
    #[inline(always)]
    fn from(variant: GSC81_A) -> Self {
        match variant {
            GSC81_A::VALUE1 => false,
            GSC81_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `GSC81`"]
pub type GSC81_R = crate::R<bool, GSC81_A>;
impl GSC81_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GSC81_A {
        match self.bits {
            false => GSC81_A::VALUE1,
            true => GSC81_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GSC81_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GSC81_A::VALUE2
    }
}
#[doc = "Write proxy for field `GSC81`"]
pub struct GSC81_W<'a> {
    w: &'a mut W,
}
impl<'a> GSC81_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GSC81_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GSC81_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GSC81_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Global Start Control CCU40"]
    #[inline(always)]
    pub fn gsc40(&self) -> GSC40_R {
        GSC40_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Global Start Control CCU41"]
    #[inline(always)]
    pub fn gsc41(&self) -> GSC41_R {
        GSC41_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Global Start Control CCU42"]
    #[inline(always)]
    pub fn gsc42(&self) -> GSC42_R {
        GSC42_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Global Start Control CCU43"]
    #[inline(always)]
    pub fn gsc43(&self) -> GSC43_R {
        GSC43_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Global Start Control CCU80"]
    #[inline(always)]
    pub fn gsc80(&self) -> GSC80_R {
        GSC80_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Global Start Control CCU81"]
    #[inline(always)]
    pub fn gsc81(&self) -> GSC81_R {
        GSC81_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Start Control CCU40"]
    #[inline(always)]
    pub fn gsc40(&mut self) -> GSC40_W {
        GSC40_W { w: self }
    }
    #[doc = "Bit 1 - Global Start Control CCU41"]
    #[inline(always)]
    pub fn gsc41(&mut self) -> GSC41_W {
        GSC41_W { w: self }
    }
    #[doc = "Bit 2 - Global Start Control CCU42"]
    #[inline(always)]
    pub fn gsc42(&mut self) -> GSC42_W {
        GSC42_W { w: self }
    }
    #[doc = "Bit 3 - Global Start Control CCU43"]
    #[inline(always)]
    pub fn gsc43(&mut self) -> GSC43_W {
        GSC43_W { w: self }
    }
    #[doc = "Bit 8 - Global Start Control CCU80"]
    #[inline(always)]
    pub fn gsc80(&mut self) -> GSC80_W {
        GSC80_W { w: self }
    }
    #[doc = "Bit 9 - Global Start Control CCU81"]
    #[inline(always)]
    pub fn gsc81(&mut self) -> GSC81_W {
        GSC81_W { w: self }
    }
}
