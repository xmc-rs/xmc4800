#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RSTSET {
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
}
#[doc = "Values that can be written to the field `HIBWK`"]
pub enum HIBWKW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset status bit"]
    VALUE2,
}
impl HIBWKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HIBWKW::VALUE1 => false,
            HIBWKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIBWKW<'a> {
    w: &'a mut W,
}
impl<'a> _HIBWKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIBWKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HIBWKW::VALUE1)
    }
    #[doc = "Assert reset status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HIBWKW::VALUE2)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HIBRS`"]
pub enum HIBRSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset"]
    VALUE2,
}
impl HIBRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HIBRSW::VALUE1 => false,
            HIBRSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIBRSW<'a> {
    w: &'a mut W,
}
impl<'a> _HIBRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIBRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HIBRSW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HIBRSW::VALUE2)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCKEN`"]
pub enum LCKENW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Enable reset when Lockup gets asserted"]
    VALUE2,
}
impl LCKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LCKENW::VALUE1 => false,
            LCKENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCKENW<'a> {
    w: &'a mut W,
}
impl<'a> _LCKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LCKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LCKENW::VALUE1)
    }
    #[doc = "Enable reset when Lockup gets asserted"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LCKENW::VALUE2)
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
#[doc = "Values that can be written to the field `ECAT0RS`"]
pub enum ECAT0RSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset status bit"]
    VALUE2,
}
impl ECAT0RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECAT0RSW::VALUE1 => false,
            ECAT0RSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECAT0RSW<'a> {
    w: &'a mut W,
}
impl<'a> _ECAT0RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECAT0RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECAT0RSW::VALUE1)
    }
    #[doc = "Assert reset status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECAT0RSW::VALUE2)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bit 8 - Set Hibernate Wake-up Reset Status"]
    #[inline]
    pub fn hibwk(&mut self) -> _HIBWKW {
        _HIBWKW { w: self }
    }
    #[doc = "Bit 9 - Set Hibernate Reset"]
    #[inline]
    pub fn hibrs(&mut self) -> _HIBRSW {
        _HIBRSW { w: self }
    }
    #[doc = "Bit 10 - Enable Lockup Reset"]
    #[inline]
    pub fn lcken(&mut self) -> _LCKENW {
        _LCKENW { w: self }
    }
    #[doc = "Bit 12 - ECAT0 Reset Status Information"]
    #[inline]
    pub fn ecat0rs(&mut self) -> _ECAT0RSW {
        _ECAT0RSW { w: self }
    }
}
