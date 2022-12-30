#[doc = "Register `CLKCALCONST` reader"]
pub struct R(crate::R<CLKCALCONST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCALCONST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCALCONST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCALCONST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKCALCONST` writer"]
pub struct W(crate::W<CLKCALCONST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCALCONST_SPEC>;
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
impl From<crate::W<CLKCALCONST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCALCONST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALIBCONST` reader - Clock Calibration Constant Value"]
pub type CALIBCONST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALIBCONST` writer - Clock Calibration Constant Value"]
pub type CALIBCONST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCALCONST_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Clock Calibration Constant Value"]
    #[inline(always)]
    pub fn calibconst(&self) -> CALIBCONST_R {
        CALIBCONST_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Calibration Constant Value"]
    #[inline(always)]
    #[must_use]
    pub fn calibconst(&mut self) -> CALIBCONST_W<0> {
        CALIBCONST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Calibration Constant Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcalconst](index.html) module"]
pub struct CLKCALCONST_SPEC;
impl crate::RegisterSpec for CLKCALCONST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkcalconst::R](R) reader structure"]
impl crate::Readable for CLKCALCONST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkcalconst::W](W) writer structure"]
impl crate::Writable for CLKCALCONST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKCALCONST to value 0"]
impl crate::Resettable for CLKCALCONST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
