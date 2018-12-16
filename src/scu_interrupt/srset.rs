#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SRSET {
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
#[doc = "Values that can be written to the field `PRWARN`"]
pub enum PRWARNW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "set the status bit"]
    VALUE2,
}
impl PRWARNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRWARNW::VALUE1 => false,
            PRWARNW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRWARNW<'a> {
    w: &'a mut W,
}
impl<'a> _PRWARNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRWARNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRWARNW::VALUE1)
    }
    #[doc = "set the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRWARNW::VALUE2)
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
#[doc = "Values that can be written to the field `PI`"]
pub enum PIW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "set the status bit"]
    VALUE2,
}
impl PIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIW::VALUE1 => false,
            PIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIW<'a> {
    w: &'a mut W,
}
impl<'a> _PIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PIW::VALUE1)
    }
    #[doc = "set the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PIW::VALUE2)
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
#[doc = "Values that can be written to the field `AI`"]
pub enum AIW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "set the status bit"]
    VALUE2,
}
impl AIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AIW::VALUE1 => false,
            AIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AIW<'a> {
    w: &'a mut W,
}
impl<'a> _AIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AIW::VALUE1)
    }
    #[doc = "set the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AIW::VALUE2)
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
#[doc = "Values that can be written to the field `DLROVR`"]
pub enum DLROVRW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "set the status bit"]
    VALUE2,
}
impl DLROVRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DLROVRW::VALUE1 => false,
            DLROVRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DLROVRW<'a> {
    w: &'a mut W,
}
impl<'a> _DLROVRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DLROVRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DLROVRW::VALUE1)
    }
    #[doc = "set the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DLROVRW::VALUE2)
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
#[doc = "Values that can be written to the field `HDCRCLR`"]
pub enum HDCRCLRW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "set the status bit"]
    VALUE2,
}
impl HDCRCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HDCRCLRW::VALUE1 => false,
            HDCRCLRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HDCRCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _HDCRCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HDCRCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HDCRCLRW::VALUE1)
    }
    #[doc = "set the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HDCRCLRW::VALUE2)
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
#[doc = "Values that can be written to the field `HDCRSET`"]
pub enum HDCRSETW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "set the status bit"]
    VALUE2,
}
impl HDCRSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HDCRSETW::VALUE1 => false,
            HDCRSETW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HDCRSETW<'a> {
    w: &'a mut W,
}
impl<'a> _HDCRSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HDCRSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HDCRSETW::VALUE1)
    }
    #[doc = "set the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HDCRSETW::VALUE2)
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
#[doc = "Values that can be written to the field `HDCR`"]
pub enum HDCRW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "set the status bit"]
    VALUE2,
}
impl HDCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HDCRW::VALUE1 => false,
            HDCRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HDCRW<'a> {
    w: &'a mut W,
}
impl<'a> _HDCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HDCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HDCRW::VALUE1)
    }
    #[doc = "set the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HDCRW::VALUE2)
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
#[doc = "Values that can be written to the field `OSCSICTRL`"]
pub enum OSCSICTRLW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "set the status bit"]
    VALUE2,
}
impl OSCSICTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OSCSICTRLW::VALUE1 => false,
            OSCSICTRLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCSICTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCSICTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCSICTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(OSCSICTRLW::VALUE1)
    }
    #[doc = "set the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(OSCSICTRLW::VALUE2)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSCULCTRL`"]
pub enum OSCULCTRLW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "set the status bit"]
    VALUE2,
}
impl OSCULCTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OSCULCTRLW::VALUE1 => false,
            OSCULCTRLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCULCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCULCTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCULCTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(OSCULCTRLW::VALUE1)
    }
    #[doc = "set the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(OSCULCTRLW::VALUE2)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_CTR`"]
pub enum RTC_CTRW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "set the status bit"]
    VALUE2,
}
impl RTC_CTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_CTRW::VALUE1 => false,
            RTC_CTRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_CTRW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_CTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_CTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_CTRW::VALUE1)
    }
    #[doc = "set the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_CTRW::VALUE2)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_ATIM0`"]
