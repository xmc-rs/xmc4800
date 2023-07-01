#[doc = "Register `GCSC` writer"]
pub struct W(crate::W<GCSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCSC_SPEC>;
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
impl From<crate::W<GCSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S0SC` writer - Slice 0 shadow transfer clear"]
pub type S0SC_W<'a, const O: u8> = crate::BitWriter<'a, GCSC_SPEC, O>;
#[doc = "Field `S0DSC` writer - Slice 0 Dither shadow transfer clear"]
pub type S0DSC_W<'a, const O: u8> = crate::BitWriter<'a, GCSC_SPEC, O>;
#[doc = "Field `S0PSC` writer - Slice 0 Prescaler shadow transfer clear"]
pub type S0PSC_W<'a, const O: u8> = crate::BitWriter<'a, GCSC_SPEC, O>;
#[doc = "Field `S1SC` writer - Slice 1 shadow transfer clear"]
pub type S1SC_W<'a, const O: u8> = crate::BitWriter<'a, GCSC_SPEC, O>;
#[doc = "Field `S1DSC` writer - Slice 1 Dither shadow transfer clear"]
pub type S1DSC_W<'a, const O: u8> = crate::BitWriter<'a, GCSC_SPEC, O>;
#[doc = "Field `S1PSC` writer - Slice 1 Prescaler shadow transfer clear"]
pub type S1PSC_W<'a, const O: u8> = crate::BitWriter<'a, GCSC_SPEC, O>;
#[doc = "Field `S2SC` writer - Slice 2 shadow transfer clear"]
pub type S2SC_W<'a, const O: u8> = crate::BitWriter<'a, GCSC_SPEC, O>;
#[doc = "Field `S2DSC` writer - Slice 2 Dither shadow transfer clear"]
pub type S2DSC_W<'a, const O: u8> = crate::BitWriter<'a, GCSC_SPEC, O>;
#[doc = "Field `S2PSC` writer - Slice 2 Prescaler shadow transfer clear"]
pub type S2PSC_W<'a, const O: u8> = crate::BitWriter<'a, GCSC_SPEC, O>;
#[doc = "Field `S3SC` writer - Slice 3 shadow transfer clear"]
pub type S3SC_W<'a, const O: u8> = crate::BitWriter<'a, GCSC_SPEC, O>;
#[doc = "Field `S3DSC` writer - Slice 3 Dither shadow transfer clear"]
pub type S3DSC_W<'a, const O: u8> = crate::BitWriter<'a, GCSC_SPEC, O>;
#[doc = "Field `S3PSC` writer - Slice 3 Prescaler shadow transfer clear"]
pub type S3PSC_W<'a, const O: u8> = crate::BitWriter<'a, GCSC_SPEC, O>;
#[doc = "Field `S0STC` writer - Slice 0 status bit clear"]
pub type S0STC_W<'a, const O: u8> = crate::BitWriter<'a, GCSC_SPEC, O>;
#[doc = "Field `S1STC` writer - Slice 1 status bit clear"]
pub type S1STC_W<'a, const O: u8> = crate::BitWriter<'a, GCSC_SPEC, O>;
#[doc = "Field `S2STC` writer - Slice 2 status bit clear"]
pub type S2STC_W<'a, const O: u8> = crate::BitWriter<'a, GCSC_SPEC, O>;
#[doc = "Field `S3STC` writer - Slice 3 status bit clear"]
pub type S3STC_W<'a, const O: u8> = crate::BitWriter<'a, GCSC_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Slice 0 shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s0sc(&mut self) -> S0SC_W<0> {
        S0SC_W::new(self)
    }
    #[doc = "Bit 1 - Slice 0 Dither shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s0dsc(&mut self) -> S0DSC_W<1> {
        S0DSC_W::new(self)
    }
    #[doc = "Bit 2 - Slice 0 Prescaler shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s0psc(&mut self) -> S0PSC_W<2> {
        S0PSC_W::new(self)
    }
    #[doc = "Bit 4 - Slice 1 shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s1sc(&mut self) -> S1SC_W<4> {
        S1SC_W::new(self)
    }
    #[doc = "Bit 5 - Slice 1 Dither shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s1dsc(&mut self) -> S1DSC_W<5> {
        S1DSC_W::new(self)
    }
    #[doc = "Bit 6 - Slice 1 Prescaler shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s1psc(&mut self) -> S1PSC_W<6> {
        S1PSC_W::new(self)
    }
    #[doc = "Bit 8 - Slice 2 shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s2sc(&mut self) -> S2SC_W<8> {
        S2SC_W::new(self)
    }
    #[doc = "Bit 9 - Slice 2 Dither shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s2dsc(&mut self) -> S2DSC_W<9> {
        S2DSC_W::new(self)
    }
    #[doc = "Bit 10 - Slice 2 Prescaler shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s2psc(&mut self) -> S2PSC_W<10> {
        S2PSC_W::new(self)
    }
    #[doc = "Bit 12 - Slice 3 shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s3sc(&mut self) -> S3SC_W<12> {
        S3SC_W::new(self)
    }
    #[doc = "Bit 13 - Slice 3 Dither shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s3dsc(&mut self) -> S3DSC_W<13> {
        S3DSC_W::new(self)
    }
    #[doc = "Bit 14 - Slice 3 Prescaler shadow transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn s3psc(&mut self) -> S3PSC_W<14> {
        S3PSC_W::new(self)
    }
    #[doc = "Bit 16 - Slice 0 status bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn s0stc(&mut self) -> S0STC_W<16> {
        S0STC_W::new(self)
    }
    #[doc = "Bit 17 - Slice 1 status bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn s1stc(&mut self) -> S1STC_W<17> {
        S1STC_W::new(self)
    }
    #[doc = "Bit 18 - Slice 2 status bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn s2stc(&mut self) -> S2STC_W<18> {
        S2STC_W::new(self)
    }
    #[doc = "Bit 19 - Slice 3 status bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn s3stc(&mut self) -> S3STC_W<19> {
        S3STC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Channel Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcsc](index.html) module"]
pub struct GCSC_SPEC;
impl crate::RegisterSpec for GCSC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gcsc::W](W) writer structure"]
impl crate::Writable for GCSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCSC to value 0"]
impl crate::Resettable for GCSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
