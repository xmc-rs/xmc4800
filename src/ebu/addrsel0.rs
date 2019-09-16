#[doc = "Reader of register ADDRSEL0"]
pub type R = crate::R<u32, super::ADDRSEL0>;
#[doc = "Writer for register ADDRSEL0"]
pub type W = crate::W<u32, super::ADDRSEL0>;
#[doc = "Register ADDRSEL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDRSEL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Memory Region Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGENAB_A {
    #[doc = "0: Memory region is disabled (default after reset)."]
    VALUE1,
    #[doc = "1: Memory region is enabled."]
    VALUE2,
}
impl From<REGENAB_A> for bool {
    #[inline(always)]
    fn from(variant: REGENAB_A) -> Self {
        match variant {
            REGENAB_A::VALUE1 => false,
            REGENAB_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `REGENAB`"]
pub type REGENAB_R = crate::R<bool, REGENAB_A>;
impl REGENAB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGENAB_A {
        match self.bits {
            false => REGENAB_A::VALUE1,
            true => REGENAB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REGENAB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REGENAB_A::VALUE2
    }
}
#[doc = "Write proxy for field `REGENAB`"]
pub struct REGENAB_W<'a> {
    w: &'a mut W,
}
impl<'a> REGENAB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGENAB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Memory region is disabled (default after reset)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REGENAB_A::VALUE1)
    }
    #[doc = "Memory region is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REGENAB_A::VALUE2)
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
#[doc = "Alternate Region Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALTENAB_A {
    #[doc = "0: Memory region is disabled (default after reset)."]
    VALUE1,
    #[doc = "1: Memory region is enabled."]
    VALUE2,
}
impl From<ALTENAB_A> for bool {
    #[inline(always)]
    fn from(variant: ALTENAB_A) -> Self {
        match variant {
            ALTENAB_A::VALUE1 => false,
            ALTENAB_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ALTENAB`"]
pub type ALTENAB_R = crate::R<bool, ALTENAB_A>;
impl ALTENAB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALTENAB_A {
        match self.bits {
            false => ALTENAB_A::VALUE1,
            true => ALTENAB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALTENAB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALTENAB_A::VALUE2
    }
}
#[doc = "Write proxy for field `ALTENAB`"]
pub struct ALTENAB_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTENAB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALTENAB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Memory region is disabled (default after reset)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALTENAB_A::VALUE1)
    }
    #[doc = "Memory region is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALTENAB_A::VALUE2)
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
#[doc = "Memory Region Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPROT_A {
    #[doc = "0: Region is enabled for write accesses"]
    VALUE1,
    #[doc = "1: Region is write protected."]
    VALUE2,
}
impl From<WPROT_A> for bool {
    #[inline(always)]
    fn from(variant: WPROT_A) -> Self {
        match variant {
            WPROT_A::VALUE1 => false,
            WPROT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `WPROT`"]
pub type WPROT_R = crate::R<bool, WPROT_A>;
impl WPROT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPROT_A {
        match self.bits {
            false => WPROT_A::VALUE1,
            true => WPROT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WPROT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WPROT_A::VALUE2
    }
}
#[doc = "Write proxy for field `WPROT`"]
pub struct WPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> WPROT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPROT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Region is enabled for write accesses"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WPROT_A::VALUE1)
    }
    #[doc = "Region is write protected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WPROT_A::VALUE2)
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
impl R {
    #[doc = "Bit 0 - Memory Region Enable"]
    #[inline(always)]
    pub fn regenab(&self) -> REGENAB_R {
        REGENAB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Alternate Region Enable"]
    #[inline(always)]
    pub fn altenab(&self) -> ALTENAB_R {
        ALTENAB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Memory Region Write Protect"]
    #[inline(always)]
    pub fn wprot(&self) -> WPROT_R {
        WPROT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Memory Region Enable"]
    #[inline(always)]
    pub fn regenab(&mut self) -> REGENAB_W {
        REGENAB_W { w: self }
    }
    #[doc = "Bit 1 - Alternate Region Enable"]
    #[inline(always)]
    pub fn altenab(&mut self) -> ALTENAB_W {
        ALTENAB_W { w: self }
    }
    #[doc = "Bit 2 - Memory Region Write Protect"]
    #[inline(always)]
    pub fn wprot(&mut self) -> WPROT_W {
        WPROT_W { w: self }
    }
}
