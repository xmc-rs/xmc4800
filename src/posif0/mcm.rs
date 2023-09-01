#[doc = "Register `MCM` reader"]
pub type R = crate::R<MCM_SPEC>;
#[doc = "Field `MCMP` reader - Multi-Channel Pattern"]
pub type MCMP_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Multi-Channel Pattern"]
    #[inline(always)]
    pub fn mcmp(&self) -> MCMP_R {
        MCMP_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Multi-Channel Pattern\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcm::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCM_SPEC;
impl crate::RegisterSpec for MCM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcm::R`](R) reader structure"]
impl crate::Readable for MCM_SPEC {}
#[doc = "`reset()` method sets MCM to value 0"]
impl crate::Resettable for MCM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
