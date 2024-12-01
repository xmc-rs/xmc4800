#[doc = "Register `PRSET2` writer"]
pub type W = crate::W<PRSET2_SPEC>;
#[doc = "WDT Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTRS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<WDTRS_A> for bool {
    #[inline(always)]
    fn from(variant: WDTRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTRS` writer - WDT Reset Assert"]
pub type WDTRS_W<'a, REG> = crate::BitWriter<'a, REG, WDTRS_A>;
impl<'a, REG> WDTRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTRS_A::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WDTRS_A::VALUE2)
    }
}
#[doc = "ETH0 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH0RS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<ETH0RS_A> for bool {
    #[inline(always)]
    fn from(variant: ETH0RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0RS` writer - ETH0 Reset Assert"]
pub type ETH0RS_W<'a, REG> = crate::BitWriter<'a, REG, ETH0RS_A>;
impl<'a, REG> ETH0RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0RS_A::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0RS_A::VALUE2)
    }
}
#[doc = "DMA0 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA0RS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<DMA0RS_A> for bool {
    #[inline(always)]
    fn from(variant: DMA0RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA0RS` writer - DMA0 Reset Assert"]
pub type DMA0RS_W<'a, REG> = crate::BitWriter<'a, REG, DMA0RS_A>;
impl<'a, REG> DMA0RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RS_A::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RS_A::VALUE2)
    }
}
#[doc = "DMA1 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1RS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<DMA1RS_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1RS` writer - DMA1 Reset Assert"]
pub type DMA1RS_W<'a, REG> = crate::BitWriter<'a, REG, DMA1RS_A>;
impl<'a, REG> DMA1RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RS_A::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RS_A::VALUE2)
    }
}
#[doc = "FCE Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCERS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<FCERS_A> for bool {
    #[inline(always)]
    fn from(variant: FCERS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCERS` writer - FCE Reset Assert"]
pub type FCERS_W<'a, REG> = crate::BitWriter<'a, REG, FCERS_A>;
impl<'a, REG> FCERS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FCERS_A::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FCERS_A::VALUE2)
    }
}
#[doc = "USB Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBRS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<USBRS_A> for bool {
    #[inline(always)]
    fn from(variant: USBRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRS` writer - USB Reset Assert"]
pub type USBRS_W<'a, REG> = crate::BitWriter<'a, REG, USBRS_A>;
impl<'a, REG> USBRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USBRS_A::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USBRS_A::VALUE2)
    }
}
#[doc = "ECAT0 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECAT0RS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<ECAT0RS_A> for bool {
    #[inline(always)]
    fn from(variant: ECAT0RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECAT0RS` writer - ECAT0 Reset Assert"]
pub type ECAT0RS_W<'a, REG> = crate::BitWriter<'a, REG, ECAT0RS_A>;
impl<'a, REG> ECAT0RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ECAT0RS_A::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ECAT0RS_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 1 - WDT Reset Assert"]
    #[inline(always)]
    pub fn wdtrs(&mut self) -> WDTRS_W<PRSET2_SPEC> {
        WDTRS_W::new(self, 1)
    }
    #[doc = "Bit 2 - ETH0 Reset Assert"]
    #[inline(always)]
    pub fn eth0rs(&mut self) -> ETH0RS_W<PRSET2_SPEC> {
        ETH0RS_W::new(self, 2)
    }
    #[doc = "Bit 4 - DMA0 Reset Assert"]
    #[inline(always)]
    pub fn dma0rs(&mut self) -> DMA0RS_W<PRSET2_SPEC> {
        DMA0RS_W::new(self, 4)
    }
    #[doc = "Bit 5 - DMA1 Reset Assert"]
    #[inline(always)]
    pub fn dma1rs(&mut self) -> DMA1RS_W<PRSET2_SPEC> {
        DMA1RS_W::new(self, 5)
    }
    #[doc = "Bit 6 - FCE Reset Assert"]
    #[inline(always)]
    pub fn fcers(&mut self) -> FCERS_W<PRSET2_SPEC> {
        FCERS_W::new(self, 6)
    }
    #[doc = "Bit 7 - USB Reset Assert"]
    #[inline(always)]
    pub fn usbrs(&mut self) -> USBRS_W<PRSET2_SPEC> {
        USBRS_W::new(self, 7)
    }
    #[doc = "Bit 10 - ECAT0 Reset Assert"]
    #[inline(always)]
    pub fn ecat0rs(&mut self) -> ECAT0RS_W<PRSET2_SPEC> {
        ECAT0RS_W::new(self, 10)
    }
}
#[doc = "RCU Peripheral 2 Reset Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prset2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRSET2_SPEC;
impl crate::RegisterSpec for PRSET2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prset2::W`](W) writer structure"]
impl crate::Writable for PRSET2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRSET2 to value 0"]
impl crate::Resettable for PRSET2_SPEC {
    const RESET_VALUE: u32 = 0;
}
