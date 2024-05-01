#[doc = "Register `PRUNS` writer"]
pub type W = crate::W<PrunsSpec>;
#[doc = "Field `SRB` writer - Set Run bit"]
pub type SrbW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set Run bit"]
    #[inline(always)]
    #[must_use]
    pub fn srb(&mut self) -> SrbW<PrunsSpec> {
        SrbW::new(self, 0)
    }
}
#[doc = "POSIF Run Bit Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pruns::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrunsSpec;
impl crate::RegisterSpec for PrunsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pruns::W`](W) writer structure"]
impl crate::Writable for PrunsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRUNS to value 0"]
impl crate::Resettable for PrunsSpec {
    const RESET_VALUE: u32 = 0;
}
