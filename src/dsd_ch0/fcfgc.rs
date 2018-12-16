#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCFGC {
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
pub struct CFMDFR {
    bits: u8,
}
impl CFMDFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CFMC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFMCR {
    #[doc = "CIC1"]
    VALUE1,
    #[doc = "CIC2"]
    VALUE2,
    #[doc = "CIC3"]
    VALUE3,
    #[doc = "CICF"]
    VALUE4,
}
impl CFMCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFMCR::VALUE1 => 0,
            CFMCR::VALUE2 => 1,
            CFMCR::VALUE3 => 2,
            CFMCR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFMCR {
        match value {
            0 => CFMCR::VALUE1,
            1 => CFMCR::VALUE2,
            2 => CFMCR::VALUE3,
            3 => CFMCR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CFMCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CFMCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CFMCR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CFMCR::VALUE4
    }
}
#[doc = "Possible values of the field `CFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFENR {
    #[doc = "CIC filter disabled and bypassed"]
    VALUE1,
    #[doc = "Enable CIC filter"]
    VALUE2,
}
impl CFENR {
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
            CFENR::VALUE1 => false,
            CFENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFENR {
        match value {
            false => CFENR::VALUE1,
            true => CFENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CFENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CFENR::VALUE2
    }
}
#[doc = "Possible values of the field `SRGM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRGMR {
    #[doc = "Never, service requests disabled"]
    VALUE1,
    #[doc = "Always, for each new result value"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRGMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRGMR::VALUE1 => 0,
            SRGMR::VALUE4 => 3,
            SRGMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRGMR {
        match value {
            0 => SRGMR::VALUE1,
            3 => SRGMR::VALUE4,
            i => SRGMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SRGMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SRGMR::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct CFMSVR {
    bits: u8,
}
impl CFMSVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CFMDCNTR {
    bits: u8,
}
impl CFMDCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CFMDFW<'a> {
    w: &'a mut W,
}
impl<'a> _CFMDFW<'a> {
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
#[doc = "Values that can be written to the field `CFMC`"]
pub enum CFMCW {
    #[doc = "CIC1"]
    VALUE1,
    #[doc = "CIC2"]
    VALUE2,
    #[doc = "CIC3"]
    VALUE3,
    #[doc = "CICF"]
    VALUE4,
}
impl CFMCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFMCW::VALUE1 => 0,
            CFMCW::VALUE2 => 1,
            CFMCW::VALUE3 => 2,
            CFMCW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFMCW<'a> {
    w: &'a mut W,
}
impl<'a> _CFMCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFMCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CIC1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFMCW::VALUE1)
    }
    #[doc = "CIC2"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFMCW::VALUE2)
    }
    #[doc = "CIC3"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CFMCW::VALUE3)
    }
    #[doc = "CICF"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CFMCW::VALUE4)
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
#[doc = "Values that can be written to the field `CFEN`"]
pub enum CFENW {
    #[doc = "CIC filter disabled and bypassed"]
    VALUE1,
    #[doc = "Enable CIC filter"]
    VALUE2,
}
impl CFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFENW::VALUE1 => false,
            CFENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFENW<'a> {
    w: &'a mut W,
}
impl<'a> _CFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CIC filter disabled and bypassed"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFENW::VALUE1)
    }
    #[doc = "Enable CIC filter"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFENW::VALUE2)
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
#[doc = "Values that can be written to the field `SRGM`"]
pub enum SRGMW {
    #[doc = "Never, service requests disabled"]
    VALUE1,
    #[doc = "Always, for each new result value"]
    VALUE4,
}
impl SRGMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRGMW::VALUE1 => 0,
            SRGMW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRGMW<'a> {
    w: &'a mut W,
}
impl<'a> _SRGMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRGMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Never, service requests disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRGMW::VALUE1)
    }
    #[doc = "Always, for each new result value"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(SRGMW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CFMSVW<'a> {
    w: &'a mut W,
}
impl<'a> _CFMSVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - CIC Filter (Main Chain) Decimation Factor"]
    #[inline]
    pub fn cfmdf(&self) -> CFMDFR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CFMDFR { bits }
    }
    #[doc = "Bits 8:9 - CIC Filter (Main Chain) Configuration"]
    #[inline]
    pub fn cfmc(&self) -> CFMCR {
        CFMCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - CIC Filter Enable"]
    #[inline]
    pub fn cfen(&self) -> CFENR {
        CFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 14:15 - Service Request Generation Main Chain"]
    #[inline]
    pub fn srgm(&self) -> SRGMR {
        SRGMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - CIC Filter (Main Chain) Start Value"]
    #[inline]
    pub fn cfmsv(&self) -> CFMSVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CFMSVR { bits }
    }
    #[doc = "Bits 24:31 - CIC Filter (Main Chain) Decimation Counter"]
    #[inline]
    pub fn cfmdcnt(&self) -> CFMDCNTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CFMDCNTR { bits }
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
    #[doc = "Bits 0:7 - CIC Filter (Main Chain) Decimation Factor"]
    #[inline]
    pub fn cfmdf(&mut self) -> _CFMDFW {
        _CFMDFW { w: self }
    }
    #[doc = "Bits 8:9 - CIC Filter (Main Chain) Configuration"]
    #[inline]
    pub fn cfmc(&mut self) -> _CFMCW {
        _CFMCW { w: self }
    }
    #[doc = "Bit 10 - CIC Filter Enable"]
    #[inline]
    pub fn cfen(&mut self) -> _CFENW {
        _CFENW { w: self }
    }
    #[doc = "Bits 14:15 - Service Request Generation Main Chain"]
    #[inline]
    pub fn srgm(&mut self) -> _SRGMW {
        _SRGMW { w: self }
    }
    #[doc = "Bits 16:23 - CIC Filter (Main Chain) Start Value"]
    #[inline]
    pub fn cfmsv(&mut self) -> _CFMSVW {
        _CFMSVW { w: self }
    }
}
