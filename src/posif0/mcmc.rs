#[doc = "Register `MCMC` writer"]
pub type W = crate::W<McmcSpec>;
#[doc = "Field `MNPC` writer - Multi-Channel Pattern Update Enable Clear"]
pub type MnpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPC` writer - Multi-Channel Pattern clear"]
pub type MpcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Multi-Channel Pattern Update Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn mnpc(&mut self) -> MnpcW<McmcSpec> {
        MnpcW::new(self, 0)
    }
    #[doc = "Bit 1 - Multi-Channel Pattern clear"]
    #[inline(always)]
    #[must_use]
    pub fn mpc(&mut self) -> MpcW<McmcSpec> {
        MpcW::new(self, 1)
    }
}
#[doc = "Multi-Channel Pattern Control clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcmc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McmcSpec;
impl crate::RegisterSpec for McmcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mcmc::W`](W) writer structure"]
impl crate::Writable for McmcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCMC to value 0"]
impl crate::Resettable for McmcSpec {
    const RESET_VALUE: u32 = 0;
}
