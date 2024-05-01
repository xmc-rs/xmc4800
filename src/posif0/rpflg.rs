#[doc = "Register `RPFLG` writer"]
pub type W = crate::W<RpflgSpec>;
#[doc = "Field `RCHE` writer - Correct Hall Event flag clear"]
pub type RcheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWHE` writer - Wrong Hall Event flag clear"]
pub type RwheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RHIE` writer - Hall Inputs Update Event flag clear"]
pub type RhieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMST` writer - Multi-Channel Pattern shadow transfer flag clear"]
pub type RmstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RINDX` writer - Quadrature Index flag clear"]
pub type RindxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RERR` writer - Quadrature Phase Error flag clear"]
pub type RerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCNT` writer - Quadrature CLK flag clear"]
pub type RcntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDIR` writer - Quadrature Direction flag clear"]
pub type RdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPCLK` writer - Quadrature period clock flag clear"]
pub type RpclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Correct Hall Event flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rche(&mut self) -> RcheW<RpflgSpec> {
        RcheW::new(self, 0)
    }
    #[doc = "Bit 1 - Wrong Hall Event flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rwhe(&mut self) -> RwheW<RpflgSpec> {
        RwheW::new(self, 1)
    }
    #[doc = "Bit 2 - Hall Inputs Update Event flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rhie(&mut self) -> RhieW<RpflgSpec> {
        RhieW::new(self, 2)
    }
    #[doc = "Bit 4 - Multi-Channel Pattern shadow transfer flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rmst(&mut self) -> RmstW<RpflgSpec> {
        RmstW::new(self, 4)
    }
    #[doc = "Bit 8 - Quadrature Index flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rindx(&mut self) -> RindxW<RpflgSpec> {
        RindxW::new(self, 8)
    }
    #[doc = "Bit 9 - Quadrature Phase Error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rerr(&mut self) -> RerrW<RpflgSpec> {
        RerrW::new(self, 9)
    }
    #[doc = "Bit 10 - Quadrature CLK flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rcnt(&mut self) -> RcntW<RpflgSpec> {
        RcntW::new(self, 10)
    }
    #[doc = "Bit 11 - Quadrature Direction flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rdir(&mut self) -> RdirW<RpflgSpec> {
        RdirW::new(self, 11)
    }
    #[doc = "Bit 12 - Quadrature period clock flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpclk(&mut self) -> RpclkW<RpflgSpec> {
        RpclkW::new(self, 12)
    }
}
#[doc = "POSIF Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpflg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RpflgSpec;
impl crate::RegisterSpec for RpflgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rpflg::W`](W) writer structure"]
impl crate::Writable for RpflgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RPFLG to value 0"]
impl crate::Resettable for RpflgSpec {
    const RESET_VALUE: u32 = 0;
}
