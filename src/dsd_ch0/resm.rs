#[doc = "Register `RESM` reader"]
pub struct R(crate::R<RESM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESULT` reader - Result of most recent conversion"]
pub type RESULT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Result of most recent conversion"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Result Register, Main Filter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resm](index.html) module"]
pub struct RESM_SPEC;
impl crate::RegisterSpec for RESM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resm::R](R) reader structure"]
impl crate::Readable for RESM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESM to value 0"]
impl crate::Resettable for RESM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
