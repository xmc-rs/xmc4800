#[doc = "Register `MCMS` writer"]
pub struct W(crate::W<MCMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCMS_SPEC>;
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
impl From<crate::W<MCMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MNPS` writer - Multi-Channel Pattern Update Enable Set"]
pub type MNPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCMS_SPEC, bool, O>;
#[doc = "Field `STHR` writer - Hall Pattern Shadow Transfer Request"]
pub type STHR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCMS_SPEC, bool, O>;
#[doc = "Field `STMR` writer - Multi-Channel Shadow Transfer Request"]
pub type STMR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCMS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Multi-Channel Pattern Update Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn mnps(&mut self) -> MNPS_W<0> {
        MNPS_W::new(self)
    }
    #[doc = "Bit 1 - Hall Pattern Shadow Transfer Request"]
    #[inline(always)]
    #[must_use]
    pub fn sthr(&mut self) -> STHR_W<1> {
        STHR_W::new(self)
    }
    #[doc = "Bit 2 - Multi-Channel Shadow Transfer Request"]
    #[inline(always)]
    #[must_use]
    pub fn stmr(&mut self) -> STMR_W<2> {
        STMR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Multi-Channel Pattern Control set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcms](index.html) module"]
pub struct MCMS_SPEC;
impl crate::RegisterSpec for MCMS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mcms::W](W) writer structure"]
impl crate::Writable for MCMS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCMS to value 0"]
impl crate::Resettable for MCMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
