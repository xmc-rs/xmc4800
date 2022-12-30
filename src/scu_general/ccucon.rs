#[doc = "Register `CCUCON` reader"]
pub struct R(crate::R<CCUCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCUCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCUCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCUCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCUCON` writer"]
pub struct W(crate::W<CCUCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCUCON_SPEC>;
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
impl From<crate::W<CCUCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCUCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GSC40` reader - Global Start Control CCU40"]
pub type GSC40_R = crate::BitReader<GSC40_A>;
#[doc = "Global Start Control CCU40\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSC40_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<GSC40_A> for bool {
    #[inline(always)]
    fn from(variant: GSC40_A) -> Self {
        variant as u8 != 0
    }
}
impl GSC40_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `GSC40` writer - Global Start Control CCU40"]
pub type GSC40_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCUCON_SPEC, GSC40_A, O>;
impl<'a, const O: u8> GSC40_W<'a, O> {
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
}
#[doc = "Field `GSC41` reader - Global Start Control CCU41"]
pub type GSC41_R = crate::BitReader<GSC41_A>;
#[doc = "Global Start Control CCU41\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSC41_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<GSC41_A> for bool {
    #[inline(always)]
    fn from(variant: GSC41_A) -> Self {
        variant as u8 != 0
    }
}
impl GSC41_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `GSC41` writer - Global Start Control CCU41"]
pub type GSC41_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCUCON_SPEC, GSC41_A, O>;
impl<'a, const O: u8> GSC41_W<'a, O> {
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
}
#[doc = "Field `GSC42` reader - Global Start Control CCU42"]
pub type GSC42_R = crate::BitReader<GSC42_A>;
#[doc = "Global Start Control CCU42\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSC42_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<GSC42_A> for bool {
    #[inline(always)]
    fn from(variant: GSC42_A) -> Self {
        variant as u8 != 0
    }
}
impl GSC42_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `GSC42` writer - Global Start Control CCU42"]
pub type GSC42_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCUCON_SPEC, GSC42_A, O>;
impl<'a, const O: u8> GSC42_W<'a, O> {
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
}
#[doc = "Field `GSC43` reader - Global Start Control CCU43"]
pub type GSC43_R = crate::BitReader<GSC43_A>;
#[doc = "Global Start Control CCU43\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSC43_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<GSC43_A> for bool {
    #[inline(always)]
    fn from(variant: GSC43_A) -> Self {
        variant as u8 != 0
    }
}
impl GSC43_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `GSC43` writer - Global Start Control CCU43"]
pub type GSC43_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCUCON_SPEC, GSC43_A, O>;
impl<'a, const O: u8> GSC43_W<'a, O> {
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
}
#[doc = "Field `GSC80` reader - Global Start Control CCU80"]
pub type GSC80_R = crate::BitReader<GSC80_A>;
#[doc = "Global Start Control CCU80\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSC80_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<GSC80_A> for bool {
    #[inline(always)]
    fn from(variant: GSC80_A) -> Self {
        variant as u8 != 0
    }
}
impl GSC80_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `GSC80` writer - Global Start Control CCU80"]
pub type GSC80_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCUCON_SPEC, GSC80_A, O>;
impl<'a, const O: u8> GSC80_W<'a, O> {
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
}
#[doc = "Field `GSC81` reader - Global Start Control CCU81"]
pub type GSC81_R = crate::BitReader<GSC81_A>;
#[doc = "Global Start Control CCU81\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSC81_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<GSC81_A> for bool {
    #[inline(always)]
    fn from(variant: GSC81_A) -> Self {
        variant as u8 != 0
    }
}
impl GSC81_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `GSC81` writer - Global Start Control CCU81"]
pub type GSC81_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCUCON_SPEC, GSC81_A, O>;
impl<'a, const O: u8> GSC81_W<'a, O> {
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
}
impl R {
    #[doc = "Bit 0 - Global Start Control CCU40"]
    #[inline(always)]
    pub fn gsc40(&self) -> GSC40_R {
        GSC40_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Global Start Control CCU41"]
    #[inline(always)]
    pub fn gsc41(&self) -> GSC41_R {
        GSC41_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global Start Control CCU42"]
    #[inline(always)]
    pub fn gsc42(&self) -> GSC42_R {
        GSC42_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global Start Control CCU43"]
    #[inline(always)]
    pub fn gsc43(&self) -> GSC43_R {
        GSC43_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Global Start Control CCU80"]
    #[inline(always)]
    pub fn gsc80(&self) -> GSC80_R {
        GSC80_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Start Control CCU81"]
    #[inline(always)]
    pub fn gsc81(&self) -> GSC81_R {
        GSC81_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Start Control CCU40"]
    #[inline(always)]
    #[must_use]
    pub fn gsc40(&mut self) -> GSC40_W<0> {
        GSC40_W::new(self)
    }
    #[doc = "Bit 1 - Global Start Control CCU41"]
    #[inline(always)]
    #[must_use]
    pub fn gsc41(&mut self) -> GSC41_W<1> {
        GSC41_W::new(self)
    }
    #[doc = "Bit 2 - Global Start Control CCU42"]
    #[inline(always)]
    #[must_use]
    pub fn gsc42(&mut self) -> GSC42_W<2> {
        GSC42_W::new(self)
    }
    #[doc = "Bit 3 - Global Start Control CCU43"]
    #[inline(always)]
    #[must_use]
    pub fn gsc43(&mut self) -> GSC43_W<3> {
        GSC43_W::new(self)
    }
    #[doc = "Bit 8 - Global Start Control CCU80"]
    #[inline(always)]
    #[must_use]
    pub fn gsc80(&mut self) -> GSC80_W<8> {
        GSC80_W::new(self)
    }
    #[doc = "Bit 9 - Global Start Control CCU81"]
    #[inline(always)]
    #[must_use]
    pub fn gsc81(&mut self) -> GSC81_W<9> {
        GSC81_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccucon](index.html) module"]
pub struct CCUCON_SPEC;
impl crate::RegisterSpec for CCUCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccucon::R](R) reader structure"]
impl crate::Readable for CCUCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccucon::W](W) writer structure"]
impl crate::Writable for CCUCON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCUCON to value 0"]
impl crate::Resettable for CCUCON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
