#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CGATSTAT0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `VADC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VADCR {
    #[doc = "Gating de-asserted"]
    VALUE1,
    #[doc = "Gating asserted"]
    VALUE2,
}
impl VADCR {
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
            VADCR::VALUE1 => false,
            VADCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VADCR {
        match value {
            false => VADCR::VALUE1,
            true => VADCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VADCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VADCR::VALUE2
    }
}
#[doc = "Possible values of the field `DSD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSDR {
    #[doc = "Gating de-asserted"]
    VALUE1,
    #[doc = "Gating asserted"]
    VALUE2,
}
impl DSDR {
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
            DSDR::VALUE1 => false,
            DSDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DSDR {
        match value {
            false => DSDR::VALUE1,
            true => DSDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DSDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DSDR::VALUE2
    }
}
#[doc = "Possible values of the field `CCU40`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU40R {
    #[doc = "Gating de-asserted"]
    VALUE1,
    #[doc = "Gating asserted"]
    VALUE2,
}
impl CCU40R {
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
            CCU40R::VALUE1 => false,
            CCU40R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCU40R {
        match value {
            false => CCU40R::VALUE1,
            true => CCU40R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CCU40R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CCU40R::VALUE2
    }
}
#[doc = "Possible values of the field `CCU41`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU41R {
    #[doc = "Gating de-asserted"]
    VALUE1,
    #[doc = "Gating asserted"]
    VALUE2,
}
impl CCU41R {
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
            CCU41R::VALUE1 => false,
            CCU41R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCU41R {
        match value {
            false => CCU41R::VALUE1,
            true => CCU41R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CCU41R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CCU41R::VALUE2
    }
}
#[doc = "Possible values of the field `CCU42`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU42R {
    #[doc = "Gating de-asserted"]
    VALUE1,
    #[doc = "Gating asserted"]
    VALUE2,
}
impl CCU42R {
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
            CCU42R::VALUE1 => false,
            CCU42R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCU42R {
        match value {
            false => CCU42R::VALUE1,
            true => CCU42R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CCU42R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CCU42R::VALUE2
    }
}
#[doc = "Possible values of the field `CCU80`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU80R {
    #[doc = "Gating de-asserted"]
    VALUE1,
    #[doc = "Gating asserted"]
    VALUE2,
}
impl CCU80R {
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
            CCU80R::VALUE1 => false,
            CCU80R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCU80R {
        match value {
            false => CCU80R::VALUE1,
            true => CCU80R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CCU80R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CCU80R::VALUE2
    }
}
#[doc = "Possible values of the field `CCU81`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU81R {
    #[doc = "Gating de-asserted"]
    VALUE1,
    #[doc = "Gating asserted"]
    VALUE2,
}
impl CCU81R {
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
            CCU81R::VALUE1 => false,
            CCU81R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCU81R {
        match value {
            false => CCU81R::VALUE1,
            true => CCU81R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CCU81R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CCU81R::VALUE2
    }
}
#[doc = "Possible values of the field `POSIF0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSIF0R {
    #[doc = "Gating de-asserted"]
    VALUE1,
    #[doc = "Gating asserted"]
    VALUE2,
}
impl POSIF0R {
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
            POSIF0R::VALUE1 => false,
            POSIF0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POSIF0R {
        match value {
            false => POSIF0R::VALUE1,
            true => POSIF0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == POSIF0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == POSIF0R::VALUE2
    }
}
#[doc = "Possible values of the field `POSIF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSIF1R {
    #[doc = "Gating de-asserted"]
    VALUE1,
    #[doc = "Gating asserted"]
    VALUE2,
}
impl POSIF1R {
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
            POSIF1R::VALUE1 => false,
            POSIF1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POSIF1R {
        match value {
            false => POSIF1R::VALUE1,
            true => POSIF1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == POSIF1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == POSIF1R::VALUE2
    }
}
#[doc = "Possible values of the field `USIC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC0R {
    #[doc = "Gating de-asserted"]
    VALUE1,
    #[doc = "Gating asserted"]
    VALUE2,
}
impl USIC0R {
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
            USIC0R::VALUE1 => false,
            USIC0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USIC0R {
        match value {
            false => USIC0R::VALUE1,
            true => USIC0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == USIC0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == USIC0R::VALUE2
    }
}
#[doc = "Possible values of the field `ERU1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERU1R {
    #[doc = "Gating de-asserted"]
    VALUE1,
    #[doc = "Gating asserted"]
    VALUE2,
}
impl ERU1R {
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
            ERU1R::VALUE1 => false,
            ERU1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERU1R {
        match value {
            false => ERU1R::VALUE1,
            true => ERU1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ERU1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ERU1R::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - VADC Gating Status"]
    #[inline]
    pub fn vadc(&self) -> VADCR {
        VADCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - DSD Gating Status"]
    #[inline]
    pub fn dsd(&self) -> DSDR {
        DSDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - CCU40 Gating Status"]
    #[inline]
    pub fn ccu40(&self) -> CCU40R {
        CCU40R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - CCU41 Gating Status"]
    #[inline]
    pub fn ccu41(&self) -> CCU41R {
        CCU41R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - CCU42 Gating Status"]
    #[inline]
    pub fn ccu42(&self) -> CCU42R {
        CCU42R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - CCU80 Gating Status"]
    #[inline]
    pub fn ccu80(&self) -> CCU80R {
        CCU80R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - CCU81 Gating Status"]
    #[inline]
    pub fn ccu81(&self) -> CCU81R {
        CCU81R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - POSIF0 Gating Status"]
    #[inline]
    pub fn posif0(&self) -> POSIF0R {
        POSIF0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - POSIF1 Gating Status"]
    #[inline]
    pub fn posif1(&self) -> POSIF1R {
        POSIF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - USIC0 Gating Status"]
    #[inline]
    pub fn usic0(&self) -> USIC0R {
        USIC0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - ERU1 Gating Status"]
    #[inline]
    pub fn eru1(&self) -> ERU1R {
        ERU1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
