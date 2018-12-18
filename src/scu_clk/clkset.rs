#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKSET {
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
#[doc = "Values that can be written to the field `USBCEN`"]
pub enum USBCENW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl USBCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBCENW::VALUE1 => false,
            USBCENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBCENW<'a> {
    w: &'a mut W,
}
impl<'a> _USBCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(USBCENW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(USBCENW::VALUE2)
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
#[doc = "Values that can be written to the field `MMCCEN`"]
pub enum MMCCENW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl MMCCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MMCCENW::VALUE1 => false,
            MMCCENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMCCENW<'a> {
    w: &'a mut W,
}
impl<'a> _MMCCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MMCCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MMCCENW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MMCCENW::VALUE2)
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
#[doc = "Values that can be written to the field `ETH0CEN`"]
pub enum ETH0CENW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl ETH0CENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ETH0CENW::VALUE1 => false,
            ETH0CENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETH0CENW<'a> {
    w: &'a mut W,
}
impl<'a> _ETH0CENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETH0CENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ETH0CENW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ETH0CENW::VALUE2)
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
#[doc = "Values that can be written to the field `EBUCEN`"]
pub enum EBUCENW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl EBUCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EBUCENW::VALUE1 => false,
            EBUCENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EBUCENW<'a> {
    w: &'a mut W,
}
impl<'a> _EBUCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EBUCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EBUCENW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EBUCENW::VALUE2)
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
#[doc = "Values that can be written to the field `CCUCEN`"]
pub enum CCUCENW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl CCUCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCUCENW::VALUE1 => false,
            CCUCENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCUCENW<'a> {
    w: &'a mut W,
}
impl<'a> _CCUCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCUCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCUCENW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCUCENW::VALUE2)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WDTCEN`"]
pub enum WDTCENW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl WDTCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDTCENW::VALUE1 => false,
            WDTCENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDTCENW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WDTCENW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WDTCENW::VALUE2)
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
        const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - USB Clock Enable"]
    #[inline]
    pub fn usbcen(&mut self) -> _USBCENW {
        _USBCENW { w: self }
    }
    #[doc = "Bit 1 - MMC Clock Enable"]
    #[inline]
    pub fn mmccen(&mut self) -> _MMCCENW {
        _MMCCENW { w: self }
    }
    #[doc = "Bit 2 - Ethernet Clock Enable"]
    #[inline]
    pub fn eth0cen(&mut self) -> _ETH0CENW {
        _ETH0CENW { w: self }
    }
    #[doc = "Bit 3 - EBU Clock Enable"]
    #[inline]
    pub fn ebucen(&mut self) -> _EBUCENW {
        _EBUCENW { w: self }
    }
    #[doc = "Bit 4 - CCU Clock Enable"]
    #[inline]
    pub fn ccucen(&mut self) -> _CCUCENW {
        _CCUCENW { w: self }
    }
    #[doc = "Bit 5 - WDT Clock Enable"]
    #[inline]
    pub fn wdtcen(&mut self) -> _WDTCENW {
        _WDTCENW { w: self }
    }
}
