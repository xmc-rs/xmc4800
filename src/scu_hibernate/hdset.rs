#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HDSET {
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
#[doc = "Values that can be written to the field `EPEV`"]
pub enum EPEVW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Set wake-up event"]
    VALUE2,
}
impl EPEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPEVW::VALUE1 => false,
            EPEVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPEVW<'a> {
    w: &'a mut W,
}
impl<'a> _EPEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EPEVW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EPEVW::VALUE2)
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
#[doc = "Values that can be written to the field `ENEV`"]
pub enum ENEVW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Set wake-up event"]
    VALUE2,
}
impl ENEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENEVW::VALUE1 => false,
            ENEVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENEVW<'a> {
    w: &'a mut W,
}
impl<'a> _ENEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENEVW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENEVW::VALUE2)
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
#[doc = "Values that can be written to the field `RTCEV`"]
pub enum RTCEVW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Set wake-up event"]
    VALUE2,
}
impl RTCEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCEVW::VALUE1 => false,
            RTCEVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCEVW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTCEVW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTCEVW::VALUE2)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ULPWDG`"]
pub enum ULPWDGW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Set watchdog alarm"]
    VALUE2,
}
impl ULPWDGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ULPWDGW::VALUE1 => false,
            ULPWDGW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ULPWDGW<'a> {
    w: &'a mut W,
}
impl<'a> _ULPWDGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ULPWDGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ULPWDGW::VALUE1)
    }
    #[doc = "Set watchdog alarm"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ULPWDGW::VALUE2)
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
    #[doc = "Bit 0 - Wake-up Pin Event Positive Edge Set"]
    #[inline]
    pub fn epev(&mut self) -> _EPEVW {
        _EPEVW { w: self }
    }
    #[doc = "Bit 1 - Wake-up Pin Event Negative Edge Set"]
    #[inline]
    pub fn enev(&mut self) -> _ENEVW {
        _ENEVW { w: self }
    }
    #[doc = "Bit 2 - RTC Event Set"]
    #[inline]
    pub fn rtcev(&mut self) -> _RTCEVW {
        _RTCEVW { w: self }
    }
    #[doc = "Bit 3 - ULP WDG Alarm Set"]
    #[inline]
    pub fn ulpwdg(&mut self) -> _ULPWDGW {
        _ULPWDGW { w: self }
    }
}
