#[doc = "Register `CGATSTAT2` reader"]
pub type R = crate::R<Cgatstat2Spec>;
#[doc = "WDT Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdt {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Wdt> for bool {
    #[inline(always)]
    fn from(variant: Wdt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT` reader - WDT Gating Status"]
pub type WdtR = crate::BitReader<Wdt>;
impl WdtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdt {
        match self.bits {
            false => Wdt::Value1,
            true => Wdt::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wdt::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wdt::Value2
    }
}
#[doc = "ETH0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eth0 {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Eth0> for bool {
    #[inline(always)]
    fn from(variant: Eth0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0` reader - ETH0 Gating Status"]
pub type Eth0R = crate::BitReader<Eth0>;
impl Eth0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eth0 {
        match self.bits {
            false => Eth0::Value1,
            true => Eth0::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Eth0::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Eth0::Value2
    }
}
#[doc = "DMA0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma0 {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Dma0> for bool {
    #[inline(always)]
    fn from(variant: Dma0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA0` reader - DMA0 Gating Status"]
pub type Dma0R = crate::BitReader<Dma0>;
impl Dma0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma0 {
        match self.bits {
            false => Dma0::Value1,
            true => Dma0::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dma0::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dma0::Value2
    }
}
#[doc = "DMA1 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma1 {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Dma1> for bool {
    #[inline(always)]
    fn from(variant: Dma1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1` reader - DMA1 Gating Status"]
pub type Dma1R = crate::BitReader<Dma1>;
impl Dma1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma1 {
        match self.bits {
            false => Dma1::Value1,
            true => Dma1::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dma1::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dma1::Value2
    }
}
#[doc = "FCE Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fce {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Fce> for bool {
    #[inline(always)]
    fn from(variant: Fce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCE` reader - FCE Gating Status"]
pub type FceR = crate::BitReader<Fce>;
impl FceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fce {
        match self.bits {
            false => Fce::Value1,
            true => Fce::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fce::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fce::Value2
    }
}
#[doc = "USB Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Usb> for bool {
    #[inline(always)]
    fn from(variant: Usb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB` reader - USB Gating Status"]
pub type UsbR = crate::BitReader<Usb>;
impl UsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb {
        match self.bits {
            false => Usb::Value1,
            true => Usb::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Usb::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Usb::Value2
    }
}
#[doc = "ECAT0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecat0 {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Ecat0> for bool {
    #[inline(always)]
    fn from(variant: Ecat0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECAT0` reader - ECAT0 Gating Status"]
pub type Ecat0R = crate::BitReader<Ecat0>;
impl Ecat0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecat0 {
        match self.bits {
            false => Ecat0::Value1,
            true => Ecat0::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ecat0::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ecat0::Value2
    }
}
impl R {
    #[doc = "Bit 1 - WDT Gating Status"]
    #[inline(always)]
    pub fn wdt(&self) -> WdtR {
        WdtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ETH0 Gating Status"]
    #[inline(always)]
    pub fn eth0(&self) -> Eth0R {
        Eth0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA0 Gating Status"]
    #[inline(always)]
    pub fn dma0(&self) -> Dma0R {
        Dma0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA1 Gating Status"]
    #[inline(always)]
    pub fn dma1(&self) -> Dma1R {
        Dma1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FCE Gating Status"]
    #[inline(always)]
    pub fn fce(&self) -> FceR {
        FceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB Gating Status"]
    #[inline(always)]
    pub fn usb(&self) -> UsbR {
        UsbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - ECAT0 Gating Status"]
    #[inline(always)]
    pub fn ecat0(&self) -> Ecat0R {
        Ecat0R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Peripheral 2 Clock Gating Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgatstat2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cgatstat2Spec;
impl crate::RegisterSpec for Cgatstat2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgatstat2::R`](R) reader structure"]
impl crate::Readable for Cgatstat2Spec {}
#[doc = "`reset()` method sets CGATSTAT2 to value 0"]
impl crate::Resettable for Cgatstat2Spec {
    const RESET_VALUE: u32 = 0;
}
