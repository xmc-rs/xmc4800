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
pub type OFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OFFSET` writer - Offset Value"]
pub type OFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFFM_SPEC, u16, u16, 16, O>;
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
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<0> {
        OFFSET_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OFFM to value 0"]
impl crate::Resettable for OFFM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
