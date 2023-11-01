#[doc = "Register `CGATCLR2` writer"]
pub type W = crate::W<CGATCLR2_SPEC>;
#[doc = "WDT Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDT_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<WDT_AW> for bool {
    #[inline(always)]
    fn from(variant: WDT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT` writer - WDT Gating Clear"]
pub type WDT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WDT_AW>;
impl<'a, REG, const O: u8> WDT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_AW::VALUE2)
    }
}
#[doc = "ETH0 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH0_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<ETH0_AW> for bool {
    #[inline(always)]
    fn from(variant: ETH0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0` writer - ETH0 Gating Clear"]
pub type ETH0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ETH0_AW>;
impl<'a, REG, const O: u8> ETH0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0_AW::VALUE2)
    }
}
#[doc = "DMA0 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA0_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<DMA0_AW> for bool {
    #[inline(always)]
    fn from(variant: DMA0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA0` writer - DMA0 Gating Clear"]
pub type DMA0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA0_AW>;
impl<'a, REG, const O: u8> DMA0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0_AW::VALUE2)
    }
}
#[doc = "DMA1 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<DMA1_AW> for bool {
    #[inline(always)]
    fn from(variant: DMA1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1` writer - DMA1 Gating Clear"]
pub type DMA1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA1_AW>;
impl<'a, REG, const O: u8> DMA1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1_AW::VALUE2)
    }
}
#[doc = "FCE Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCE_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<FCE_AW> for bool {
    #[inline(always)]
    fn from(variant: FCE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCE` writer - FCE Gating Clear"]
pub type FCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FCE_AW>;
impl<'a, REG, const O: u8> FCE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FCE_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FCE_AW::VALUE2)
    }
}
#[doc = "USB Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USB_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<USB_AW> for bool {
    #[inline(always)]
    fn from(variant: USB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB` writer - USB Gating Clear"]
pub type USB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USB_AW>;
impl<'a, REG, const O: u8> USB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USB_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USB_AW::VALUE2)
    }
}
#[doc = "ECAT0 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECAT0_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<ECAT0_AW> for bool {
    #[inline(always)]
    fn from(variant: ECAT0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECAT0` writer - ECAT0 Gating Clear"]
pub type ECAT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ECAT0_AW>;
impl<'a, REG, const O: u8> ECAT0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ECAT0_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ECAT0_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 1 - WDT Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn wdt(&mut self) -> WDT_W<CGATCLR2_SPEC, 1> {
        WDT_W::new(self)
    }
    #[doc = "Bit 2 - ETH0 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eth0(&mut self) -> ETH0_W<CGATCLR2_SPEC, 2> {
        ETH0_W::new(self)
    }
    #[doc = "Bit 4 - DMA0 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dma0(&mut self) -> DMA0_W<CGATCLR2_SPEC, 4> {
        DMA0_W::new(self)
    }
    #[doc = "Bit 5 - DMA1 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dma1(&mut self) -> DMA1_W<CGATCLR2_SPEC, 5> {
        DMA1_W::new(self)
    }
    #[doc = "Bit 6 - FCE Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fce(&mut self) -> FCE_W<CGATCLR2_SPEC, 6> {
        FCE_W::new(self)
    }
    #[doc = "Bit 7 - USB Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> USB_W<CGATCLR2_SPEC, 7> {
        USB_W::new(self)
    }
    #[doc = "Bit 10 - ECAT0 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ecat0(&mut self) -> ECAT0_W<CGATCLR2_SPEC, 10> {
        ECAT0_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral 2 Clock Gating Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatclr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CGATCLR2_SPEC;
impl crate::RegisterSpec for CGATCLR2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cgatclr2::W`](W) writer structure"]
impl crate::Writable for CGATCLR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CGATCLR2 to value 0"]
impl crate::Resettable for CGATCLR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
