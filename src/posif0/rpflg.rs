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
pub struct RCHE_W<'a> {
    w: &'a mut W,
}
impl<'a> RCHE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `RWHE` writer - Wrong Hall Event flag clear"]
pub struct RWHE_W<'a> {
    w: &'a mut W,
}
impl<'a> RWHE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RHIE` writer - Hall Inputs Update Event flag clear"]
pub struct RHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RHIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `RMST` writer - Multi-Channel Pattern shadow transfer flag clear"]
pub struct RMST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `RINDX` writer - Quadrature Index flag clear"]
pub struct RINDX_W<'a> {
    w: &'a mut W,
}
impl<'a> RINDX_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `RERR` writer - Quadrature Phase Error flag clear"]
pub struct RERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RERR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `RCNT` writer - Quadrature CLK flag clear"]
pub struct RCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RCNT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `RDIR` writer - Quadrature Direction flag clear"]
pub struct RDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> RDIR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `RPCLK` writer - Quadrature period clock flag clear"]
pub struct RPCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> RPCLK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Correct Hall Event flag clear"]
    #[inline(always)]
    pub fn rche(&mut self) -> RCHE_W {
        RCHE_W { w: self }
    }
    #[doc = "Bit 1 - Wrong Hall Event flag clear"]
    #[inline(always)]
    pub fn rwhe(&mut self) -> RWHE_W {
        RWHE_W { w: self }
    }
    #[doc = "Bit 2 - Hall Inputs Update Event flag clear"]
    #[inline(always)]
    pub fn rhie(&mut self) -> RHIE_W {
        RHIE_W { w: self }
    }
    #[doc = "Bit 4 - Multi-Channel Pattern shadow transfer flag clear"]
    #[inline(always)]
    pub fn rmst(&mut self) -> RMST_W {
        RMST_W { w: self }
    }
    #[doc = "Bit 8 - Quadrature Index flag clear"]
    #[inline(always)]
    pub fn rindx(&mut self) -> RINDX_W {
        RINDX_W { w: self }
    }
    #[doc = "Bit 9 - Quadrature Phase Error flag clear"]
    #[inline(always)]
    pub fn rerr(&mut self) -> RERR_W {
        RERR_W { w: self }
    }
    #[doc = "Bit 10 - Quadrature CLK flag clear"]
    #[inline(always)]
    pub fn rcnt(&mut self) -> RCNT_W {
        RCNT_W { w: self }
    }
    #[doc = "Bit 11 - Quadrature Direction flag clear"]
    #[inline(always)]
    pub fn rdir(&mut self) -> RDIR_W {
        RDIR_W { w: self }
    }
    #[doc = "Bit 12 - Quadrature period clock flag clear"]
    #[inline(always)]
    pub fn rpclk(&mut self) -> RPCLK_W {
        RPCLK_W { w: self }
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
}
#[doc = "`reset()` method sets RPFLG to value 0"]
impl crate::Resettable for RPFLG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
