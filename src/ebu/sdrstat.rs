#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SDRSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `SDERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDERRR {
    #[doc = "Reads running successfully"]
    VALUE1,
    #[doc = "Read error condition has been detected"]
    VALUE2,
}
impl SDERRR {
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
            SDERRR::VALUE1 => false,
            SDERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDERRR {
        match value {
            false => SDERRR::VALUE1,
            true => SDERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SDERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SDERRR::VALUE2
    }
}
#[doc = "Possible values of the field `SDRMBUSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDRMBUSYR {
    #[doc = "Power-up initialization sequence is not running"]
    VALUE1,
    #[doc = "Power-up initialization sequence is running"]
    VALUE2,
}
impl SDRMBUSYR {
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
            SDRMBUSYR::VALUE1 => false,
            SDRMBUSYR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDRMBUSYR {
        match value {
            false => SDRMBUSYR::VALUE1,
            true => SDRMBUSYR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SDRMBUSYR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SDRMBUSYR::VALUE2
    }
}
#[doc = "Possible values of the field `REFERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFERRR {
    #[doc = "No refresh error."]
    VALUE1,
    #[doc = "Refresh error occurred."]
    VALUE2,
}
impl REFERRR {
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
            REFERRR::VALUE1 => false,
            REFERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REFERRR {
        match value {
            false => REFERRR::VALUE1,
            true => REFERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REFERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REFERRR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 2 - SDRAM read error"]
    #[inline]
    pub fn sderr(&self) -> SDERRR {
        SDERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - SDRAM Busy"]
    #[inline]
    pub fn sdrmbusy(&self) -> SDRMBUSYR {
        SDRMBUSYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - SDRAM Refresh Error"]
    #[inline]
    pub fn referr(&self) -> REFERRR {
        REFERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
