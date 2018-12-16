#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PRSTAT2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `WDTRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTRSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl WDTRSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            WDTRSR::VALUE1 => false,
            WDTRSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDTRSR {
        match value {
            false => WDTRSR::VALUE1,
            true => WDTRSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WDTRSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WDTRSR::VALUE2
    }
}
#[doc = "Possible values of the field `ETH0RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETH0RSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl ETH0RSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ETH0RSR::VALUE1 => false,
            ETH0RSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ETH0RSR {
        match value {
            false => ETH0RSR::VALUE1,
            true => ETH0RSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ETH0RSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ETH0RSR::VALUE2
    }
}
#[doc = "Possible values of the field `DMA0RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA0RSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl DMA0RSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DMA0RSR::VALUE1 => false,
            DMA0RSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA0RSR {
        match value {
            false => DMA0RSR::VALUE1,
            true => DMA0RSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DMA0RSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DMA0RSR::VALUE2
    }
}
#[doc = "Possible values of the field `DMA1RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1RSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl DMA1RSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DMA1RSR::VALUE1 => false,
            DMA1RSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA1RSR {
        match value {
            false => DMA1RSR::VALUE1,
            true => DMA1RSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DMA1RSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DMA1RSR::VALUE2
    }
}
#[doc = "Possible values of the field `FCERS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCERSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl FCERSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FCERSR::VALUE1 => false,
            FCERSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FCERSR {
        match value {
            false => FCERSR::VALUE1,
            true => FCERSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FCERSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FCERSR::VALUE2
    }
}
#[doc = "Possible values of the field `USBRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl USBRSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            USBRSR::VALUE1 => false,
            USBRSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBRSR {
        match value {
            false => USBRSR::VALUE1,
            true => USBRSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == USBRSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == USBRSR::VALUE2
    }
}
#[doc = "Possible values of the field `ECAT0RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECAT0RSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl ECAT0RSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ECAT0RSR::VALUE1 => false,
            ECAT0RSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECAT0RSR {
        match value {
            false => ECAT0RSR::VALUE1,
            true => ECAT0RSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ECAT0RSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ECAT0RSR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - WDT Reset Status"]
    #[inline]
    pub fn wdtrs(&self) -> WDTRSR {
        WDTRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - ETH0 Reset Status"]
    #[inline]
    pub fn eth0rs(&self) -> ETH0RSR {
        ETH0RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - DMA0 Reset Status"]
    #[inline]
    pub fn dma0rs(&self) -> DMA0RSR {
        DMA0RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - DMA1 Reset Status"]
    #[inline]
    pub fn dma1rs(&self) -> DMA1RSR {
        DMA1RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - FCE Reset Status"]
    #[inline]
    pub fn fcers(&self) -> FCERSR {
        FCERSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - USB Reset Status"]
    #[inline]
    pub fn usbrs(&self) -> USBRSR {
        USBRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - ECAT0 Reset Status"]
    #[inline]
    pub fn ecat0rs(&self) -> ECAT0RSR {
        ECAT0RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
