#[doc = "Register `PRUNC` writer"]
pub struct W(crate::W<PRUNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRUNC_SPEC>;
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
impl From<crate::W<PRUNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRUNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRB` writer - Clear Run bit"]
pub struct CRB_W<'a> {
    w: &'a mut W,
}
impl<'a> CRB_W<'a> {
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
#[doc = "Field `CSM` writer - Clear Current internal status"]
pub struct CSM_W<'a> {
    w: &'a mut W,
}
impl<'a> CSM_W<'a> {
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
    #[doc = "Bit 0 - Clear Run bit"]
    #[inline(always)]
    pub fn crb(&mut self) -> CRB_W {
        CRB_W { w: self }
    }
    #[doc = "Bit 1 - Clear Current internal status"]
    #[inline(always)]
    pub fn csm(&mut self) -> CSM_W {
        CSM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "POSIF Run Bit Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prunc](index.html) module"]
pub struct PRUNC_SPEC;
impl crate::RegisterSpec for PRUNC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [prunc::W](W) writer structure"]
impl crate::Writable for PRUNC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRUNC to value 0"]
impl crate::Resettable for PRUNC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
