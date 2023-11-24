#[doc = "Register `CGATSET1` writer"]
pub type W = crate::W<CGATSET1_SPEC>;
#[doc = "CCU43 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU43_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<CCU43_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU43_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU43` writer - CCU43 Gating Set"]
pub type CCU43_W<'a, REG> = crate::BitWriter<'a, REG, CCU43_AW>;
impl<'a, REG> CCU43_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCU43_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCU43_AW::VALUE2)
    }
}
#[doc = "LEDTS Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEDTSCU0_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<LEDTSCU0_AW> for bool {
    #[inline(always)]
    fn from(variant: LEDTSCU0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDTSCU0` writer - LEDTS Gating Set"]
pub type LEDTSCU0_W<'a, REG> = crate::BitWriter<'a, REG, LEDTSCU0_AW>;
impl<'a, REG> LEDTSCU0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LEDTSCU0_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LEDTSCU0_AW::VALUE2)
    }
}
#[doc = "MultiCAN Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCAN0_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<MCAN0_AW> for bool {
    #[inline(always)]
    fn from(variant: MCAN0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCAN0` writer - MultiCAN Gating Set"]
pub type MCAN0_W<'a, REG> = crate::BitWriter<'a, REG, MCAN0_AW>;
impl<'a, REG> MCAN0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MCAN0_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MCAN0_AW::VALUE2)
    }
}
#[doc = "DAC Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<DAC_AW> for bool {
    #[inline(always)]
    fn from(variant: DAC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC` writer - DAC Gating Set"]
pub type DAC_W<'a, REG> = crate::BitWriter<'a, REG, DAC_AW>;
impl<'a, REG> DAC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_AW::VALUE2)
    }
}
#[doc = "MMC Interface Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMCI_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<MMCI_AW> for bool {
    #[inline(always)]
    fn from(variant: MMCI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCI` writer - MMC Interface Gating Set"]
pub type MMCI_W<'a, REG> = crate::BitWriter<'a, REG, MMCI_AW>;
impl<'a, REG> MMCI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MMCI_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MMCI_AW::VALUE2)
    }
}
#[doc = "USIC1 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC1_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<USIC1_AW> for bool {
    #[inline(always)]
    fn from(variant: USIC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1` writer - USIC1 Gating Set"]
pub type USIC1_W<'a, REG> = crate::BitWriter<'a, REG, USIC1_AW>;
impl<'a, REG> USIC1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USIC1_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USIC1_AW::VALUE2)
    }
}
#[doc = "USIC2 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC2_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<USIC2_AW> for bool {
    #[inline(always)]
    fn from(variant: USIC2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC2` writer - USIC2 Gating Set"]
pub type USIC2_W<'a, REG> = crate::BitWriter<'a, REG, USIC2_AW>;
impl<'a, REG> USIC2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USIC2_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USIC2_AW::VALUE2)
    }
}
#[doc = "PORTS Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPORTS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<PPORTS_AW> for bool {
    #[inline(always)]
    fn from(variant: PPORTS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPORTS` writer - PORTS Gating Set"]
pub type PPORTS_W<'a, REG> = crate::BitWriter<'a, REG, PPORTS_AW>;
impl<'a, REG> PPORTS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PPORTS_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PPORTS_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - CCU43 Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn ccu43(&mut self) -> CCU43_W<CGATSET1_SPEC> {
        CCU43_W::new(self, 0)
    }
    #[doc = "Bit 3 - LEDTS Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn ledtscu0(&mut self) -> LEDTSCU0_W<CGATSET1_SPEC> {
        LEDTSCU0_W::new(self, 3)
    }
    #[doc = "Bit 4 - MultiCAN Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn mcan0(&mut self) -> MCAN0_W<CGATSET1_SPEC> {
        MCAN0_W::new(self, 4)
    }
    #[doc = "Bit 5 - DAC Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn dac(&mut self) -> DAC_W<CGATSET1_SPEC> {
        DAC_W::new(self, 5)
    }
    #[doc = "Bit 6 - MMC Interface Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn mmci(&mut self) -> MMCI_W<CGATSET1_SPEC> {
        MMCI_W::new(self, 6)
    }
    #[doc = "Bit 7 - USIC1 Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn usic1(&mut self) -> USIC1_W<CGATSET1_SPEC> {
        USIC1_W::new(self, 7)
    }
    #[doc = "Bit 8 - USIC2 Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn usic2(&mut self) -> USIC2_W<CGATSET1_SPEC> {
        USIC2_W::new(self, 8)
    }
    #[doc = "Bit 9 - PORTS Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn pports(&mut self) -> PPORTS_W<CGATSET1_SPEC> {
        PPORTS_W::new(self, 9)
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
#[doc = "Peripheral 1 Clock Gating Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatset1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CGATSET1_SPEC;
impl crate::RegisterSpec for CGATSET1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cgatset1::W`](W) writer structure"]
impl crate::Writable for CGATSET1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CGATSET1 to value 0"]
impl crate::Resettable for CGATSET1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
