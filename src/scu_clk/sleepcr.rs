#[doc = "Register `SLEEPCR` reader"]
pub type R = crate::R<SleepcrSpec>;
#[doc = "Register `SLEEPCR` writer"]
pub type W = crate::W<SleepcrSpec>;
#[doc = "System Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syssel {
    #[doc = "0: fOFI clock"]
    Value1 = 0,
    #[doc = "1: fPLL clock"]
    Value2 = 1,
}
impl From<Syssel> for bool {
    #[inline(always)]
    fn from(variant: Syssel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSSEL` reader - System Clock Selection Value"]
pub type SysselR = crate::BitReader<Syssel>;
impl SysselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syssel {
        match self.bits {
            false => Syssel::Value1,
            true => Syssel::Value2,
        }
    }
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Syssel::Value1
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Syssel::Value2
    }
}
#[doc = "Field `SYSSEL` writer - System Clock Selection Value"]
pub type SysselW<'a, REG> = crate::BitWriter<'a, REG, Syssel>;
impl<'a, REG> SysselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Syssel::Value1)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Syssel::Value2)
    }
}
#[doc = "USB Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbcr {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<Usbcr> for bool {
    #[inline(always)]
    fn from(variant: Usbcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCR` reader - USB Clock Control"]
pub type UsbcrR = crate::BitReader<Usbcr>;
impl UsbcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbcr {
        match self.bits {
            false => Usbcr::Value1,
            true => Usbcr::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Usbcr::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Usbcr::Value2
    }
}
#[doc = "Field `USBCR` writer - USB Clock Control"]
pub type UsbcrW<'a, REG> = crate::BitWriter<'a, REG, Usbcr>;
impl<'a, REG> UsbcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbcr::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Usbcr::Value2)
    }
}
#[doc = "MMC Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmccr {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<Mmccr> for bool {
    #[inline(always)]
    fn from(variant: Mmccr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCCR` reader - MMC Clock Control"]
pub type MmccrR = crate::BitReader<Mmccr>;
impl MmccrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mmccr {
        match self.bits {
            false => Mmccr::Value1,
            true => Mmccr::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mmccr::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mmccr::Value2
    }
}
#[doc = "Field `MMCCR` writer - MMC Clock Control"]
pub type MmccrW<'a, REG> = crate::BitWriter<'a, REG, Mmccr>;
impl<'a, REG> MmccrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mmccr::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mmccr::Value2)
    }
}
#[doc = "Ethernet Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eth0cr {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<Eth0cr> for bool {
    #[inline(always)]
    fn from(variant: Eth0cr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0CR` reader - Ethernet Clock Control"]
pub type Eth0crR = crate::BitReader<Eth0cr>;
impl Eth0crR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eth0cr {
        match self.bits {
            false => Eth0cr::Value1,
            true => Eth0cr::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Eth0cr::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Eth0cr::Value2
    }
}
#[doc = "Field `ETH0CR` writer - Ethernet Clock Control"]
pub type Eth0crW<'a, REG> = crate::BitWriter<'a, REG, Eth0cr>;
impl<'a, REG> Eth0crW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Eth0cr::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Eth0cr::Value2)
    }
}
#[doc = "EBU Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ebucr {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<Ebucr> for bool {
    #[inline(always)]
    fn from(variant: Ebucr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBUCR` reader - EBU Clock Control"]
pub type EbucrR = crate::BitReader<Ebucr>;
impl EbucrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ebucr {
        match self.bits {
            false => Ebucr::Value1,
            true => Ebucr::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ebucr::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ebucr::Value2
    }
}
#[doc = "Field `EBUCR` writer - EBU Clock Control"]
pub type EbucrW<'a, REG> = crate::BitWriter<'a, REG, Ebucr>;
impl<'a, REG> EbucrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ebucr::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ebucr::Value2)
    }
}
#[doc = "CCU Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccucr {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<Ccucr> for bool {
    #[inline(always)]
    fn from(variant: Ccucr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCR` reader - CCU Clock Control"]
pub type CcucrR = crate::BitReader<Ccucr>;
impl CcucrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccucr {
        match self.bits {
            false => Ccucr::Value1,
            true => Ccucr::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ccucr::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ccucr::Value2
    }
}
#[doc = "Field `CCUCR` writer - CCU Clock Control"]
pub type CcucrW<'a, REG> = crate::BitWriter<'a, REG, Ccucr>;
impl<'a, REG> CcucrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccucr::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccucr::Value2)
    }
}
#[doc = "WDT Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtcr {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<Wdtcr> for bool {
    #[inline(always)]
    fn from(variant: Wdtcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCR` reader - WDT Clock Control"]
pub type WdtcrR = crate::BitReader<Wdtcr>;
impl WdtcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtcr {
        match self.bits {
            false => Wdtcr::Value1,
            true => Wdtcr::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wdtcr::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wdtcr::Value2
    }
}
#[doc = "Field `WDTCR` writer - WDT Clock Control"]
pub type WdtcrW<'a, REG> = crate::BitWriter<'a, REG, Wdtcr>;
impl<'a, REG> WdtcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtcr::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtcr::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&self) -> SysselR {
        SysselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - USB Clock Control"]
    #[inline(always)]
    pub fn usbcr(&self) -> UsbcrR {
        UsbcrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Clock Control"]
    #[inline(always)]
    pub fn mmccr(&self) -> MmccrR {
        MmccrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Ethernet Clock Control"]
    #[inline(always)]
    pub fn eth0cr(&self) -> Eth0crR {
        Eth0crR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - EBU Clock Control"]
    #[inline(always)]
    pub fn ebucr(&self) -> EbucrR {
        EbucrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CCU Clock Control"]
    #[inline(always)]
    pub fn ccucr(&self) -> CcucrR {
        CcucrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - WDT Clock Control"]
    #[inline(always)]
    pub fn wdtcr(&self) -> WdtcrR {
        WdtcrR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn syssel(&mut self) -> SysselW<SleepcrSpec> {
        SysselW::new(self, 0)
    }
    #[doc = "Bit 16 - USB Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn usbcr(&mut self) -> UsbcrW<SleepcrSpec> {
        UsbcrW::new(self, 16)
    }
    #[doc = "Bit 17 - MMC Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn mmccr(&mut self) -> MmccrW<SleepcrSpec> {
        MmccrW::new(self, 17)
    }
    #[doc = "Bit 18 - Ethernet Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn eth0cr(&mut self) -> Eth0crW<SleepcrSpec> {
        Eth0crW::new(self, 18)
    }
    #[doc = "Bit 19 - EBU Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn ebucr(&mut self) -> EbucrW<SleepcrSpec> {
        EbucrW::new(self, 19)
    }
    #[doc = "Bit 20 - CCU Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn ccucr(&mut self) -> CcucrW<SleepcrSpec> {
        CcucrW::new(self, 20)
    }
    #[doc = "Bit 21 - WDT Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn wdtcr(&mut self) -> WdtcrW<SleepcrSpec> {
        WdtcrW::new(self, 21)
    }
}
#[doc = "Sleep Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleepcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleepcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SleepcrSpec;
impl crate::RegisterSpec for SleepcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleepcr::R`](R) reader structure"]
impl crate::Readable for SleepcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sleepcr::W`](W) writer structure"]
impl crate::Writable for SleepcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLEEPCR to value 0"]
impl crate::Resettable for SleepcrSpec {
    const RESET_VALUE: u32 = 0;
}
