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
#[doc = "Modulator Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `MCSEL` reader - Modulator Clock Select"]
pub struct MCSEL_R(crate::FieldReader<u8, MCSEL_A>);
impl MCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == MCSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MCSEL_A::VALUE2
    }
}
impl core::ops::Deref for MCSEL_R {
    type Target = crate::FieldReader<u8, MCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCSEL` writer - Modulator Clock Select"]
pub struct MCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Modulator Clock Select"]
    #[inline(always)]
    pub fn mcsel(&self) -> MCSEL_R {
        MCSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Modulator Clock Select"]
    #[inline(always)]
    pub fn mcsel(&mut self) -> MCSEL_W {
        MCSEL_W { w: self }
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
}
#[doc = "`reset()` method sets GLOBCFG to value 0"]
impl crate::Resettable for GLOBCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
