#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRSET0 {
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
#[doc = "Values that can be written to the field `VADCRS`"]
pub enum VADCRSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset"]
    VALUE2,
}
impl VADCRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VADCRSW::VALUE1 => false,
            VADCRSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VADCRSW<'a> {
    w: &'a mut W,
}
impl<'a> _VADCRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VADCRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VADCRSW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VADCRSW::VALUE2)
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
#[doc = "Values that can be written to the field `DSDRS`"]
pub enum DSDRSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset"]
    VALUE2,
}
impl DSDRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DSDRSW::VALUE1 => false,
            DSDRSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSDRSW<'a> {
    w: &'a mut W,
}
impl<'a> _DSDRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSDRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DSDRSW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DSDRSW::VALUE2)
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
#[doc = "Values that can be written to the field `CCU40RS`"]
pub enum CCU40RSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset"]
    VALUE2,
}
impl CCU40RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCU40RSW::VALUE1 => false,
            CCU40RSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCU40RSW<'a> {
    w: &'a mut W,
}
impl<'a> _CCU40RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCU40RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU40RSW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU40RSW::VALUE2)
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
#[doc = "Values that can be written to the field `CCU41RS`"]
pub enum CCU41RSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset"]
    VALUE2,
}
impl CCU41RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCU41RSW::VALUE1 => false,
            CCU41RSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCU41RSW<'a> {
    w: &'a mut W,
}
impl<'a> _CCU41RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCU41RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU41RSW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU41RSW::VALUE2)
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
#[doc = "Values that can be written to the field `CCU42RS`"]
pub enum CCU42RSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset"]
    VALUE2,
}
impl CCU42RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCU42RSW::VALUE1 => false,
            CCU42RSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCU42RSW<'a> {
    w: &'a mut W,
}
impl<'a> _CCU42RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCU42RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU42RSW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU42RSW::VALUE2)
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
#[doc = "Values that can be written to the field `CCU80RS`"]
pub enum CCU80RSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset"]
    VALUE2,
}
impl CCU80RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCU80RSW::VALUE1 => false,
            CCU80RSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCU80RSW<'a> {
    w: &'a mut W,
}
impl<'a> _CCU80RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCU80RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU80RSW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU80RSW::VALUE2)
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
#[doc = "Values that can be written to the field `CCU81RS`"]
pub enum CCU81RSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset"]
    VALUE2,
}
impl CCU81RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCU81RSW::VALUE1 => false,
            CCU81RSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCU81RSW<'a> {
    w: &'a mut W,
}
impl<'a> _CCU81RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCU81RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU81RSW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU81RSW::VALUE2)
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
#[doc = "Values that can be written to the field `POSIF0RS`"]
pub enum POSIF0RSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset"]
    VALUE2,
}
impl POSIF0RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POSIF0RSW::VALUE1 => false,
            POSIF0RSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POSIF0RSW<'a> {
    w: &'a mut W,
}
impl<'a> _POSIF0RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POSIF0RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(POSIF0RSW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(POSIF0RSW::VALUE2)
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
#[doc = "Values that can be written to the field `POSIF1RS`"]
pub enum POSIF1RSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset"]
    VALUE2,
}
impl POSIF1RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POSIF1RSW::VALUE1 => false,
            POSIF1RSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POSIF1RSW<'a> {
    w: &'a mut W,
}
impl<'a> _POSIF1RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POSIF1RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(POSIF1RSW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(POSIF1RSW::VALUE2)
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
#[doc = "Values that can be written to the field `USIC0RS`"]
pub enum USIC0RSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset"]
    VALUE2,
}
impl USIC0RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USIC0RSW::VALUE1 => false,
            USIC0RSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USIC0RSW<'a> {
    w: &'a mut W,
}
impl<'a> _USIC0RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USIC0RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(USIC0RSW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(USIC0RSW::VALUE2)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERU1RS`"]
pub enum ERU1RSW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Assert reset"]
    VALUE2,
}
impl ERU1RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERU1RSW::VALUE1 => false,
            ERU1RSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERU1RSW<'a> {
    w: &'a mut W,
}
impl<'a> _ERU1RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERU1RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ERU1RSW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ERU1RSW::VALUE2)
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
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - VADC Reset Assert"]
    #[inline]
    pub fn vadcrs(&mut self) -> _VADCRSW {
        _VADCRSW { w: self }
    }
    #[doc = "Bit 1 - DSD Reset Assert"]
    #[inline]
    pub fn dsdrs(&mut self) -> _DSDRSW {
        _DSDRSW { w: self }
    }
    #[doc = "Bit 2 - CCU40 Reset Assert"]
    #[inline]
    pub fn ccu40rs(&mut self) -> _CCU40RSW {
        _CCU40RSW { w: self }
    }
    #[doc = "Bit 3 - CCU41 Reset Assert"]
    #[inline]
    pub fn ccu41rs(&mut self) -> _CCU41RSW {
        _CCU41RSW { w: self }
    }
    #[doc = "Bit 4 - CCU42 Reset Assert"]
    #[inline]
    pub fn ccu42rs(&mut self) -> _CCU42RSW {
        _CCU42RSW { w: self }
    }
    #[doc = "Bit 7 - CCU80 Reset Assert"]
    #[inline]
    pub fn ccu80rs(&mut self) -> _CCU80RSW {
        _CCU80RSW { w: self }
    }
    #[doc = "Bit 8 - CCU81 Reset Assert"]
    #[inline]
    pub fn ccu81rs(&mut self) -> _CCU81RSW {
        _CCU81RSW { w: self }
    }
    #[doc = "Bit 9 - POSIF0 Reset Assert"]
    #[inline]
    pub fn posif0rs(&mut self) -> _POSIF0RSW {
        _POSIF0RSW { w: self }
    }
    #[doc = "Bit 10 - POSIF1 Reset Assert"]
    #[inline]
    pub fn posif1rs(&mut self) -> _POSIF1RSW {
        _POSIF1RSW { w: self }
    }
    #[doc = "Bit 11 - USIC0 Reset Assert"]
    #[inline]
    pub fn usic0rs(&mut self) -> _USIC0RSW {
        _USIC0RSW { w: self }
    }
    #[doc = "Bit 16 - ERU1 Reset Assert"]
    #[inline]
    pub fn eru1rs(&mut self) -> _ERU1RSW {
        _ERU1RSW { w: self }
    }
}
