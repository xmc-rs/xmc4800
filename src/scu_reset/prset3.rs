#[doc = "Register `PRSET3` writer"]
pub struct W(crate::W<PRSET3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSET3_SPEC>;
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
impl From<crate::W<PRSET3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSET3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EBU Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EBURS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<EBURS_AW> for bool {
    #[inline(always)]
    fn from(variant: EBURS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBURS` writer - EBU Reset Assert"]
pub type EBURS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSET3_SPEC, EBURS_AW, O>;
impl<'a, const O: u8> EBURS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EBURS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EBURS_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 2 - EBU Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn eburs(&mut self) -> EBURS_W<2> {
        EBURS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCU Peripheral 3 Reset Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prset3](index.html) module"]
pub struct PRSET3_SPEC;
impl crate::RegisterSpec for PRSET3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [prset3::W](W) writer structure"]
impl crate::Writable for PRSET3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSET3 to value 0"]
impl crate::Resettable for PRSET3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
