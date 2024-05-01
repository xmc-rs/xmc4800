#[doc = "Register `SPFLG` writer"]
pub type W = crate::W<SpflgSpec>;
#[doc = "Field `SCHE` writer - Correct Hall Event flag set"]
pub type ScheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWHE` writer - Wrong Hall Event flag set"]
pub type SwheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHIE` writer - Hall Inputs Update Event flag set"]
pub type ShieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMST` writer - Multi-Channel Pattern shadow transfer flag set"]
pub type SmstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINDX` writer - Quadrature Index flag set"]
pub type SindxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERR` writer - Quadrature Phase Error flag set"]
pub type SerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCNT` writer - Quadrature CLK flag set"]
pub type ScntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIR` writer - Quadrature Direction flag set"]
pub type SdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPCLK` writer - Quadrature period clock flag set"]
pub type SpclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Correct Hall Event flag set"]
    #[inline(always)]
    #[must_use]
    pub fn sche(&mut self) -> ScheW<SpflgSpec> {
        ScheW::new(self, 0)
    }
    #[doc = "Bit 1 - Wrong Hall Event flag set"]
    #[inline(always)]
    #[must_use]
    pub fn swhe(&mut self) -> SwheW<SpflgSpec> {
        SwheW::new(self, 1)
    }
    #[doc = "Bit 2 - Hall Inputs Update Event flag set"]
    #[inline(always)]
    #[must_use]
    pub fn shie(&mut self) -> ShieW<SpflgSpec> {
        ShieW::new(self, 2)
    }
    #[doc = "Bit 4 - Multi-Channel Pattern shadow transfer flag set"]
    #[inline(always)]
    #[must_use]
    pub fn smst(&mut self) -> SmstW<SpflgSpec> {
        SmstW::new(self, 4)
    }
    #[doc = "Bit 8 - Quadrature Index flag set"]
    #[inline(always)]
    #[must_use]
    pub fn sindx(&mut self) -> SindxW<SpflgSpec> {
        SindxW::new(self, 8)
    }
    #[doc = "Bit 9 - Quadrature Phase Error flag set"]
    #[inline(always)]
    #[must_use]
    pub fn serr(&mut self) -> SerrW<SpflgSpec> {
        SerrW::new(self, 9)
    }
    #[doc = "Bit 10 - Quadrature CLK flag set"]
    #[inline(always)]
    #[must_use]
    pub fn scnt(&mut self) -> ScntW<SpflgSpec> {
        ScntW::new(self, 10)
    }
    #[doc = "Bit 11 - Quadrature Direction flag set"]
    #[inline(always)]
    #[must_use]
    pub fn sdir(&mut self) -> SdirW<SpflgSpec> {
        SdirW::new(self, 11)
    }
    #[doc = "Bit 12 - Quadrature period clock flag set"]
    #[inline(always)]
    #[must_use]
    pub fn spclk(&mut self) -> SpclkW<SpflgSpec> {
        SpclkW::new(self, 12)
    }
}
#[doc = "POSIF Interrupt Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spflg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpflgSpec;
impl crate::RegisterSpec for SpflgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spflg::W`](W) writer structure"]
impl crate::Writable for SpflgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPFLG to value 0"]
impl crate::Resettable for SpflgSpec {
    const RESET_VALUE: u32 = 0;
}
