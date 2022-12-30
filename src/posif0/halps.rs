#[doc = "Register `HALPS` reader"]
pub struct R(crate::R<HALPS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HALPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HALPS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HALPS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HALPS` writer"]
pub struct W(crate::W<HALPS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HALPS_SPEC>;
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
impl From<crate::W<HALPS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HALPS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HCPS` reader - Shadow Hall Current Pattern"]
pub type HCPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HCPS` writer - Shadow Hall Current Pattern"]
pub type HCPS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HALPS_SPEC, u8, u8, 3, O>;
#[doc = "Field `HEPS` reader - Shadow Hall expected Pattern"]
pub type HEPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HEPS` writer - Shadow Hall expected Pattern"]
pub type HEPS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HALPS_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Shadow Hall Current Pattern"]
    #[inline(always)]
    pub fn hcps(&self) -> HCPS_R {
        HCPS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Shadow Hall expected Pattern"]
    #[inline(always)]
    pub fn heps(&self) -> HEPS_R {
        HEPS_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Shadow Hall Current Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn hcps(&mut self) -> HCPS_W<0> {
        HCPS_W::new(self)
    }
    #[doc = "Bits 3:5 - Shadow Hall expected Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn heps(&mut self) -> HEPS_W<3> {
        HEPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hall Sensor Shadow Patterns\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [halps](index.html) module"]
pub struct HALPS_SPEC;
impl crate::RegisterSpec for HALPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [halps::R](R) reader structure"]
impl crate::Readable for HALPS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [halps::W](W) writer structure"]
impl crate::Writable for HALPS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HALPS to value 0"]
impl crate::Resettable for HALPS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
