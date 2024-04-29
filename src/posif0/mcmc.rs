#[doc = "Register `MCMC` writer"]
pub type W = crate::W<MCMC_SPEC>;
#[doc = "Field `MNPC` writer - Multi-Channel Pattern Update Enable Clear"]
pub type MNPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPC` writer - Multi-Channel Pattern clear"]
pub type MPC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Multi-Channel Pattern Update Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn mnpc(&mut self) -> MNPC_W<MCMC_SPEC> {
        MNPC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Multi-Channel Pattern clear"]
    #[inline(always)]
    #[must_use]
    pub fn mpc(&mut self) -> MPC_W<MCMC_SPEC> {
        MPC_W::new(self, 1)
    }
}
#[doc = "Multi-Channel Pattern Control clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcmc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCMC_SPEC;
impl crate::RegisterSpec for MCMC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mcmc::W`](W) writer structure"]
impl crate::Writable for MCMC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCMC to value 0"]
impl crate::Resettable for MCMC_SPEC {
    const RESET_VALUE: u32 = 0;
}
