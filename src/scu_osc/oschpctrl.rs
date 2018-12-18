#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OSCHPCTRL {
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
#[doc = "Possible values of the field `X1DEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum X1DENR {
    #[doc = "Bit X1D is not updated"]
    VALUE1,
    #[doc = "Bit X1D can be updated"]
    VALUE2,
}
impl X1DENR {
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
            X1DENR::VALUE1 => false,
            X1DENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> X1DENR {
        match value {
            false => X1DENR::VALUE1,
            true => X1DENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == X1DENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == X1DENR::VALUE2
    }
}
#[doc = "Possible values of the field `SHBY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHBYR {
    #[doc = "The shaper is not bypassed"]
    VALUE1,
    #[doc = "The shaper is bypassed"]
    VALUE2,
}
impl SHBYR {
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
            SHBYR::VALUE1 => false,
            SHBYR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SHBYR {
        match value {
            false => SHBYR::VALUE1,
            true => SHBYR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SHBYR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SHBYR::VALUE2
    }
}
#[doc = "Possible values of the field `GAINSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAINSELR {
    #[doc = "The gain control is configured for frequencies from 4 MHz to 8 MHz"]
    VALUE1,
    #[doc = "The gain control is configured for frequencies from 4 MHz to 16 MHz"]
    VALUE2,
    #[doc = "The gain control is configured for frequencies from 4 MHz to 20 MHz"]
    VALUE3,
    #[doc = "The gain control is configured for frequencies from 4 MHz to 25 MHz"]
    VALUE4,
}
impl GAINSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GAINSELR::VALUE1 => 0,
            GAINSELR::VALUE2 => 1,
            GAINSELR::VALUE3 => 2,
            GAINSELR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GAINSELR {
        match value {
            0 => GAINSELR::VALUE1,
            1 => GAINSELR::VALUE2,
            2 => GAINSELR::VALUE3,
            3 => GAINSELR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GAINSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GAINSELR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == GAINSELR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == GAINSELR::VALUE4
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "External Crystal Mode and External Input Clock Mode. The oscillator Power-Saving Mode is not entered."]
    VALUE1,
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is not entered."]
    VALUE2,
    #[doc = "External Input Clock Mode and the oscillator Power-Saving Mode is entered"]
    VALUE3,
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is entered."]
    VALUE4,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::VALUE1 => 0,
            MODER::VALUE2 => 1,
            MODER::VALUE3 => 2,
            MODER::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::VALUE1,
            1 => MODER::VALUE2,
            2 => MODER::VALUE3,
            3 => MODER::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MODER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MODER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == MODER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == MODER::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct OSCVALR {
    bits: u8,
}
impl OSCVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `X1DEN`"]
pub enum X1DENW {
    #[doc = "Bit X1D is not updated"]
    VALUE1,
    #[doc = "Bit X1D can be updated"]
    VALUE2,
}
impl X1DENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            X1DENW::VALUE1 => false,
            X1DENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _X1DENW<'a> {
    w: &'a mut W,
}
impl<'a> _X1DENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: X1DENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bit X1D is not updated"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(X1DENW::VALUE1)
    }
    #[doc = "Bit X1D can be updated"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(X1DENW::VALUE2)
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
#[doc = "Values that can be written to the field `SHBY`"]
pub enum SHBYW {
    #[doc = "The shaper is not bypassed"]
    VALUE1,
    #[doc = "The shaper is bypassed"]
    VALUE2,
}
impl SHBYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SHBYW::VALUE1 => false,
            SHBYW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SHBYW<'a> {
    w: &'a mut W,
}
impl<'a> _SHBYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SHBYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The shaper is not bypassed"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SHBYW::VALUE1)
    }
    #[doc = "The shaper is bypassed"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SHBYW::VALUE2)
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
#[doc = "Values that can be written to the field `GAINSEL`"]
pub enum GAINSELW {
    #[doc = "The gain control is configured for frequencies from 4 MHz to 8 MHz"]
    VALUE1,
    #[doc = "The gain control is configured for frequencies from 4 MHz to 16 MHz"]
    VALUE2,
    #[doc = "The gain control is configured for frequencies from 4 MHz to 20 MHz"]
    VALUE3,
    #[doc = "The gain control is configured for frequencies from 4 MHz to 25 MHz"]
    VALUE4,
}
impl GAINSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GAINSELW::VALUE1 => 0,
            GAINSELW::VALUE2 => 1,
            GAINSELW::VALUE3 => 2,
            GAINSELW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GAINSELW<'a> {
    w: &'a mut W,
}
impl<'a> _GAINSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GAINSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 8 MHz"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GAINSELW::VALUE1)
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 16 MHz"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GAINSELW::VALUE2)
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 20 MHz"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(GAINSELW::VALUE3)
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 25 MHz"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(GAINSELW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "External Crystal Mode and External Input Clock Mode. The oscillator Power-Saving Mode is not entered."]
    VALUE1,
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is not entered."]
    VALUE2,
    #[doc = "External Input Clock Mode and the oscillator Power-Saving Mode is entered"]
    VALUE3,
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is entered."]
    VALUE4,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::VALUE1 => 0,
            MODEW::VALUE2 => 1,
            MODEW::VALUE3 => 2,
            MODEW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External Crystal Mode and External Input Clock Mode. The oscillator Power-Saving Mode is not entered."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MODEW::VALUE1)
    }
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is not entered."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MODEW::VALUE2)
    }
    #[doc = "External Input Clock Mode and the oscillator Power-Saving Mode is entered"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(MODEW::VALUE3)
    }
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is entered."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(MODEW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OSCVALW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCVALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bit 0 - XTAL1 Data Enable"]
    #[inline]
    pub fn x1den(&self) -> X1DENR {
        X1DENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Shaper Bypass"]
    #[inline]
    pub fn shby(&self) -> SHBYR {
        SHBYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:3 - Oscillator Gain Selection"]
    #[inline]
    pub fn gainsel(&self) -> GAINSELR {
        GAINSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - OSC Frequency Value"]
    #[inline]
    pub fn oscval(&self) -> OSCVALR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OSCVALR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 60 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - XTAL1 Data Enable"]
    #[inline]
    pub fn x1den(&mut self) -> _X1DENW {
        _X1DENW { w: self }
    }
    #[doc = "Bit 1 - Shaper Bypass"]
    #[inline]
    pub fn shby(&mut self) -> _SHBYW {
        _SHBYW { w: self }
    }
    #[doc = "Bits 2:3 - Oscillator Gain Selection"]
    #[inline]
    pub fn gainsel(&mut self) -> _GAINSELW {
        _GAINSELW { w: self }
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bits 16:19 - OSC Frequency Value"]
    #[inline]
    pub fn oscval(&mut self) -> _OSCVALW {
        _OSCVALW { w: self }
    }
}
