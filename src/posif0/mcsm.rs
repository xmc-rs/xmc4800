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
pub type MCMPS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MCMPS` writer - Shadow Multi-Channel Pattern"]
pub type MCMPS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCSM_SPEC, u16, u16, 16, O>;
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
    #[must_use]
    pub fn mcmps(&mut self) -> MCMPS_W<0> {
        MCMPS_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCSM to value 0"]
impl crate::Resettable for MCSM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
