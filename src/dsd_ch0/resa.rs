#[doc = "Register `RESA` reader"]
pub type R = crate::R<RESA_SPEC>;
#[doc = "Field `RESULT` reader - Result of most recent conversion"]
pub type RESULT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Result of most recent conversion"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Result Register, Auxiliary Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resa::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESA_SPEC;
impl crate::RegisterSpec for RESA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resa::R`](R) reader structure"]
impl crate::Readable for RESA_SPEC {}
#[doc = "`reset()` method sets RESA to value 0"]
impl crate::Resettable for RESA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
