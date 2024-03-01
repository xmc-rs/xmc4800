#[doc = "Register `RESM` reader"]
pub type R = crate::R<ResmSpec>;
#[doc = "Field `RESULT` reader - Result of most recent conversion"]
pub type ResultR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Result of most recent conversion"]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Result Register, Main Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resm::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResmSpec;
impl crate::RegisterSpec for ResmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resm::R`](R) reader structure"]
impl crate::Readable for ResmSpec {}
#[doc = "`reset()` method sets RESM to value 0"]
impl crate::Resettable for ResmSpec {
    const RESET_VALUE: u32 = 0;
}
