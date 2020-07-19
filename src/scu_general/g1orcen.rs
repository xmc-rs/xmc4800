#[doc = "Reader of register G1ORCEN"]
pub type R = crate::R<u32, super::G1ORCEN>;
#[doc = "Writer for register G1ORCEN"]
pub type W = crate::W<u32, super::G1ORCEN>;
#[doc = "Register G1ORCEN `reset()`'s with value 0"]
impl crate::ResetValue for super::G1ORCEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable Out of Range Comparator, Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENORC6_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<ENORC6_A> for bool {
    #[inline(always)]
    fn from(variant: ENORC6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENORC6`"]
pub type ENORC6_R = crate::R<bool, ENORC6_A>;
impl ENORC6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENORC6_A {
        match self.bits {
            false => ENORC6_A::VALUE1,
            true => ENORC6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENORC6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENORC6_A::VALUE2
    }
}
#[doc = "Write proxy for field `ENORC6`"]
pub struct ENORC6_W<'a> {
    w: &'a mut W,
}
impl<'a> ENORC6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENORC6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENORC6_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENORC6_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Enable Out of Range Comparator, Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENORC7_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<ENORC7_A> for bool {
    #[inline(always)]
    fn from(variant: ENORC7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENORC7`"]
pub type ENORC7_R = crate::R<bool, ENORC7_A>;
impl ENORC7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENORC7_A {
        match self.bits {
            false => ENORC7_A::VALUE1,
            true => ENORC7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENORC7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENORC7_A::VALUE2
    }
}
#[doc = "Write proxy for field `ENORC7`"]
pub struct ENORC7_W<'a> {
    w: &'a mut W,
}
impl<'a> ENORC7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENORC7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENORC7_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENORC7_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - Enable Out of Range Comparator, Channel 6"]
    #[inline(always)]
    pub fn enorc6(&self) -> ENORC6_R {
        ENORC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable Out of Range Comparator, Channel 7"]
    #[inline(always)]
    pub fn enorc7(&self) -> ENORC7_R {
        ENORC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Enable Out of Range Comparator, Channel 6"]
    #[inline(always)]
    pub fn enorc6(&mut self) -> ENORC6_W {
        ENORC6_W { w: self }
    }
    #[doc = "Bit 7 - Enable Out of Range Comparator, Channel 7"]
    #[inline(always)]
    pub fn enorc7(&mut self) -> ENORC7_W {
        ENORC7_W { w: self }
    }
}
