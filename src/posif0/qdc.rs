#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::QDC {
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
#[doc = "Possible values of the field `PALS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PALSR {
    #[doc = "Phase A is active HIGH"]
    VALUE1,
    #[doc = "Phase A is active LOW"]
    VALUE2,
}
impl PALSR {
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
            PALSR::VALUE1 => false,
            PALSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PALSR {
        match value {
            false => PALSR::VALUE1,
            true => PALSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PALSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PALSR::VALUE2
    }
}
#[doc = "Possible values of the field `PBLS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBLSR {
    #[doc = "Phase B is active HIGH"]
    VALUE1,
    #[doc = "Phase B is active LOW"]
    VALUE2,
}
impl PBLSR {
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
            PBLSR::VALUE1 => false,
            PBLSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PBLSR {
        match value {
            false => PBLSR::VALUE1,
            true => PBLSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PBLSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PBLSR::VALUE2
    }
}
#[doc = "Possible values of the field `PHS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHSR {
    #[doc = "Phase A is the leading signal for clockwise rotation"]
    VALUE1,
    #[doc = "Phase B is the leading signal for clockwise rotation"]
    VALUE2,
}
impl PHSR {
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
            PHSR::VALUE1 => false,
            PHSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PHSR {
        match value {
            false => PHSR::VALUE1,
            true => PHSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PHSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PHSR::VALUE2
    }
}
#[doc = "Possible values of the field `ICM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICMR {
    #[doc = "No index marker generation on POSIFx.OUT3"]
    VALUE1,
    #[doc = "Only first index occurrence generated on POSIFx.OUT3"]
    VALUE2,
    #[doc = "All index occurrences generated on POSIFx.OUT3"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ICMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICMR::VALUE1 => 0,
            ICMR::VALUE2 => 1,
            ICMR::VALUE3 => 2,
            ICMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICMR {
        match value {
            0 => ICMR::VALUE1,
            1 => ICMR::VALUE2,
            2 => ICMR::VALUE3,
            i => ICMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ICMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ICMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ICMR::VALUE3
    }
}
#[doc = "Possible values of the field `DVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DVALR {
    #[doc = "Counterclockwise rotation"]
    VALUE1,
    #[doc = "Clockwise rotation"]
    VALUE2,
}
impl DVALR {
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
            DVALR::VALUE1 => false,
            DVALR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DVALR {
        match value {
            false => DVALR::VALUE1,
            true => DVALR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DVALR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DVALR::VALUE2
    }
}
#[doc = "Values that can be written to the field `PALS`"]
pub enum PALSW {
    #[doc = "Phase A is active HIGH"]
    VALUE1,
    #[doc = "Phase A is active LOW"]
    VALUE2,
}
impl PALSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PALSW::VALUE1 => false,
            PALSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PALSW<'a> {
    w: &'a mut W,
}
impl<'a> _PALSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PALSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Phase A is active HIGH"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PALSW::VALUE1)
    }
    #[doc = "Phase A is active LOW"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PALSW::VALUE2)
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
#[doc = "Values that can be written to the field `PBLS`"]
pub enum PBLSW {
    #[doc = "Phase B is active HIGH"]
    VALUE1,
    #[doc = "Phase B is active LOW"]
    VALUE2,
}
impl PBLSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PBLSW::VALUE1 => false,
            PBLSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PBLSW<'a> {
    w: &'a mut W,
}
impl<'a> _PBLSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PBLSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Phase B is active HIGH"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PBLSW::VALUE1)
    }
    #[doc = "Phase B is active LOW"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PBLSW::VALUE2)
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
#[doc = "Values that can be written to the field `PHS`"]
pub enum PHSW {
    #[doc = "Phase A is the leading signal for clockwise rotation"]
    VALUE1,
    #[doc = "Phase B is the leading signal for clockwise rotation"]
    VALUE2,
}
impl PHSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PHSW::VALUE1 => false,
            PHSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PHSW<'a> {
    w: &'a mut W,
}
impl<'a> _PHSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PHSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Phase A is the leading signal for clockwise rotation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PHSW::VALUE1)
    }
    #[doc = "Phase B is the leading signal for clockwise rotation"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PHSW::VALUE2)
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
#[doc = "Values that can be written to the field `ICM`"]
pub enum ICMW {
    #[doc = "No index marker generation on POSIFx.OUT3"]
    VALUE1,
    #[doc = "Only first index occurrence generated on POSIFx.OUT3"]
    VALUE2,
    #[doc = "All index occurrences generated on POSIFx.OUT3"]
    VALUE3,
}
impl ICMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICMW::VALUE1 => 0,
            ICMW::VALUE2 => 1,
            ICMW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICMW<'a> {
    w: &'a mut W,
}
impl<'a> _ICMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No index marker generation on POSIFx.OUT3"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ICMW::VALUE1)
    }
    #[doc = "Only first index occurrence generated on POSIFx.OUT3"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ICMW::VALUE2)
    }
    #[doc = "All index occurrences generated on POSIFx.OUT3"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ICMW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Phase A Level selector"]
    #[inline]
    pub fn pals(&self) -> PALSR {
        PALSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Phase B Level selector"]
    #[inline]
    pub fn pbls(&self) -> PBLSR {
        PBLSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Phase signals swap"]
    #[inline]
    pub fn phs(&self) -> PHSR {
        PHSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Index Marker generations control"]
    #[inline]
    pub fn icm(&self) -> ICMR {
        ICMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Current rotation direction"]
    #[inline]
    pub fn dval(&self) -> DVALR {
        DVALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Phase A Level selector"]
    #[inline]
    pub fn pals(&mut self) -> _PALSW {
        _PALSW { w: self }
    }
    #[doc = "Bit 1 - Phase B Level selector"]
    #[inline]
    pub fn pbls(&mut self) -> _PBLSW {
        _PBLSW { w: self }
    }
    #[doc = "Bit 2 - Phase signals swap"]
    #[inline]
    pub fn phs(&mut self) -> _PHSW {
        _PHSW { w: self }
    }
    #[doc = "Bits 4:5 - Index Marker generations control"]
    #[inline]
    pub fn icm(&mut self) -> _ICMW {
        _ICMW { w: self }
    }
}
