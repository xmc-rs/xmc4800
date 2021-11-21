#[doc = "Register `SPFLG` writer"]
pub struct W(crate::W<SPFLG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPFLG_SPEC>;
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
impl From<crate::W<SPFLG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPFLG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCHE` writer - Correct Hall Event flag set"]
pub struct SCHE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHE_W<'a> {
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
#[doc = "Field `SWHE` writer - Wrong Hall Event flag set"]
pub struct SWHE_W<'a> {
    w: &'a mut W,
}
impl<'a> SWHE_W<'a> {
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
#[doc = "Field `SHIE` writer - Hall Inputs Update Event flag set"]
pub struct SHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIE_W<'a> {
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
#[doc = "Field `SMST` writer - Multi-Channel Pattern shadow transfer flag set"]
pub struct SMST_W<'a> {
    w: &'a mut W,
}
impl<'a> SMST_W<'a> {
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
#[doc = "Field `SINDX` writer - Quadrature Index flag set"]
pub struct SINDX_W<'a> {
    w: &'a mut W,
}
impl<'a> SINDX_W<'a> {
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
#[doc = "Field `SERR` writer - Quadrature Phase Error flag set"]
pub struct SERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SERR_W<'a> {
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
#[doc = "Field `SCNT` writer - Quadrature CLK flag set"]
pub struct SCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCNT_W<'a> {
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
#[doc = "Field `SDIR` writer - Quadrature Direction flag set"]
pub struct SDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIR_W<'a> {
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
#[doc = "Field `SPCLK` writer - Quadrature period clock flag set"]
pub struct SPCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SPCLK_W<'a> {
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
    #[doc = "Bit 0 - Correct Hall Event flag set"]
    #[inline(always)]
    pub fn sche(&mut self) -> SCHE_W {
        SCHE_W { w: self }
    }
    #[doc = "Bit 1 - Wrong Hall Event flag set"]
    #[inline(always)]
    pub fn swhe(&mut self) -> SWHE_W {
        SWHE_W { w: self }
    }
    #[doc = "Bit 2 - Hall Inputs Update Event flag set"]
    #[inline(always)]
    pub fn shie(&mut self) -> SHIE_W {
        SHIE_W { w: self }
    }
    #[doc = "Bit 4 - Multi-Channel Pattern shadow transfer flag set"]
    #[inline(always)]
    pub fn smst(&mut self) -> SMST_W {
        SMST_W { w: self }
    }
    #[doc = "Bit 8 - Quadrature Index flag set"]
    #[inline(always)]
    pub fn sindx(&mut self) -> SINDX_W {
        SINDX_W { w: self }
    }
    #[doc = "Bit 9 - Quadrature Phase Error flag set"]
    #[inline(always)]
    pub fn serr(&mut self) -> SERR_W {
        SERR_W { w: self }
    }
    #[doc = "Bit 10 - Quadrature CLK flag set"]
    #[inline(always)]
    pub fn scnt(&mut self) -> SCNT_W {
        SCNT_W { w: self }
    }
    #[doc = "Bit 11 - Quadrature Direction flag set"]
    #[inline(always)]
    pub fn sdir(&mut self) -> SDIR_W {
        SDIR_W { w: self }
    }
    #[doc = "Bit 12 - Quadrature period clock flag set"]
    #[inline(always)]
    pub fn spclk(&mut self) -> SPCLK_W {
        SPCLK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "POSIF Interrupt Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spflg](index.html) module"]
pub struct SPFLG_SPEC;
impl crate::RegisterSpec for SPFLG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [spflg::W](W) writer structure"]
impl crate::Writable for SPFLG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPFLG to value 0"]
impl crate::Resettable for SPFLG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
