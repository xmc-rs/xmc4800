#[doc = "Register `CLKSTAT` reader"]
pub type R = crate::R<ClkstatSpec>;
#[doc = "USB Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbcst {
    #[doc = "0: Clock disabled"]
    Value1 = 0,
    #[doc = "1: Clock enabled"]
    Value2 = 1,
}
impl From<Usbcst> for bool {
    #[inline(always)]
    fn from(variant: Usbcst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCST` reader - USB Clock Status"]
pub type UsbcstR = crate::BitReader<Usbcst>;
impl UsbcstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbcst {
        match self.bits {
            false => Usbcst::Value1,
            true => Usbcst::Value2,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Usbcst::Value1
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Usbcst::Value2
    }
}
#[doc = "MMC Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmccst {
    #[doc = "0: Clock disabled"]
    Value1 = 0,
    #[doc = "1: Clock enabled"]
    Value2 = 1,
}
impl From<Mmccst> for bool {
    #[inline(always)]
    fn from(variant: Mmccst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCCST` reader - MMC Clock Status"]
pub type MmccstR = crate::BitReader<Mmccst>;
impl MmccstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mmccst {
        match self.bits {
            false => Mmccst::Value1,
            true => Mmccst::Value2,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mmccst::Value1
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mmccst::Value2
    }
}
#[doc = "Ethernet Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eth0cst {
    #[doc = "0: Clock disabled"]
    Value1 = 0,
    #[doc = "1: Clock enabled"]
    Value2 = 1,
}
impl From<Eth0cst> for bool {
    #[inline(always)]
    fn from(variant: Eth0cst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0CST` reader - Ethernet Clock Status"]
pub type Eth0cstR = crate::BitReader<Eth0cst>;
impl Eth0cstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eth0cst {
        match self.bits {
            false => Eth0cst::Value1,
            true => Eth0cst::Value2,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Eth0cst::Value1
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Eth0cst::Value2
    }
}
#[doc = "EBU Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ebucst {
    #[doc = "0: Clock disabled"]
    Value1 = 0,
    #[doc = "1: Clock enabled"]
    Value2 = 1,
}
impl From<Ebucst> for bool {
    #[inline(always)]
    fn from(variant: Ebucst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBUCST` reader - EBU Clock Status"]
pub type EbucstR = crate::BitReader<Ebucst>;
impl EbucstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ebucst {
        match self.bits {
            false => Ebucst::Value1,
            true => Ebucst::Value2,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ebucst::Value1
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ebucst::Value2
    }
}
#[doc = "CCU Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccucst {
    #[doc = "0: Clock disabled"]
    Value1 = 0,
    #[doc = "1: Clock enabled"]
    Value2 = 1,
}
impl From<Ccucst> for bool {
    #[inline(always)]
    fn from(variant: Ccucst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCST` reader - CCU Clock Status"]
pub type CcucstR = crate::BitReader<Ccucst>;
impl CcucstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccucst {
        match self.bits {
            false => Ccucst::Value1,
            true => Ccucst::Value2,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ccucst::Value1
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ccucst::Value2
    }
}
#[doc = "WDT Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtcst {
    #[doc = "0: Clock disabled"]
    Value1 = 0,
    #[doc = "1: Clock enabled"]
    Value2 = 1,
}
impl From<Wdtcst> for bool {
    #[inline(always)]
    fn from(variant: Wdtcst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCST` reader - WDT Clock Status"]
pub type WdtcstR = crate::BitReader<Wdtcst>;
impl WdtcstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtcst {
        match self.bits {
            false => Wdtcst::Value1,
            true => Wdtcst::Value2,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wdtcst::Value1
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wdtcst::Value2
    }
}
impl R {
    #[doc = "Bit 0 - USB Clock Status"]
    #[inline(always)]
    pub fn usbcst(&self) -> UsbcstR {
        UsbcstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Clock Status"]
    #[inline(always)]
    pub fn mmccst(&self) -> MmccstR {
        MmccstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ethernet Clock Status"]
    #[inline(always)]
    pub fn eth0cst(&self) -> Eth0cstR {
        Eth0cstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EBU Clock Status"]
    #[inline(always)]
    pub fn ebucst(&self) -> EbucstR {
        EbucstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CCU Clock Status"]
    #[inline(always)]
    pub fn ccucst(&self) -> CcucstR {
        CcucstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WDT Clock Status"]
    #[inline(always)]
    pub fn wdtcst(&self) -> WdtcstR {
        WdtcstR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Clock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkstatSpec;
impl crate::RegisterSpec for ClkstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkstat::R`](R) reader structure"]
impl crate::Readable for ClkstatSpec {}
#[doc = "`reset()` method sets CLKSTAT to value 0"]
impl crate::Resettable for ClkstatSpec {
    const RESET_VALUE: u32 = 0;
}
