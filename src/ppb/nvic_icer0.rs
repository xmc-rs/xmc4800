#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NVIC_ICER0 {
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
#[doc = "Possible values of the field `CLRENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRENAR {
    #[doc = "interrupt disabled"]
    VALUE3,
    #[doc = "interrupt enabled."]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl CLRENAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            CLRENAR::VALUE3 => 0,
            CLRENAR::VALUE4 => 1,
            CLRENAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> CLRENAR {
        match value {
            0 => CLRENAR::VALUE3,
            1 => CLRENAR::VALUE4,
            i => CLRENAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CLRENAR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CLRENAR::VALUE4
    }
}
#[doc = "Values that can be written to the field `CLRENA`"]
pub enum CLRENAW {
    #[doc = "interrupt disabled"]
    VALUE3,
    #[doc = "interrupt enabled."]
    VALUE4,
}
impl CLRENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            CLRENAW::VALUE3 => 0,
            CLRENAW::VALUE4 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLRENAW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLRENAW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "interrupt disabled"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CLRENAW::VALUE3)
    }
    #[doc = "interrupt enabled."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CLRENAW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
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
    #[doc = "Bits 0:31 - Interrupt clear-enable bits."]
    #[inline]
    pub fn clrena(&self) -> CLRENAR {
        CLRENAR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
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
    #[doc = "Bits 0:31 - Interrupt clear-enable bits."]
    #[inline]
    pub fn clrena(&mut self) -> _CLRENAW {
        _CLRENAW { w: self }
    }
}
