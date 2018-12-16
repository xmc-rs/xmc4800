#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IWCTR {
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
pub struct NVALCNTR {
    bits: u8,
}
impl NVALCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTENR {
    #[doc = "Integration stopped. INTEN is cleared at the end of the integration window, i.e. upon the inverse trigger event transition of the external trigger signal."]
    VALUE1,
    #[doc = "Integration enabled. INTEN is set upon the defined trigger event."]
    VALUE2,
}
impl INTENR {
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
            INTENR::VALUE1 => false,
            INTENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTENR {
        match value {
            false => INTENR::VALUE1,
            true => INTENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == INTENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == INTENR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct REPCNTR {
    bits: u8,
}
impl REPCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REPVALR {
    bits: u8,
}
impl REPVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NVALDISR {
    bits: u8,
}
impl NVALDISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `IWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IWSR {
    #[doc = "Internal control: stop integrator after REPVAL+1 integration cycles"]
    VALUE1,
    #[doc = "External control: stop integrator when bit INTEN becomes 0"]
    VALUE2,
}
impl IWSR {
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
            IWSR::VALUE1 => false,
            IWSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IWSR {
        match value {
            false => IWSR::VALUE1,
            true => IWSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IWSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == IWSR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct NVALINTR {
    bits: u8,
}
impl NVALINTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _REPVALW<'a> {
    w: &'a mut W,
}
impl<'a> _REPVALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NVALDISW<'a> {
    w: &'a mut W,
}
impl<'a> _NVALDISW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IWS`"]
pub enum IWSW {
    #[doc = "Internal control: stop integrator after REPVAL+1 integration cycles"]
    VALUE1,
    #[doc = "External control: stop integrator when bit INTEN becomes 0"]
    VALUE2,
}
impl IWSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IWSW::VALUE1 => false,
            IWSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IWSW<'a> {
    w: &'a mut W,
}
impl<'a> _IWSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IWSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal control: stop integrator after REPVAL+1 integration cycles"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(IWSW::VALUE1)
    }
    #[doc = "External control: stop integrator when bit INTEN becomes 0"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(IWSW::VALUE2)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NVALINTW<'a> {
    w: &'a mut W,
}
impl<'a> _NVALINTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:5 - Number of Values Counted"]
    #[inline]
    pub fn nvalcnt(&self) -> NVALCNTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NVALCNTR { bits }
    }
    #[doc = "Bit 7 - Integration Enable"]
    #[inline]
    pub fn inten(&self) -> INTENR {
        INTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Integration Cycle Counter"]
    #[inline]
    pub fn repcnt(&self) -> REPCNTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REPCNTR { bits }
    }
    #[doc = "Bits 12:15 - Number of Integration Cycles"]
    #[inline]
    pub fn repval(&self) -> REPVALR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REPVALR { bits }
    }
    #[doc = "Bits 16:21 - Number of Values Discarded"]
    #[inline]
    pub fn nvaldis(&self) -> NVALDISR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NVALDISR { bits }
    }
    #[doc = "Bit 23 - Integration Window SIze"]
    #[inline]
    pub fn iws(&self) -> IWSR {
        IWSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:29 - Number of Values Integrated"]
    #[inline]
    pub fn nvalint(&self) -> NVALINTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NVALINTR { bits }
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
    #[doc = "Bits 12:15 - Number of Integration Cycles"]
    #[inline]
    pub fn repval(&mut self) -> _REPVALW {
        _REPVALW { w: self }
    }
    #[doc = "Bits 16:21 - Number of Values Discarded"]
    #[inline]
    pub fn nvaldis(&mut self) -> _NVALDISW {
        _NVALDISW { w: self }
    }
    #[doc = "Bit 23 - Integration Window SIze"]
    #[inline]
    pub fn iws(&mut self) -> _IWSW {
        _IWSW { w: self }
    }
    #[doc = "Bits 24:29 - Number of Values Integrated"]
    #[inline]
    pub fn nvalint(&mut self) -> _NVALINTW {
        _NVALINTW { w: self }
    }
}
