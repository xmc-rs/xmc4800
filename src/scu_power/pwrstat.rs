#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PWRSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `HIBEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBENR {
    #[doc = "Inactive"]
    VALUE1,
    #[doc = "Active"]
    VALUE2,
}
impl HIBENR {
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
            HIBENR::VALUE1 => false,
            HIBENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIBENR {
        match value {
            false => HIBENR::VALUE1,
            true => HIBENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HIBENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HIBENR::VALUE2
    }
}
#[doc = "Possible values of the field `USBPHYPDQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPHYPDQR {
    #[doc = "Power-down"]
    VALUE1,
    #[doc = "Active"]
    VALUE2,
}
impl USBPHYPDQR {
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
            USBPHYPDQR::VALUE1 => false,
            USBPHYPDQR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBPHYPDQR {
        match value {
            false => USBPHYPDQR::VALUE1,
            true => USBPHYPDQR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == USBPHYPDQR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == USBPHYPDQR::VALUE2
    }
}
#[doc = "Possible values of the field `USBOTGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBOTGENR {
    #[doc = "Power-down"]
    VALUE1,
    #[doc = "Active"]
    VALUE2,
}
impl USBOTGENR {
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
            USBOTGENR::VALUE1 => false,
            USBOTGENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBOTGENR {
        match value {
            false => USBOTGENR::VALUE1,
            true => USBOTGENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == USBOTGENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == USBOTGENR::VALUE2
    }
}
#[doc = "Possible values of the field `USBPUWQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPUWQR {
    #[doc = "Pull-up active"]
    VALUE1,
    #[doc = "Pull-up not active"]
    VALUE2,
}
impl USBPUWQR {
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
            USBPUWQR::VALUE1 => false,
            USBPUWQR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBPUWQR {
        match value {
            false => USBPUWQR::VALUE1,
            true => USBPUWQR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == USBPUWQR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == USBPUWQR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Hibernate Domain Enable Status"]
    #[inline]
    pub fn hiben(&self) -> HIBENR {
        HIBENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - USB PHY Transceiver State"]
    #[inline]
    pub fn usbphypdq(&self) -> USBPHYPDQR {
        USBPHYPDQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - USB On-The-Go Comparators State"]
    #[inline]
    pub fn usbotgen(&self) -> USBOTGENR {
        USBOTGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - USB Weak Pull-Up at PADN State"]
    #[inline]
    pub fn usbpuwq(&self) -> USBPUWQR {
        USBPUWQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
