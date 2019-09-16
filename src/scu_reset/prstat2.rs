#[doc = "Reader of register PRSTAT2"]
pub type R = crate::R<u32, super::PRSTAT2>;
#[doc = "WDT Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTRS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1,
    #[doc = "1: Reset asserted"]
    VALUE2,
}
impl From<WDTRS_A> for bool {
    #[inline(always)]
    fn from(variant: WDTRS_A) -> Self {
        match variant {
            WDTRS_A::VALUE1 => false,
            WDTRS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `WDTRS`"]
pub type WDTRS_R = crate::R<bool, WDTRS_A>;
impl WDTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTRS_A {
        match self.bits {
            false => WDTRS_A::VALUE1,
            true => WDTRS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WDTRS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WDTRS_A::VALUE2
    }
}
#[doc = "ETH0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETH0RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1,
    #[doc = "1: Reset asserted"]
    VALUE2,
}
impl From<ETH0RS_A> for bool {
    #[inline(always)]
    fn from(variant: ETH0RS_A) -> Self {
        match variant {
            ETH0RS_A::VALUE1 => false,
            ETH0RS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ETH0RS`"]
pub type ETH0RS_R = crate::R<bool, ETH0RS_A>;
impl ETH0RS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETH0RS_A {
        match self.bits {
            false => ETH0RS_A::VALUE1,
            true => ETH0RS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ETH0RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ETH0RS_A::VALUE2
    }
}
#[doc = "DMA0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA0RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1,
    #[doc = "1: Reset asserted"]
    VALUE2,
}
impl From<DMA0RS_A> for bool {
    #[inline(always)]
    fn from(variant: DMA0RS_A) -> Self {
        match variant {
            DMA0RS_A::VALUE1 => false,
            DMA0RS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DMA0RS`"]
pub type DMA0RS_R = crate::R<bool, DMA0RS_A>;
impl DMA0RS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA0RS_A {
        match self.bits {
            false => DMA0RS_A::VALUE1,
            true => DMA0RS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DMA0RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DMA0RS_A::VALUE2
    }
}
#[doc = "DMA1 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1,
    #[doc = "1: Reset asserted"]
    VALUE2,
}
impl From<DMA1RS_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1RS_A) -> Self {
        match variant {
            DMA1RS_A::VALUE1 => false,
            DMA1RS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DMA1RS`"]
pub type DMA1RS_R = crate::R<bool, DMA1RS_A>;
impl DMA1RS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1RS_A {
        match self.bits {
            false => DMA1RS_A::VALUE1,
            true => DMA1RS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DMA1RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DMA1RS_A::VALUE2
    }
}
#[doc = "FCE Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCERS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1,
    #[doc = "1: Reset asserted"]
    VALUE2,
}
impl From<FCERS_A> for bool {
    #[inline(always)]
    fn from(variant: FCERS_A) -> Self {
        match variant {
            FCERS_A::VALUE1 => false,
            FCERS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `FCERS`"]
pub type FCERS_R = crate::R<bool, FCERS_A>;
impl FCERS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCERS_A {
        match self.bits {
            false => FCERS_A::VALUE1,
            true => FCERS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FCERS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FCERS_A::VALUE2
    }
}
#[doc = "USB Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1,
    #[doc = "1: Reset asserted"]
    VALUE2,
}
impl From<USBRS_A> for bool {
    #[inline(always)]
    fn from(variant: USBRS_A) -> Self {
        match variant {
            USBRS_A::VALUE1 => false,
            USBRS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `USBRS`"]
pub type USBRS_R = crate::R<bool, USBRS_A>;
impl USBRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBRS_A {
        match self.bits {
            false => USBRS_A::VALUE1,
            true => USBRS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USBRS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USBRS_A::VALUE2
    }
}
#[doc = "ECAT0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECAT0RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1,
    #[doc = "1: Reset asserted"]
    VALUE2,
}
impl From<ECAT0RS_A> for bool {
    #[inline(always)]
    fn from(variant: ECAT0RS_A) -> Self {
        match variant {
            ECAT0RS_A::VALUE1 => false,
            ECAT0RS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ECAT0RS`"]
pub type ECAT0RS_R = crate::R<bool, ECAT0RS_A>;
impl ECAT0RS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECAT0RS_A {
        match self.bits {
            false => ECAT0RS_A::VALUE1,
            true => ECAT0RS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ECAT0RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ECAT0RS_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 1 - WDT Reset Status"]
    #[inline(always)]
    pub fn wdtrs(&self) -> WDTRS_R {
        WDTRS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ETH0 Reset Status"]
    #[inline(always)]
    pub fn eth0rs(&self) -> ETH0RS_R {
        ETH0RS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DMA0 Reset Status"]
    #[inline(always)]
    pub fn dma0rs(&self) -> DMA0RS_R {
        DMA0RS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DMA1 Reset Status"]
    #[inline(always)]
    pub fn dma1rs(&self) -> DMA1RS_R {
        DMA1RS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FCE Reset Status"]
    #[inline(always)]
    pub fn fcers(&self) -> FCERS_R {
        FCERS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB Reset Status"]
    #[inline(always)]
    pub fn usbrs(&self) -> USBRS_R {
        USBRS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ECAT0 Reset Status"]
    #[inline(always)]
    pub fn ecat0rs(&self) -> ECAT0RS_R {
        ECAT0RS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
