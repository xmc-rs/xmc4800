#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CLKSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `USBCST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBCSTR {
    #[doc = "Clock disabled"]
    VALUE1,
    #[doc = "Clock enabled"]
    VALUE2,
}
impl USBCSTR {
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
            USBCSTR::VALUE1 => false,
            USBCSTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBCSTR {
        match value {
            false => USBCSTR::VALUE1,
            true => USBCSTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == USBCSTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == USBCSTR::VALUE2
    }
}
#[doc = "Possible values of the field `MMCCST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMCCSTR {
    #[doc = "Clock disabled"]
    VALUE1,
    #[doc = "Clock enabled"]
    VALUE2,
}
impl MMCCSTR {
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
            MMCCSTR::VALUE1 => false,
            MMCCSTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MMCCSTR {
        match value {
            false => MMCCSTR::VALUE1,
            true => MMCCSTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MMCCSTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MMCCSTR::VALUE2
    }
}
#[doc = "Possible values of the field `ETH0CST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETH0CSTR {
    #[doc = "Clock disabled"]
    VALUE1,
    #[doc = "Clock enabled"]
    VALUE2,
}
impl ETH0CSTR {
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
            ETH0CSTR::VALUE1 => false,
            ETH0CSTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ETH0CSTR {
        match value {
            false => ETH0CSTR::VALUE1,
            true => ETH0CSTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ETH0CSTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ETH0CSTR::VALUE2
    }
}
#[doc = "Possible values of the field `EBUCST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBUCSTR {
    #[doc = "Clock disabled"]
    VALUE1,
    #[doc = "Clock enabled"]
    VALUE2,
}
impl EBUCSTR {
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
            EBUCSTR::VALUE1 => false,
            EBUCSTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EBUCSTR {
        match value {
            false => EBUCSTR::VALUE1,
            true => EBUCSTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EBUCSTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EBUCSTR::VALUE2
    }
}
#[doc = "Possible values of the field `CCUCST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCUCSTR {
    #[doc = "Clock disabled"]
    VALUE1,
    #[doc = "Clock enabled"]
    VALUE2,
}
impl CCUCSTR {
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
            CCUCSTR::VALUE1 => false,
            CCUCSTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCUCSTR {
        match value {
            false => CCUCSTR::VALUE1,
            true => CCUCSTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CCUCSTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CCUCSTR::VALUE2
    }
}
#[doc = "Possible values of the field `WDTCST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTCSTR {
    #[doc = "Clock disabled"]
    VALUE1,
    #[doc = "Clock enabled"]
    VALUE2,
}
impl WDTCSTR {
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
            WDTCSTR::VALUE1 => false,
            WDTCSTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDTCSTR {
        match value {
            false => WDTCSTR::VALUE1,
            true => WDTCSTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WDTCSTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WDTCSTR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - USB Clock Status"]
    #[inline]
    pub fn usbcst(&self) -> USBCSTR {
        USBCSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - MMC Clock Status"]
    #[inline]
    pub fn mmccst(&self) -> MMCCSTR {
        MMCCSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Ethernet Clock Status"]
    #[inline]
    pub fn eth0cst(&self) -> ETH0CSTR {
        ETH0CSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - EBU Clock Status"]
    #[inline]
    pub fn ebucst(&self) -> EBUCSTR {
        EBUCSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - CCU Clock Status"]
    #[inline]
    pub fn ccucst(&self) -> CCUCSTR {
        CCUCSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - WDT Clock Status"]
    #[inline]
    pub fn wdtcst(&self) -> WDTCSTR {
        WDTCSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
