#[doc = "Register `CLKSTAT` reader"]
pub type R = crate::R<CLKSTAT_SPEC>;
#[doc = "Field `USBCST` reader - USB Clock Status"]
pub type USBCST_R = crate::BitReader<USBCST_A>;
#[doc = "USB Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBCST_A {
    #[doc = "0: Clock disabled"]
    VALUE1 = 0,
    #[doc = "1: Clock enabled"]
    VALUE2 = 1,
}
impl From<USBCST_A> for bool {
    #[inline(always)]
    fn from(variant: USBCST_A) -> Self {
        variant as u8 != 0
    }
}
impl USBCST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBCST_A {
        match self.bits {
            false => USBCST_A::VALUE1,
            true => USBCST_A::VALUE2,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USBCST_A::VALUE1
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USBCST_A::VALUE2
    }
}
#[doc = "Field `MMCCST` reader - MMC Clock Status"]
pub type MMCCST_R = crate::BitReader<MMCCST_A>;
#[doc = "MMC Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMCCST_A {
    #[doc = "0: Clock disabled"]
    VALUE1 = 0,
    #[doc = "1: Clock enabled"]
    VALUE2 = 1,
}
impl From<MMCCST_A> for bool {
    #[inline(always)]
    fn from(variant: MMCCST_A) -> Self {
        variant as u8 != 0
    }
}
impl MMCCST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MMCCST_A {
        match self.bits {
            false => MMCCST_A::VALUE1,
            true => MMCCST_A::VALUE2,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MMCCST_A::VALUE1
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MMCCST_A::VALUE2
    }
}
#[doc = "Field `ETH0CST` reader - Ethernet Clock Status"]
pub type ETH0CST_R = crate::BitReader<ETH0CST_A>;
#[doc = "Ethernet Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH0CST_A {
    #[doc = "0: Clock disabled"]
    VALUE1 = 0,
    #[doc = "1: Clock enabled"]
    VALUE2 = 1,
}
impl From<ETH0CST_A> for bool {
    #[inline(always)]
    fn from(variant: ETH0CST_A) -> Self {
        variant as u8 != 0
    }
}
impl ETH0CST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ETH0CST_A {
        match self.bits {
            false => ETH0CST_A::VALUE1,
            true => ETH0CST_A::VALUE2,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ETH0CST_A::VALUE1
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ETH0CST_A::VALUE2
    }
}
#[doc = "Field `EBUCST` reader - EBU Clock Status"]
pub type EBUCST_R = crate::BitReader<EBUCST_A>;
#[doc = "EBU Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EBUCST_A {
    #[doc = "0: Clock disabled"]
    VALUE1 = 0,
    #[doc = "1: Clock enabled"]
    VALUE2 = 1,
}
impl From<EBUCST_A> for bool {
    #[inline(always)]
    fn from(variant: EBUCST_A) -> Self {
        variant as u8 != 0
    }
}
impl EBUCST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EBUCST_A {
        match self.bits {
            false => EBUCST_A::VALUE1,
            true => EBUCST_A::VALUE2,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EBUCST_A::VALUE1
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EBUCST_A::VALUE2
    }
}
#[doc = "Field `CCUCST` reader - CCU Clock Status"]
pub type CCUCST_R = crate::BitReader<CCUCST_A>;
#[doc = "CCU Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUCST_A {
    #[doc = "0: Clock disabled"]
    VALUE1 = 0,
    #[doc = "1: Clock enabled"]
    VALUE2 = 1,
}
impl From<CCUCST_A> for bool {
    #[inline(always)]
    fn from(variant: CCUCST_A) -> Self {
        variant as u8 != 0
    }
}
impl CCUCST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCUCST_A {
        match self.bits {
            false => CCUCST_A::VALUE1,
            true => CCUCST_A::VALUE2,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCUCST_A::VALUE1
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCUCST_A::VALUE2
    }
}
#[doc = "Field `WDTCST` reader - WDT Clock Status"]
pub type WDTCST_R = crate::BitReader<WDTCST_A>;
#[doc = "WDT Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTCST_A {
    #[doc = "0: Clock disabled"]
    VALUE1 = 0,
    #[doc = "1: Clock enabled"]
    VALUE2 = 1,
}
impl From<WDTCST_A> for bool {
    #[inline(always)]
    fn from(variant: WDTCST_A) -> Self {
        variant as u8 != 0
    }
}
impl WDTCST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDTCST_A {
        match self.bits {
            false => WDTCST_A::VALUE1,
            true => WDTCST_A::VALUE2,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WDTCST_A::VALUE1
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WDTCST_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - USB Clock Status"]
    #[inline(always)]
    pub fn usbcst(&self) -> USBCST_R {
        USBCST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Clock Status"]
    #[inline(always)]
    pub fn mmccst(&self) -> MMCCST_R {
        MMCCST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ethernet Clock Status"]
    #[inline(always)]
    pub fn eth0cst(&self) -> ETH0CST_R {
        ETH0CST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EBU Clock Status"]
    #[inline(always)]
    pub fn ebucst(&self) -> EBUCST_R {
        EBUCST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CCU Clock Status"]
    #[inline(always)]
    pub fn ccucst(&self) -> CCUCST_R {
        CCUCST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WDT Clock Status"]
    #[inline(always)]
    pub fn wdtcst(&self) -> WDTCST_R {
        WDTCST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Clock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKSTAT_SPEC;
impl crate::RegisterSpec for CLKSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkstat::R`](R) reader structure"]
impl crate::Readable for CLKSTAT_SPEC {}
#[doc = "`reset()` method sets CLKSTAT to value 0"]
impl crate::Resettable for CLKSTAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
