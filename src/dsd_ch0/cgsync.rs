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
pub struct SDCOUNT_R(crate::FieldReader<u8, u8>);
impl SDCOUNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDCOUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDCAP` reader - Sign Delay Capture Value"]
pub struct SDCAP_R(crate::FieldReader<u8, u8>);
impl SDCAP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDCAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDCAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDPOS` reader - Sign Delay Value for Positive Halfwave"]
pub struct SDPOS_R(crate::FieldReader<u8, u8>);
impl SDPOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDPOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDPOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDPOS` writer - Sign Delay Value for Positive Halfwave"]
pub struct SDPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> SDPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `SDNEG` reader - Sign Delay Value for Negative Halfwave"]
pub struct SDNEG_R(crate::FieldReader<u8, u8>);
impl SDNEG_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDNEG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDNEG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDNEG` writer - Sign Delay Value for Negative Halfwave"]
pub struct SDNEG_W<'a> {
    w: &'a mut W,
}
impl<'a> SDNEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
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
    pub fn sdpos(&mut self) -> SDPOS_W {
        SDPOS_W { w: self }
    }
    #[doc = "Bits 24:31 - Sign Delay Value for Negative Halfwave"]
    #[inline(always)]
    pub fn sdneg(&mut self) -> SDNEG_W {
        SDNEG_W { w: self }
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
}
#[doc = "`reset()` method sets CGSYNC to value 0"]
impl crate::Resettable for CGSYNC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