pub enum RTC_ATIM0W {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "set the status bit"]
    VALUE2,
}
impl RTC_ATIM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_ATIM0W::VALUE1 => false,
            RTC_ATIM0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_ATIM0W<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_ATIM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_ATIM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_ATIM0W::VALUE1)
    }
    #[doc = "set the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_ATIM0W::VALUE2)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_ATIM1`"]
pub enum RTC_ATIM1W {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "set the status bit"]
    VALUE2,
}
impl RTC_ATIM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_ATIM1W::VALUE1 => false,
            RTC_ATIM1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_ATIM1W<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_ATIM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_ATIM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_ATIM1W::VALUE1)
    }
    #[doc = "set the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_ATIM1W::VALUE2)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_TIM0`"]
pub enum RTC_TIM0W {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "set the status bit"]
    VALUE2,
}
impl RTC_TIM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_TIM0W::VALUE1 => false,
            RTC_TIM0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_TIM0W<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_TIM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_TIM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_TIM0W::VALUE1)
    }
    #[doc = "set the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_TIM0W::VALUE2)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_TIM1`"]
pub enum RTC_TIM1W {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "set the status bit"]
    VALUE2,
}
impl RTC_TIM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_TIM1W::VALUE1 => false,
            RTC_TIM1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_TIM1W<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_TIM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_TIM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_TIM1W::VALUE1)
    }
    #[doc = "set the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_TIM1W::VALUE2)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RMX`"]
pub enum RMXW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "set the status bit"]
    VALUE2,
}
impl RMXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RMXW::VALUE1 => false,
            RMXW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RMXW<'a> {
    w: &'a mut W,
}
impl<'a> _RMXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RMXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RMXW::VALUE1)
    }
    #[doc = "set the status bit"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RMXW::VALUE2)
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
        const OFFSET: u8 = 29;
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
    #[doc = "Bit 0 - WDT pre-warning Interrupt Set"]
    #[inline]
    pub fn prwarn(&mut self) -> _PRWARNW {
        _PRWARNW { w: self }
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Set"]
    #[inline]
    pub fn pi(&mut self) -> _PIW {
        _PIW { w: self }
    }
    #[doc = "Bit 2 - RTC Alarm Interrupt Set"]
    #[inline]
    pub fn ai(&mut self) -> _AIW {
        _AIW { w: self }
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt Set"]
    #[inline]
    pub fn dlrovr(&mut self) -> _DLROVRW {
        _DLROVRW { w: self }
    }
    #[doc = "Bit 17 - HDCRCLR Mirror Register Update Set"]
    #[inline]
    pub fn hdcrclr(&mut self) -> _HDCRCLRW {
        _HDCRCLRW { w: self }
    }
    #[doc = "Bit 18 - HDCRSET Mirror Register Update Set"]
    #[inline]
    pub fn hdcrset(&mut self) -> _HDCRSETW {
        _HDCRSETW { w: self }
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Set"]
    #[inline]
    pub fn hdcr(&mut self) -> _HDCRW {
        _HDCRW { w: self }
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Set"]
    #[inline]
    pub fn oscsictrl(&mut self) -> _OSCSICTRLW {
        _OSCSICTRLW { w: self }
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Set"]
    #[inline]
    pub fn osculctrl(&mut self) -> _OSCULCTRLW {
        _OSCULCTRLW { w: self }
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Set"]
    #[inline]
    pub fn rtc_ctr(&mut self) -> _RTC_CTRW {
        _RTC_CTRW { w: self }
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Set"]
    #[inline]
    pub fn rtc_atim0(&mut self) -> _RTC_ATIM0W {
        _RTC_ATIM0W { w: self }
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Set"]
    #[inline]
    pub fn rtc_atim1(&mut self) -> _RTC_ATIM1W {
        _RTC_ATIM1W { w: self }
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Set"]
    #[inline]
    pub fn rtc_tim0(&mut self) -> _RTC_TIM0W {
        _RTC_TIM0W { w: self }
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Set"]
    #[inline]
    pub fn rtc_tim1(&mut self) -> _RTC_TIM1W {
        _RTC_TIM1W { w: self }
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Set"]
    #[inline]
    pub fn rmx(&mut self) -> _RMXW {
        _RMXW { w: self }
    }
}
