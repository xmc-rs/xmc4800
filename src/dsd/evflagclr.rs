#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVFLAGCLR {
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
#[doc = "Values that can be written to the field `RESEC0`"]
pub enum RESEC0W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear bit RESEVx"]
    VALUE2,
}
impl RESEC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESEC0W::VALUE1 => false,
            RESEC0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESEC0W<'a> {
    w: &'a mut W,
}
impl<'a> _RESEC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESEC0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESEC0W::VALUE1)
    }
    #[doc = "Clear bit RESEVx"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESEC0W::VALUE2)
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
#[doc = "Values that can be written to the field `RESEC1`"]
pub enum RESEC1W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear bit RESEVx"]
    VALUE2,
}
impl RESEC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESEC1W::VALUE1 => false,
            RESEC1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESEC1W<'a> {
    w: &'a mut W,
}
impl<'a> _RESEC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESEC1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESEC1W::VALUE1)
    }
    #[doc = "Clear bit RESEVx"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESEC1W::VALUE2)
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
#[doc = "Values that can be written to the field `RESEC2`"]
pub enum RESEC2W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear bit RESEVx"]
    VALUE2,
}
impl RESEC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESEC2W::VALUE1 => false,
            RESEC2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESEC2W<'a> {
    w: &'a mut W,
}
impl<'a> _RESEC2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESEC2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESEC2W::VALUE1)
    }
    #[doc = "Clear bit RESEVx"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESEC2W::VALUE2)
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
#[doc = "Values that can be written to the field `RESEC3`"]
pub enum RESEC3W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear bit RESEVx"]
    VALUE2,
}
impl RESEC3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESEC3W::VALUE1 => false,
            RESEC3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESEC3W<'a> {
    w: &'a mut W,
}
impl<'a> _RESEC3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESEC3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESEC3W::VALUE1)
    }
    #[doc = "Clear bit RESEVx"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESEC3W::VALUE2)
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
#[doc = "Values that can be written to the field `ALEC0`"]
pub enum ALEC0W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear bit ALEVx"]
    VALUE2,
}
impl ALEC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALEC0W::VALUE1 => false,
            ALEC0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALEC0W<'a> {
    w: &'a mut W,
}
impl<'a> _ALEC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALEC0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEC0W::VALUE1)
    }
    #[doc = "Clear bit ALEVx"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEC0W::VALUE2)
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
#[doc = "Values that can be written to the field `ALEC1`"]
pub enum ALEC1W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear bit ALEVx"]
    VALUE2,
}
impl ALEC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALEC1W::VALUE1 => false,
            ALEC1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALEC1W<'a> {
    w: &'a mut W,
}
impl<'a> _ALEC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALEC1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEC1W::VALUE1)
    }
    #[doc = "Clear bit ALEVx"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEC1W::VALUE2)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALEC2`"]
pub enum ALEC2W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear bit ALEVx"]
    VALUE2,
}
impl ALEC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALEC2W::VALUE1 => false,
            ALEC2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALEC2W<'a> {
    w: &'a mut W,
}
impl<'a> _ALEC2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALEC2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEC2W::VALUE1)
    }
    #[doc = "Clear bit ALEVx"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEC2W::VALUE2)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALEC3`"]
pub enum ALEC3W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear bit ALEVx"]
    VALUE2,
}
impl ALEC3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALEC3W::VALUE1 => false,
            ALEC3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALEC3W<'a> {
    w: &'a mut W,
}
impl<'a> _ALEC3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALEC3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEC3W::VALUE1)
    }
    #[doc = "Clear bit ALEVx"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEC3W::VALUE2)
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
        const OFFSET: u8 = 19;
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
    #[doc = "Bit 0 - Result Event Clear"]
    #[inline]
    pub fn resec0(&mut self) -> _RESEC0W {
        _RESEC0W { w: self }
    }
    #[doc = "Bit 1 - Result Event Clear"]
    #[inline]
    pub fn resec1(&mut self) -> _RESEC1W {
        _RESEC1W { w: self }
    }
    #[doc = "Bit 2 - Result Event Clear"]
    #[inline]
    pub fn resec2(&mut self) -> _RESEC2W {
        _RESEC2W { w: self }
    }
    #[doc = "Bit 3 - Result Event Clear"]
    #[inline]
    pub fn resec3(&mut self) -> _RESEC3W {
        _RESEC3W { w: self }
    }
    #[doc = "Bit 16 - Alarm Event Clear"]
    #[inline]
    pub fn alec0(&mut self) -> _ALEC0W {
        _ALEC0W { w: self }
    }
    #[doc = "Bit 17 - Alarm Event Clear"]
    #[inline]
    pub fn alec1(&mut self) -> _ALEC1W {
        _ALEC1W { w: self }
    }
    #[doc = "Bit 18 - Alarm Event Clear"]
    #[inline]
    pub fn alec2(&mut self) -> _ALEC2W {
        _ALEC2W { w: self }
    }
    #[doc = "Bit 19 - Alarm Event Clear"]
    #[inline]
    pub fn alec3(&mut self) -> _ALEC3W {
        _ALEC3W { w: self }
    }
}
