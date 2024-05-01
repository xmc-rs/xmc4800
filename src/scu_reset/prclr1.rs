#[doc = "Register `PRCLR1` writer"]
pub type W = crate::W<Prclr1Spec>;
#[doc = "CCU43 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu43rs {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: De-assert reset"]
    Value2 = 1,
}
impl From<Ccu43rs> for bool {
    #[inline(always)]
    fn from(variant: Ccu43rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU43RS` writer - CCU43 Reset Clear"]
pub type Ccu43rsW<'a, REG> = crate::BitWriter<'a, REG, Ccu43rs>;
impl<'a, REG> Ccu43rsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu43rs::Value1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu43rs::Value2)
    }
}
#[doc = "LEDTS Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ledtscu0rs {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: De-assert reset"]
    Value2 = 1,
}
impl From<Ledtscu0rs> for bool {
    #[inline(always)]
    fn from(variant: Ledtscu0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDTSCU0RS` writer - LEDTS Reset Clear"]
pub type Ledtscu0rsW<'a, REG> = crate::BitWriter<'a, REG, Ledtscu0rs>;
impl<'a, REG> Ledtscu0rsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ledtscu0rs::Value1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ledtscu0rs::Value2)
    }
}
#[doc = "MultiCAN Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mcan0rs {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: De-assert reset"]
    Value2 = 1,
}
impl From<Mcan0rs> for bool {
    #[inline(always)]
    fn from(variant: Mcan0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCAN0RS` writer - MultiCAN Reset Clear"]
pub type Mcan0rsW<'a, REG> = crate::BitWriter<'a, REG, Mcan0rs>;
impl<'a, REG> Mcan0rsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mcan0rs::Value1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mcan0rs::Value2)
    }
}
#[doc = "DAC Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacrs {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: De-assert reset"]
    Value2 = 1,
}
impl From<Dacrs> for bool {
    #[inline(always)]
    fn from(variant: Dacrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACRS` writer - DAC Reset Clear"]
pub type DacrsW<'a, REG> = crate::BitWriter<'a, REG, Dacrs>;
impl<'a, REG> DacrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dacrs::Value1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dacrs::Value2)
    }
}
#[doc = "MMC Interface Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmcirs {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: De-assert reset"]
    Value2 = 1,
}
impl From<Mmcirs> for bool {
    #[inline(always)]
    fn from(variant: Mmcirs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCIRS` writer - MMC Interface Reset Clear"]
pub type MmcirsW<'a, REG> = crate::BitWriter<'a, REG, Mmcirs>;
impl<'a, REG> MmcirsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mmcirs::Value1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mmcirs::Value2)
    }
}
#[doc = "USIC1 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usic1rs {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: De-assert reset"]
    Value2 = 1,
}
impl From<Usic1rs> for bool {
    #[inline(always)]
    fn from(variant: Usic1rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1RS` writer - USIC1 Reset Clear"]
pub type Usic1rsW<'a, REG> = crate::BitWriter<'a, REG, Usic1rs>;
impl<'a, REG> Usic1rsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Usic1rs::Value1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Usic1rs::Value2)
    }
}
#[doc = "USIC2 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usic2rs {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: De-assert reset"]
    Value2 = 1,
}
impl From<Usic2rs> for bool {
    #[inline(always)]
    fn from(variant: Usic2rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC2RS` writer - USIC2 Reset Clear"]
pub type Usic2rsW<'a, REG> = crate::BitWriter<'a, REG, Usic2rs>;
impl<'a, REG> Usic2rsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Usic2rs::Value1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Usic2rs::Value2)
    }
}
#[doc = "PORTS Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pportsrs {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: De-assert reset"]
    Value2 = 1,
}
impl From<Pportsrs> for bool {
    #[inline(always)]
    fn from(variant: Pportsrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPORTSRS` writer - PORTS Reset Clear"]
pub type PportsrsW<'a, REG> = crate::BitWriter<'a, REG, Pportsrs>;
impl<'a, REG> PportsrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pportsrs::Value1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pportsrs::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - CCU43 Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccu43rs(&mut self) -> Ccu43rsW<Prclr1Spec> {
        Ccu43rsW::new(self, 0)
    }
    #[doc = "Bit 3 - LEDTS Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ledtscu0rs(&mut self) -> Ledtscu0rsW<Prclr1Spec> {
        Ledtscu0rsW::new(self, 3)
    }
    #[doc = "Bit 4 - MultiCAN Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn mcan0rs(&mut self) -> Mcan0rsW<Prclr1Spec> {
        Mcan0rsW::new(self, 4)
    }
    #[doc = "Bit 5 - DAC Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dacrs(&mut self) -> DacrsW<Prclr1Spec> {
        DacrsW::new(self, 5)
    }
    #[doc = "Bit 6 - MMC Interface Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn mmcirs(&mut self) -> MmcirsW<Prclr1Spec> {
        MmcirsW::new(self, 6)
    }
    #[doc = "Bit 7 - USIC1 Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usic1rs(&mut self) -> Usic1rsW<Prclr1Spec> {
        Usic1rsW::new(self, 7)
    }
    #[doc = "Bit 8 - USIC2 Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usic2rs(&mut self) -> Usic2rsW<Prclr1Spec> {
        Usic2rsW::new(self, 8)
    }
    #[doc = "Bit 9 - PORTS Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pportsrs(&mut self) -> PportsrsW<Prclr1Spec> {
        PportsrsW::new(self, 9)
    }
}
#[doc = "RCU Peripheral 1 Reset Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prclr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prclr1Spec;
impl crate::RegisterSpec for Prclr1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prclr1::W`](W) writer structure"]
impl crate::Writable for Prclr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRCLR1 to value 0"]
impl crate::Resettable for Prclr1Spec {
    const RESET_VALUE: u32 = 0;
}
