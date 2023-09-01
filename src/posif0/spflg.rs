#[doc = "Register `SPFLG` writer"]
pub type W = crate::W<SPFLG_SPEC>;
#[doc = "Field `SCHE` writer - Correct Hall Event flag set"]
pub type SCHE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWHE` writer - Wrong Hall Event flag set"]
pub type SWHE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SHIE` writer - Hall Inputs Update Event flag set"]
pub type SHIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMST` writer - Multi-Channel Pattern shadow transfer flag set"]
pub type SMST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SINDX` writer - Quadrature Index flag set"]
pub type SINDX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SERR` writer - Quadrature Phase Error flag set"]
pub type SERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCNT` writer - Quadrature CLK flag set"]
pub type SCNT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIR` writer - Quadrature Direction flag set"]
pub type SDIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPCLK` writer - Quadrature period clock flag set"]
pub type SPCLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Correct Hall Event flag set"]
    #[inline(always)]
    #[must_use]
    pub fn sche(&mut self) -> SCHE_W<SPFLG_SPEC, 0> {
        SCHE_W::new(self)
    }
    #[doc = "Bit 1 - Wrong Hall Event flag set"]
    #[inline(always)]
    #[must_use]
    pub fn swhe(&mut self) -> SWHE_W<SPFLG_SPEC, 1> {
        SWHE_W::new(self)
    }
    #[doc = "Bit 2 - Hall Inputs Update Event flag set"]
    #[inline(always)]
    #[must_use]
    pub fn shie(&mut self) -> SHIE_W<SPFLG_SPEC, 2> {
        SHIE_W::new(self)
    }
    #[doc = "Bit 4 - Multi-Channel Pattern shadow transfer flag set"]
    #[inline(always)]
    #[must_use]
    pub fn smst(&mut self) -> SMST_W<SPFLG_SPEC, 4> {
        SMST_W::new(self)
    }
    #[doc = "Bit 8 - Quadrature Index flag set"]
    #[inline(always)]
    #[must_use]
    pub fn sindx(&mut self) -> SINDX_W<SPFLG_SPEC, 8> {
        SINDX_W::new(self)
    }
    #[doc = "Bit 9 - Quadrature Phase Error flag set"]
    #[inline(always)]
    #[must_use]
    pub fn serr(&mut self) -> SERR_W<SPFLG_SPEC, 9> {
        SERR_W::new(self)
    }
    #[doc = "Bit 10 - Quadrature CLK flag set"]
    #[inline(always)]
    #[must_use]
    pub fn scnt(&mut self) -> SCNT_W<SPFLG_SPEC, 10> {
        SCNT_W::new(self)
    }
    #[doc = "Bit 11 - Quadrature Direction flag set"]
    #[inline(always)]
    #[must_use]
    pub fn sdir(&mut self) -> SDIR_W<SPFLG_SPEC, 11> {
        SDIR_W::new(self)
    }
    #[doc = "Bit 12 - Quadrature period clock flag set"]
    #[inline(always)]
    #[must_use]
    pub fn spclk(&mut self) -> SPCLK_W<SPFLG_SPEC, 12> {
        SPCLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "POSIF Interrupt Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spflg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPFLG_SPEC;
impl crate::RegisterSpec for SPFLG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spflg::W`](W) writer structure"]
impl crate::Writable for SPFLG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPFLG to value 0"]
impl crate::Resettable for SPFLG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
