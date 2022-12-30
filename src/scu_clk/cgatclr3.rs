#[doc = "Register `CGATCLR3` writer"]
pub struct W(crate::W<CGATCLR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGATCLR3_SPEC>;
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
impl From<crate::W<CGATCLR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGATCLR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EBU Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EBU_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<EBU_AW> for bool {
    #[inline(always)]
    fn from(variant: EBU_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBU` writer - EBU Gating Clear"]
pub type EBU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGATCLR3_SPEC, EBU_AW, O>;
impl<'a, const O: u8> EBU_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EBU_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EBU_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 2 - EBU Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ebu(&mut self) -> EBU_W<2> {
        EBU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral 3 Clock Gating Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgatclr3](index.html) module"]
pub struct CGATCLR3_SPEC;
impl crate::RegisterSpec for CGATCLR3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cgatclr3::W](W) writer structure"]
impl crate::Writable for CGATCLR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CGATCLR3 to value 0"]
impl crate::Resettable for CGATCLR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
