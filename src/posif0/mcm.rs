#[doc = "Register `MCM` reader"]
pub type R = crate::R<McmSpec>;
#[doc = "Field `MCMP` reader - Multi-Channel Pattern"]
pub type McmpR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Multi-Channel Pattern"]
    #[inline(always)]
    pub fn mcmp(&self) -> McmpR {
        McmpR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Multi-Channel Pattern\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcm::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McmSpec;
impl crate::RegisterSpec for McmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcm::R`](R) reader structure"]
impl crate::Readable for McmSpec {}
#[doc = "`reset()` method sets MCM to value 0"]
impl crate::Resettable for McmSpec {
    const RESET_VALUE: u32 = 0;
}
