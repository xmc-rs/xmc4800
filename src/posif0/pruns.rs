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
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "POSIF Run Bit Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pruns::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRUNS_SPEC;
impl crate::RegisterSpec for PRUNS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pruns::W`](W) writer structure"]
impl crate::Writable for PRUNS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRUNS to value 0"]
impl crate::Resettable for PRUNS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
