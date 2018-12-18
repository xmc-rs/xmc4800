#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CGATSET2 {
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
#[doc = "Values that can be written to the field `WDT`"]
pub enum WDTW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Enable gating"]
    VALUE2,
}
impl WDTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDTW::VALUE1 => false,
            WDTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WDTW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WDTW::VALUE2)
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
#[doc = "Values that can be written to the field `ETH0`"]
pub enum ETH0W {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Enable gating"]
    VALUE2,
}
impl ETH0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ETH0W::VALUE1 => false,
            ETH0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETH0W<'a> {
    w: &'a mut W,
}
impl<'a> _ETH0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETH0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ETH0W::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ETH0W::VALUE2)
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
#[doc = "Values that can be written to the field `DMA0`"]
pub enum DMA0W {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Enable gating"]
    VALUE2,
}
impl DMA0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA0W::VALUE1 => false,
            DMA0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA0W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DMA0W::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DMA0W::VALUE2)
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
#[doc = "Values that can be written to the field `DMA1`"]
pub enum DMA1W {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Enable gating"]
    VALUE2,
}
impl DMA1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA1W::VALUE1 => false,
            DMA1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA1W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DMA1W::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DMA1W::VALUE2)
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
#[doc = "Values that can be written to the field `FCE`"]
pub enum FCEW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Enable gating"]
    VALUE2,
}
impl FCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FCEW::VALUE1 => false,
            FCEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCEW<'a> {
    w: &'a mut W,
}
impl<'a> _FCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FCEW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FCEW::VALUE2)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USB`"]
pub enum USBW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Enable gating"]
    VALUE2,
}
impl USBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBW::VALUE1 => false,
            USBW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBW<'a> {
    w: &'a mut W,
}
impl<'a> _USBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(USBW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(USBW::VALUE2)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ECAT0`"]
pub enum ECAT0W {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Enable gating"]
    VALUE2,
}
impl ECAT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECAT0W::VALUE1 => false,
            ECAT0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECAT0W<'a> {
    w: &'a mut W,
}
impl<'a> _ECAT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECAT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECAT0W::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECAT0W::VALUE2)
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
    #[doc = "Bit 1 - WDT Gating Set"]
    #[inline]
    pub fn wdt(&mut self) -> _WDTW {
        _WDTW { w: self }
    }
    #[doc = "Bit 2 - ETH0 Gating Set"]
    #[inline]
    pub fn eth0(&mut self) -> _ETH0W {
        _ETH0W { w: self }
    }
    #[doc = "Bit 4 - DMA0 Gating Set"]
    #[inline]
    pub fn dma0(&mut self) -> _DMA0W {
        _DMA0W { w: self }
    }
    #[doc = "Bit 5 - DMA1 Gating Set"]
    #[inline]
    pub fn dma1(&mut self) -> _DMA1W {
        _DMA1W { w: self }
    }
    #[doc = "Bit 6 - FCE Gating Set"]
    #[inline]
    pub fn fce(&mut self) -> _FCEW {
        _FCEW { w: self }
    }
    #[doc = "Bit 7 - USB Gating Set"]
    #[inline]
    pub fn usb(&mut self) -> _USBW {
        _USBW { w: self }
    }
    #[doc = "Bit 10 - ECAT0 Gating Set"]
    #[inline]
    pub fn ecat0(&mut self) -> _ECAT0W {
        _ECAT0W { w: self }
    }
}
