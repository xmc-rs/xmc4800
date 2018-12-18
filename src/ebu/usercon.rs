#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USERCON {
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
#[doc = r" Value of the field"]
pub struct DIPR {
    bits: bool,
}
impl DIPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `ADDIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDIOR {
    #[doc = "Address Bit is required for addressing memory"]
    VALUE1,
    #[doc = "Address Bit is available for GPIO function"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl ADDIOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            ADDIOR::VALUE1 => 0,
            ADDIOR::VALUE2 => 1,
            ADDIOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> ADDIOR {
        match value {
            0 => ADDIOR::VALUE1,
            1 => ADDIOR::VALUE2,
            i => ADDIOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ADDIOR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ADDIOR::VALUE2
    }
}
#[doc = "Possible values of the field `ADVIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADVIOR {
    #[doc = "ADV pin is required for controlling memory"]
    VALUE1,
    #[doc = "ADV pin is available for GPIO function"]
    VALUE2,
}
impl ADVIOR {
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
            ADVIOR::VALUE1 => false,
            ADVIOR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADVIOR {
        match value {
            false => ADVIOR::VALUE1,
            true => ADVIOR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ADVIOR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ADVIOR::VALUE2
    }
}
#[doc = r" Proxy"]
pub struct _DIPW<'a> {
    w: &'a mut W,
}
impl<'a> _DIPW<'a> {
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
#[doc = "Values that can be written to the field `ADDIO`"]
pub enum ADDIOW {
    #[doc = "Address Bit is required for addressing memory"]
    VALUE1,
    #[doc = "Address Bit is available for GPIO function"]
    VALUE2,
}
impl ADDIOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            ADDIOW::VALUE1 => 0,
            ADDIOW::VALUE2 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDIOW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDIOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDIOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Address Bit is required for addressing memory"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ADDIOW::VALUE1)
    }
    #[doc = "Address Bit is available for GPIO function"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ADDIOW::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADVIO`"]
pub enum ADVIOW {
    #[doc = "ADV pin is required for controlling memory"]
    VALUE1,
    #[doc = "ADV pin is available for GPIO function"]
    VALUE2,
}
impl ADVIOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADVIOW::VALUE1 => false,
            ADVIOW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADVIOW<'a> {
    w: &'a mut W,
}
impl<'a> _ADVIOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADVIOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADV pin is required for controlling memory"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ADVIOW::VALUE1)
    }
    #[doc = "ADV pin is available for GPIO function"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ADVIOW::VALUE2)
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
        const OFFSET: u8 = 25;
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
    #[doc = "Bit 0 - Disable Internal Pipelining"]
    #[inline]
    pub fn dip(&self) -> DIPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIPR { bits }
    }
    #[doc = "Bits 16:24 - Address Pins to GPIO Mode"]
    #[inline]
    pub fn addio(&self) -> ADDIOR {
        ADDIOR::_from({
            const MASK: u16 = 511;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bit 25 - ADV Pin to GPIO Mode"]
    #[inline]
    pub fn advio(&self) -> ADVIOR {
        ADVIOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
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
    #[doc = "Bit 0 - Disable Internal Pipelining"]
    #[inline]
    pub fn dip(&mut self) -> _DIPW {
        _DIPW { w: self }
    }
    #[doc = "Bits 16:24 - Address Pins to GPIO Mode"]
    #[inline]
    pub fn addio(&mut self) -> _ADDIOW {
        _ADDIOW { w: self }
    }
    #[doc = "Bit 25 - ADV Pin to GPIO Mode"]
    #[inline]
    pub fn advio(&mut self) -> _ADVIOW {
        _ADVIOW { w: self }
    }
}
