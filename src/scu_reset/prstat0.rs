#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PRSTAT0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `VADCRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VADCRSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl VADCRSR {
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
            VADCRSR::VALUE1 => false,
            VADCRSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VADCRSR {
        match value {
            false => VADCRSR::VALUE1,
            true => VADCRSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VADCRSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VADCRSR::VALUE2
    }
}
#[doc = "Possible values of the field `DSDRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSDRSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl DSDRSR {
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
            DSDRSR::VALUE1 => false,
            DSDRSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DSDRSR {
        match value {
            false => DSDRSR::VALUE1,
            true => DSDRSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DSDRSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DSDRSR::VALUE2
    }
}
#[doc = "Possible values of the field `CCU40RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU40RSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl CCU40RSR {
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
            CCU40RSR::VALUE1 => false,
            CCU40RSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCU40RSR {
        match value {
            false => CCU40RSR::VALUE1,
            true => CCU40RSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CCU40RSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CCU40RSR::VALUE2
    }
}
#[doc = "Possible values of the field `CCU41RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU41RSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl CCU41RSR {
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
            CCU41RSR::VALUE1 => false,
            CCU41RSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCU41RSR {
        match value {
            false => CCU41RSR::VALUE1,
            true => CCU41RSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CCU41RSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CCU41RSR::VALUE2
    }
}
#[doc = "Possible values of the field `CCU42RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU42RSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl CCU42RSR {
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
            CCU42RSR::VALUE1 => false,
            CCU42RSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCU42RSR {
        match value {
            false => CCU42RSR::VALUE1,
            true => CCU42RSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CCU42RSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CCU42RSR::VALUE2
    }
}
#[doc = "Possible values of the field `CCU80RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU80RSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl CCU80RSR {
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
            CCU80RSR::VALUE1 => false,
            CCU80RSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCU80RSR {
        match value {
            false => CCU80RSR::VALUE1,
            true => CCU80RSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CCU80RSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CCU80RSR::VALUE2
    }
}
#[doc = "Possible values of the field `CCU81RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU81RSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl CCU81RSR {
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
            CCU81RSR::VALUE1 => false,
            CCU81RSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCU81RSR {
        match value {
            false => CCU81RSR::VALUE1,
            true => CCU81RSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CCU81RSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CCU81RSR::VALUE2
    }
}
#[doc = "Possible values of the field `POSIF0RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSIF0RSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl POSIF0RSR {
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
            POSIF0RSR::VALUE1 => false,
            POSIF0RSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POSIF0RSR {
        match value {
            false => POSIF0RSR::VALUE1,
            true => POSIF0RSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == POSIF0RSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == POSIF0RSR::VALUE2
    }
}
#[doc = "Possible values of the field `POSIF1RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSIF1RSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl POSIF1RSR {
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
            POSIF1RSR::VALUE1 => false,
            POSIF1RSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POSIF1RSR {
        match value {
            false => POSIF1RSR::VALUE1,
            true => POSIF1RSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == POSIF1RSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == POSIF1RSR::VALUE2
    }
}
#[doc = "Possible values of the field `USIC0RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC0RSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl USIC0RSR {
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
            USIC0RSR::VALUE1 => false,
            USIC0RSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USIC0RSR {
        match value {
            false => USIC0RSR::VALUE1,
            true => USIC0RSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == USIC0RSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == USIC0RSR::VALUE2
    }
}
#[doc = "Possible values of the field `ERU1RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERU1RSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl ERU1RSR {
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
            ERU1RSR::VALUE1 => false,
            ERU1RSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERU1RSR {
        match value {
            false => ERU1RSR::VALUE1,
            true => ERU1RSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ERU1RSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ERU1RSR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - VADC Reset Status"]
    #[inline]
    pub fn vadcrs(&self) -> VADCRSR {
        VADCRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - DSD Reset Status"]
    #[inline]
    pub fn dsdrs(&self) -> DSDRSR {
        DSDRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - CCU40 Reset Status"]
    #[inline]
    pub fn ccu40rs(&self) -> CCU40RSR {
        CCU40RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - CCU41 Reset Status"]
    #[inline]
    pub fn ccu41rs(&self) -> CCU41RSR {
        CCU41RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - CCU42 Reset Status"]
    #[inline]
    pub fn ccu42rs(&self) -> CCU42RSR {
        CCU42RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - CCU80 Reset Status"]
    #[inline]
    pub fn ccu80rs(&self) -> CCU80RSR {
        CCU80RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - CCU81 Reset Status"]
    #[inline]
    pub fn ccu81rs(&self) -> CCU81RSR {
        CCU81RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - POSIF0 Reset Status"]
    #[inline]
    pub fn posif0rs(&self) -> POSIF0RSR {
        POSIF0RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - POSIF1 Reset Status"]
    #[inline]
    pub fn posif1rs(&self) -> POSIF1RSR {
        POSIF1RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - USIC0 Reset Status"]
    #[inline]
    pub fn usic0rs(&self) -> USIC0RSR {
        USIC0RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - ERU1 Reset Status"]
    #[inline]
    pub fn eru1rs(&self) -> ERU1RSR {
        ERU1RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
