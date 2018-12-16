#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MARP {
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
#[doc = "Possible values of the field `MARGIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MARGINR {
    #[doc = "Standard (default) margin."]
    VALUE1,
    #[doc = "Tight margin for 0 (low) level. Suboptimal 0-bits are read as 1s."]
    VALUE2,
    #[doc = "Tight margin for 1 (high) level. Suboptimal 1-bits are read as 0s."]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MARGINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MARGINR::VALUE1 => 0,
            MARGINR::VALUE2 => 1,
            MARGINR::VALUE3 => 4,
            MARGINR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MARGINR {
        match value {
            0 => MARGINR::VALUE1,
            1 => MARGINR::VALUE2,
            4 => MARGINR::VALUE3,
            i => MARGINR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MARGINR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MARGINR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == MARGINR::VALUE3
    }
}
#[doc = "Possible values of the field `TRAPDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRAPDISR {
    #[doc = "If a double-bit error occurs in PFLASH, a bus error trap is generated."]
    VALUE1,
    #[doc = "The double-bit error trap is disabled. Shall be used only during margin check"]
    VALUE2,
}
impl TRAPDISR {
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
            TRAPDISR::VALUE1 => false,
            TRAPDISR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRAPDISR {
        match value {
            false => TRAPDISR::VALUE1,
            true => TRAPDISR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TRAPDISR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TRAPDISR::VALUE2
    }
}
#[doc = "Values that can be written to the field `MARGIN`"]
pub enum MARGINW {
    #[doc = "Standard (default) margin."]
    VALUE1,
    #[doc = "Tight margin for 0 (low) level. Suboptimal 0-bits are read as 1s."]
    VALUE2,
    #[doc = "Tight margin for 1 (high) level. Suboptimal 1-bits are read as 0s."]
    VALUE3,
}
impl MARGINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MARGINW::VALUE1 => 0,
            MARGINW::VALUE2 => 1,
            MARGINW::VALUE3 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MARGINW<'a> {
    w: &'a mut W,
}
impl<'a> _MARGINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MARGINW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Standard (default) margin."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MARGINW::VALUE1)
    }
    #[doc = "Tight margin for 0 (low) level. Suboptimal 0-bits are read as 1s."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MARGINW::VALUE2)
    }
    #[doc = "Tight margin for 1 (high) level. Suboptimal 1-bits are read as 0s."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(MARGINW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRAPDIS`"]
pub enum TRAPDISW {
    #[doc = "If a double-bit error occurs in PFLASH, a bus error trap is generated."]
    VALUE1,
    #[doc = "The double-bit error trap is disabled. Shall be used only during margin check"]
    VALUE2,
}
impl TRAPDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRAPDISW::VALUE1 => false,
            TRAPDISW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRAPDISW<'a> {
    w: &'a mut W,
}
impl<'a> _TRAPDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRAPDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "If a double-bit error occurs in PFLASH, a bus error trap is generated."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRAPDISW::VALUE1)
    }
    #[doc = "The double-bit error trap is disabled. Shall be used only during margin check"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRAPDISW::VALUE2)
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bits 0:3 - PFLASH Margin Selection"]
    #[inline]
    pub fn margin(&self) -> MARGINR {
        MARGINR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - PFLASH Double-Bit Error Trap Disable"]
    #[inline]
    pub fn trapdis(&self) -> TRAPDISR {
        TRAPDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
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
    #[doc = "Bits 0:3 - PFLASH Margin Selection"]
    #[inline]
    pub fn margin(&mut self) -> _MARGINW {
        _MARGINW { w: self }
    }
    #[doc = "Bit 15 - PFLASH Double-Bit Error Trap Disable"]
    #[inline]
    pub fn trapdis(&mut self) -> _TRAPDISW {
        _TRAPDISW { w: self }
    }
}
