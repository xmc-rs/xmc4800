#[doc = "Register `RESA` reader"]
pub type R = crate::R<ResaSpec>;
#[doc = "Field `RESULT` reader - Result of most recent conversion"]
pub type ResultR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Result of most recent conversion"]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Result Register, Auxiliary Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resa::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResaSpec;
impl crate::RegisterSpec for ResaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resa::R`](R) reader structure"]
impl crate::Readable for ResaSpec {}
#[doc = "`reset()` method sets RESA to value 0"]
impl crate::Resettable for ResaSpec {
    const RESET_VALUE: u32 = 0;
}
