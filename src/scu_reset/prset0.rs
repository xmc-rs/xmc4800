#[doc = "Register `PRSET0` writer"]
pub struct W(crate::W<PRSET0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSET0_SPEC>;
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
impl From<crate::W<PRSET0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSET0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "VADC Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VADCRS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<VADCRS_AW> for bool {
    #[inline(always)]
    fn from(variant: VADCRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VADCRS` writer - VADC Reset Assert"]
pub type VADCRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSET0_SPEC, VADCRS_AW, O>;
impl<'a, const O: u8> VADCRS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VADCRS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VADCRS_AW::VALUE2)
    }
}
#[doc = "DSD Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSDRS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<DSDRS_AW> for bool {
    #[inline(always)]
    fn from(variant: DSDRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSDRS` writer - DSD Reset Assert"]
pub type DSDRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSET0_SPEC, DSDRS_AW, O>;
impl<'a, const O: u8> DSDRS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DSDRS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DSDRS_AW::VALUE2)
    }
}
#[doc = "CCU40 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU40RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<CCU40RS_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU40RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU40RS` writer - CCU40 Reset Assert"]
pub type CCU40RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSET0_SPEC, CCU40RS_AW, O>;
impl<'a, const O: u8> CCU40RS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU40RS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU40RS_AW::VALUE2)
    }
}
#[doc = "CCU41 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU41RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<CCU41RS_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU41RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU41RS` writer - CCU41 Reset Assert"]
pub type CCU41RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSET0_SPEC, CCU41RS_AW, O>;
impl<'a, const O: u8> CCU41RS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU41RS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU41RS_AW::VALUE2)
    }
}
#[doc = "CCU42 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU42RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<CCU42RS_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU42RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU42RS` writer - CCU42 Reset Assert"]
pub type CCU42RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSET0_SPEC, CCU42RS_AW, O>;
impl<'a, const O: u8> CCU42RS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU42RS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU42RS_AW::VALUE2)
    }
}
#[doc = "CCU80 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU80RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<CCU80RS_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU80RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU80RS` writer - CCU80 Reset Assert"]
pub type CCU80RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSET0_SPEC, CCU80RS_AW, O>;
impl<'a, const O: u8> CCU80RS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU80RS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU80RS_AW::VALUE2)
    }
}
#[doc = "CCU81 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU81RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<CCU81RS_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU81RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU81RS` writer - CCU81 Reset Assert"]
pub type CCU81RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSET0_SPEC, CCU81RS_AW, O>;
impl<'a, const O: u8> CCU81RS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU81RS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU81RS_AW::VALUE2)
    }
}
#[doc = "POSIF0 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POSIF0RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<POSIF0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: POSIF0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSIF0RS` writer - POSIF0 Reset Assert"]
pub type POSIF0RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSET0_SPEC, POSIF0RS_AW, O>;
impl<'a, const O: u8> POSIF0RS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(POSIF0RS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(POSIF0RS_AW::VALUE2)
    }
}
#[doc = "POSIF1 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POSIF1RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<POSIF1RS_AW> for bool {
    #[inline(always)]
    fn from(variant: POSIF1RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSIF1RS` writer - POSIF1 Reset Assert"]
pub type POSIF1RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSET0_SPEC, POSIF1RS_AW, O>;
impl<'a, const O: u8> POSIF1RS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(POSIF1RS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(POSIF1RS_AW::VALUE2)
    }
}
#[doc = "USIC0 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC0RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<USIC0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: USIC0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC0RS` writer - USIC0 Reset Assert"]
pub type USIC0RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSET0_SPEC, USIC0RS_AW, O>;
impl<'a, const O: u8> USIC0RS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USIC0RS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USIC0RS_AW::VALUE2)
    }
}
#[doc = "ERU1 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERU1RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<ERU1RS_AW> for bool {
    #[inline(always)]
    fn from(variant: ERU1RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU1RS` writer - ERU1 Reset Assert"]
pub type ERU1RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSET0_SPEC, ERU1RS_AW, O>;
impl<'a, const O: u8> ERU1RS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ERU1RS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ERU1RS_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - VADC Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn vadcrs(&mut self) -> VADCRS_W<0> {
        VADCRS_W::new(self)
    }
    #[doc = "Bit 1 - DSD Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn dsdrs(&mut self) -> DSDRS_W<1> {
        DSDRS_W::new(self)
    }
    #[doc = "Bit 2 - CCU40 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn ccu40rs(&mut self) -> CCU40RS_W<2> {
        CCU40RS_W::new(self)
    }
    #[doc = "Bit 3 - CCU41 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn ccu41rs(&mut self) -> CCU41RS_W<3> {
        CCU41RS_W::new(self)
    }
    #[doc = "Bit 4 - CCU42 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn ccu42rs(&mut self) -> CCU42RS_W<4> {
        CCU42RS_W::new(self)
    }
    #[doc = "Bit 7 - CCU80 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn ccu80rs(&mut self) -> CCU80RS_W<7> {
        CCU80RS_W::new(self)
    }
    #[doc = "Bit 8 - CCU81 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn ccu81rs(&mut self) -> CCU81RS_W<8> {
        CCU81RS_W::new(self)
    }
    #[doc = "Bit 9 - POSIF0 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn posif0rs(&mut self) -> POSIF0RS_W<9> {
        POSIF0RS_W::new(self)
    }
    #[doc = "Bit 10 - POSIF1 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn posif1rs(&mut self) -> POSIF1RS_W<10> {
        POSIF1RS_W::new(self)
    }
    #[doc = "Bit 11 - USIC0 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn usic0rs(&mut self) -> USIC0RS_W<11> {
        USIC0RS_W::new(self)
    }
    #[doc = "Bit 16 - ERU1 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn eru1rs(&mut self) -> ERU1RS_W<16> {
        ERU1RS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCU Peripheral 0 Reset Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prset0](index.html) module"]
pub struct PRSET0_SPEC;
impl crate::RegisterSpec for PRSET0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [prset0::W](W) writer structure"]
impl crate::Writable for PRSET0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSET0 to value 0"]
impl crate::Resettable for PRSET0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
