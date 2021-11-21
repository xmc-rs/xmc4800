#[doc = "Register `ADDRSEL0` reader"]
pub struct R(crate::R<ADDRSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDRSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDRSEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDRSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDRSEL0` writer"]
pub struct W(crate::W<ADDRSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDRSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ADDRSEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDRSEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Memory Region Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGENAB_A {
    #[doc = "0: Memory region is disabled (default after reset)."]
    VALUE1 = 0,
    #[doc = "1: Memory region is enabled."]
    VALUE2 = 1,
}
impl From<REGENAB_A> for bool {
    #[inline(always)]
    fn from(variant: REGENAB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGENAB` reader - Memory Region Enable"]
pub struct REGENAB_R(crate::FieldReader<bool, REGENAB_A>);
impl REGENAB_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGENAB_R(crate::FieldReader::new(bits))
    }
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
        **self == REGENAB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == REGENAB_A::VALUE2
    }
}
impl core::ops::Deref for REGENAB_R {
    type Target = crate::FieldReader<bool, REGENAB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGENAB` writer - Memory Region Enable"]
pub struct REGENAB_W<'a> {
    w: &'a mut W,
}
impl<'a> REGENAB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGENAB_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Alternate Region Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALTENAB_A {
    #[doc = "0: Memory region is disabled (default after reset)."]
    VALUE1 = 0,
    #[doc = "1: Memory region is enabled."]
    VALUE2 = 1,
}
impl From<ALTENAB_A> for bool {
    #[inline(always)]
    fn from(variant: ALTENAB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALTENAB` reader - Alternate Region Enable"]
pub struct ALTENAB_R(crate::FieldReader<bool, ALTENAB_A>);
impl ALTENAB_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALTENAB_R(crate::FieldReader::new(bits))
    }
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
        **self == ALTENAB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ALTENAB_A::VALUE2
    }
}
impl core::ops::Deref for ALTENAB_R {
    type Target = crate::FieldReader<bool, ALTENAB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALTENAB` writer - Alternate Region Enable"]
pub struct ALTENAB_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTENAB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALTENAB_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Memory Region Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPROT_A {
    #[doc = "0: Region is enabled for write accesses"]
    VALUE1 = 0,
    #[doc = "1: Region is write protected."]
    VALUE2 = 1,
}
impl From<WPROT_A> for bool {
    #[inline(always)]
    fn from(variant: WPROT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPROT` reader - Memory Region Write Protect"]
pub struct WPROT_R(crate::FieldReader<bool, WPROT_A>);
impl WPROT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WPROT_R(crate::FieldReader::new(bits))
    }
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
        **self == WPROT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WPROT_A::VALUE2
    }
}
impl core::ops::Deref for WPROT_R {
    type Target = crate::FieldReader<bool, WPROT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPROT` writer - Memory Region Write Protect"]
pub struct WPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> WPROT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPROT_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBU Address Select Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addrsel0](index.html) module"]
pub struct ADDRSEL0_SPEC;
impl crate::RegisterSpec for ADDRSEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addrsel0::R](R) reader structure"]
impl crate::Readable for ADDRSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addrsel0::W](W) writer structure"]
impl crate::Writable for ADDRSEL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDRSEL0 to value 0"]
impl crate::Resettable for ADDRSEL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
