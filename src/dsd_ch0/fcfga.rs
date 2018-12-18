#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCFGA {
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
pub struct CFADFR {
    bits: u8,
}
impl CFADFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CFAC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFACR {
    #[doc = "CIC1"]
    VALUE1,
    #[doc = "CIC2"]
    VALUE2,
    #[doc = "CIC3"]
    VALUE3,
    #[doc = "CICF"]
    VALUE4,
}
impl CFACR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFACR::VALUE1 => 0,
            CFACR::VALUE2 => 1,
            CFACR::VALUE3 => 2,
            CFACR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFACR {
        match value {
            0 => CFACR::VALUE1,
            1 => CFACR::VALUE2,
            2 => CFACR::VALUE3,
            3 => CFACR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CFACR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CFACR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CFACR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CFACR::VALUE4
    }
}
#[doc = "Possible values of the field `SRGA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRGAR {
    #[doc = "Never, service requests disabled"]
    VALUE1,
    #[doc = "Auxiliary filter: As selected by bitfields ESEL and EGT (if integrator enabled)"]
    VALUE2,
    #[doc = "Alternate source: Capturing of a sign delay value to register CGSYNCx (x = 0 - 3)"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRGAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRGAR::VALUE1 => 0,
            SRGAR::VALUE2 => 1,
            SRGAR::VALUE3 => 2,
            SRGAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRGAR {
        match value {
            0 => SRGAR::VALUE1,
            1 => SRGAR::VALUE2,
            2 => SRGAR::VALUE3,
            i => SRGAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SRGAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SRGAR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SRGAR::VALUE3
    }
}
#[doc = "Possible values of the field `ESEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESELR {
    #[doc = "Always, for each new result value"]
    VALUE1,
    #[doc = "If result is inside the boundary band"]
    VALUE2,
    #[doc = "If result is outside the boundary band"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ESELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ESELR::VALUE1 => 0,
            ESELR::VALUE2 => 1,
            ESELR::VALUE3 => 2,
            ESELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ESELR {
        match value {
            0 => ESELR::VALUE1,
            1 => ESELR::VALUE2,
            2 => ESELR::VALUE3,
            i => ESELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ESELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ESELR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ESELR::VALUE3
    }
}
#[doc = "Possible values of the field `EGT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EGTR {
    #[doc = "Separate: generate events according to ESEL"]
    VALUE1,
    #[doc = "Coupled: generate events only when the integrator is enabled and after the discard phase defined by bitfield NVALDISWhile the integrator is bypassed, event gating is disabled, i.e. service requests are generated according to bitfield ESEL. The event gating suppresses service requests, result values are still stored in register RESAx."]
    VALUE2,
}
impl EGTR {
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
            EGTR::VALUE1 => false,
            EGTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EGTR {
        match value {
            false => EGTR::VALUE1,
            true => EGTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EGTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EGTR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct CFADCNTR {
    bits: u8,
}
impl CFADCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CFADFW<'a> {
    w: &'a mut W,
}
impl<'a> _CFADFW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFAC`"]
pub enum CFACW {
    #[doc = "CIC1"]
    VALUE1,
    #[doc = "CIC2"]
    VALUE2,
    #[doc = "CIC3"]
    VALUE3,
    #[doc = "CICF"]
    VALUE4,
}
impl CFACW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFACW::VALUE1 => 0,
            CFACW::VALUE2 => 1,
            CFACW::VALUE3 => 2,
            CFACW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFACW<'a> {
    w: &'a mut W,
}
impl<'a> _CFACW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFACW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CIC1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFACW::VALUE1)
    }
    #[doc = "CIC2"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFACW::VALUE2)
    }
    #[doc = "CIC3"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CFACW::VALUE3)
    }
    #[doc = "CICF"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CFACW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRGA`"]
