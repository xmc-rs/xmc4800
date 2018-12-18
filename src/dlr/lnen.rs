#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LNEN {
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
#[doc = "Possible values of the field `LN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LN0R {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN0R {
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
            LN0R::VALUE1 => false,
            LN0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LN0R {
        match value {
            false => LN0R::VALUE1,
            true => LN0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LN0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LN0R::VALUE2
    }
}
#[doc = "Possible values of the field `LN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LN1R {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN1R {
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
            LN1R::VALUE1 => false,
            LN1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LN1R {
        match value {
            false => LN1R::VALUE1,
            true => LN1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LN1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LN1R::VALUE2
    }
}
#[doc = "Possible values of the field `LN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LN2R {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN2R {
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
            LN2R::VALUE1 => false,
            LN2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LN2R {
        match value {
            false => LN2R::VALUE1,
            true => LN2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LN2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LN2R::VALUE2
    }
}
#[doc = "Possible values of the field `LN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LN3R {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN3R {
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
            LN3R::VALUE1 => false,
            LN3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LN3R {
        match value {
            false => LN3R::VALUE1,
            true => LN3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LN3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LN3R::VALUE2
    }
}
#[doc = "Possible values of the field `LN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LN4R {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN4R {
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
            LN4R::VALUE1 => false,
            LN4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LN4R {
        match value {
            false => LN4R::VALUE1,
            true => LN4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LN4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LN4R::VALUE2
    }
}
#[doc = "Possible values of the field `LN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LN5R {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN5R {
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
            LN5R::VALUE1 => false,
            LN5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LN5R {
        match value {
            false => LN5R::VALUE1,
            true => LN5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LN5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LN5R::VALUE2
    }
}
#[doc = "Possible values of the field `LN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LN6R {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN6R {
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
            LN6R::VALUE1 => false,
            LN6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LN6R {
        match value {
            false => LN6R::VALUE1,
            true => LN6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LN6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LN6R::VALUE2
    }
}
#[doc = "Possible values of the field `LN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LN7R {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN7R {
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
            LN7R::VALUE1 => false,
            LN7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LN7R {
        match value {
            false => LN7R::VALUE1,
            true => LN7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LN7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LN7R::VALUE2
    }
}
#[doc = "Possible values of the field `LN8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LN8R {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN8R {
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
            LN8R::VALUE1 => false,
            LN8R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LN8R {
        match value {
            false => LN8R::VALUE1,
            true => LN8R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LN8R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LN8R::VALUE2
    }
}
#[doc = "Possible values of the field `LN9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LN9R {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN9R {
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
            LN9R::VALUE1 => false,
            LN9R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LN9R {
        match value {
            false => LN9R::VALUE1,
            true => LN9R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LN9R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LN9R::VALUE2
    }
}
#[doc = "Possible values of the field `LN10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LN10R {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN10R {
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
            LN10R::VALUE1 => false,
            LN10R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LN10R {
        match value {
            false => LN10R::VALUE1,
            true => LN10R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LN10R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LN10R::VALUE2
    }
}
#[doc = "Possible values of the field `LN11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LN11R {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN11R {
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
            LN11R::VALUE1 => false,
            LN11R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LN11R {
        match value {
            false => LN11R::VALUE1,
            true => LN11R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LN11R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LN11R::VALUE2
    }
}
#[doc = "Values that can be written to the field `LN0`"]
pub enum LN0W {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LN0W::VALUE1 => false,
            LN0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LN0W<'a> {
    w: &'a mut W,
}
impl<'a> _LN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the line"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LN0W::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LN0W::VALUE2)
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
#[doc = "Values that can be written to the field `LN1`"]
pub enum LN1W {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LN1W::VALUE1 => false,
            LN1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LN1W<'a> {
    w: &'a mut W,
}
impl<'a> _LN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the line"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LN1W::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LN1W::VALUE2)
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
#[doc = "Values that can be written to the field `LN2`"]
pub enum LN2W {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LN2W::VALUE1 => false,
            LN2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LN2W<'a> {
    w: &'a mut W,
}
impl<'a> _LN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the line"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LN2W::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LN2W::VALUE2)
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
#[doc = "Values that can be written to the field `LN3`"]
pub enum LN3W {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LN3W::VALUE1 => false,
            LN3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LN3W<'a> {
    w: &'a mut W,
}
impl<'a> _LN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the line"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LN3W::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LN3W::VALUE2)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LN4`"]
pub enum LN4W {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LN4W::VALUE1 => false,
            LN4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LN4W<'a> {
    w: &'a mut W,
}
impl<'a> _LN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the line"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LN4W::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LN4W::VALUE2)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LN5`"]
pub enum LN5W {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LN5W::VALUE1 => false,
            LN5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LN5W<'a> {
    w: &'a mut W,
}
impl<'a> _LN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LN5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the line"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LN5W::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LN5W::VALUE2)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LN6`"]
pub enum LN6W {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LN6W::VALUE1 => false,
            LN6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LN6W<'a> {
    w: &'a mut W,
}
impl<'a> _LN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LN6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the line"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LN6W::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LN6W::VALUE2)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LN7`"]
pub enum LN7W {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LN7W::VALUE1 => false,
            LN7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LN7W<'a> {
    w: &'a mut W,
}
impl<'a> _LN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LN7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the line"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LN7W::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LN7W::VALUE2)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LN8`"]
pub enum LN8W {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LN8W::VALUE1 => false,
            LN8W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LN8W<'a> {
    w: &'a mut W,
}
impl<'a> _LN8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LN8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the line"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LN8W::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LN8W::VALUE2)
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
#[doc = "Values that can be written to the field `LN9`"]
pub enum LN9W {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LN9W::VALUE1 => false,
            LN9W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LN9W<'a> {
    w: &'a mut W,
}
impl<'a> _LN9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LN9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the line"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LN9W::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LN9W::VALUE2)
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
#[doc = "Values that can be written to the field `LN10`"]
pub enum LN10W {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LN10W::VALUE1 => false,
            LN10W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LN10W<'a> {
    w: &'a mut W,
}
impl<'a> _LN10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LN10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the line"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LN10W::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LN10W::VALUE2)
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
#[doc = "Values that can be written to the field `LN11`"]
pub enum LN11W {
    #[doc = "Disables the line"]
    VALUE1,
    #[doc = "Enables the line and resets a pending request"]
    VALUE2,
}
impl LN11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LN11W::VALUE1 => false,
            LN11W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LN11W<'a> {
    w: &'a mut W,
}
impl<'a> _LN11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LN11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the line"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LN11W::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LN11W::VALUE2)
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
        const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - Line 0 Enable"]
    #[inline]
    pub fn ln0(&self) -> LN0R {
        LN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Line 1 Enable"]
    #[inline]
    pub fn ln1(&self) -> LN1R {
        LN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Line 2 Enable"]
    #[inline]
    pub fn ln2(&self) -> LN2R {
        LN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Line 3 Enable"]
    #[inline]
    pub fn ln3(&self) -> LN3R {
        LN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Line 4 Enable"]
    #[inline]
    pub fn ln4(&self) -> LN4R {
        LN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Line 5 Enable"]
    #[inline]
    pub fn ln5(&self) -> LN5R {
        LN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Line 6 Enable"]
    #[inline]
    pub fn ln6(&self) -> LN6R {
        LN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Line 7 Enable"]
    #[inline]
    pub fn ln7(&self) -> LN7R {
        LN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Line 8 Enable"]
    #[inline]
    pub fn ln8(&self) -> LN8R {
        LN8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Line 9 Enable"]
    #[inline]
    pub fn ln9(&self) -> LN9R {
        LN9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Line 10 Enable"]
    #[inline]
    pub fn ln10(&self) -> LN10R {
        LN10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Line 11 Enable"]
    #[inline]
    pub fn ln11(&self) -> LN11R {
        LN11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - Line 0 Enable"]
    #[inline]
    pub fn ln0(&mut self) -> _LN0W {
        _LN0W { w: self }
    }
    #[doc = "Bit 1 - Line 1 Enable"]
    #[inline]
    pub fn ln1(&mut self) -> _LN1W {
        _LN1W { w: self }
    }
    #[doc = "Bit 2 - Line 2 Enable"]
    #[inline]
    pub fn ln2(&mut self) -> _LN2W {
        _LN2W { w: self }
    }
    #[doc = "Bit 3 - Line 3 Enable"]
    #[inline]
    pub fn ln3(&mut self) -> _LN3W {
        _LN3W { w: self }
    }
    #[doc = "Bit 4 - Line 4 Enable"]
    #[inline]
    pub fn ln4(&mut self) -> _LN4W {
        _LN4W { w: self }
    }
    #[doc = "Bit 5 - Line 5 Enable"]
    #[inline]
    pub fn ln5(&mut self) -> _LN5W {
        _LN5W { w: self }
    }
    #[doc = "Bit 6 - Line 6 Enable"]
    #[inline]
    pub fn ln6(&mut self) -> _LN6W {
        _LN6W { w: self }
    }
    #[doc = "Bit 7 - Line 7 Enable"]
    #[inline]
    pub fn ln7(&mut self) -> _LN7W {
        _LN7W { w: self }
    }
    #[doc = "Bit 8 - Line 8 Enable"]
    #[inline]
    pub fn ln8(&mut self) -> _LN8W {
        _LN8W { w: self }
    }
    #[doc = "Bit 9 - Line 9 Enable"]
    #[inline]
    pub fn ln9(&mut self) -> _LN9W {
        _LN9W { w: self }
    }
    #[doc = "Bit 10 - Line 10 Enable"]
    #[inline]
    pub fn ln10(&mut self) -> _LN10W {
        _LN10W { w: self }
    }
    #[doc = "Bit 11 - Line 11 Enable"]
    #[inline]
    pub fn ln11(&mut self) -> _LN11W {
        _LN11W { w: self }
    }
}
