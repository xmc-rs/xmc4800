#[doc = "Register `DSLEEPCR` reader"]
pub type R = crate::R<DsleepcrSpec>;
#[doc = "Register `DSLEEPCR` writer"]
pub type W = crate::W<DsleepcrSpec>;
#[doc = "System Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Syssel {
    #[doc = "0: fOFI clock"]
    Value1 = 0,
    #[doc = "1: fPLL clock"]
    Value2 = 1,
}
impl From<Syssel> for u8 {
    #[inline(always)]
    fn from(variant: Syssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Syssel {
    type Ux = u8;
}
#[doc = "Field `SYSSEL` reader - System Clock Selection Value"]
pub type SysselR = crate::FieldReader<Syssel>;
impl SysselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Syssel> {
        match self.bits {
            0 => Some(Syssel::Value1),
            1 => Some(Syssel::Value2),
            _ => None,
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
pub type SysselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Syssel>;
impl<'a, REG> SysselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
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
#[doc = "Flash Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fpdn {
    #[doc = "1: Flash power down module"]
    Value1 = 1,
    #[doc = "0: No effect"]
    Value2 = 0,
}
impl From<Fpdn> for bool {
    #[inline(always)]
    fn from(variant: Fpdn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPDN` reader - Flash Power Down"]
pub type FpdnR = crate::BitReader<Fpdn>;
impl FpdnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fpdn {
        match self.bits {
            true => Fpdn::Value1,
            false => Fpdn::Value2,
        }
    }
    #[doc = "Flash power down module"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fpdn::Value1
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fpdn::Value2
    }
}
#[doc = "Field `FPDN` writer - Flash Power Down"]
pub type FpdnW<'a, REG> = crate::BitWriter<'a, REG, Fpdn>;
impl<'a, REG> FpdnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash power down module"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Fpdn::Value1)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Fpdn::Value2)
    }
}
#[doc = "PLL Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllpdn {
    #[doc = "1: Switch off main PLL"]
    Value1 = 1,
    #[doc = "0: No effect"]
    Value2 = 0,
}
impl From<Pllpdn> for bool {
    #[inline(always)]
    fn from(variant: Pllpdn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLPDN` reader - PLL Power Down"]
pub type PllpdnR = crate::BitReader<Pllpdn>;
impl PllpdnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllpdn {
        match self.bits {
            true => Pllpdn::Value1,
            false => Pllpdn::Value2,
        }
    }
    #[doc = "Switch off main PLL"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pllpdn::Value1
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pllpdn::Value2
    }
}
#[doc = "Field `PLLPDN` writer - PLL Power Down"]
pub type PllpdnW<'a, REG> = crate::BitWriter<'a, REG, Pllpdn>;
impl<'a, REG> PllpdnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Switch off main PLL"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllpdn::Value1)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllpdn::Value2)
    }
}
#[doc = "VCO Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vcopdn {
    #[doc = "1: Switch off VCO of main PLL"]
    Value1 = 1,
    #[doc = "0: No effect"]
    Value2 = 0,
}
impl From<Vcopdn> for bool {
    #[inline(always)]
    fn from(variant: Vcopdn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOPDN` reader - VCO Power Down"]
pub type VcopdnR = crate::BitReader<Vcopdn>;
impl VcopdnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vcopdn {
        match self.bits {
            true => Vcopdn::Value1,
            false => Vcopdn::Value2,
        }
    }
    #[doc = "Switch off VCO of main PLL"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vcopdn::Value1
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vcopdn::Value2
    }
}
#[doc = "Field `VCOPDN` writer - VCO Power Down"]
pub type VcopdnW<'a, REG> = crate::BitWriter<'a, REG, Vcopdn>;
impl<'a, REG> VcopdnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Switch off VCO of main PLL"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vcopdn::Value1)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vcopdn::Value2)
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
    #[doc = "Bits 0:1 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&self) -> SysselR {
        SysselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 11 - Flash Power Down"]
    #[inline(always)]
    pub fn fpdn(&self) -> FpdnR {
        FpdnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL Power Down"]
    #[inline(always)]
    pub fn pllpdn(&self) -> PllpdnR {
        PllpdnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VCO Power Down"]
    #[inline(always)]
    pub fn vcopdn(&self) -> VcopdnR {
        VcopdnR::new(((self.bits >> 13) & 1) != 0)
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
    #[doc = "Bits 0:1 - System Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn syssel(&mut self) -> SysselW<DsleepcrSpec> {
        SysselW::new(self, 0)
    }
    #[doc = "Bit 11 - Flash Power Down"]
    #[inline(always)]
    #[must_use]
    pub fn fpdn(&mut self) -> FpdnW<DsleepcrSpec> {
        FpdnW::new(self, 11)
    }
    #[doc = "Bit 12 - PLL Power Down"]
    #[inline(always)]
    #[must_use]
    pub fn pllpdn(&mut self) -> PllpdnW<DsleepcrSpec> {
        PllpdnW::new(self, 12)
    }
    #[doc = "Bit 13 - VCO Power Down"]
    #[inline(always)]
    #[must_use]
    pub fn vcopdn(&mut self) -> VcopdnW<DsleepcrSpec> {
        VcopdnW::new(self, 13)
    }
    #[doc = "Bit 16 - USB Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn usbcr(&mut self) -> UsbcrW<DsleepcrSpec> {
        UsbcrW::new(self, 16)
    }
    #[doc = "Bit 17 - MMC Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn mmccr(&mut self) -> MmccrW<DsleepcrSpec> {
        MmccrW::new(self, 17)
    }
    #[doc = "Bit 18 - Ethernet Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn eth0cr(&mut self) -> Eth0crW<DsleepcrSpec> {
        Eth0crW::new(self, 18)
    }
    #[doc = "Bit 19 - EBU Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn ebucr(&mut self) -> EbucrW<DsleepcrSpec> {
        EbucrW::new(self, 19)
    }
    #[doc = "Bit 20 - CCU Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn ccucr(&mut self) -> CcucrW<DsleepcrSpec> {
        CcucrW::new(self, 20)
    }
    #[doc = "Bit 21 - WDT Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn wdtcr(&mut self) -> WdtcrW<DsleepcrSpec> {
        WdtcrW::new(self, 21)
    }
}
#[doc = "Deep Sleep Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsleepcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsleepcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsleepcrSpec;
impl crate::RegisterSpec for DsleepcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsleepcr::R`](R) reader structure"]
impl crate::Readable for DsleepcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsleepcr::W`](W) writer structure"]
impl crate::Writable for DsleepcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSLEEPCR to value 0"]
impl crate::Resettable for DsleepcrSpec {
    const RESET_VALUE: u32 = 0;
}