pub enum SRGAW {
    #[doc = "Never, service requests disabled"]
    VALUE1,
    #[doc = "Auxiliary filter: As selected by bitfields ESEL and EGT (if integrator enabled)"]
    VALUE2,
    #[doc = "Alternate source: Capturing of a sign delay value to register CGSYNCx (x = 0 - 3)"]
    VALUE3,
}
impl SRGAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRGAW::VALUE1 => 0,
            SRGAW::VALUE2 => 1,
            SRGAW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRGAW<'a> {
    w: &'a mut W,
}
impl<'a> _SRGAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRGAW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Never, service requests disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRGAW::VALUE1)
    }
    #[doc = "Auxiliary filter: As selected by bitfields ESEL and EGT (if integrator enabled)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRGAW::VALUE2)
    }
    #[doc = "Alternate source: Capturing of a sign delay value to register CGSYNCx (x = 0 - 3)"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SRGAW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ESEL`"]
pub enum ESELW {
    #[doc = "Always, for each new result value"]
    VALUE1,
    #[doc = "If result is inside the boundary band"]
    VALUE2,
    #[doc = "If result is outside the boundary band"]
    VALUE3,
}
impl ESELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ESELW::VALUE1 => 0,
            ESELW::VALUE2 => 1,
            ESELW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ESELW<'a> {
    w: &'a mut W,
}
impl<'a> _ESELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ESELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Always, for each new result value"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ESELW::VALUE1)
    }
    #[doc = "If result is inside the boundary band"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ESELW::VALUE2)
    }
    #[doc = "If result is outside the boundary band"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ESELW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EGT`"]
pub enum EGTW {
    #[doc = "Separate: generate events according to ESEL"]
    VALUE1,
    #[doc = "Coupled: generate events only when the integrator is enabled and after the discard phase defined by bitfield NVALDISWhile the integrator is bypassed, event gating is disabled, i.e. service requests are generated according to bitfield ESEL. The event gating suppresses service requests, result values are still stored in register RESAx."]
    VALUE2,
}
impl EGTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EGTW::VALUE1 => false,
            EGTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EGTW<'a> {
    w: &'a mut W,
}
impl<'a> _EGTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EGTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Separate: generate events according to ESEL"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EGTW::VALUE1)
    }
    #[doc = "Coupled: generate events only when the integrator is enabled and after the discard phase defined by bitfield NVALDISWhile the integrator is bypassed, event gating is disabled, i.e. service requests are generated according to bitfield ESEL. The event gating suppresses service requests, result values are still stored in register RESAx."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EGTW::VALUE2)
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
        const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:7 - CIC Filter (Auxiliary) Decimation Factor"]
    #[inline]
    pub fn cfadf(&self) -> CFADFR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CFADFR { bits }
    }
    #[doc = "Bits 8:9 - CIC Filter (Auxiliary) Configuration"]
    #[inline]
    pub fn cfac(&self) -> CFACR {
        CFACR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Service Request Generation Auxiliary Filter"]
    #[inline]
    pub fn srga(&self) -> SRGAR {
        SRGAR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Event Select"]
    #[inline]
    pub fn esel(&self) -> ESELR {
        ESELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 14 - Event Gating"]
    #[inline]
    pub fn egt(&self) -> EGTR {
        EGTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:31 - CIC Filter (Auxiliary) Decimation Counter"]
    #[inline]
    pub fn cfadcnt(&self) -> CFADCNTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CFADCNTR { bits }
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
    #[doc = "Bits 0:7 - CIC Filter (Auxiliary) Decimation Factor"]
    #[inline]
    pub fn cfadf(&mut self) -> _CFADFW {
        _CFADFW { w: self }
    }
    #[doc = "Bits 8:9 - CIC Filter (Auxiliary) Configuration"]
    #[inline]
    pub fn cfac(&mut self) -> _CFACW {
        _CFACW { w: self }
    }
    #[doc = "Bits 10:11 - Service Request Generation Auxiliary Filter"]
    #[inline]
    pub fn srga(&mut self) -> _SRGAW {
        _SRGAW { w: self }
    }
    #[doc = "Bits 12:13 - Event Select"]
    #[inline]
    pub fn esel(&mut self) -> _ESELW {
        _ESELW { w: self }
    }
    #[doc = "Bit 14 - Event Gating"]
    #[inline]
    pub fn egt(&mut self) -> _EGTW {
        _EGTW { w: self }
    }
}
