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
pub type CRB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRUNC_SPEC, bool, O>;
#[doc = "Field `CSM` writer - Clear Current internal status"]
pub type CSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRUNC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear Run bit"]
    #[inline(always)]
    #[must_use]
    pub fn crb(&mut self) -> CRB_W<0> {
        CRB_W::new(self)
    }
    #[doc = "Bit 1 - Clear Current internal status"]
    #[inline(always)]
    #[must_use]
    pub fn csm(&mut self) -> CSM_W<1> {
        CSM_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRUNC to value 0"]
impl crate::Resettable for PRUNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
