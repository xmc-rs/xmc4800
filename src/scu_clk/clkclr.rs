#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKCLR {
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
#[doc = "Values that can be written to the field `USBCDI`"]
pub enum USBCDIW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Disable clock"]
    VALUE2,
}
impl USBCDIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBCDIW::VALUE1 => false,
            USBCDIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBCDIW<'a> {
    w: &'a mut W,
}
impl<'a> _USBCDIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBCDIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(USBCDIW::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(USBCDIW::VALUE2)
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
#[doc = "Values that can be written to the field `MMCCDI`"]
pub enum MMCCDIW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Disable clock"]
    VALUE2,
}
impl MMCCDIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MMCCDIW::VALUE1 => false,
            MMCCDIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMCCDIW<'a> {
    w: &'a mut W,
}
impl<'a> _MMCCDIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MMCCDIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MMCCDIW::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MMCCDIW::VALUE2)
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
#[doc = "Values that can be written to the field `ETH0CDI`"]
pub enum ETH0CDIW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Disable clock"]
    VALUE2,
}
impl ETH0CDIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ETH0CDIW::VALUE1 => false,
            ETH0CDIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETH0CDIW<'a> {
    w: &'a mut W,
}
impl<'a> _ETH0CDIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETH0CDIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ETH0CDIW::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ETH0CDIW::VALUE2)
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
#[doc = "Values that can be written to the field `EBUCDI`"]
pub enum EBUCDIW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Disable clock"]
    VALUE2,
}
impl EBUCDIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EBUCDIW::VALUE1 => false,
            EBUCDIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EBUCDIW<'a> {
    w: &'a mut W,
}
impl<'a> _EBUCDIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EBUCDIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EBUCDIW::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EBUCDIW::VALUE2)
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
#[doc = "Values that can be written to the field `CCUCDI`"]
pub enum CCUCDIW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Disable clock"]
    VALUE2,
}
impl CCUCDIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCUCDIW::VALUE1 => false,
            CCUCDIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCUCDIW<'a> {
    w: &'a mut W,
}
impl<'a> _CCUCDIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCUCDIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCUCDIW::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCUCDIW::VALUE2)
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
#[doc = "Values that can be written to the field `WDTCDI`"]
pub enum WDTCDIW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Disable clock"]
    VALUE2,
}
impl WDTCDIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDTCDIW::VALUE1 => false,
            WDTCDIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDTCDIW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTCDIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTCDIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WDTCDIW::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WDTCDIW::VALUE2)
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
    #[doc = "Bit 0 - USB Clock Disable"]
    #[inline]
    pub fn usbcdi(&mut self) -> _USBCDIW {
        _USBCDIW { w: self }
    }
    #[doc = "Bit 1 - MMC Clock Disable"]
    #[inline]
    pub fn mmccdi(&mut self) -> _MMCCDIW {
        _MMCCDIW { w: self }
    }
    #[doc = "Bit 2 - Ethernet Clock Disable"]
    #[inline]
    pub fn eth0cdi(&mut self) -> _ETH0CDIW {
        _ETH0CDIW { w: self }
    }
    #[doc = "Bit 3 - EBU Clock Disable"]
    #[inline]
    pub fn ebucdi(&mut self) -> _EBUCDIW {
        _EBUCDIW { w: self }
    }
    #[doc = "Bit 4 - CCU Clock Disable"]
    #[inline]
    pub fn ccucdi(&mut self) -> _CCUCDIW {
        _CCUCDIW { w: self }
    }
    #[doc = "Bit 5 - WDT Clock Disable"]
    #[inline]
    pub fn wdtcdi(&mut self) -> _WDTCDIW {
        _WDTCDIW { w: self }
    }
}
