#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NMIREQEN {
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
#[doc = "Possible values of the field `PRWARN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRWARNR {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PRWARNR {
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
            PRWARNR::VALUE1 => false,
            PRWARNR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRWARNR {
        match value {
            false => PRWARNR::VALUE1,
            true => PRWARNR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PRWARNR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PRWARNR::VALUE2
    }
}
#[doc = "Possible values of the field `PI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIR {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PIR {
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
            PIR::VALUE1 => false,
            PIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIR {
        match value {
            false => PIR::VALUE1,
            true => PIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PIR::VALUE2
    }
}
#[doc = "Possible values of the field `AI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIR {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl AIR {
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
            AIR::VALUE1 => false,
            AIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AIR {
        match value {
            false => AIR::VALUE1,
            true => AIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AIR::VALUE2
    }
}
#[doc = "Possible values of the field `ERU00`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERU00R {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl ERU00R {
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
            ERU00R::VALUE1 => false,
            ERU00R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERU00R {
        match value {
            false => ERU00R::VALUE1,
            true => ERU00R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ERU00R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ERU00R::VALUE2
    }
}
#[doc = "Possible values of the field `ERU01`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERU01R {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl ERU01R {
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
            ERU01R::VALUE1 => false,
            ERU01R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERU01R {
        match value {
            false => ERU01R::VALUE1,
            true => ERU01R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ERU01R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ERU01R::VALUE2
    }
}
#[doc = "Possible values of the field `ERU02`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERU02R {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl ERU02R {
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
            ERU02R::VALUE1 => false,
            ERU02R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERU02R {
        match value {
            false => ERU02R::VALUE1,
            true => ERU02R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ERU02R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ERU02R::VALUE2
    }
}
#[doc = "Possible values of the field `ERU03`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERU03R {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl ERU03R {
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
            ERU03R::VALUE1 => false,
            ERU03R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERU03R {
        match value {
            false => ERU03R::VALUE1,
            true => ERU03R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ERU03R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ERU03R::VALUE2
    }
}
#[doc = "Values that can be written to the field `PRWARN`"]
pub enum PRWARNW {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PRWARNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRWARNW::VALUE1 => false,
            PRWARNW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRWARNW<'a> {
    w: &'a mut W,
}
impl<'a> _PRWARNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRWARNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRWARNW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRWARNW::VALUE2)
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
#[doc = "Values that can be written to the field `PI`"]
pub enum PIW {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl PIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIW::VALUE1 => false,
            PIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIW<'a> {
    w: &'a mut W,
}
impl<'a> _PIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PIW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PIW::VALUE2)
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
#[doc = "Values that can be written to the field `AI`"]
pub enum AIW {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl AIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AIW::VALUE1 => false,
            AIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AIW<'a> {
    w: &'a mut W,
}
impl<'a> _AIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AIW::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AIW::VALUE2)
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
#[doc = "Values that can be written to the field `ERU00`"]
pub enum ERU00W {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl ERU00W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERU00W::VALUE1 => false,
            ERU00W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERU00W<'a> {
    w: &'a mut W,
}
impl<'a> _ERU00W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERU00W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ERU00W::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ERU00W::VALUE2)
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
#[doc = "Values that can be written to the field `ERU01`"]
pub enum ERU01W {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl ERU01W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERU01W::VALUE1 => false,
            ERU01W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERU01W<'a> {
    w: &'a mut W,
}
impl<'a> _ERU01W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERU01W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ERU01W::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ERU01W::VALUE2)
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
#[doc = "Values that can be written to the field `ERU02`"]
pub enum ERU02W {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl ERU02W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERU02W::VALUE1 => false,
            ERU02W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERU02W<'a> {
    w: &'a mut W,
}
impl<'a> _ERU02W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERU02W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ERU02W::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ERU02W::VALUE2)
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
#[doc = "Values that can be written to the field `ERU03`"]
pub enum ERU03W {
    #[doc = "Disabled"]
    VALUE1,
    #[doc = "Enabled"]
    VALUE2,
}
impl ERU03W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERU03W::VALUE1 => false,
            ERU03W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERU03W<'a> {
    w: &'a mut W,
}
impl<'a> _ERU03W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERU03W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ERU03W::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ERU03W::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Promote Pre-Warning Interrupt Request to NMI Request"]
    #[inline]
    pub fn prwarn(&self) -> PRWARNR {
        PRWARNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Promote RTC Periodic Interrupt request to NMI Request"]
    #[inline]
    pub fn pi(&self) -> PIR {
        PIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Promote RTC Alarm Interrupt Request to NMI Request"]
    #[inline]
    pub fn ai(&self) -> AIR {
        AIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Promote Channel 0 Interrupt of ERU0 Request to NMI Request"]
    #[inline]
    pub fn eru00(&self) -> ERU00R {
        ERU00R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Promote Channel 1 Interrupt of ERU0 Request to NMI Request"]
    #[inline]
    pub fn eru01(&self) -> ERU01R {
        ERU01R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Promote Channel 2 Interrupt of ERU0 Request to NMI Request"]
    #[inline]
    pub fn eru02(&self) -> ERU02R {
        ERU02R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Promote Channel 3 Interrupt of ERU0 Request to NMI Request"]
    #[inline]
    pub fn eru03(&self) -> ERU03R {
        ERU03R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
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
    #[doc = "Bit 0 - Promote Pre-Warning Interrupt Request to NMI Request"]
    #[inline]
    pub fn prwarn(&mut self) -> _PRWARNW {
        _PRWARNW { w: self }
    }
    #[doc = "Bit 1 - Promote RTC Periodic Interrupt request to NMI Request"]
    #[inline]
    pub fn pi(&mut self) -> _PIW {
        _PIW { w: self }
    }
    #[doc = "Bit 2 - Promote RTC Alarm Interrupt Request to NMI Request"]
    #[inline]
    pub fn ai(&mut self) -> _AIW {
        _AIW { w: self }
    }
    #[doc = "Bit 16 - Promote Channel 0 Interrupt of ERU0 Request to NMI Request"]
    #[inline]
    pub fn eru00(&mut self) -> _ERU00W {
        _ERU00W { w: self }
    }
    #[doc = "Bit 17 - Promote Channel 1 Interrupt of ERU0 Request to NMI Request"]
    #[inline]
    pub fn eru01(&mut self) -> _ERU01W {
        _ERU01W { w: self }
    }
    #[doc = "Bit 18 - Promote Channel 2 Interrupt of ERU0 Request to NMI Request"]
    #[inline]
    pub fn eru02(&mut self) -> _ERU02W {
        _ERU02W { w: self }
    }
    #[doc = "Bit 19 - Promote Channel 3 Interrupt of ERU0 Request to NMI Request"]
    #[inline]
    pub fn eru03(&mut self) -> _ERU03W {
        _ERU03W { w: self }
    }
}
