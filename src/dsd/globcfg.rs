#[doc = "Register `GLOBCFG` reader"]
pub struct R(crate::R<GLOBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLOBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLOBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLOBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLOBCFG` writer"]
pub struct W(crate::W<GLOBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLOBCFG_SPEC>;
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
impl From<crate::W<GLOBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLOBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCSEL` reader - Modulator Clock Select"]
pub type MCSEL_R = crate::FieldReader<u8, MCSEL_A>;
#[doc = "Modulator Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCSEL_A {
    #[doc = "0: Internal clock off, no source selected"]
    VALUE1 = 0,
    #[doc = "1: fDSD"]
    VALUE2 = 1,
}
impl From<MCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MCSEL_A) -> Self {
        variant as _
    }
}
impl MCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MCSEL_A> {
        match self.bits {
            0 => Some(MCSEL_A::VALUE1),
            1 => Some(MCSEL_A::VALUE2),
            _ => None,
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
#[doc = "Field `MCSEL` writer - Modulator Clock Select"]
pub type MCSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GLOBCFG_SPEC, u8, MCSEL_A, 3, O>;
impl<'a, const O: u8> MCSEL_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:2 - Modulator Clock Select"]
    #[inline(always)]
    pub fn mcsel(&self) -> MCSEL_R {
        MCSEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Modulator Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn mcsel(&mut self) -> MCSEL_W<0> {
        MCSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globcfg](index.html) module"]
pub struct GLOBCFG_SPEC;
impl crate::RegisterSpec for GLOBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [globcfg::R](R) reader structure"]
impl crate::Readable for GLOBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [globcfg::W](W) writer structure"]
impl crate::Writable for GLOBCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GLOBCFG to value 0"]
impl crate::Resettable for GLOBCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
