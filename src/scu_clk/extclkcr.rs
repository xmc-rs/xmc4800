#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXTCLKCR {
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
#[doc = "Possible values of the field `ECKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECKSELR {
    #[doc = "fSYS clock"]
    VALUE1,
    #[doc = "fUSB clock"]
    VALUE3,
    #[doc = "fPLL clock divided according to ECKDIV bit field configuration"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ECKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ECKSELR::VALUE1 => 0,
            ECKSELR::VALUE3 => 2,
            ECKSELR::VALUE4 => 3,
            ECKSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ECKSELR {
        match value {
            0 => ECKSELR::VALUE1,
            2 => ECKSELR::VALUE3,
            3 => ECKSELR::VALUE4,
            i => ECKSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ECKSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ECKSELR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == ECKSELR::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct ECKDIVR {
    bits: u16,
}
impl ECKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `ECKSEL`"]
pub enum ECKSELW {
    #[doc = "fSYS clock"]
    VALUE1,
    #[doc = "fUSB clock"]
    VALUE3,
    #[doc = "fPLL clock divided according to ECKDIV bit field configuration"]
    VALUE4,
}
impl ECKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ECKSELW::VALUE1 => 0,
            ECKSELW::VALUE3 => 2,
            ECKSELW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ECKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "fSYS clock"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECKSELW::VALUE1)
    }
    #[doc = "fUSB clock"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ECKSELW::VALUE3)
    }
    #[doc = "fPLL clock divided according to ECKDIV bit field configuration"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(ECKSELW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ECKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _ECKDIVW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - External Clock Selection Value"]
    #[inline]
    pub fn ecksel(&self) -> ECKSELR {
        ECKSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:24 - External Clock Divider Value"]
    #[inline]
    pub fn eckdiv(&self) -> ECKDIVR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        ECKDIVR { bits }
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
    #[doc = "Bits 0:1 - External Clock Selection Value"]
    #[inline]
    pub fn ecksel(&mut self) -> _ECKSELW {
        _ECKSELW { w: self }
    }
    #[doc = "Bits 16:24 - External Clock Divider Value"]
    #[inline]
    pub fn eckdiv(&mut self) -> _ECKDIVW {
        _ECKDIVW { w: self }
    }
}
