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
pub struct MNPC_W<'a> {
    w: &'a mut W,
}
impl<'a> MNPC_W<'a> {
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
#[doc = "Field `MPC` writer - Multi-Channel Pattern clear"]
pub struct MPC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPC_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Multi-Channel Pattern Update Enable Clear"]
    #[inline(always)]
    pub fn mnpc(&mut self) -> MNPC_W {
        MNPC_W { w: self }
    }
    #[doc = "Bit 1 - Multi-Channel Pattern clear"]
    #[inline(always)]
    pub fn mpc(&mut self) -> MPC_W {
        MPC_W { w: self }
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
}
#[doc = "`reset()` method sets MCMC to value 0"]
impl crate::Resettable for MCMC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
