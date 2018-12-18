#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PETE {
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
#[doc = "Possible values of the field `PETEPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEPSR {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEPSR {
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
            PETEPSR::VALUE1 => false,
            PETEPSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PETEPSR {
        match value {
            false => PETEPSR::VALUE1,
            true => PETEPSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PETEPSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PETEPSR::VALUE2
    }
}
#[doc = "Possible values of the field `PETEDS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEDS1R {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEDS1R {
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
            PETEDS1R::VALUE1 => false,
            PETEDS1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PETEDS1R {
        match value {
            false => PETEDS1R::VALUE1,
            true => PETEDS1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PETEDS1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PETEDS1R::VALUE2
    }
}
#[doc = "Possible values of the field `PETEDS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEDS2R {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEDS2R {
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
            PETEDS2R::VALUE1 => false,
            PETEDS2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PETEDS2R {
        match value {
            false => PETEDS2R::VALUE1,
            true => PETEDS2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PETEDS2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PETEDS2R::VALUE2
    }
}
#[doc = "Possible values of the field `PETEU0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEU0R {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEU0R {
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
            PETEU0R::VALUE1 => false,
            PETEU0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PETEU0R {
        match value {
            false => PETEU0R::VALUE1,
            true => PETEU0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PETEU0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PETEU0R::VALUE2
    }
}
#[doc = "Possible values of the field `PETEU1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEU1R {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEU1R {
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
            PETEU1R::VALUE1 => false,
            PETEU1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PETEU1R {
        match value {
            false => PETEU1R::VALUE1,
            true => PETEU1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PETEU1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PETEU1R::VALUE2
    }
}
#[doc = "Possible values of the field `PETEU2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEU2R {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEU2R {
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
            PETEU2R::VALUE1 => false,
            PETEU2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PETEU2R {
        match value {
            false => PETEU2R::VALUE1,
            true => PETEU2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PETEU2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PETEU2R::VALUE2
    }
}
#[doc = "Possible values of the field `PETEMC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEMCR {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEMCR {
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
            PETEMCR::VALUE1 => false,
            PETEMCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PETEMCR {
        match value {
            false => PETEMCR::VALUE1,
            true => PETEMCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PETEMCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PETEMCR::VALUE2
    }
}
#[doc = "Possible values of the field `PETEPPRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEPPRFR {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEPPRFR {
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
            PETEPPRFR::VALUE1 => false,
            PETEPPRFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PETEPPRFR {
        match value {
            false => PETEPPRFR::VALUE1,
            true => PETEPPRFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PETEPPRFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PETEPPRFR::VALUE2
    }
}
#[doc = "Possible values of the field `PETEUSB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEUSBR {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEUSBR {
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
            PETEUSBR::VALUE1 => false,
            PETEUSBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PETEUSBR {
        match value {
            false => PETEUSBR::VALUE1,
            true => PETEUSBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PETEUSBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PETEUSBR::VALUE2
    }
}
#[doc = "Possible values of the field `PETEETH0TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEETH0TXR {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEETH0TXR {
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
            PETEETH0TXR::VALUE1 => false,
            PETEETH0TXR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PETEETH0TXR {
        match value {
            false => PETEETH0TXR::VALUE1,
            true => PETEETH0TXR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PETEETH0TXR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PETEETH0TXR::VALUE2
    }
}
#[doc = "Possible values of the field `PETEETH0RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEETH0RXR {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEETH0RXR {
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
            PETEETH0RXR::VALUE1 => false,
            PETEETH0RXR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PETEETH0RXR {
        match value {
            false => PETEETH0RXR::VALUE1,
            true => PETEETH0RXR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PETEETH0RXR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PETEETH0RXR::VALUE2
    }
}
#[doc = "Possible values of the field `PETESD0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETESD0R {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETESD0R {
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
            PETESD0R::VALUE1 => false,
            PETESD0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PETESD0R {
        match value {
            false => PETESD0R::VALUE1,
            true => PETESD0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PETESD0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PETESD0R::VALUE2
    }
}
#[doc = "Possible values of the field `PETESD1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETESD1R {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETESD1R {
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
            PETESD1R::VALUE1 => false,
            PETESD1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PETESD1R {
        match value {
            false => PETESD1R::VALUE1,
            true => PETESD1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PETESD1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PETESD1R::VALUE2
    }
}
#[doc = "Possible values of the field `PETEECAT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEECAT0R {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEECAT0R {
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
            PETEECAT0R::VALUE1 => false,
            PETEECAT0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PETEECAT0R {
        match value {
            false => PETEECAT0R::VALUE1,
            true => PETEECAT0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PETEECAT0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PETEECAT0R::VALUE2
    }
}
#[doc = "Values that can be written to the field `PETEPS`"]
pub enum PETEPSW {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PETEPSW::VALUE1 => false,
            PETEPSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PETEPSW<'a> {
    w: &'a mut W,
}
impl<'a> _PETEPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PETEPSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEPSW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEPSW::VALUE2)
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
#[doc = "Values that can be written to the field `PETEDS1`"]
pub enum PETEDS1W {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEDS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PETEDS1W::VALUE1 => false,
            PETEDS1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PETEDS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PETEDS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PETEDS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEDS1W::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEDS1W::VALUE2)
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
#[doc = "Values that can be written to the field `PETEDS2`"]
pub enum PETEDS2W {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEDS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PETEDS2W::VALUE1 => false,
            PETEDS2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PETEDS2W<'a> {
    w: &'a mut W,
}
impl<'a> _PETEDS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PETEDS2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEDS2W::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEDS2W::VALUE2)
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
#[doc = "Values that can be written to the field `PETEU0`"]
pub enum PETEU0W {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEU0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PETEU0W::VALUE1 => false,
            PETEU0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PETEU0W<'a> {
    w: &'a mut W,
}
impl<'a> _PETEU0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PETEU0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEU0W::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEU0W::VALUE2)
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
#[doc = "Values that can be written to the field `PETEU1`"]
pub enum PETEU1W {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEU1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PETEU1W::VALUE1 => false,
            PETEU1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PETEU1W<'a> {
    w: &'a mut W,
}
impl<'a> _PETEU1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PETEU1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEU1W::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEU1W::VALUE2)
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
#[doc = "Values that can be written to the field `PETEU2`"]
pub enum PETEU2W {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEU2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PETEU2W::VALUE1 => false,
            PETEU2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PETEU2W<'a> {
    w: &'a mut W,
}
impl<'a> _PETEU2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PETEU2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEU2W::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEU2W::VALUE2)
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
#[doc = "Values that can be written to the field `PETEMC`"]
pub enum PETEMCW {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEMCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PETEMCW::VALUE1 => false,
            PETEMCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PETEMCW<'a> {
    w: &'a mut W,
}
impl<'a> _PETEMCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PETEMCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEMCW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEMCW::VALUE2)
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
#[doc = "Values that can be written to the field `PETEPPRF`"]
pub enum PETEPPRFW {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEPPRFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PETEPPRFW::VALUE1 => false,
            PETEPPRFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PETEPPRFW<'a> {
    w: &'a mut W,
}
impl<'a> _PETEPPRFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PETEPPRFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEPPRFW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEPPRFW::VALUE2)
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
#[doc = "Values that can be written to the field `PETEUSB`"]
pub enum PETEUSBW {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEUSBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PETEUSBW::VALUE1 => false,
            PETEUSBW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PETEUSBW<'a> {
    w: &'a mut W,
}
impl<'a> _PETEUSBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PETEUSBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEUSBW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEUSBW::VALUE2)
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
#[doc = "Values that can be written to the field `PETEETH0TX`"]
pub enum PETEETH0TXW {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEETH0TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PETEETH0TXW::VALUE1 => false,
            PETEETH0TXW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PETEETH0TXW<'a> {
    w: &'a mut W,
}
impl<'a> _PETEETH0TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PETEETH0TXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEETH0TXW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEETH0TXW::VALUE2)
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
#[doc = "Values that can be written to the field `PETEETH0RX`"]
pub enum PETEETH0RXW {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEETH0RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PETEETH0RXW::VALUE1 => false,
            PETEETH0RXW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PETEETH0RXW<'a> {
    w: &'a mut W,
}
impl<'a> _PETEETH0RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PETEETH0RXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEETH0RXW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEETH0RXW::VALUE2)
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
#[doc = "Values that can be written to the field `PETESD0`"]
pub enum PETESD0W {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETESD0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PETESD0W::VALUE1 => false,
            PETESD0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PETESD0W<'a> {
    w: &'a mut W,
}
impl<'a> _PETESD0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PETESD0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETESD0W::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETESD0W::VALUE2)
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
#[doc = "Values that can be written to the field `PETESD1`"]
pub enum PETESD1W {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETESD1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PETESD1W::VALUE1 => false,
            PETESD1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PETESD1W<'a> {
    w: &'a mut W,
}
impl<'a> _PETESD1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PETESD1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETESD1W::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETESD1W::VALUE2)
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
#[doc = "Values that can be written to the field `PETEECAT0`"]
pub enum PETEECAT0W {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PETEECAT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PETEECAT0W::VALUE1 => false,
            PETEECAT0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PETEECAT0W<'a> {
    w: &'a mut W,
}
impl<'a> _PETEECAT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PETEECAT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEECAT0W::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEECAT0W::VALUE2)
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
    #[doc = "Bit 0 - Parity Error Trap Enable for PSRAM"]
    #[inline]
    pub fn peteps(&self) -> PETEPSR {
        PETEPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Parity Error Trap Enable for DSRAM1"]
    #[inline]
    pub fn peteds1(&self) -> PETEDS1R {
        PETEDS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Parity Error Trap Enable for DSRAM2"]
    #[inline]
    pub fn peteds2(&self) -> PETEDS2R {
        PETEDS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Parity Error Trap Enable for USIC0 Memory"]
    #[inline]
    pub fn peteu0(&self) -> PETEU0R {
        PETEU0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Parity Error Trap Enable for USIC1 Memory"]
    #[inline]
    pub fn peteu1(&self) -> PETEU1R {
        PETEU1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Parity Error Trap Enable for USIC2 Memory"]
    #[inline]
    pub fn peteu2(&self) -> PETEU2R {
        PETEU2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Parity Error Trap Enable for MultiCAN Memory"]
    #[inline]
    pub fn petemc(&self) -> PETEMCR {
        PETEMCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Parity Error Trap Enable for PMU Prefetch Memory"]
    #[inline]
    pub fn petepprf(&self) -> PETEPPRFR {
        PETEPPRFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Parity Error Trap Enable for USB Memory"]
    #[inline]
    pub fn peteusb(&self) -> PETEUSBR {
        PETEUSBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Parity Error Trap Enable for ETH 0TX Memory"]
    #[inline]
    pub fn peteeth0tx(&self) -> PETEETH0TXR {
        PETEETH0TXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Parity Error Trap Enable for ETH0 RX Memory"]
    #[inline]
    pub fn peteeth0rx(&self) -> PETEETH0RXR {
        PETEETH0RXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Parity Error Trap Enable for SDMMC SRAM 0 Memory"]
    #[inline]
    pub fn petesd0(&self) -> PETESD0R {
        PETESD0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Parity Error Trap Enable for SDMMC SRAM 1 Memory"]
    #[inline]
    pub fn petesd1(&self) -> PETESD1R {
        PETESD1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Parity Error Trap Enable for ECAT0 SRAM Memory"]
    #[inline]
    pub fn peteecat0(&self) -> PETEECAT0R {
        PETEECAT0R::_from({
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
    #[doc = "Bit 0 - Parity Error Trap Enable for PSRAM"]
    #[inline]
    pub fn peteps(&mut self) -> _PETEPSW {
        _PETEPSW { w: self }
    }
    #[doc = "Bit 1 - Parity Error Trap Enable for DSRAM1"]
    #[inline]
    pub fn peteds1(&mut self) -> _PETEDS1W {
        _PETEDS1W { w: self }
    }
    #[doc = "Bit 2 - Parity Error Trap Enable for DSRAM2"]
    #[inline]
    pub fn peteds2(&mut self) -> _PETEDS2W {
        _PETEDS2W { w: self }
    }
    #[doc = "Bit 8 - Parity Error Trap Enable for USIC0 Memory"]
    #[inline]
    pub fn peteu0(&mut self) -> _PETEU0W {
        _PETEU0W { w: self }
    }
    #[doc = "Bit 9 - Parity Error Trap Enable for USIC1 Memory"]
    #[inline]
    pub fn peteu1(&mut self) -> _PETEU1W {
        _PETEU1W { w: self }
    }
    #[doc = "Bit 10 - Parity Error Trap Enable for USIC2 Memory"]
    #[inline]
    pub fn peteu2(&mut self) -> _PETEU2W {
        _PETEU2W { w: self }
    }
    #[doc = "Bit 12 - Parity Error Trap Enable for MultiCAN Memory"]
    #[inline]
    pub fn petemc(&mut self) -> _PETEMCW {
        _PETEMCW { w: self }
    }
    #[doc = "Bit 13 - Parity Error Trap Enable for PMU Prefetch Memory"]
    #[inline]
    pub fn petepprf(&mut self) -> _PETEPPRFW {
        _PETEPPRFW { w: self }
    }
    #[doc = "Bit 16 - Parity Error Trap Enable for USB Memory"]
    #[inline]
    pub fn peteusb(&mut self) -> _PETEUSBW {
        _PETEUSBW { w: self }
    }
    #[doc = "Bit 17 - Parity Error Trap Enable for ETH 0TX Memory"]
    #[inline]
    pub fn peteeth0tx(&mut self) -> _PETEETH0TXW {
        _PETEETH0TXW { w: self }
    }
    #[doc = "Bit 18 - Parity Error Trap Enable for ETH0 RX Memory"]
    #[inline]
    pub fn peteeth0rx(&mut self) -> _PETEETH0RXW {
        _PETEETH0RXW { w: self }
    }
    #[doc = "Bit 19 - Parity Error Trap Enable for SDMMC SRAM 0 Memory"]
    #[inline]
    pub fn petesd0(&mut self) -> _PETESD0W {
        _PETESD0W { w: self }
    }
    #[doc = "Bit 20 - Parity Error Trap Enable for SDMMC SRAM 1 Memory"]
    #[inline]
    pub fn petesd1(&mut self) -> _PETESD1W {
        _PETESD1W { w: self }
    }
    #[doc = "Bit 24 - Parity Error Trap Enable for ECAT0 SRAM Memory"]
    #[inline]
    pub fn peteecat0(&mut self) -> _PETEECAT0W {
        _PETEECAT0W { w: self }
    }
}
