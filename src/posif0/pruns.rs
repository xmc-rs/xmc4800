#[doc = "Register `PRUNS` writer"]
pub type W = crate::W<PRUNS_SPEC>;
#[doc = "Field `SRB` writer - Set Run bit"]
pub type SRB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set Run bit"]
    #[inline(always)]
    #[must_use]
    pub fn srb(&mut self) -> SRB_W<PRUNS_SPEC> {
        SRB_W::new(self, 0)
    }
}
#[doc = "POSIF Run Bit Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pruns::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRUNS_SPEC;
impl crate::RegisterSpec for PRUNS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pruns::W`](W) writer structure"]
impl crate::Writable for PRUNS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRUNS to value 0"]
impl crate::Resettable for PRUNS_SPEC {
    const RESET_VALUE: u32 = 0;
}
