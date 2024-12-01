#[doc = "Register `CLKCLR` writer"]
pub type W = crate::W<CLKCLR_SPEC>;
#[doc = "USB Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBCDI_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable clock"]
    VALUE2 = 1,
}
impl From<USBCDI_A> for bool {
    #[inline(always)]
    fn from(variant: USBCDI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCDI` writer - USB Clock Disable"]
pub type USBCDI_W<'a, REG> = crate::BitWriter<'a, REG, USBCDI_A>;
impl<'a, REG> USBCDI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USBCDI_A::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USBCDI_A::VALUE2)
    }
}
#[doc = "MMC Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMCCDI_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable clock"]
    VALUE2 = 1,
}
impl From<MMCCDI_A> for bool {
    #[inline(always)]
    fn from(variant: MMCCDI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCCDI` writer - MMC Clock Disable"]
pub type MMCCDI_W<'a, REG> = crate::BitWriter<'a, REG, MMCCDI_A>;
impl<'a, REG> MMCCDI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MMCCDI_A::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MMCCDI_A::VALUE2)
    }
}
#[doc = "Ethernet Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH0CDI_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable clock"]
    VALUE2 = 1,
}
impl From<ETH0CDI_A> for bool {
    #[inline(always)]
    fn from(variant: ETH0CDI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0CDI` writer - Ethernet Clock Disable"]
pub type ETH0CDI_W<'a, REG> = crate::BitWriter<'a, REG, ETH0CDI_A>;
impl<'a, REG> ETH0CDI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0CDI_A::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0CDI_A::VALUE2)
    }
}
#[doc = "EBU Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EBUCDI_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable clock"]
    VALUE2 = 1,
}
impl From<EBUCDI_A> for bool {
    #[inline(always)]
    fn from(variant: EBUCDI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBUCDI` writer - EBU Clock Disable"]
pub type EBUCDI_W<'a, REG> = crate::BitWriter<'a, REG, EBUCDI_A>;
impl<'a, REG> EBUCDI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EBUCDI_A::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EBUCDI_A::VALUE2)
    }
}
#[doc = "CCU Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUCDI_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable clock"]
    VALUE2 = 1,
}
impl From<CCUCDI_A> for bool {
    #[inline(always)]
    fn from(variant: CCUCDI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCDI` writer - CCU Clock Disable"]
pub type CCUCDI_W<'a, REG> = crate::BitWriter<'a, REG, CCUCDI_A>;
impl<'a, REG> CCUCDI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCUCDI_A::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCUCDI_A::VALUE2)
    }
}
#[doc = "WDT Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTCDI_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable clock"]
    VALUE2 = 1,
}
impl From<WDTCDI_A> for bool {
    #[inline(always)]
    fn from(variant: WDTCDI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCDI` writer - WDT Clock Disable"]
pub type WDTCDI_W<'a, REG> = crate::BitWriter<'a, REG, WDTCDI_A>;
impl<'a, REG> WDTCDI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTCDI_A::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WDTCDI_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - USB Clock Disable"]
    #[inline(always)]
    pub fn usbcdi(&mut self) -> USBCDI_W<CLKCLR_SPEC> {
        USBCDI_W::new(self, 0)
    }
    #[doc = "Bit 1 - MMC Clock Disable"]
    #[inline(always)]
    pub fn mmccdi(&mut self) -> MMCCDI_W<CLKCLR_SPEC> {
        MMCCDI_W::new(self, 1)
    }
    #[doc = "Bit 2 - Ethernet Clock Disable"]
    #[inline(always)]
    pub fn eth0cdi(&mut self) -> ETH0CDI_W<CLKCLR_SPEC> {
        ETH0CDI_W::new(self, 2)
    }
    #[doc = "Bit 3 - EBU Clock Disable"]
    #[inline(always)]
    pub fn ebucdi(&mut self) -> EBUCDI_W<CLKCLR_SPEC> {
        EBUCDI_W::new(self, 3)
    }
    #[doc = "Bit 4 - CCU Clock Disable"]
    #[inline(always)]
    pub fn ccucdi(&mut self) -> CCUCDI_W<CLKCLR_SPEC> {
        CCUCDI_W::new(self, 4)
    }
    #[doc = "Bit 5 - WDT Clock Disable"]
    #[inline(always)]
    pub fn wdtcdi(&mut self) -> WDTCDI_W<CLKCLR_SPEC> {
        WDTCDI_W::new(self, 5)
    }
}
#[doc = "CLK Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKCLR_SPEC;
impl crate::RegisterSpec for CLKCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clkclr::W`](W) writer structure"]
impl crate::Writable for CLKCLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCLR to value 0"]
impl crate::Resettable for CLKCLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
