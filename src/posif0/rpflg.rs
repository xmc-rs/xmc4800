#[doc = "Register `RPFLG` writer"]
pub type W = crate::W<RPFLG_SPEC>;
#[doc = "Field `RCHE` writer - Correct Hall Event flag clear"]
pub type RCHE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWHE` writer - Wrong Hall Event flag clear"]
pub type RWHE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RHIE` writer - Hall Inputs Update Event flag clear"]
pub type RHIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMST` writer - Multi-Channel Pattern shadow transfer flag clear"]
pub type RMST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RINDX` writer - Quadrature Index flag clear"]
pub type RINDX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RERR` writer - Quadrature Phase Error flag clear"]
pub type RERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCNT` writer - Quadrature CLK flag clear"]
pub type RCNT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDIR` writer - Quadrature Direction flag clear"]
pub type RDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPCLK` writer - Quadrature period clock flag clear"]
pub type RPCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Correct Hall Event flag clear"]
    #[inline(always)]
    pub fn rche(&mut self) -> RCHE_W<RPFLG_SPEC> {
        RCHE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Wrong Hall Event flag clear"]
    #[inline(always)]
    pub fn rwhe(&mut self) -> RWHE_W<RPFLG_SPEC> {
        RWHE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Hall Inputs Update Event flag clear"]
    #[inline(always)]
    pub fn rhie(&mut self) -> RHIE_W<RPFLG_SPEC> {
        RHIE_W::new(self, 2)
    }
    #[doc = "Bit 4 - Multi-Channel Pattern shadow transfer flag clear"]
    #[inline(always)]
    pub fn rmst(&mut self) -> RMST_W<RPFLG_SPEC> {
        RMST_W::new(self, 4)
    }
    #[doc = "Bit 8 - Quadrature Index flag clear"]
    #[inline(always)]
    pub fn rindx(&mut self) -> RINDX_W<RPFLG_SPEC> {
        RINDX_W::new(self, 8)
    }
    #[doc = "Bit 9 - Quadrature Phase Error flag clear"]
    #[inline(always)]
    pub fn rerr(&mut self) -> RERR_W<RPFLG_SPEC> {
        RERR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Quadrature CLK flag clear"]
    #[inline(always)]
    pub fn rcnt(&mut self) -> RCNT_W<RPFLG_SPEC> {
        RCNT_W::new(self, 10)
    }
    #[doc = "Bit 11 - Quadrature Direction flag clear"]
    #[inline(always)]
    pub fn rdir(&mut self) -> RDIR_W<RPFLG_SPEC> {
        RDIR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Quadrature period clock flag clear"]
    #[inline(always)]
    pub fn rpclk(&mut self) -> RPCLK_W<RPFLG_SPEC> {
        RPCLK_W::new(self, 12)
    }
}
#[doc = "POSIF Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpflg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RPFLG_SPEC;
impl crate::RegisterSpec for RPFLG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rpflg::W`](W) writer structure"]
impl crate::Writable for RPFLG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RPFLG to value 0"]
impl crate::Resettable for RPFLG_SPEC {
    const RESET_VALUE: u32 = 0;
}
