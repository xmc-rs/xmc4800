#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODCFG {
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
#[doc = "Possible values of the field `DIVM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVMR {
    #[doc = "fMOD = fCLK / 2"]
    VALUE1,
    #[doc = "fMOD = fCLK / 4"]
    VALUE2,
    #[doc = "fMOD = fCLK / 6"]
    VALUE3,
    #[doc = "fMOD = fCLK / 32"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DIVMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIVMR::VALUE1 => 0,
            DIVMR::VALUE2 => 1,
            DIVMR::VALUE3 => 2,
            DIVMR::VALUE4 => 15,
            DIVMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIVMR {
        match value {
            0 => DIVMR::VALUE1,
            1 => DIVMR::VALUE2,
            2 => DIVMR::VALUE3,
            15 => DIVMR::VALUE4,
            i => DIVMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIVMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIVMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DIVMR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == DIVMR::VALUE4
    }
}
#[doc = "Values that can be written to the field `DIVM`"]
pub enum DIVMW {
    #[doc = "fMOD = fCLK / 2"]
    VALUE1,
    #[doc = "fMOD = fCLK / 4"]
    VALUE2,
    #[doc = "fMOD = fCLK / 6"]
    VALUE3,
    #[doc = "fMOD = fCLK / 32"]
    VALUE4,
}
impl DIVMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIVMW::VALUE1 => 0,
            DIVMW::VALUE2 => 1,
            DIVMW::VALUE3 => 2,
            DIVMW::VALUE4 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVMW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "fMOD = fCLK / 2"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVMW::VALUE1)
    }
    #[doc = "fMOD = fCLK / 4"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVMW::VALUE2)
    }
    #[doc = "fMOD = fCLK / 6"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DIVMW::VALUE3)
    }
    #[doc = "fMOD = fCLK / 32"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(DIVMW::VALUE4)
    }
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
#[doc = "Values that can be written to the field `DWC`"]
pub enum DWCW {
    #[doc = "No write access to divider factor"]
    VALUE1,
    #[doc = "Bitfield DIVM can be written"]
    VALUE2,
}
impl DWCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DWCW::VALUE1 => false,
            DWCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DWCW<'a> {
    w: &'a mut W,
}
impl<'a> _DWCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DWCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No write access to divider factor"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DWCW::VALUE1)
    }
    #[doc = "Bitfield DIVM can be written"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DWCW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 16:19 - Divider Factor for Modulator Clock"]
    #[inline]
    pub fn divm(&self) -> DIVMR {
        DIVMR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 16:19 - Divider Factor for Modulator Clock"]
    #[inline]
    pub fn divm(&mut self) -> _DIVMW {
        _DIVMW { w: self }
    }
    #[doc = "Bit 23 - Write Control for Divider Factor"]
    #[inline]
    pub fn dwc(&mut self) -> _DWCW {
        _DWCW { w: self }
    }
}
