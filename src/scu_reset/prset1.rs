#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRSET1 {
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
#[doc = "Values that can be written to the field `CCU43RS`"]
pub enum CCU43RSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset"]
    VALUE2,
}
impl CCU43RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCU43RSW::VALUE1 => false,
            CCU43RSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCU43RSW<'a> {
    w: &'a mut W,
}
impl<'a> _CCU43RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCU43RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU43RSW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU43RSW::VALUE2)
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
#[doc = "Values that can be written to the field `LEDTSCU0RS`"]
pub enum LEDTSCU0RSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset"]
    VALUE2,
}
impl LEDTSCU0RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LEDTSCU0RSW::VALUE1 => false,
            LEDTSCU0RSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LEDTSCU0RSW<'a> {
    w: &'a mut W,
}
impl<'a> _LEDTSCU0RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LEDTSCU0RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LEDTSCU0RSW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LEDTSCU0RSW::VALUE2)
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
#[doc = "Values that can be written to the field `MCAN0RS`"]
pub enum MCAN0RSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset"]
    VALUE2,
}
impl MCAN0RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCAN0RSW::VALUE1 => false,
            MCAN0RSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCAN0RSW<'a> {
    w: &'a mut W,
}
impl<'a> _MCAN0RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCAN0RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCAN0RSW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCAN0RSW::VALUE2)
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
#[doc = "Values that can be written to the field `DACRS`"]
pub enum DACRSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset"]
    VALUE2,
}
impl DACRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACRSW::VALUE1 => false,
            DACRSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACRSW<'a> {
    w: &'a mut W,
}
impl<'a> _DACRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DACRSW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DACRSW::VALUE2)
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
#[doc = "Values that can be written to the field `MMCIRS`"]
pub enum MMCIRSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset"]
    VALUE2,
}
impl MMCIRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MMCIRSW::VALUE1 => false,
            MMCIRSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMCIRSW<'a> {
    w: &'a mut W,
}
impl<'a> _MMCIRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MMCIRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MMCIRSW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MMCIRSW::VALUE2)
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
#[doc = "Values that can be written to the field `USIC1RS`"]
pub enum USIC1RSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset"]
    VALUE2,
}
impl USIC1RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USIC1RSW::VALUE1 => false,
            USIC1RSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USIC1RSW<'a> {
    w: &'a mut W,
}
impl<'a> _USIC1RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USIC1RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(USIC1RSW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(USIC1RSW::VALUE2)
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
#[doc = "Values that can be written to the field `USIC2RS`"]
pub enum USIC2RSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset"]
    VALUE2,
}
impl USIC2RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USIC2RSW::VALUE1 => false,
            USIC2RSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USIC2RSW<'a> {
    w: &'a mut W,
}
impl<'a> _USIC2RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USIC2RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(USIC2RSW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(USIC2RSW::VALUE2)
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
#[doc = "Values that can be written to the field `PPORTSRS`"]
pub enum PPORTSRSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset"]
    VALUE2,
}
impl PPORTSRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPORTSRSW::VALUE1 => false,
            PPORTSRSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPORTSRSW<'a> {
    w: &'a mut W,
}
impl<'a> _PPORTSRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPORTSRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPORTSRSW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPORTSRSW::VALUE2)
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
    #[doc = "Bit 0 - CCU43 Reset Assert"]
    #[inline]
    pub fn ccu43rs(&mut self) -> _CCU43RSW {
        _CCU43RSW { w: self }
    }
    #[doc = "Bit 3 - LEDTS Reset Assert"]
    #[inline]
    pub fn ledtscu0rs(&mut self) -> _LEDTSCU0RSW {
        _LEDTSCU0RSW { w: self }
    }
    #[doc = "Bit 4 - MultiCAN Reset Assert"]
    #[inline]
    pub fn mcan0rs(&mut self) -> _MCAN0RSW {
        _MCAN0RSW { w: self }
    }
    #[doc = "Bit 5 - DAC Reset Assert"]
    #[inline]
    pub fn dacrs(&mut self) -> _DACRSW {
        _DACRSW { w: self }
    }
    #[doc = "Bit 6 - MMC Interface Reset Assert"]
    #[inline]
    pub fn mmcirs(&mut self) -> _MMCIRSW {
        _MMCIRSW { w: self }
    }
    #[doc = "Bit 7 - USIC1 Reset Assert"]
    #[inline]
    pub fn usic1rs(&mut self) -> _USIC1RSW {
        _USIC1RSW { w: self }
    }
    #[doc = "Bit 8 - USIC2 Reset Assert"]
    #[inline]
    pub fn usic2rs(&mut self) -> _USIC2RSW {
        _USIC2RSW { w: self }
    }
    #[doc = "Bit 9 - PORTS Reset Assert"]
    #[inline]
    pub fn pportsrs(&mut self) -> _PPORTSRSW {
        _PPORTSRSW { w: self }
    }
}
