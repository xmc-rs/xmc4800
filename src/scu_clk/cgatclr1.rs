#[doc = "Register `CGATCLR1` writer"]
pub type W = crate::W<CGATCLR1_SPEC>;
#[doc = "CCU43 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU43_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<CCU43_A> for bool {
    #[inline(always)]
    fn from(variant: CCU43_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU43` writer - CCU43 Gating Clear"]
pub type CCU43_W<'a, REG> = crate::BitWriter<'a, REG, CCU43_A>;
impl<'a, REG> CCU43_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCU43_A::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCU43_A::VALUE2)
    }
}
#[doc = "LEDTS Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEDTSCU0_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<LEDTSCU0_A> for bool {
    #[inline(always)]
    fn from(variant: LEDTSCU0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDTSCU0` writer - LEDTS Gating Clear"]
pub type LEDTSCU0_W<'a, REG> = crate::BitWriter<'a, REG, LEDTSCU0_A>;
impl<'a, REG> LEDTSCU0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LEDTSCU0_A::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LEDTSCU0_A::VALUE2)
    }
}
#[doc = "MultiCAN Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCAN0_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<MCAN0_A> for bool {
    #[inline(always)]
    fn from(variant: MCAN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCAN0` writer - MultiCAN Gating Clear"]
pub type MCAN0_W<'a, REG> = crate::BitWriter<'a, REG, MCAN0_A>;
impl<'a, REG> MCAN0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MCAN0_A::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MCAN0_A::VALUE2)
    }
}
#[doc = "DAC Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<DAC_A> for bool {
    #[inline(always)]
    fn from(variant: DAC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC` writer - DAC Gating Clear"]
pub type DAC_W<'a, REG> = crate::BitWriter<'a, REG, DAC_A>;
impl<'a, REG> DAC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_A::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_A::VALUE2)
    }
}
#[doc = "MMC Interface Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMCI_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<MMCI_A> for bool {
    #[inline(always)]
    fn from(variant: MMCI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCI` writer - MMC Interface Gating Clear"]
pub type MMCI_W<'a, REG> = crate::BitWriter<'a, REG, MMCI_A>;
impl<'a, REG> MMCI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MMCI_A::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MMCI_A::VALUE2)
    }
}
#[doc = "USIC1 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC1_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<USIC1_A> for bool {
    #[inline(always)]
    fn from(variant: USIC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1` writer - USIC1 Gating Clear"]
pub type USIC1_W<'a, REG> = crate::BitWriter<'a, REG, USIC1_A>;
impl<'a, REG> USIC1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USIC1_A::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USIC1_A::VALUE2)
    }
}
#[doc = "USIC2 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC2_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<USIC2_A> for bool {
    #[inline(always)]
    fn from(variant: USIC2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC2` writer - USIC2 Gating Clear"]
pub type USIC2_W<'a, REG> = crate::BitWriter<'a, REG, USIC2_A>;
impl<'a, REG> USIC2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USIC2_A::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USIC2_A::VALUE2)
    }
}
#[doc = "PORTS Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPORTS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<PPORTS_A> for bool {
    #[inline(always)]
    fn from(variant: PPORTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPORTS` writer - PORTS Gating Clear"]
pub type PPORTS_W<'a, REG> = crate::BitWriter<'a, REG, PPORTS_A>;
impl<'a, REG> PPORTS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PPORTS_A::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PPORTS_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - CCU43 Gating Clear"]
    #[inline(always)]
    pub fn ccu43(&mut self) -> CCU43_W<CGATCLR1_SPEC> {
        CCU43_W::new(self, 0)
    }
    #[doc = "Bit 3 - LEDTS Gating Clear"]
    #[inline(always)]
    pub fn ledtscu0(&mut self) -> LEDTSCU0_W<CGATCLR1_SPEC> {
        LEDTSCU0_W::new(self, 3)
    }
    #[doc = "Bit 4 - MultiCAN Gating Clear"]
    #[inline(always)]
    pub fn mcan0(&mut self) -> MCAN0_W<CGATCLR1_SPEC> {
        MCAN0_W::new(self, 4)
    }
    #[doc = "Bit 5 - DAC Gating Clear"]
    #[inline(always)]
    pub fn dac(&mut self) -> DAC_W<CGATCLR1_SPEC> {
        DAC_W::new(self, 5)
    }
    #[doc = "Bit 6 - MMC Interface Gating Clear"]
    #[inline(always)]
    pub fn mmci(&mut self) -> MMCI_W<CGATCLR1_SPEC> {
        MMCI_W::new(self, 6)
    }
    #[doc = "Bit 7 - USIC1 Gating Clear"]
    #[inline(always)]
    pub fn usic1(&mut self) -> USIC1_W<CGATCLR1_SPEC> {
        USIC1_W::new(self, 7)
    }
    #[doc = "Bit 8 - USIC2 Gating Clear"]
    #[inline(always)]
    pub fn usic2(&mut self) -> USIC2_W<CGATCLR1_SPEC> {
        USIC2_W::new(self, 8)
    }
    #[doc = "Bit 9 - PORTS Gating Clear"]
    #[inline(always)]
    pub fn pports(&mut self) -> PPORTS_W<CGATCLR1_SPEC> {
        PPORTS_W::new(self, 9)
    }
}
#[doc = "Peripheral 1 Clock Gating Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgatclr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CGATCLR1_SPEC;
impl crate::RegisterSpec for CGATCLR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cgatclr1::W`](W) writer structure"]
impl crate::Writable for CGATCLR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CGATCLR1 to value 0"]
impl crate::Resettable for CGATCLR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
