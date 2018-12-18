#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ADDRSEL2 {
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
#[doc = "Possible values of the field `REGENAB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGENABR {
    #[doc = "Memory region is disabled (default after reset)."]
    VALUE1,
    #[doc = "Memory region is enabled."]
    VALUE2,
}
impl REGENABR {
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
            REGENABR::VALUE1 => false,
            REGENABR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGENABR {
        match value {
            false => REGENABR::VALUE1,
            true => REGENABR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REGENABR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == REGENABR::VALUE2
    }
}
#[doc = "Possible values of the field `ALTENAB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALTENABR {
    #[doc = "Memory region is disabled (default after reset)."]
    VALUE1,
    #[doc = "Memory region is enabled."]
    VALUE2,
}
impl ALTENABR {
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
            ALTENABR::VALUE1 => false,
            ALTENABR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALTENABR {
        match value {
            false => ALTENABR::VALUE1,
            true => ALTENABR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ALTENABR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ALTENABR::VALUE2
    }
}
#[doc = "Possible values of the field `WPROT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPROTR {
    #[doc = "Region is enabled for write accesses"]
    VALUE1,
    #[doc = "Region is write protected."]
    VALUE2,
}
impl WPROTR {
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
            WPROTR::VALUE1 => false,
            WPROTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WPROTR {
        match value {
            false => WPROTR::VALUE1,
            true => WPROTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WPROTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WPROTR::VALUE2
    }
}
#[doc = "Values that can be written to the field `REGENAB`"]
pub enum REGENABW {
    #[doc = "Memory region is disabled (default after reset)."]
    VALUE1,
    #[doc = "Memory region is enabled."]
    VALUE2,
}
impl REGENABW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGENABW::VALUE1 => false,
            REGENABW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGENABW<'a> {
    w: &'a mut W,
}
impl<'a> _REGENABW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGENABW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Memory region is disabled (default after reset)."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REGENABW::VALUE1)
    }
    #[doc = "Memory region is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REGENABW::VALUE2)
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
#[doc = "Values that can be written to the field `ALTENAB`"]
pub enum ALTENABW {
    #[doc = "Memory region is disabled (default after reset)."]
    VALUE1,
    #[doc = "Memory region is enabled."]
    VALUE2,
}
impl ALTENABW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALTENABW::VALUE1 => false,
            ALTENABW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALTENABW<'a> {
    w: &'a mut W,
}
impl<'a> _ALTENABW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALTENABW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Memory region is disabled (default after reset)."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALTENABW::VALUE1)
    }
    #[doc = "Memory region is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALTENABW::VALUE2)
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
#[doc = "Values that can be written to the field `WPROT`"]
pub enum WPROTW {
    #[doc = "Region is enabled for write accesses"]
    VALUE1,
    #[doc = "Region is write protected."]
    VALUE2,
}
impl WPROTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WPROTW::VALUE1 => false,
            WPROTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WPROTW<'a> {
    w: &'a mut W,
}
impl<'a> _WPROTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WPROTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Region is enabled for write accesses"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WPROTW::VALUE1)
    }
    #[doc = "Region is write protected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WPROTW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Memory Region Enable"]
    #[inline]
    pub fn regenab(&self) -> REGENABR {
        REGENABR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Alternate Region Enable"]
    #[inline]
    pub fn altenab(&self) -> ALTENABR {
        ALTENABR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Memory Region Write Protect"]
    #[inline]
    pub fn wprot(&self) -> WPROTR {
        WPROTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Memory Region Enable"]
    #[inline]
    pub fn regenab(&mut self) -> _REGENABW {
        _REGENABW { w: self }
    }
    #[doc = "Bit 1 - Alternate Region Enable"]
    #[inline]
    pub fn altenab(&mut self) -> _ALTENABW {
        _ALTENABW { w: self }
    }
    #[doc = "Bit 2 - Memory Region Write Protect"]
    #[inline]
    pub fn wprot(&mut self) -> _WPROTW {
        _WPROTW { w: self }
    }
}
