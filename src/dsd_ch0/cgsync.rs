#[doc = "Register `CGSYNC` reader"]
pub struct R(crate::R<CGSYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGSYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGSYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGSYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CGSYNC` writer"]
pub struct W(crate::W<CGSYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGSYNC_SPEC>;
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
impl From<crate::W<CGSYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGSYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDCOUNT` reader - Sign Delay Counter"]
pub type SDCOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDCAP` reader - Sign Delay Capture Value"]
pub type SDCAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDPOS` reader - Sign Delay Value for Positive Halfwave"]
pub type SDPOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDPOS` writer - Sign Delay Value for Positive Halfwave"]
pub type SDPOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CGSYNC_SPEC, u8, u8, 8, O>;
#[doc = "Field `SDNEG` reader - Sign Delay Value for Negative Halfwave"]
pub type SDNEG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDNEG` writer - Sign Delay Value for Negative Halfwave"]
pub type SDNEG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CGSYNC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Sign Delay Counter"]
    #[inline(always)]
    pub fn sdcount(&self) -> SDCOUNT_R {
        SDCOUNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sign Delay Capture Value"]
    #[inline(always)]
    pub fn sdcap(&self) -> SDCAP_R {
        SDCAP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sign Delay Value for Positive Halfwave"]
    #[inline(always)]
    pub fn sdpos(&self) -> SDPOS_R {
        SDPOS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sign Delay Value for Negative Halfwave"]
    #[inline(always)]
    pub fn sdneg(&self) -> SDNEG_R {
        SDNEG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Sign Delay Value for Positive Halfwave"]
    #[inline(always)]
    #[must_use]
    pub fn sdpos(&mut self) -> SDPOS_W<16> {
        SDPOS_W::new(self)
    }
    #[doc = "Bits 24:31 - Sign Delay Value for Negative Halfwave"]
    #[inline(always)]
    #[must_use]
    pub fn sdneg(&mut self) -> SDNEG_W<24> {
        SDNEG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Carrier Generator Synchronization Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgsync](index.html) module"]
pub struct CGSYNC_SPEC;
impl crate::RegisterSpec for CGSYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgsync::R](R) reader structure"]
impl crate::Readable for CGSYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgsync::W](W) writer structure"]
impl crate::Writable for CGSYNC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CGSYNC to value 0"]
impl crate::Resettable for CGSYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
