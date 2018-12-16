#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMTSR {
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
#[doc = "Possible values of the field `MTENPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTENPSR {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTENPSR {
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
            MTENPSR::VALUE1 => false,
            MTENPSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTENPSR {
        match value {
            false => MTENPSR::VALUE1,
            true => MTENPSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTENPSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTENPSR::VALUE2
    }
}
#[doc = "Possible values of the field `MTENDS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTENDS1R {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTENDS1R {
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
            MTENDS1R::VALUE1 => false,
            MTENDS1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTENDS1R {
        match value {
            false => MTENDS1R::VALUE1,
            true => MTENDS1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTENDS1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTENDS1R::VALUE2
    }
}
#[doc = "Possible values of the field `MTENDS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTENDS2R {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTENDS2R {
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
            MTENDS2R::VALUE1 => false,
            MTENDS2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTENDS2R {
        match value {
            false => MTENDS2R::VALUE1,
            true => MTENDS2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTENDS2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTENDS2R::VALUE2
    }
}
#[doc = "Possible values of the field `MTEU0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEU0R {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTEU0R {
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
            MTEU0R::VALUE1 => false,
            MTEU0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTEU0R {
        match value {
            false => MTEU0R::VALUE1,
            true => MTEU0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTEU0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTEU0R::VALUE2
    }
}
#[doc = "Possible values of the field `MTEU1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEU1R {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTEU1R {
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
            MTEU1R::VALUE1 => false,
            MTEU1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTEU1R {
        match value {
            false => MTEU1R::VALUE1,
            true => MTEU1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTEU1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTEU1R::VALUE2
    }
}
#[doc = "Possible values of the field `MTEU2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEU2R {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTEU2R {
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
            MTEU2R::VALUE1 => false,
            MTEU2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTEU2R {
        match value {
            false => MTEU2R::VALUE1,
            true => MTEU2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTEU2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTEU2R::VALUE2
    }
}
#[doc = "Possible values of the field `MTEMC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEMCR {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTEMCR {
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
            MTEMCR::VALUE1 => false,
            MTEMCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTEMCR {
        match value {
            false => MTEMCR::VALUE1,
            true => MTEMCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTEMCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTEMCR::VALUE2
    }
}
#[doc = "Possible values of the field `MTEPPRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEPPRFR {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTEPPRFR {
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
            MTEPPRFR::VALUE1 => false,
            MTEPPRFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTEPPRFR {
        match value {
            false => MTEPPRFR::VALUE1,
            true => MTEPPRFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTEPPRFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTEPPRFR::VALUE2
    }
}
#[doc = "Possible values of the field `MTUSB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTUSBR {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTUSBR {
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
            MTUSBR::VALUE1 => false,
            MTUSBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTUSBR {
        match value {
            false => MTUSBR::VALUE1,
            true => MTUSBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTUSBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTUSBR::VALUE2
    }
}
#[doc = "Possible values of the field `MTETH0TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTETH0TXR {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTETH0TXR {
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
            MTETH0TXR::VALUE1 => false,
            MTETH0TXR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTETH0TXR {
        match value {
            false => MTETH0TXR::VALUE1,
            true => MTETH0TXR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTETH0TXR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTETH0TXR::VALUE2
    }
}
#[doc = "Possible values of the field `MTETH0RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTETH0RXR {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTETH0RXR {
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
            MTETH0RXR::VALUE1 => false,
            MTETH0RXR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTETH0RXR {
        match value {
            false => MTETH0RXR::VALUE1,
            true => MTETH0RXR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTETH0RXR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTETH0RXR::VALUE2
    }
}
#[doc = "Possible values of the field `MTSD0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTSD0R {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTSD0R {
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
            MTSD0R::VALUE1 => false,
            MTSD0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTSD0R {
        match value {
            false => MTSD0R::VALUE1,
            true => MTSD0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTSD0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTSD0R::VALUE2
    }
}
#[doc = "Possible values of the field `MTSD1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTSD1R {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTSD1R {
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
            MTSD1R::VALUE1 => false,
            MTSD1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTSD1R {
        match value {
            false => MTSD1R::VALUE1,
            true => MTSD1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTSD1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTSD1R::VALUE2
    }
}
#[doc = "Possible values of the field `MTECAT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTECAT0R {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTECAT0R {
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
            MTECAT0R::VALUE1 => false,
            MTECAT0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTECAT0R {
        match value {
            false => MTECAT0R::VALUE1,
            true => MTECAT0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MTECAT0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MTECAT0R::VALUE2
    }
}
#[doc = "Values that can be written to the field `MTENPS`"]
pub enum MTENPSW {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTENPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTENPSW::VALUE1 => false,
            MTENPSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTENPSW<'a> {
    w: &'a mut W,
}
impl<'a> _MTENPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTENPSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTENPSW::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTENPSW::VALUE2)
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
#[doc = "Values that can be written to the field `MTENDS1`"]
pub enum MTENDS1W {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTENDS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTENDS1W::VALUE1 => false,
            MTENDS1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTENDS1W<'a> {
    w: &'a mut W,
}
impl<'a> _MTENDS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTENDS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTENDS1W::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTENDS1W::VALUE2)
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
#[doc = "Values that can be written to the field `MTENDS2`"]
pub enum MTENDS2W {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTENDS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTENDS2W::VALUE1 => false,
            MTENDS2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTENDS2W<'a> {
    w: &'a mut W,
}
impl<'a> _MTENDS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTENDS2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTENDS2W::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTENDS2W::VALUE2)
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
#[doc = "Values that can be written to the field `MTEU0`"]
pub enum MTEU0W {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTEU0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTEU0W::VALUE1 => false,
            MTEU0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTEU0W<'a> {
    w: &'a mut W,
}
impl<'a> _MTEU0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTEU0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTEU0W::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTEU0W::VALUE2)
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
#[doc = "Values that can be written to the field `MTEU1`"]
pub enum MTEU1W {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTEU1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTEU1W::VALUE1 => false,
            MTEU1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTEU1W<'a> {
    w: &'a mut W,
}
impl<'a> _MTEU1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTEU1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTEU1W::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTEU1W::VALUE2)
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
#[doc = "Values that can be written to the field `MTEU2`"]
pub enum MTEU2W {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTEU2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTEU2W::VALUE1 => false,
            MTEU2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTEU2W<'a> {
    w: &'a mut W,
}
impl<'a> _MTEU2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTEU2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTEU2W::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTEU2W::VALUE2)
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
#[doc = "Values that can be written to the field `MTEMC`"]
pub enum MTEMCW {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTEMCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTEMCW::VALUE1 => false,
            MTEMCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTEMCW<'a> {
    w: &'a mut W,
}
impl<'a> _MTEMCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTEMCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTEMCW::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTEMCW::VALUE2)
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
#[doc = "Values that can be written to the field `MTEPPRF`"]
pub enum MTEPPRFW {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTEPPRFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTEPPRFW::VALUE1 => false,
            MTEPPRFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTEPPRFW<'a> {
    w: &'a mut W,
}
impl<'a> _MTEPPRFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTEPPRFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTEPPRFW::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTEPPRFW::VALUE2)
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
#[doc = "Values that can be written to the field `MTUSB`"]
pub enum MTUSBW {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTUSBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTUSBW::VALUE1 => false,
            MTUSBW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTUSBW<'a> {
    w: &'a mut W,
}
impl<'a> _MTUSBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTUSBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTUSBW::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTUSBW::VALUE2)
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
#[doc = "Values that can be written to the field `MTETH0TX`"]
pub enum MTETH0TXW {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTETH0TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTETH0TXW::VALUE1 => false,
            MTETH0TXW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTETH0TXW<'a> {
    w: &'a mut W,
}
impl<'a> _MTETH0TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTETH0TXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTETH0TXW::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTETH0TXW::VALUE2)
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
#[doc = "Values that can be written to the field `MTETH0RX`"]
pub enum MTETH0RXW {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTETH0RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTETH0RXW::VALUE1 => false,
            MTETH0RXW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTETH0RXW<'a> {
    w: &'a mut W,
}
impl<'a> _MTETH0RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTETH0RXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTETH0RXW::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTETH0RXW::VALUE2)
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
#[doc = "Values that can be written to the field `MTSD0`"]
pub enum MTSD0W {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTSD0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTSD0W::VALUE1 => false,
            MTSD0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTSD0W<'a> {
    w: &'a mut W,
}
impl<'a> _MTSD0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTSD0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTSD0W::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTSD0W::VALUE2)
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
#[doc = "Values that can be written to the field `MTSD1`"]
pub enum MTSD1W {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTSD1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTSD1W::VALUE1 => false,
            MTSD1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTSD1W<'a> {
    w: &'a mut W,
}
impl<'a> _MTSD1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTSD1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTSD1W::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTSD1W::VALUE2)
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
#[doc = "Values that can be written to the field `MTECAT0`"]
pub enum MTECAT0W {
    #[doc = "Standard operation"]
    VALUE1,
    #[doc = "Parity bits under test"]
    VALUE2,
}
impl MTECAT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTECAT0W::VALUE1 => false,
            MTECAT0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTECAT0W<'a> {
    w: &'a mut W,
}
impl<'a> _MTECAT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTECAT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTECAT0W::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTECAT0W::VALUE2)
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
    #[doc = "Bit 0 - Test Enable Control for PSRAM"]
    #[inline]
    pub fn mtenps(&self) -> MTENPSR {
        MTENPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Test Enable Control for DSRAM1"]
    #[inline]
    pub fn mtends1(&self) -> MTENDS1R {
        MTENDS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Test Enable Control for DSRAM2"]
    #[inline]
    pub fn mtends2(&self) -> MTENDS2R {
        MTENDS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Test Enable Control for USIC0 Memory"]
    #[inline]
    pub fn mteu0(&self) -> MTEU0R {
        MTEU0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Test Enable Control for USIC1 Memory"]
    #[inline]
    pub fn mteu1(&self) -> MTEU1R {
        MTEU1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Test Enable Control for USIC2 Memory"]
    #[inline]
    pub fn mteu2(&self) -> MTEU2R {
        MTEU2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Test Enable Control for MultiCAN Memory"]
    #[inline]
    pub fn mtemc(&self) -> MTEMCR {
        MTEMCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Test Enable Control for PMU Prefetch Memory"]
    #[inline]
    pub fn mtepprf(&self) -> MTEPPRFR {
        MTEPPRFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Test Enable Control for USB Memory"]
    #[inline]
    pub fn mtusb(&self) -> MTUSBR {
        MTUSBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Test Enable Control for ETH TX Memory"]
    #[inline]
    pub fn mteth0tx(&self) -> MTETH0TXR {
        MTETH0TXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Test Enable Control for ETH RX Memory"]
    #[inline]
    pub fn mteth0rx(&self) -> MTETH0RXR {
        MTETH0RXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Test Enable Control for SDMMC Memory 0"]
    #[inline]
    pub fn mtsd0(&self) -> MTSD0R {
        MTSD0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Test Enable Control for SDMMC Memory 1"]
    #[inline]
    pub fn mtsd1(&self) -> MTSD1R {
        MTSD1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Test Enable Control for ECAT0 Memory"]
    #[inline]
    pub fn mtecat0(&self) -> MTECAT0R {
        MTECAT0R::_from({
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
    #[doc = "Bit 0 - Test Enable Control for PSRAM"]
    #[inline]
    pub fn mtenps(&mut self) -> _MTENPSW {
        _MTENPSW { w: self }
    }
    #[doc = "Bit 1 - Test Enable Control for DSRAM1"]
    #[inline]
    pub fn mtends1(&mut self) -> _MTENDS1W {
        _MTENDS1W { w: self }
    }
    #[doc = "Bit 2 - Test Enable Control for DSRAM2"]
    #[inline]
    pub fn mtends2(&mut self) -> _MTENDS2W {
        _MTENDS2W { w: self }
    }
    #[doc = "Bit 8 - Test Enable Control for USIC0 Memory"]
    #[inline]
    pub fn mteu0(&mut self) -> _MTEU0W {
        _MTEU0W { w: self }
    }
    #[doc = "Bit 9 - Test Enable Control for USIC1 Memory"]
    #[inline]
    pub fn mteu1(&mut self) -> _MTEU1W {
        _MTEU1W { w: self }
    }
    #[doc = "Bit 10 - Test Enable Control for USIC2 Memory"]
    #[inline]
    pub fn mteu2(&mut self) -> _MTEU2W {
        _MTEU2W { w: self }
    }
    #[doc = "Bit 12 - Test Enable Control for MultiCAN Memory"]
    #[inline]
    pub fn mtemc(&mut self) -> _MTEMCW {
        _MTEMCW { w: self }
    }
    #[doc = "Bit 13 - Test Enable Control for PMU Prefetch Memory"]
    #[inline]
    pub fn mtepprf(&mut self) -> _MTEPPRFW {
        _MTEPPRFW { w: self }
    }
    #[doc = "Bit 16 - Test Enable Control for USB Memory"]
    #[inline]
    pub fn mtusb(&mut self) -> _MTUSBW {
        _MTUSBW { w: self }
    }
    #[doc = "Bit 17 - Test Enable Control for ETH TX Memory"]
    #[inline]
    pub fn mteth0tx(&mut self) -> _MTETH0TXW {
        _MTETH0TXW { w: self }
    }
    #[doc = "Bit 18 - Test Enable Control for ETH RX Memory"]
    #[inline]
    pub fn mteth0rx(&mut self) -> _MTETH0RXW {
        _MTETH0RXW { w: self }
    }
    #[doc = "Bit 19 - Test Enable Control for SDMMC Memory 0"]
    #[inline]
    pub fn mtsd0(&mut self) -> _MTSD0W {
        _MTSD0W { w: self }
    }
    #[doc = "Bit 20 - Test Enable Control for SDMMC Memory 1"]
    #[inline]
    pub fn mtsd1(&mut self) -> _MTSD1W {
        _MTSD1W { w: self }
    }
    #[doc = "Bit 24 - Test Enable Control for ECAT0 Memory"]
    #[inline]
    pub fn mtecat0(&mut self) -> _MTECAT0W {
        _MTECAT0W { w: self }
    }
}
