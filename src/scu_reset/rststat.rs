#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RSTSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `RSTSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTSTATR {
    #[doc = "PORST reset"]
    VALUE1,
    #[doc = "SWD reset"]
    VALUE2,
    #[doc = "PV reset"]
    VALUE3,
    #[doc = "CPU system reset"]
    VALUE4,
    #[doc = "CPU lockup reset"]
    VALUE5,
    #[doc = "WDT reset"]
    VALUE6,
    #[doc = "Parity Error reset"]
    VALUE8,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RSTSTATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RSTSTATR::VALUE1 => 1,
            RSTSTATR::VALUE2 => 2,
            RSTSTATR::VALUE3 => 4,
            RSTSTATR::VALUE4 => 8,
            RSTSTATR::VALUE5 => 16,
            RSTSTATR::VALUE6 => 32,
            RSTSTATR::VALUE8 => 128,
            RSTSTATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RSTSTATR {
        match value {
            1 => RSTSTATR::VALUE1,
            2 => RSTSTATR::VALUE2,
            4 => RSTSTATR::VALUE3,
            8 => RSTSTATR::VALUE4,
            16 => RSTSTATR::VALUE5,
            32 => RSTSTATR::VALUE6,
            128 => RSTSTATR::VALUE8,
            i => RSTSTATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RSTSTATR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RSTSTATR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RSTSTATR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == RSTSTATR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == RSTSTATR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == RSTSTATR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == RSTSTATR::VALUE8
    }
}
#[doc = "Possible values of the field `HIBWK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBWKR {
    #[doc = "No Wake-up"]
    VALUE1,
    #[doc = "Wake-up event"]
    VALUE2,
}
impl HIBWKR {
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
            HIBWKR::VALUE1 => false,
            HIBWKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIBWKR {
        match value {
            false => HIBWKR::VALUE1,
            true => HIBWKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HIBWKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HIBWKR::VALUE2
    }
}
#[doc = "Possible values of the field `HIBRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBRSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl HIBRSR {
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
            HIBRSR::VALUE1 => false,
            HIBRSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIBRSR {
        match value {
            false => HIBRSR::VALUE1,
            true => HIBRSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HIBRSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HIBRSR::VALUE2
    }
}
#[doc = "Possible values of the field `LCKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCKENR {
    #[doc = "Reset by Lockup disabled"]
    VALUE1,
    #[doc = "Reset by Lockup enabled"]
    VALUE2,
}
impl LCKENR {
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
            LCKENR::VALUE1 => false,
            LCKENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LCKENR {
        match value {
            false => LCKENR::VALUE1,
            true => LCKENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LCKENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LCKENR::VALUE2
    }
}
#[doc = "Possible values of the field `ECAT0RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECAT0RSR {
    #[doc = "Reset did not occur"]
    VALUE1,
    #[doc = "Reset occurred"]
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
    #[doc = "Bits 0:7 - Reset Status Information"]
    #[inline]
    pub fn rststat(&self) -> RSTSTATR {
        RSTSTATR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Hibernate Wake-up Status"]
    #[inline]
    pub fn hibwk(&self) -> HIBWKR {
        HIBWKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Hibernate Reset Status"]
    #[inline]
    pub fn hibrs(&self) -> HIBRSR {
        HIBRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Enable Lockup Status"]
    #[inline]
    pub fn lcken(&self) -> LCKENR {
        LCKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - ECAT0 Reset Status Information"]
    #[inline]
    pub fn ecat0rs(&self) -> ECAT0RSR {
        ECAT0RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
