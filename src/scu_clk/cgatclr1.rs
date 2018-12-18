#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CGATCLR1 {
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
#[doc = "Values that can be written to the field `CCU43`"]
pub enum CCU43W {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Disable gating"]
    VALUE2,
}
impl CCU43W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCU43W::VALUE1 => false,
            CCU43W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCU43W<'a> {
    w: &'a mut W,
}
impl<'a> _CCU43W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCU43W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU43W::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU43W::VALUE2)
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
#[doc = "Values that can be written to the field `LEDTSCU0`"]
pub enum LEDTSCU0W {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Disable gating"]
    VALUE2,
}
impl LEDTSCU0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LEDTSCU0W::VALUE1 => false,
            LEDTSCU0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LEDTSCU0W<'a> {
    w: &'a mut W,
}
impl<'a> _LEDTSCU0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LEDTSCU0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LEDTSCU0W::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LEDTSCU0W::VALUE2)
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
#[doc = "Values that can be written to the field `MCAN0`"]
pub enum MCAN0W {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Disable gating"]
    VALUE2,
}
impl MCAN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCAN0W::VALUE1 => false,
            MCAN0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCAN0W<'a> {
    w: &'a mut W,
}
impl<'a> _MCAN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCAN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCAN0W::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCAN0W::VALUE2)
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
#[doc = "Values that can be written to the field `DAC`"]
pub enum DACW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Disable gating"]
    VALUE2,
}
impl DACW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACW::VALUE1 => false,
            DACW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACW<'a> {
    w: &'a mut W,
}
impl<'a> _DACW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DACW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DACW::VALUE2)
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
#[doc = "Values that can be written to the field `MMCI`"]
pub enum MMCIW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Disable gating"]
    VALUE2,
}
impl MMCIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MMCIW::VALUE1 => false,
            MMCIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMCIW<'a> {
    w: &'a mut W,
}
impl<'a> _MMCIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MMCIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MMCIW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MMCIW::VALUE2)
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
#[doc = "Values that can be written to the field `USIC1`"]
pub enum USIC1W {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Disable gating"]
    VALUE2,
}
impl USIC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USIC1W::VALUE1 => false,
            USIC1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USIC1W<'a> {
    w: &'a mut W,
}
impl<'a> _USIC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USIC1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(USIC1W::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(USIC1W::VALUE2)
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
#[doc = "Values that can be written to the field `USIC2`"]
pub enum USIC2W {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Disable gating"]
    VALUE2,
}
impl USIC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USIC2W::VALUE1 => false,
            USIC2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USIC2W<'a> {
    w: &'a mut W,
}
impl<'a> _USIC2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USIC2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(USIC2W::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(USIC2W::VALUE2)
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
#[doc = "Values that can be written to the field `PPORTS`"]
pub enum PPORTSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Disable gating"]
    VALUE2,
}
impl PPORTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPORTSW::VALUE1 => false,
            PPORTSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPORTSW<'a> {
    w: &'a mut W,
}
impl<'a> _PPORTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPORTSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPORTSW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPORTSW::VALUE2)
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
    #[doc = "Bit 0 - CCU43 Gating Clear"]
    #[inline]
    pub fn ccu43(&mut self) -> _CCU43W {
        _CCU43W { w: self }
    }
    #[doc = "Bit 3 - LEDTS Gating Clear"]
    #[inline]
    pub fn ledtscu0(&mut self) -> _LEDTSCU0W {
        _LEDTSCU0W { w: self }
    }
    #[doc = "Bit 4 - MultiCAN Gating Clear"]
    #[inline]
    pub fn mcan0(&mut self) -> _MCAN0W {
        _MCAN0W { w: self }
    }
    #[doc = "Bit 5 - DAC Gating Clear"]
    #[inline]
    pub fn dac(&mut self) -> _DACW {
        _DACW { w: self }
    }
    #[doc = "Bit 6 - MMC Interface Gating Clear"]
    #[inline]
    pub fn mmci(&mut self) -> _MMCIW {
        _MMCIW { w: self }
    }
    #[doc = "Bit 7 - USIC1 Gating Clear"]
    #[inline]
    pub fn usic1(&mut self) -> _USIC1W {
        _USIC1W { w: self }
    }
    #[doc = "Bit 8 - USIC2 Gating Clear"]
    #[inline]
    pub fn usic2(&mut self) -> _USIC2W {
        _USIC2W { w: self }
    }
    #[doc = "Bit 9 - PORTS Gating Clear"]
    #[inline]
    pub fn pports(&mut self) -> _PPORTSW {
        _PPORTSW { w: self }
    }
}
