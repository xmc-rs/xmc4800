#[doc = "Register `MCSM` reader"]
pub struct R(crate::R<MCSM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCSM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCSM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCSM` writer"]
pub struct W(crate::W<MCSM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCSM_SPEC>;
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
impl From<crate::W<MCSM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCSM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCMPS` reader - Shadow Multi-Channel Pattern"]
pub struct MCMPS_R(crate::FieldReader<u16, u16>);
impl MCMPS_R {
    pub(crate) fn new(bits: u16) -> Self {
        MCMPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCMPS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCMPS` writer - Shadow Multi-Channel Pattern"]
pub struct MCMPS_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Shadow Multi-Channel Pattern"]
    #[inline(always)]
    pub fn mcmps(&self) -> MCMPS_R {
        MCMPS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow Multi-Channel Pattern"]
    #[inline(always)]
    pub fn mcmps(&mut self) -> MCMPS_W {
        MCMPS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Multi-Channel Shadow Pattern\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcsm](index.html) module"]
pub struct MCSM_SPEC;
impl crate::RegisterSpec for MCSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcsm::R](R) reader structure"]
impl crate::Readable for MCSM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcsm::W](W) writer structure"]
impl crate::Writable for MCSM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCSM to value 0"]
impl crate::Resettable for MCSM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
