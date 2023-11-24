#[doc = "Register `MCSM` reader"]
pub type R = crate::R<MCSM_SPEC>;
#[doc = "Register `MCSM` writer"]
pub type W = crate::W<MCSM_SPEC>;
#[doc = "Field `MCMPS` reader - Shadow Multi-Channel Pattern"]
pub type MCMPS_R = crate::FieldReader<u16>;
#[doc = "Field `MCMPS` writer - Shadow Multi-Channel Pattern"]
pub type MCMPS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shadow Multi-Channel Pattern"]
    #[inline(always)]
    pub fn mcmps(&self) -> MCMPS_R {
        MCMPS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow Multi-Channel Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn mcmps(&mut self) -> MCMPS_W<MCSM_SPEC> {
        MCMPS_W::new(self, 0)
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
#[doc = "Multi-Channel Shadow Pattern\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcsm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcsm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCSM_SPEC;
impl crate::RegisterSpec for MCSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcsm::R`](R) reader structure"]
impl crate::Readable for MCSM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcsm::W`](W) writer structure"]
impl crate::Writable for MCSM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCSM to value 0"]
impl crate::Resettable for MCSM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
