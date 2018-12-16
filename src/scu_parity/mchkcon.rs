#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCHKCON {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SELPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELPSR {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl SELPSR {
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
            SELPSR::VALUE1 => false,
            SELPSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELPSR {
        match value {
            false => SELPSR::VALUE1,
            true => SELPSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SELPSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SELPSR::VALUE2
    }
}
#[doc = "Possible values of the field `SELDS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELDS1R {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl SELDS1R {
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
            SELDS1R::VALUE1 => false,
            SELDS1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELDS1R {
        match value {
            false => SELDS1R::VALUE1,
            true => SELDS1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SELDS1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SELDS1R::VALUE2
    }
}
#[doc = "Possible values of the field `SELDS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELDS2R {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl SELDS2R {
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
            SELDS2R::VALUE1 => false,
            SELDS2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELDS2R {
        match value {
            false => SELDS2R::VALUE1,
            true => SELDS2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SELDS2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SELDS2R::VALUE2
    }
}
#[doc = "Possible values of the field `USIC0DRA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC0DRAR {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl USIC0DRAR {
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
            USIC0DRAR::VALUE1 => false,
            USIC0DRAR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USIC0DRAR {
        match value {
            false => USIC0DRAR::VALUE1,
            true => USIC0DRAR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == USIC0DRAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == USIC0DRAR::VALUE2
    }
}
#[doc = "Possible values of the field `USIC1DRA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC1DRAR {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl USIC1DRAR {
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
            USIC1DRAR::VALUE1 => false,
            USIC1DRAR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USIC1DRAR {
        match value {
            false => USIC1DRAR::VALUE1,
            true => USIC1DRAR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == USIC1DRAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == USIC1DRAR::VALUE2
    }
}
#[doc = "Possible values of the field `USIC2DRA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC2DRAR {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl USIC2DRAR {
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
            USIC2DRAR::VALUE1 => false,
            USIC2DRAR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USIC2DRAR {
        match value {
            false => USIC2DRAR::VALUE1,
            true => USIC2DRAR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == USIC2DRAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == USIC2DRAR::VALUE2
    }
}
#[doc = "Possible values of the field `MCANDRA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCANDRAR {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl MCANDRAR {
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
            MCANDRAR::VALUE1 => false,
            MCANDRAR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCANDRAR {
        match value {
            false => MCANDRAR::VALUE1,
            true => MCANDRAR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MCANDRAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MCANDRAR::VALUE2
    }
}
#[doc = "Possible values of the field `PPRFDRA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPRFDRAR {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl PPRFDRAR {
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
            PPRFDRAR::VALUE1 => false,
            PPRFDRAR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPRFDRAR {
        match value {
            false => PPRFDRAR::VALUE1,
            true => PPRFDRAR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PPRFDRAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PPRFDRAR::VALUE2
    }
}
#[doc = "Possible values of the field `SELUSB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELUSBR {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl SELUSBR {
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
            SELUSBR::VALUE1 => false,
            SELUSBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELUSBR {
        match value {
            false => SELUSBR::VALUE1,
            true => SELUSBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SELUSBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SELUSBR::VALUE2
    }
}
#[doc = "Possible values of the field `SELETH0TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELETH0TXR {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl SELETH0TXR {
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
            SELETH0TXR::VALUE1 => false,
            SELETH0TXR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELETH0TXR {
        match value {
            false => SELETH0TXR::VALUE1,
            true => SELETH0TXR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SELETH0TXR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SELETH0TXR::VALUE2
    }
}
#[doc = "Possible values of the field `SELETH0RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELETH0RXR {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl SELETH0RXR {
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
            SELETH0RXR::VALUE1 => false,
            SELETH0RXR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELETH0RXR {
        match value {
            false => SELETH0RXR::VALUE1,
            true => SELETH0RXR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SELETH0RXR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SELETH0RXR::VALUE2
    }
}
#[doc = "Possible values of the field `SELSD0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELSD0R {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl SELSD0R {
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
            SELSD0R::VALUE1 => false,
            SELSD0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELSD0R {
        match value {
            false => SELSD0R::VALUE1,
            true => SELSD0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SELSD0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SELSD0R::VALUE2
    }
}
#[doc = "Possible values of the field `SELSD1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELSD1R {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl SELSD1R {
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
            SELSD1R::VALUE1 => false,
            SELSD1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELSD1R {
        match value {
            false => SELSD1R::VALUE1,
            true => SELSD1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SELSD1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SELSD1R::VALUE2
    }
}
#[doc = "Possible values of the field `SELECAT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELECAT0R {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl SELECAT0R {
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
            SELECAT0R::VALUE1 => false,
            SELECAT0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELECAT0R {
        match value {
            false => SELECAT0R::VALUE1,
            true => SELECAT0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SELECAT0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SELECAT0R::VALUE2
    }
}
#[doc = "Values that can be written to the field `SELPS`"]
pub enum SELPSW {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl SELPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELPSW::VALUE1 => false,
            SELPSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELPSW<'a> {
    w: &'a mut W,
}
impl<'a> _SELPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELPSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELPSW::VALUE1)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELPSW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELDS1`"]
pub enum SELDS1W {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl SELDS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELDS1W::VALUE1 => false,
            SELDS1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELDS1W<'a> {
    w: &'a mut W,
}
impl<'a> _SELDS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELDS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELDS1W::VALUE1)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELDS1W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELDS2`"]
pub enum SELDS2W {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl SELDS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELDS2W::VALUE1 => false,
            SELDS2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELDS2W<'a> {
    w: &'a mut W,
}
impl<'a> _SELDS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELDS2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELDS2W::VALUE1)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELDS2W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USIC0DRA`"]
pub enum USIC0DRAW {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl USIC0DRAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USIC0DRAW::VALUE1 => false,
            USIC0DRAW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USIC0DRAW<'a> {
    w: &'a mut W,
}
impl<'a> _USIC0DRAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USIC0DRAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(USIC0DRAW::VALUE1)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(USIC0DRAW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USIC1DRA`"]
pub enum USIC1DRAW {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl USIC1DRAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USIC1DRAW::VALUE1 => false,
            USIC1DRAW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USIC1DRAW<'a> {
    w: &'a mut W,
}
impl<'a> _USIC1DRAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USIC1DRAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(USIC1DRAW::VALUE1)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(USIC1DRAW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USIC2DRA`"]
pub enum USIC2DRAW {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl USIC2DRAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USIC2DRAW::VALUE1 => false,
            USIC2DRAW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USIC2DRAW<'a> {
    w: &'a mut W,
}
impl<'a> _USIC2DRAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USIC2DRAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(USIC2DRAW::VALUE1)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(USIC2DRAW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MCANDRA`"]
pub enum MCANDRAW {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl MCANDRAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCANDRAW::VALUE1 => false,
            MCANDRAW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCANDRAW<'a> {
    w: &'a mut W,
}
impl<'a> _MCANDRAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCANDRAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCANDRAW::VALUE1)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCANDRAW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PPRFDRA`"]
pub enum PPRFDRAW {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl PPRFDRAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPRFDRAW::VALUE1 => false,
            PPRFDRAW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPRFDRAW<'a> {
    w: &'a mut W,
}
impl<'a> _PPRFDRAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPRFDRAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPRFDRAW::VALUE1)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPRFDRAW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELUSB`"]
pub enum SELUSBW {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl SELUSBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELUSBW::VALUE1 => false,
            SELUSBW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELUSBW<'a> {
    w: &'a mut W,
}
impl<'a> _SELUSBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELUSBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELUSBW::VALUE1)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELUSBW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELETH0TX`"]
pub enum SELETH0TXW {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl SELETH0TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELETH0TXW::VALUE1 => false,
            SELETH0TXW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELETH0TXW<'a> {
    w: &'a mut W,
}
impl<'a> _SELETH0TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELETH0TXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELETH0TXW::VALUE1)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELETH0TXW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELETH0RX`"]
pub enum SELETH0RXW {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl SELETH0RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELETH0RXW::VALUE1 => false,
            SELETH0RXW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELETH0RXW<'a> {
    w: &'a mut W,
}
impl<'a> _SELETH0RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELETH0RXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELETH0RXW::VALUE1)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELETH0RXW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELSD0`"]
pub enum SELSD0W {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl SELSD0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELSD0W::VALUE1 => false,
            SELSD0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELSD0W<'a> {
    w: &'a mut W,
}
impl<'a> _SELSD0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELSD0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELSD0W::VALUE1)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELSD0W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELSD1`"]
pub enum SELSD1W {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl SELSD1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELSD1W::VALUE1 => false,
            SELSD1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELSD1W<'a> {
    w: &'a mut W,
}
impl<'a> _SELSD1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELSD1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELSD1W::VALUE1)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELSD1W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELECAT0`"]
pub enum SELECAT0W {
    #[doc = "Not selected"]
    VALUE1,
    #[doc = "Selected"]
    VALUE2,
}
impl SELECAT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELECAT0W::VALUE1 => false,
            SELECAT0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELECAT0W<'a> {
    w: &'a mut W,
}
impl<'a> _SELECAT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELECAT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELECAT0W::VALUE1)
    }
    #[doc = "Selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELECAT0W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Select Memory Check for PSRAM"]
    #[inline]
    pub fn selps(&self) -> SELPSR {
        SELPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Select Memory Check for DSRAM1"]
    #[inline]
    pub fn selds1(&self) -> SELDS1R {
        SELDS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Select Memory Check for DSRAM2"]
    #[inline]
    pub fn selds2(&self) -> SELDS2R {
        SELDS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Select Memory Check for USIC0"]
    #[inline]
    pub fn usic0dra(&self) -> USIC0DRAR {
        USIC0DRAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Select Memory Check for USIC1"]
    #[inline]
    pub fn usic1dra(&self) -> USIC1DRAR {
        USIC1DRAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Select Memory Check for USIC2"]
    #[inline]
    pub fn usic2dra(&self) -> USIC2DRAR {
        USIC2DRAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Select Memory Check for MultiCAN"]
    #[inline]
    pub fn mcandra(&self) -> MCANDRAR {
        MCANDRAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Select Memory Check for PMU"]
    #[inline]
    pub fn pprfdra(&self) -> PPRFDRAR {
        PPRFDRAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Select Memory Check for USB SRAM"]
    #[inline]
    pub fn selusb(&self) -> SELUSBR {
        SELUSBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Select Memory Check for ETH0 TX SRAM"]
    #[inline]
    pub fn seleth0tx(&self) -> SELETH0TXR {
        SELETH0TXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Select Memory Check for ETH0 RX SRAM"]
    #[inline]
    pub fn seleth0rx(&self) -> SELETH0RXR {
        SELETH0RXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Select Memory Check for SDMMC SRAM 0"]
    #[inline]
    pub fn selsd0(&self) -> SELSD0R {
        SELSD0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Select Memory Check for SDMMC SRAM 1"]
    #[inline]
    pub fn selsd1(&self) -> SELSD1R {
        SELSD1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Select Memory Check for ECAT0 SRAM 1"]
    #[inline]
    pub fn selecat0(&self) -> SELECAT0R {
        SELECAT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Select Memory Check for PSRAM"]
    #[inline]
    pub fn selps(&mut self) -> _SELPSW {
        _SELPSW { w: self }
    }
    #[doc = "Bit 1 - Select Memory Check for DSRAM1"]
    #[inline]
    pub fn selds1(&mut self) -> _SELDS1W {
        _SELDS1W { w: self }
    }
    #[doc = "Bit 2 - Select Memory Check for DSRAM2"]
    #[inline]
    pub fn selds2(&mut self) -> _SELDS2W {
        _SELDS2W { w: self }
    }
    #[doc = "Bit 8 - Select Memory Check for USIC0"]
    #[inline]
    pub fn usic0dra(&mut self) -> _USIC0DRAW {
        _USIC0DRAW { w: self }
    }
    #[doc = "Bit 9 - Select Memory Check for USIC1"]
    #[inline]
    pub fn usic1dra(&mut self) -> _USIC1DRAW {
        _USIC1DRAW { w: self }
    }
    #[doc = "Bit 10 - Select Memory Check for USIC2"]
    #[inline]
    pub fn usic2dra(&mut self) -> _USIC2DRAW {
        _USIC2DRAW { w: self }
    }
    #[doc = "Bit 12 - Select Memory Check for MultiCAN"]
    #[inline]
    pub fn mcandra(&mut self) -> _MCANDRAW {
        _MCANDRAW { w: self }
    }
    #[doc = "Bit 13 - Select Memory Check for PMU"]
    #[inline]
    pub fn pprfdra(&mut self) -> _PPRFDRAW {
        _PPRFDRAW { w: self }
    }
    #[doc = "Bit 16 - Select Memory Check for USB SRAM"]
    #[inline]
    pub fn selusb(&mut self) -> _SELUSBW {
        _SELUSBW { w: self }
    }
    #[doc = "Bit 17 - Select Memory Check for ETH0 TX SRAM"]
    #[inline]
    pub fn seleth0tx(&mut self) -> _SELETH0TXW {
        _SELETH0TXW { w: self }
    }
    #[doc = "Bit 18 - Select Memory Check for ETH0 RX SRAM"]
    #[inline]
    pub fn seleth0rx(&mut self) -> _SELETH0RXW {
        _SELETH0RXW { w: self }
    }
    #[doc = "Bit 19 - Select Memory Check for SDMMC SRAM 0"]
    #[inline]
    pub fn selsd0(&mut self) -> _SELSD0W {
        _SELSD0W { w: self }
    }
    #[doc = "Bit 20 - Select Memory Check for SDMMC SRAM 1"]
    #[inline]
    pub fn selsd1(&mut self) -> _SELSD1W {
        _SELSD1W { w: self }
    }
    #[doc = "Bit 24 - Select Memory Check for ECAT0 SRAM 1"]
    #[inline]
    pub fn selecat0(&mut self) -> _SELECAT0W {
        _SELECAT0W { w: self }
    }
}
