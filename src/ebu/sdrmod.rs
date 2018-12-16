#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDRMOD {
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
pub struct XBAR {
    bits: u8,
}
impl XBAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XOPMR {
    bits: u16,
}
impl XOPMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `OPMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPMODER {
    #[doc = "Only this value must be written (default after reset)"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPMODER::VALUE1 => 0,
            OPMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPMODER {
        match value {
            0 => OPMODER::VALUE1,
            i => OPMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OPMODER::VALUE1
    }
}
#[doc = "Possible values of the field `CASLAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CASLATR {
    #[doc = "Two clocks (default after reset)"]
    VALUE1,
    #[doc = "Three clocks"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CASLATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CASLATR::VALUE1 => 2,
            CASLATR::VALUE2 => 3,
            CASLATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CASLATR {
        match value {
            2 => CASLATR::VALUE1,
            3 => CASLATR::VALUE2,
            i => CASLATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CASLATR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CASLATR::VALUE2
    }
}
#[doc = "Possible values of the field `BTYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTYPR {
    #[doc = "Only this value should be written (default after reset)"]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BTYPR {
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
            BTYPR::VALUE1 => false,
            BTYPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BTYPR {
        match value {
            false => BTYPR::VALUE1,
            i => BTYPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BTYPR::VALUE1
    }
}
#[doc = "Possible values of the field `BURSTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTLR {
    #[doc = "1 (default after reset)"]
    VALUE1,
    #[doc = "2"]
    VALUE2,
    #[doc = "4"]
    VALUE3,
    #[doc = "8"]
    VALUE4,
    #[doc = "16"]
    VALUE5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BURSTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BURSTLR::VALUE1 => 0,
            BURSTLR::VALUE2 => 1,
            BURSTLR::VALUE3 => 2,
            BURSTLR::VALUE4 => 3,
            BURSTLR::VALUE5 => 4,
            BURSTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BURSTLR {
        match value {
            0 => BURSTLR::VALUE1,
            1 => BURSTLR::VALUE2,
            2 => BURSTLR::VALUE3,
            3 => BURSTLR::VALUE4,
            4 => BURSTLR::VALUE5,
            i => BURSTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BURSTLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BURSTLR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == BURSTLR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == BURSTLR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == BURSTLR::VALUE5
    }
}
#[doc = r" Proxy"]
pub struct _XBAW<'a> {
    w: &'a mut W,
}
impl<'a> _XBAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XOPMW<'a> {
    w: &'a mut W,
}
impl<'a> _XOPMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COLDSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _COLDSTARTW<'a> {
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
#[doc = "Values that can be written to the field `OPMODE`"]
pub enum OPMODEW {
    #[doc = "Only this value must be written (default after reset)"]
    VALUE1,
}
impl OPMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPMODEW::VALUE1 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _OPMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Only this value must be written (default after reset)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(OPMODEW::VALUE1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CASLAT`"]
pub enum CASLATW {
    #[doc = "Two clocks (default after reset)"]
    VALUE1,
    #[doc = "Three clocks"]
    VALUE2,
}
impl CASLATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CASLATW::VALUE1 => 2,
            CASLATW::VALUE2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CASLATW<'a> {
    w: &'a mut W,
}
impl<'a> _CASLATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CASLATW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Two clocks (default after reset)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CASLATW::VALUE1)
    }
    #[doc = "Three clocks"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CASLATW::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BTYP`"]
pub enum BTYPW {
    #[doc = "Only this value should be written (default after reset)"]
    VALUE1,
}
impl BTYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BTYPW::VALUE1 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BTYPW<'a> {
    w: &'a mut W,
}
impl<'a> _BTYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BTYPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Only this value should be written (default after reset)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BTYPW::VALUE1)
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
#[doc = "Values that can be written to the field `BURSTL`"]
pub enum BURSTLW {
    #[doc = "1 (default after reset)"]
    VALUE1,
    #[doc = "2"]
    VALUE2,
    #[doc = "4"]
    VALUE3,
    #[doc = "8"]
    VALUE4,
    #[doc = "16"]
    VALUE5,
}
impl BURSTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BURSTLW::VALUE1 => 0,
            BURSTLW::VALUE2 => 1,
            BURSTLW::VALUE3 => 2,
            BURSTLW::VALUE4 => 3,
            BURSTLW::VALUE5 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BURSTLW<'a> {
    w: &'a mut W,
}
impl<'a> _BURSTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BURSTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 (default after reset)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BURSTLW::VALUE1)
    }
    #[doc = "2"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BURSTLW::VALUE2)
    }
    #[doc = "4"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(BURSTLW::VALUE3)
    }
    #[doc = "8"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(BURSTLW::VALUE4)
    }
    #[doc = "16"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(BURSTLW::VALUE5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 28:31 - Extended Operation Bank Select"]
    #[inline]
    pub fn xba(&self) -> XBAR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XBAR { bits }
    }
    #[doc = "Bits 16:27 - Extended Operation Mode"]
    #[inline]
    pub fn xopm(&self) -> XOPMR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        XOPMR { bits }
    }
    #[doc = "Bits 7:13 - Operation Mode"]
    #[inline]
    pub fn opmode(&self) -> OPMODER {
        OPMODER::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - CAS latency"]
    #[inline]
    pub fn caslat(&self) -> CASLATR {
        CASLATR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Burst type"]
    #[inline]
    pub fn btyp(&self) -> BTYPR {
        BTYPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:2 - Burst length"]
    #[inline]
    pub fn burstl(&self) -> BURSTLR {
        BURSTLR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 32 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:31 - Extended Operation Bank Select"]
    #[inline]
    pub fn xba(&mut self) -> _XBAW {
        _XBAW { w: self }
    }
    #[doc = "Bits 16:27 - Extended Operation Mode"]
    #[inline]
    pub fn xopm(&mut self) -> _XOPMW {
        _XOPMW { w: self }
    }
    #[doc = "Bit 15 - SDRAM coldstart"]
    #[inline]
    pub fn coldstart(&mut self) -> _COLDSTARTW {
        _COLDSTARTW { w: self }
    }
    #[doc = "Bits 7:13 - Operation Mode"]
    #[inline]
    pub fn opmode(&mut self) -> _OPMODEW {
        _OPMODEW { w: self }
    }
    #[doc = "Bits 4:6 - CAS latency"]
    #[inline]
    pub fn caslat(&mut self) -> _CASLATW {
        _CASLATW { w: self }
    }
    #[doc = "Bit 3 - Burst type"]
    #[inline]
    pub fn btyp(&mut self) -> _BTYPW {
        _BTYPW { w: self }
    }
    #[doc = "Bits 0:2 - Burst length"]
    #[inline]
    pub fn burstl(&mut self) -> _BURSTLW {
        _BURSTLW { w: self }
    }
}
