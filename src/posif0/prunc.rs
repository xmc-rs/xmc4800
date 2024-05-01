#[doc = "Register `PRUNC` writer"]
pub type W = crate::W<PruncSpec>;
#[doc = "Field `CRB` writer - Clear Run bit"]
pub type CrbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSM` writer - Clear Current internal status"]
pub type CsmW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear Run bit"]
    #[inline(always)]
    #[must_use]
    pub fn crb(&mut self) -> CrbW<PruncSpec> {
        CrbW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Current internal status"]
    #[inline(always)]
    #[must_use]
    pub fn csm(&mut self) -> CsmW<PruncSpec> {
        CsmW::new(self, 1)
    }
}
#[doc = "POSIF Run Bit Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prunc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PruncSpec;
impl crate::RegisterSpec for PruncSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prunc::W`](W) writer structure"]
impl crate::Writable for PruncSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRUNC to value 0"]
impl crate::Resettable for PruncSpec {
    const RESET_VALUE: u32 = 0;
}
