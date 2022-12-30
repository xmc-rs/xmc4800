#[doc = "Register `MCMC` writer"]
pub struct W(crate::W<MCMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MCMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MNPC` writer - Multi-Channel Pattern Update Enable Clear"]
pub type MNPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCMC_SPEC, bool, O>;
#[doc = "Field `MPC` writer - Multi-Channel Pattern clear"]
pub type MPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCMC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Multi-Channel Pattern Update Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn mnpc(&mut self) -> MNPC_W<0> {
        MNPC_W::new(self)
    }
    #[doc = "Bit 1 - Multi-Channel Pattern clear"]
    #[inline(always)]
    #[must_use]
    pub fn mpc(&mut self) -> MPC_W<1> {
        MPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Multi-Channel Pattern Control clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcmc](index.html) module"]
pub struct MCMC_SPEC;
impl crate::RegisterSpec for MCMC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mcmc::W](W) writer structure"]
impl crate::Writable for MCMC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCMC to value 0"]
impl crate::Resettable for MCMC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
