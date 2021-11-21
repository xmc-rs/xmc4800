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
pub struct HCPS_R(crate::FieldReader<u8, u8>);
impl HCPS_R {
    pub(crate) fn new(bits: u8) -> Self {
        HCPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCPS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCPS` writer - Shadow Hall Current Pattern"]
pub struct HCPS_W<'a> {
    w: &'a mut W,
}
impl<'a> HCPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `HEPS` reader - Shadow Hall expected Pattern"]
pub struct HEPS_R(crate::FieldReader<u8, u8>);
impl HEPS_R {
    pub(crate) fn new(bits: u8) -> Self {
        HEPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HEPS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HEPS` writer - Shadow Hall expected Pattern"]
pub struct HEPS_W<'a> {
    w: &'a mut W,
}
impl<'a> HEPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Shadow Hall Current Pattern"]
    #[inline(always)]
    pub fn hcps(&self) -> HCPS_R {
        HCPS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Shadow Hall expected Pattern"]
    #[inline(always)]
    pub fn heps(&self) -> HEPS_R {
        HEPS_R::new(((self.bits >> 3) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Shadow Hall Current Pattern"]
    #[inline(always)]
    pub fn hcps(&mut self) -> HCPS_W {
        HCPS_W { w: self }
    }
    #[doc = "Bits 3:5 - Shadow Hall expected Pattern"]
    #[inline(always)]
    pub fn heps(&mut self) -> HEPS_W {
        HEPS_W { w: self }
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
}
#[doc = "`reset()` method sets HALPS to value 0"]
impl crate::Resettable for HALPS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
