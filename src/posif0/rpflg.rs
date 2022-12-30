#[doc = "Register `RPFLG` writer"]
pub struct W(crate::W<RPFLG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPFLG_SPEC>;
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
impl From<crate::W<RPFLG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RPFLG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCHE` writer - Correct Hall Event flag clear"]
pub type RCHE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RPFLG_SPEC, bool, O>;
#[doc = "Field `RWHE` writer - Wrong Hall Event flag clear"]
pub type RWHE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RPFLG_SPEC, bool, O>;
#[doc = "Field `RHIE` writer - Hall Inputs Update Event flag clear"]
pub type RHIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RPFLG_SPEC, bool, O>;
#[doc = "Field `RMST` writer - Multi-Channel Pattern shadow transfer flag clear"]
pub type RMST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RPFLG_SPEC, bool, O>;
#[doc = "Field `RINDX` writer - Quadrature Index flag clear"]
pub type RINDX_W<'a, const O: u8> = crate::BitWriter<'a, u32, RPFLG_SPEC, bool, O>;
#[doc = "Field `RERR` writer - Quadrature Phase Error flag clear"]
pub type RERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RPFLG_SPEC, bool, O>;
#[doc = "Field `RCNT` writer - Quadrature CLK flag clear"]
pub type RCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RPFLG_SPEC, bool, O>;
#[doc = "Field `RDIR` writer - Quadrature Direction flag clear"]
pub type RDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RPFLG_SPEC, bool, O>;
#[doc = "Field `RPCLK` writer - Quadrature period clock flag clear"]
pub type RPCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, RPFLG_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Correct Hall Event flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rche(&mut self) -> RCHE_W<0> {
        RCHE_W::new(self)
    }
    #[doc = "Bit 1 - Wrong Hall Event flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rwhe(&mut self) -> RWHE_W<1> {
        RWHE_W::new(self)
    }
    #[doc = "Bit 2 - Hall Inputs Update Event flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rhie(&mut self) -> RHIE_W<2> {
        RHIE_W::new(self)
    }
    #[doc = "Bit 4 - Multi-Channel Pattern shadow transfer flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rmst(&mut self) -> RMST_W<4> {
        RMST_W::new(self)
    }
    #[doc = "Bit 8 - Quadrature Index flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rindx(&mut self) -> RINDX_W<8> {
        RINDX_W::new(self)
    }
    #[doc = "Bit 9 - Quadrature Phase Error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rerr(&mut self) -> RERR_W<9> {
        RERR_W::new(self)
    }
    #[doc = "Bit 10 - Quadrature CLK flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rcnt(&mut self) -> RCNT_W<10> {
        RCNT_W::new(self)
    }
    #[doc = "Bit 11 - Quadrature Direction flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rdir(&mut self) -> RDIR_W<11> {
        RDIR_W::new(self)
    }
    #[doc = "Bit 12 - Quadrature period clock flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rpclk(&mut self) -> RPCLK_W<12> {
        RPCLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "POSIF Interrupt Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpflg](index.html) module"]
pub struct RPFLG_SPEC;
impl crate::RegisterSpec for RPFLG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rpflg::W](W) writer structure"]
impl crate::Writable for RPFLG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RPFLG to value 0"]
impl crate::Resettable for RPFLG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
