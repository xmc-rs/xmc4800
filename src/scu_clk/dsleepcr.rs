#[doc = "Register `DSLEEPCR` reader"]
pub type R = crate::R<DSLEEPCR_SPEC>;
#[doc = "Register `DSLEEPCR` writer"]
pub type W = crate::W<DSLEEPCR_SPEC>;
#[doc = "System Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSSEL_A {
    #[doc = "0: fOFI clock"]
    VALUE1 = 0,
    #[doc = "1: fPLL clock"]
    VALUE2 = 1,
}
impl From<SYSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for SYSSEL_A {}
#[doc = "Field `SYSSEL` reader - System Clock Selection Value"]
pub type SYSSEL_R = crate::FieldReader<SYSSEL_A>;
impl SYSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSSEL_A> {
        match self.bits {
            0 => Some(SYSSEL_A::VALUE1),
            1 => Some(SYSSEL_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYSSEL_A::VALUE1
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYSSEL_A::VALUE2
    }
}
#[doc = "Field `SYSSEL` writer - System Clock Selection Value"]
pub type SYSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SYSSEL_A>;
impl<'a, REG> SYSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SYSSEL_A::VALUE1)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SYSSEL_A::VALUE2)
    }
}
#[doc = "Flash Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPDN_A {
    #[doc = "1: Flash power down module"]
    VALUE1 = 1,
    #[doc = "0: No effect"]
    VALUE2 = 0,
}
impl From<FPDN_A> for bool {
    #[inline(always)]
    fn from(variant: FPDN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPDN` reader - Flash Power Down"]
pub type FPDN_R = crate::BitReader<FPDN_A>;
impl FPDN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPDN_A {
        match self.bits {
            true => FPDN_A::VALUE1,
            false => FPDN_A::VALUE2,
        }
    }
    #[doc = "Flash power down module"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FPDN_A::VALUE1
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FPDN_A::VALUE2
    }
}
#[doc = "Field `FPDN` writer - Flash Power Down"]
pub type FPDN_W<'a, REG> = crate::BitWriter<'a, REG, FPDN_A>;
impl<'a, REG> FPDN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash power down module"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FPDN_A::VALUE1)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FPDN_A::VALUE2)
    }
}
#[doc = "PLL Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLPDN_A {
    #[doc = "1: Switch off main PLL"]
    VALUE1 = 1,
    #[doc = "0: No effect"]
    VALUE2 = 0,
}
impl From<PLLPDN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLPDN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLPDN` reader - PLL Power Down"]
pub type PLLPDN_R = crate::BitReader<PLLPDN_A>;
impl PLLPDN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLPDN_A {
        match self.bits {
            true => PLLPDN_A::VALUE1,
            false => PLLPDN_A::VALUE2,
        }
    }
    #[doc = "Switch off main PLL"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PLLPDN_A::VALUE1
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PLLPDN_A::VALUE2
    }
}
#[doc = "Field `PLLPDN` writer - PLL Power Down"]
pub type PLLPDN_W<'a, REG> = crate::BitWriter<'a, REG, PLLPDN_A>;
impl<'a, REG> PLLPDN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Switch off main PLL"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDN_A::VALUE1)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDN_A::VALUE2)
    }
}
#[doc = "VCO Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCOPDN_A {
    #[doc = "1: Switch off VCO of main PLL"]
    VALUE1 = 1,
    #[doc = "0: No effect"]
    VALUE2 = 0,
}
impl From<VCOPDN_A> for bool {
    #[inline(always)]
    fn from(variant: VCOPDN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOPDN` reader - VCO Power Down"]
pub type VCOPDN_R = crate::BitReader<VCOPDN_A>;
impl VCOPDN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCOPDN_A {
        match self.bits {
            true => VCOPDN_A::VALUE1,
            false => VCOPDN_A::VALUE2,
        }
    }
    #[doc = "Switch off VCO of main PLL"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VCOPDN_A::VALUE1
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VCOPDN_A::VALUE2
    }
}
#[doc = "Field `VCOPDN` writer - VCO Power Down"]
pub type VCOPDN_W<'a, REG> = crate::BitWriter<'a, REG, VCOPDN_A>;
impl<'a, REG> VCOPDN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Switch off VCO of main PLL"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VCOPDN_A::VALUE1)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VCOPDN_A::VALUE2)
    }
}
#[doc = "USB Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBCR_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<USBCR_A> for bool {
    #[inline(always)]
    fn from(variant: USBCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCR` reader - USB Clock Control"]
pub type USBCR_R = crate::BitReader<USBCR_A>;
impl USBCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBCR_A {
        match self.bits {
            false => USBCR_A::VALUE1,
            true => USBCR_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USBCR_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USBCR_A::VALUE2
    }
}
#[doc = "Field `USBCR` writer - USB Clock Control"]
pub type USBCR_W<'a, REG> = crate::BitWriter<'a, REG, USBCR_A>;
impl<'a, REG> USBCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USBCR_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USBCR_A::VALUE2)
    }
}
#[doc = "MMC Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMCCR_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<MMCCR_A> for bool {
    #[inline(always)]
    fn from(variant: MMCCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCCR` reader - MMC Clock Control"]
pub type MMCCR_R = crate::BitReader<MMCCR_A>;
impl MMCCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MMCCR_A {
        match self.bits {
            false => MMCCR_A::VALUE1,
            true => MMCCR_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MMCCR_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MMCCR_A::VALUE2
    }
}
#[doc = "Field `MMCCR` writer - MMC Clock Control"]
pub type MMCCR_W<'a, REG> = crate::BitWriter<'a, REG, MMCCR_A>;
impl<'a, REG> MMCCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MMCCR_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MMCCR_A::VALUE2)
    }
}
#[doc = "Ethernet Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH0CR_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<ETH0CR_A> for bool {
    #[inline(always)]
    fn from(variant: ETH0CR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0CR` reader - Ethernet Clock Control"]
pub type ETH0CR_R = crate::BitReader<ETH0CR_A>;
impl ETH0CR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ETH0CR_A {
        match self.bits {
            false => ETH0CR_A::VALUE1,
            true => ETH0CR_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ETH0CR_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ETH0CR_A::VALUE2
    }
}
#[doc = "Field `ETH0CR` writer - Ethernet Clock Control"]
pub type ETH0CR_W<'a, REG> = crate::BitWriter<'a, REG, ETH0CR_A>;
impl<'a, REG> ETH0CR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0CR_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0CR_A::VALUE2)
    }
}
#[doc = "EBU Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EBUCR_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<EBUCR_A> for bool {
    #[inline(always)]
    fn from(variant: EBUCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBUCR` reader - EBU Clock Control"]
pub type EBUCR_R = crate::BitReader<EBUCR_A>;
impl EBUCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EBUCR_A {
        match self.bits {
            false => EBUCR_A::VALUE1,
            true => EBUCR_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EBUCR_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EBUCR_A::VALUE2
    }
}
#[doc = "Field `EBUCR` writer - EBU Clock Control"]
pub type EBUCR_W<'a, REG> = crate::BitWriter<'a, REG, EBUCR_A>;
impl<'a, REG> EBUCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EBUCR_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EBUCR_A::VALUE2)
    }
}
#[doc = "CCU Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUCR_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<CCUCR_A> for bool {
    #[inline(always)]
    fn from(variant: CCUCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCR` reader - CCU Clock Control"]
pub type CCUCR_R = crate::BitReader<CCUCR_A>;
impl CCUCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCUCR_A {
        match self.bits {
            false => CCUCR_A::VALUE1,
            true => CCUCR_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCUCR_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCUCR_A::VALUE2
    }
}
#[doc = "Field `CCUCR` writer - CCU Clock Control"]
pub type CCUCR_W<'a, REG> = crate::BitWriter<'a, REG, CCUCR_A>;
impl<'a, REG> CCUCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCUCR_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCUCR_A::VALUE2)
    }
}
#[doc = "WDT Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTCR_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<WDTCR_A> for bool {
    #[inline(always)]
    fn from(variant: WDTCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCR` reader - WDT Clock Control"]
pub type WDTCR_R = crate::BitReader<WDTCR_A>;
impl WDTCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDTCR_A {
        match self.bits {
            false => WDTCR_A::VALUE1,
            true => WDTCR_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WDTCR_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WDTCR_A::VALUE2
    }
}
#[doc = "Field `WDTCR` writer - WDT Clock Control"]
pub type WDTCR_W<'a, REG> = crate::BitWriter<'a, REG, WDTCR_A>;
impl<'a, REG> WDTCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTCR_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WDTCR_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:1 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&self) -> SYSSEL_R {
        SYSSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 11 - Flash Power Down"]
    #[inline(always)]
    pub fn fpdn(&self) -> FPDN_R {
        FPDN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL Power Down"]
    #[inline(always)]
    pub fn pllpdn(&self) -> PLLPDN_R {
        PLLPDN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VCO Power Down"]
    #[inline(always)]
    pub fn vcopdn(&self) -> VCOPDN_R {
        VCOPDN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - USB Clock Control"]
    #[inline(always)]
    pub fn usbcr(&self) -> USBCR_R {
        USBCR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Clock Control"]
    #[inline(always)]
    pub fn mmccr(&self) -> MMCCR_R {
        MMCCR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Ethernet Clock Control"]
    #[inline(always)]
    pub fn eth0cr(&self) -> ETH0CR_R {
        ETH0CR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - EBU Clock Control"]
    #[inline(always)]
    pub fn ebucr(&self) -> EBUCR_R {
        EBUCR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CCU Clock Control"]
    #[inline(always)]
    pub fn ccucr(&self) -> CCUCR_R {
        CCUCR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - WDT Clock Control"]
    #[inline(always)]
    pub fn wdtcr(&self) -> WDTCR_R {
        WDTCR_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&mut self) -> SYSSEL_W<DSLEEPCR_SPEC> {
        SYSSEL_W::new(self, 0)
    }
    #[doc = "Bit 11 - Flash Power Down"]
    #[inline(always)]
    pub fn fpdn(&mut self) -> FPDN_W<DSLEEPCR_SPEC> {
        FPDN_W::new(self, 11)
    }
    #[doc = "Bit 12 - PLL Power Down"]
    #[inline(always)]
    pub fn pllpdn(&mut self) -> PLLPDN_W<DSLEEPCR_SPEC> {
        PLLPDN_W::new(self, 12)
    }
    #[doc = "Bit 13 - VCO Power Down"]
    #[inline(always)]
    pub fn vcopdn(&mut self) -> VCOPDN_W<DSLEEPCR_SPEC> {
        VCOPDN_W::new(self, 13)
    }
    #[doc = "Bit 16 - USB Clock Control"]
    #[inline(always)]
    pub fn usbcr(&mut self) -> USBCR_W<DSLEEPCR_SPEC> {
        USBCR_W::new(self, 16)
    }
    #[doc = "Bit 17 - MMC Clock Control"]
    #[inline(always)]
    pub fn mmccr(&mut self) -> MMCCR_W<DSLEEPCR_SPEC> {
        MMCCR_W::new(self, 17)
    }
    #[doc = "Bit 18 - Ethernet Clock Control"]
    #[inline(always)]
    pub fn eth0cr(&mut self) -> ETH0CR_W<DSLEEPCR_SPEC> {
        ETH0CR_W::new(self, 18)
    }
    #[doc = "Bit 19 - EBU Clock Control"]
    #[inline(always)]
    pub fn ebucr(&mut self) -> EBUCR_W<DSLEEPCR_SPEC> {
        EBUCR_W::new(self, 19)
    }
    #[doc = "Bit 20 - CCU Clock Control"]
    #[inline(always)]
    pub fn ccucr(&mut self) -> CCUCR_W<DSLEEPCR_SPEC> {
        CCUCR_W::new(self, 20)
    }
    #[doc = "Bit 21 - WDT Clock Control"]
    #[inline(always)]
    pub fn wdtcr(&mut self) -> WDTCR_W<DSLEEPCR_SPEC> {
        WDTCR_W::new(self, 21)
    }
}
#[doc = "Deep Sleep Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsleepcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsleepcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSLEEPCR_SPEC;
impl crate::RegisterSpec for DSLEEPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsleepcr::R`](R) reader structure"]
impl crate::Readable for DSLEEPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsleepcr::W`](W) writer structure"]
impl crate::Writable for DSLEEPCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSLEEPCR to value 0"]
impl crate::Resettable for DSLEEPCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
