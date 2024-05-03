#[doc = "Register `SPFLG` writer"]
pub type W = crate::W<SPFLG_SPEC>;
#[doc = "Field `SCHE` writer - Correct Hall Event flag set"]
pub type SCHE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWHE` writer - Wrong Hall Event flag set"]
pub type SWHE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHIE` writer - Hall Inputs Update Event flag set"]
pub type SHIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMST` writer - Multi-Channel Pattern shadow transfer flag set"]
pub type SMST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINDX` writer - Quadrature Index flag set"]
pub type SINDX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERR` writer - Quadrature Phase Error flag set"]
pub type SERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCNT` writer - Quadrature CLK flag set"]
pub type SCNT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIR` writer - Quadrature Direction flag set"]
pub type SDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPCLK` writer - Quadrature period clock flag set"]
pub type SPCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Correct Hall Event flag set"]
    #[inline(always)]
    #[must_use]
    pub fn sche(&mut self) -> SCHE_W<SPFLG_SPEC> {
        SCHE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Wrong Hall Event flag set"]
    #[inline(always)]
    #[must_use]
    pub fn swhe(&mut self) -> SWHE_W<SPFLG_SPEC> {
        SWHE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Hall Inputs Update Event flag set"]
    #[inline(always)]
    #[must_use]
    pub fn shie(&mut self) -> SHIE_W<SPFLG_SPEC> {
        SHIE_W::new(self, 2)
    }
    #[doc = "Bit 4 - Multi-Channel Pattern shadow transfer flag set"]
    #[inline(always)]
    #[must_use]
    pub fn smst(&mut self) -> SMST_W<SPFLG_SPEC> {
        SMST_W::new(self, 4)
    }
    #[doc = "Bit 8 - Quadrature Index flag set"]
    #[inline(always)]
    #[must_use]
    pub fn sindx(&mut self) -> SINDX_W<SPFLG_SPEC> {
        SINDX_W::new(self, 8)
    }
    #[doc = "Bit 9 - Quadrature Phase Error flag set"]
    #[inline(always)]
    #[must_use]
    pub fn serr(&mut self) -> SERR_W<SPFLG_SPEC> {
        SERR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Quadrature CLK flag set"]
    #[inline(always)]
    #[must_use]
    pub fn scnt(&mut self) -> SCNT_W<SPFLG_SPEC> {
        SCNT_W::new(self, 10)
    }
    #[doc = "Bit 11 - Quadrature Direction flag set"]
    #[inline(always)]
    #[must_use]
    pub fn sdir(&mut self) -> SDIR_W<SPFLG_SPEC> {
        SDIR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Quadrature period clock flag set"]
    #[inline(always)]
    #[must_use]
    pub fn spclk(&mut self) -> SPCLK_W<SPFLG_SPEC> {
        SPCLK_W::new(self, 12)
    }
}
#[doc = "POSIF Interrupt Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spflg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPFLG_SPEC;
impl crate::RegisterSpec for SPFLG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spflg::W`](W) writer structure"]
impl crate::Writable for SPFLG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPFLG to value 0"]
impl crate::Resettable for SPFLG_SPEC {
    const RESET_VALUE: u32 = 0;
}
