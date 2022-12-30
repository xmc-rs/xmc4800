#[doc = "Register `CLKSET` writer"]
pub struct W(crate::W<CLKSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKSET_SPEC>;
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
impl From<crate::W<CLKSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBCEN_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<USBCEN_AW> for bool {
    #[inline(always)]
    fn from(variant: USBCEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCEN` writer - USB Clock Enable"]
pub type USBCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKSET_SPEC, USBCEN_AW, O>;
impl<'a, const O: u8> USBCEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USBCEN_AW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USBCEN_AW::VALUE2)
    }
}
#[doc = "MMC Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMCCEN_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<MMCCEN_AW> for bool {
    #[inline(always)]
    fn from(variant: MMCCEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCCEN` writer - MMC Clock Enable"]
pub type MMCCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKSET_SPEC, MMCCEN_AW, O>;
impl<'a, const O: u8> MMCCEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MMCCEN_AW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MMCCEN_AW::VALUE2)
    }
}
#[doc = "Ethernet Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH0CEN_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<ETH0CEN_AW> for bool {
    #[inline(always)]
    fn from(variant: ETH0CEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0CEN` writer - Ethernet Clock Enable"]
pub type ETH0CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKSET_SPEC, ETH0CEN_AW, O>;
impl<'a, const O: u8> ETH0CEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ETH0CEN_AW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ETH0CEN_AW::VALUE2)
    }
}
#[doc = "EBU Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EBUCEN_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<EBUCEN_AW> for bool {
    #[inline(always)]
    fn from(variant: EBUCEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBUCEN` writer - EBU Clock Enable"]
pub type EBUCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKSET_SPEC, EBUCEN_AW, O>;
impl<'a, const O: u8> EBUCEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EBUCEN_AW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EBUCEN_AW::VALUE2)
    }
}
#[doc = "CCU Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUCEN_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<CCUCEN_AW> for bool {
    #[inline(always)]
    fn from(variant: CCUCEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCEN` writer - CCU Clock Enable"]
pub type CCUCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKSET_SPEC, CCUCEN_AW, O>;
impl<'a, const O: u8> CCUCEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCUCEN_AW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCUCEN_AW::VALUE2)
    }
}
#[doc = "WDT Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTCEN_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<WDTCEN_AW> for bool {
    #[inline(always)]
    fn from(variant: WDTCEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCEN` writer - WDT Clock Enable"]
pub type WDTCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKSET_SPEC, WDTCEN_AW, O>;
impl<'a, const O: u8> WDTCEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WDTCEN_AW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WDTCEN_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - USB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbcen(&mut self) -> USBCEN_W<0> {
        USBCEN_W::new(self)
    }
    #[doc = "Bit 1 - MMC Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mmccen(&mut self) -> MMCCEN_W<1> {
        MMCCEN_W::new(self)
    }
    #[doc = "Bit 2 - Ethernet Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eth0cen(&mut self) -> ETH0CEN_W<2> {
        ETH0CEN_W::new(self)
    }
    #[doc = "Bit 3 - EBU Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ebucen(&mut self) -> EBUCEN_W<3> {
        EBUCEN_W::new(self)
    }
    #[doc = "Bit 4 - CCU Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccucen(&mut self) -> CCUCEN_W<4> {
        CCUCEN_W::new(self)
    }
    #[doc = "Bit 5 - WDT Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdtcen(&mut self) -> WDTCEN_W<5> {
        WDTCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLK Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkset](index.html) module"]
pub struct CLKSET_SPEC;
impl crate::RegisterSpec for CLKSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clkset::W](W) writer structure"]
impl crate::Writable for CLKSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKSET to value 0"]
impl crate::Resettable for CLKSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
