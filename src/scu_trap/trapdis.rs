#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TRAPDIS {
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
#[doc = "Possible values of the field `SOSCWDGT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCWDGTR {
    #[doc = "Trap request enabled"]
    VALUE1,
    #[doc = "Trap request disabled"]
    VALUE2,
}
impl SOSCWDGTR {
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
            SOSCWDGTR::VALUE1 => false,
            SOSCWDGTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOSCWDGTR {
        match value {
            false => SOSCWDGTR::VALUE1,
            true => SOSCWDGTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SOSCWDGTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SOSCWDGTR::VALUE2
    }
}
#[doc = "Possible values of the field `SVCOLCKT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCOLCKTR {
    #[doc = "Trap request enabled"]
    VALUE1,
    #[doc = "Trap request disabled"]
    VALUE2,
}
impl SVCOLCKTR {
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
            SVCOLCKTR::VALUE1 => false,
            SVCOLCKTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SVCOLCKTR {
        match value {
            false => SVCOLCKTR::VALUE1,
            true => SVCOLCKTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SVCOLCKTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SVCOLCKTR::VALUE2
    }
}
#[doc = "Possible values of the field `UVCOLCKT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UVCOLCKTR {
    #[doc = "Trap request enabled"]
    VALUE1,
    #[doc = "Trap request disabled"]
    VALUE2,
}
impl UVCOLCKTR {
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
            UVCOLCKTR::VALUE1 => false,
            UVCOLCKTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UVCOLCKTR {
        match value {
            false => UVCOLCKTR::VALUE1,
            true => UVCOLCKTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == UVCOLCKTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == UVCOLCKTR::VALUE2
    }
}
#[doc = "Possible values of the field `PET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETR {
    #[doc = "Trap request enabled"]
    VALUE1,
    #[doc = "Trap request disabled"]
    VALUE2,
}
impl PETR {
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
            PETR::VALUE1 => false,
            PETR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PETR {
        match value {
            false => PETR::VALUE1,
            true => PETR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PETR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PETR::VALUE2
    }
}
#[doc = "Possible values of the field `BRWNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRWNTR {
    #[doc = "Trap request enabled"]
    VALUE1,
    #[doc = "Trap request disabled"]
    VALUE2,
}
impl BRWNTR {
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
            BRWNTR::VALUE1 => false,
            BRWNTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRWNTR {
        match value {
            false => BRWNTR::VALUE1,
            true => BRWNTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BRWNTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BRWNTR::VALUE2
    }
}
#[doc = "Possible values of the field `ULPWDGT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULPWDGTR {
    #[doc = "Trap request enabled"]
    VALUE1,
    #[doc = "Trap request disabled"]
    VALUE2,
}
impl ULPWDGTR {
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
            ULPWDGTR::VALUE1 => false,
            ULPWDGTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ULPWDGTR {
        match value {
            false => ULPWDGTR::VALUE1,
            true => ULPWDGTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ULPWDGTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ULPWDGTR::VALUE2
    }
}
#[doc = "Possible values of the field `BWERR0T`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWERR0TR {
    #[doc = "Trap request enabled"]
    VALUE1,
    #[doc = "Trap request disabled"]
    VALUE2,
}
impl BWERR0TR {
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
            BWERR0TR::VALUE1 => false,
            BWERR0TR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWERR0TR {
        match value {
            false => BWERR0TR::VALUE1,
            true => BWERR0TR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BWERR0TR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BWERR0TR::VALUE2
    }
}
#[doc = "Possible values of the field `BWERR1T`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWERR1TR {
    #[doc = "Trap request enabled"]
    VALUE1,
    #[doc = "Trap request disabled"]
    VALUE2,
}
impl BWERR1TR {
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
            BWERR1TR::VALUE1 => false,
            BWERR1TR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWERR1TR {
        match value {
            false => BWERR1TR::VALUE1,
            true => BWERR1TR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BWERR1TR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BWERR1TR::VALUE2
    }
}
#[doc = "Possible values of the field `ECAT0RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECAT0RSTR {
    #[doc = "Trap request enabled"]
    VALUE1,
    #[doc = "Trap request disabled"]
    VALUE2,
}
impl ECAT0RSTR {
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
            ECAT0RSTR::VALUE1 => false,
            ECAT0RSTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECAT0RSTR {
        match value {
            false => ECAT0RSTR::VALUE1,
            true => ECAT0RSTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ECAT0RSTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ECAT0RSTR::VALUE2
    }
}
#[doc = "Values that can be written to the field `SOSCWDGT`"]
pub enum SOSCWDGTW {
    #[doc = "Trap request enabled"]
    VALUE1,
    #[doc = "Trap request disabled"]
    VALUE2,
}
impl SOSCWDGTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOSCWDGTW::VALUE1 => false,
            SOSCWDGTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOSCWDGTW<'a> {
    w: &'a mut W,
}
impl<'a> _SOSCWDGTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOSCWDGTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap request enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SOSCWDGTW::VALUE1)
    }
    #[doc = "Trap request disabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SOSCWDGTW::VALUE2)
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
#[doc = "Values that can be written to the field `SVCOLCKT`"]
pub enum SVCOLCKTW {
    #[doc = "Trap request enabled"]
    VALUE1,
    #[doc = "Trap request disabled"]
    VALUE2,
}
impl SVCOLCKTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SVCOLCKTW::VALUE1 => false,
            SVCOLCKTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SVCOLCKTW<'a> {
    w: &'a mut W,
}
impl<'a> _SVCOLCKTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SVCOLCKTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap request enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SVCOLCKTW::VALUE1)
    }
    #[doc = "Trap request disabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SVCOLCKTW::VALUE2)
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
#[doc = "Values that can be written to the field `UVCOLCKT`"]
pub enum UVCOLCKTW {
    #[doc = "Trap request enabled"]
    VALUE1,
    #[doc = "Trap request disabled"]
    VALUE2,
}
impl UVCOLCKTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UVCOLCKTW::VALUE1 => false,
            UVCOLCKTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UVCOLCKTW<'a> {
    w: &'a mut W,
}
impl<'a> _UVCOLCKTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UVCOLCKTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap request enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(UVCOLCKTW::VALUE1)
    }
    #[doc = "Trap request disabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(UVCOLCKTW::VALUE2)
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
#[doc = "Values that can be written to the field `PET`"]
pub enum PETW {
    #[doc = "Trap request enabled"]
    VALUE1,
    #[doc = "Trap request disabled"]
    VALUE2,
}
impl PETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PETW::VALUE1 => false,
            PETW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PETW<'a> {
    w: &'a mut W,
}
impl<'a> _PETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap request enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETW::VALUE1)
    }
    #[doc = "Trap request disabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETW::VALUE2)
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
#[doc = "Values that can be written to the field `BRWNT`"]
pub enum BRWNTW {
    #[doc = "Trap request enabled"]
    VALUE1,
    #[doc = "Trap request disabled"]
    VALUE2,
}
impl BRWNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BRWNTW::VALUE1 => false,
            BRWNTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRWNTW<'a> {
    w: &'a mut W,
}
impl<'a> _BRWNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRWNTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap request enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BRWNTW::VALUE1)
    }
    #[doc = "Trap request disabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BRWNTW::VALUE2)
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
#[doc = "Values that can be written to the field `ULPWDGT`"]
pub enum ULPWDGTW {
    #[doc = "Trap request enabled"]
    VALUE1,
    #[doc = "Trap request disabled"]
    VALUE2,
}
impl ULPWDGTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ULPWDGTW::VALUE1 => false,
            ULPWDGTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ULPWDGTW<'a> {
    w: &'a mut W,
}
impl<'a> _ULPWDGTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ULPWDGTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap request enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ULPWDGTW::VALUE1)
    }
    #[doc = "Trap request disabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ULPWDGTW::VALUE2)
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
#[doc = "Values that can be written to the field `BWERR0T`"]
pub enum BWERR0TW {
    #[doc = "Trap request enabled"]
    VALUE1,
    #[doc = "Trap request disabled"]
    VALUE2,
}
impl BWERR0TW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BWERR0TW::VALUE1 => false,
            BWERR0TW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWERR0TW<'a> {
    w: &'a mut W,
}
impl<'a> _BWERR0TW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWERR0TW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap request enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BWERR0TW::VALUE1)
    }
    #[doc = "Trap request disabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BWERR0TW::VALUE2)
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
#[doc = "Values that can be written to the field `BWERR1T`"]
pub enum BWERR1TW {
    #[doc = "Trap request enabled"]
    VALUE1,
    #[doc = "Trap request disabled"]
    VALUE2,
}
impl BWERR1TW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BWERR1TW::VALUE1 => false,
            BWERR1TW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWERR1TW<'a> {
    w: &'a mut W,
}
impl<'a> _BWERR1TW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWERR1TW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap request enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BWERR1TW::VALUE1)
    }
    #[doc = "Trap request disabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BWERR1TW::VALUE2)
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
#[doc = "Values that can be written to the field `ECAT0RST`"]
pub enum ECAT0RSTW {
    #[doc = "Trap request enabled"]
    VALUE1,
    #[doc = "Trap request disabled"]
    VALUE2,
}
impl ECAT0RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECAT0RSTW::VALUE1 => false,
            ECAT0RSTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECAT0RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ECAT0RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECAT0RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap request enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECAT0RSTW::VALUE1)
    }
    #[doc = "Trap request disabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECAT0RSTW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Disable"]
    #[inline]
    pub fn soscwdgt(&self) -> SOSCWDGTR {
        SOSCWDGTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - System VCO Lock Trap Disable"]
    #[inline]
    pub fn svcolckt(&self) -> SVCOLCKTR {
        SVCOLCKTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Disable"]
    #[inline]
    pub fn uvcolckt(&self) -> UVCOLCKTR {
        UVCOLCKTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Parity Error Trap Disable"]
    #[inline]
    pub fn pet(&self) -> PETR {
        PETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Brown Out Trap Disable"]
    #[inline]
    pub fn brwnt(&self) -> BRWNTR {
        BRWNTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Disable"]
    #[inline]
    pub fn ulpwdgt(&self) -> ULPWDGTR {
        ULPWDGTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Disable"]
    #[inline]
    pub fn bwerr0t(&self) -> BWERR0TR {
        BWERR0TR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Disable"]
    #[inline]
    pub fn bwerr1t(&self) -> BWERR1TR {
        BWERR1TR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - EtherCat Reset 0 Trap Disable"]
    #[inline]
    pub fn ecat0rst(&self) -> ECAT0RSTR {
        ECAT0RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 66045 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Disable"]
    #[inline]
    pub fn soscwdgt(&mut self) -> _SOSCWDGTW {
        _SOSCWDGTW { w: self }
    }
    #[doc = "Bit 2 - System VCO Lock Trap Disable"]
    #[inline]
    pub fn svcolckt(&mut self) -> _SVCOLCKTW {
        _SVCOLCKTW { w: self }
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Disable"]
    #[inline]
    pub fn uvcolckt(&mut self) -> _UVCOLCKTW {
        _UVCOLCKTW { w: self }
    }
    #[doc = "Bit 4 - Parity Error Trap Disable"]
    #[inline]
    pub fn pet(&mut self) -> _PETW {
        _PETW { w: self }
    }
    #[doc = "Bit 5 - Brown Out Trap Disable"]
    #[inline]
    pub fn brwnt(&mut self) -> _BRWNTW {
        _BRWNTW { w: self }
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Disable"]
    #[inline]
    pub fn ulpwdgt(&mut self) -> _ULPWDGTW {
        _ULPWDGTW { w: self }
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Disable"]
    #[inline]
    pub fn bwerr0t(&mut self) -> _BWERR0TW {
        _BWERR0TW { w: self }
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Disable"]
    #[inline]
    pub fn bwerr1t(&mut self) -> _BWERR1TW {
        _BWERR1TW { w: self }
    }
    #[doc = "Bit 16 - EtherCat Reset 0 Trap Disable"]
    #[inline]
    pub fn ecat0rst(&mut self) -> _ECAT0RSTW {
        _ECAT0RSTW { w: self }
    }
}
