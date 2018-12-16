#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GLOBCFG {
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
#[doc = "Possible values of the field `MCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCSELR {
    #[doc = "Internal clock off, no source selected"]
    VALUE1,
    #[doc = "fDSD"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MCSELR::VALUE1 => 0,
            MCSELR::VALUE2 => 1,
            MCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MCSELR {
        match value {
            0 => MCSELR::VALUE1,
            1 => MCSELR::VALUE2,
            i => MCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MCSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MCSELR::VALUE2
    }
}
#[doc = "Values that can be written to the field `MCSEL`"]
pub enum MCSELW {
    #[doc = "Internal clock off, no source selected"]
    VALUE1,
    #[doc = "fDSD"]
    VALUE2,
}
impl MCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MCSELW::VALUE1 => 0,
            MCSELW::VALUE2 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Internal clock off, no source selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCSELW::VALUE1)
    }
    #[doc = "fDSD"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCSELW::VALUE2)
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
    #[doc = "Bits 0:2 - Modulator Clock Select"]
    #[inline]
    pub fn mcsel(&self) -> MCSELR {
        MCSELR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Modulator Clock Select"]
    #[inline]
    pub fn mcsel(&mut self) -> _MCSELW {
        _MCSELW { w: self }
    }
}
