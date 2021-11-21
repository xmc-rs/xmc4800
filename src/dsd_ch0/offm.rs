#[doc = "Register `OFFM` reader"]
pub struct R(crate::R<OFFM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFFM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFFM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFFM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFFM` writer"]
pub struct W(crate::W<OFFM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFFM_SPEC>;
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
impl From<crate::W<OFFM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFFM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET` reader - Offset Value"]
pub struct OFFSET_R(crate::FieldReader<u16, u16>);
impl OFFSET_R {
    pub(crate) fn new(bits: u16) -> Self {
        OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSET_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET` writer - Offset Value"]
pub struct OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Offset Value"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Offset Value"]
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W {
        OFFSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset Register, Main Filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [offm](index.html) module"]
pub struct OFFM_SPEC;
impl crate::RegisterSpec for OFFM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [offm::R](R) reader structure"]
impl crate::Readable for OFFM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [offm::W](W) writer structure"]
impl crate::Writable for OFFM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OFFM to value 0"]
impl crate::Resettable for OFFM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
