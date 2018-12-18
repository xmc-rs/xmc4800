#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CGCFG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
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
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `CGMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGMODR {
    #[doc = "Stopped"]
    VALUE1,
    #[doc = "Square wave"]
    VALUE2,
    #[doc = "Triangle"]
    VALUE3,
    #[doc = "Sine wave"]
    VALUE4,
}
impl CGMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CGMODR::VALUE1 => 0,
            CGMODR::VALUE2 => 1,
            CGMODR::VALUE3 => 2,
            CGMODR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CGMODR {
        match value {
            0 => CGMODR::VALUE1,
            1 => CGMODR::VALUE2,
            2 => CGMODR::VALUE3,
            3 => CGMODR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CGMODR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CGMODR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CGMODR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CGMODR::VALUE4
    }
}
#[doc = "Possible values of the field `BREV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BREVR {
    #[doc = "Normal mode"]
    VALUE1,
    #[doc = "Bit-reverse mode"]
    VALUE2,
}
impl BREVR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BREVR::VALUE1 => false,
            BREVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BREVR {
        match value {
            false => BREVR::VALUE1,
            true => BREVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BREVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BREVR::VALUE2
    }
}
#[doc = "Possible values of the field `SIGPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIGPOLR {
    #[doc = "Normal: carrier signal begins with +1"]
    VALUE1,
    #[doc = "Inverted: carrier signal begins with -1"]
    VALUE2,
}
impl SIGPOLR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SIGPOLR::VALUE1 => false,
            SIGPOLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SIGPOLR {
        match value {
            false => SIGPOLR::VALUE1,
            true => SIGPOLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SIGPOLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SIGPOLR::VALUE2
    }
}
#[doc = "Possible values of the field `DIVCG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVCGR {
    #[doc = "fCG = fCLK / 2"]
    VALUE1,
    #[doc = "fCG = fCLK / 4"]
    VALUE2,
    #[doc = "fCG = fCLK / 6"]
    VALUE3,
    #[doc = "fCG = fCLK / 32"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DIVCGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIVCGR::VALUE1 => 0,
            DIVCGR::VALUE2 => 1,
            DIVCGR::VALUE3 => 2,
            DIVCGR::VALUE4 => 15,
            DIVCGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIVCGR {
        match value {
            0 => DIVCGR::VALUE1,
            1 => DIVCGR::VALUE2,
            2 => DIVCGR::VALUE3,
            15 => DIVCGR::VALUE4,
            i => DIVCGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIVCGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIVCGR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DIVCGR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == DIVCGR::VALUE4
    }
}
#[doc = "Possible values of the field `RUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUNR {
    #[doc = "Stopped (cleared at the end of a period)"]
    VALUE1,
    #[doc = "Running"]
    VALUE2,
}
impl RUNR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RUNR::VALUE1 => false,
            RUNR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RUNR {
        match value {
            false => RUNR::VALUE1,
            true => RUNR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RUNR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RUNR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct BITCOUNTR {
    bits: u8,
}
impl BITCOUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STEPCOUNTR {
    bits: u8,
}
impl STEPCOUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `STEPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STEPSR {
    #[doc = "Step counter value is positive"]
    VALUE1,
    #[doc = "Step counter value is negative"]
    VALUE2,
}
impl STEPSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            STEPSR::VALUE1 => false,
            STEPSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STEPSR {
        match value {
            false => STEPSR::VALUE1,
            true => STEPSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STEPSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STEPSR::VALUE2
    }
}
#[doc = "Possible values of the field `STEPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STEPDR {
    #[doc = "Step counter is counting up"]
    VALUE1,
    #[doc = "Step counter is counting down"]
    VALUE2,
}
impl STEPDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            STEPDR::VALUE1 => false,
            STEPDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STEPDR {
        match value {
            false => STEPDR::VALUE1,
            true => STEPDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STEPDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STEPDR::VALUE2
    }
}
#[doc = "Possible values of the field `SGNCG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SGNCGR {
    #[doc = "Positive values"]
    VALUE1,
    #[doc = "Negative values"]
    VALUE2,
}
impl SGNCGR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SGNCGR::VALUE1 => false,
            SGNCGR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SGNCGR {
        match value {
            false => SGNCGR::VALUE1,
            true => SGNCGR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SGNCGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SGNCGR::VALUE2
    }
}
#[doc = "Values that can be written to the field `CGMOD`"]
pub enum CGMODW {
    #[doc = "Stopped"]
    VALUE1,
    #[doc = "Square wave"]
    VALUE2,
    #[doc = "Triangle"]
    VALUE3,
    #[doc = "Sine wave"]
    VALUE4,
}
impl CGMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CGMODW::VALUE1 => 0,
            CGMODW::VALUE2 => 1,
            CGMODW::VALUE3 => 2,
            CGMODW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CGMODW<'a> {
    w: &'a mut W,
}
impl<'a> _CGMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGMODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Stopped"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CGMODW::VALUE1)
    }
    #[doc = "Square wave"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CGMODW::VALUE2)
    }
    #[doc = "Triangle"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CGMODW::VALUE3)
    }
    #[doc = "Sine wave"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CGMODW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BREV`"]
pub enum BREVW {
    #[doc = "Normal mode"]
    VALUE1,
    #[doc = "Bit-reverse mode"]
    VALUE2,
}
impl BREVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BREVW::VALUE1 => false,
            BREVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BREVW<'a> {
    w: &'a mut W,
}
impl<'a> _BREVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BREVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BREVW::VALUE1)
    }
    #[doc = "Bit-reverse mode"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BREVW::VALUE2)
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
#[doc = "Values that can be written to the field `SIGPOL`"]
pub enum SIGPOLW {
    #[doc = "Normal: carrier signal begins with +1"]
    VALUE1,
    #[doc = "Inverted: carrier signal begins with -1"]
    VALUE2,
}
impl SIGPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIGPOLW::VALUE1 => false,
            SIGPOLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIGPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _SIGPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIGPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal: carrier signal begins with +1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIGPOLW::VALUE1)
    }
    #[doc = "Inverted: carrier signal begins with -1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIGPOLW::VALUE2)
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
#[doc = "Values that can be written to the field `DIVCG`"]
pub enum DIVCGW {
    #[doc = "fCG = fCLK / 2"]
    VALUE1,
    #[doc = "fCG = fCLK / 4"]
    VALUE2,
    #[doc = "fCG = fCLK / 6"]
    VALUE3,
    #[doc = "fCG = fCLK / 32"]
    VALUE4,
}
impl DIVCGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIVCGW::VALUE1 => 0,
            DIVCGW::VALUE2 => 1,
            DIVCGW::VALUE3 => 2,
            DIVCGW::VALUE4 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVCGW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVCGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVCGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "fCG = fCLK / 2"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVCGW::VALUE1)
    }
    #[doc = "fCG = fCLK / 4"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVCGW::VALUE2)
    }
    #[doc = "fCG = fCLK / 6"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DIVCGW::VALUE3)
    }
    #[doc = "fCG = fCLK / 32"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(DIVCGW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Carrier Generator Operating Mode"]
    #[inline]
    pub fn cgmod(&self) -> CGMODR {
        CGMODR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Bit-Reverse PWM Generation"]
    #[inline]
    pub fn brev(&self) -> BREVR {
        BREVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Signal Polarity"]
    #[inline]
    pub fn sigpol(&self) -> SIGPOLR {
        SIGPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:7 - Divider Factor for the PWM Pattern Signal Generator"]
    #[inline]
    pub fn divcg(&self) -> DIVCGR {
        DIVCGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Run Indicator"]
    #[inline]
    pub fn run(&self) -> RUNR {
        RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:20 - Bit Counter"]
    #[inline]
    pub fn bitcount(&self) -> BITCOUNTR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BITCOUNTR { bits }
    }
    #[doc = "Bits 24:27 - Step Counter"]
    #[inline]
    pub fn stepcount(&self) -> STEPCOUNTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STEPCOUNTR { bits }
    }
    #[doc = "Bit 28 - Step Counter Sign"]
    #[inline]
    pub fn steps(&self) -> STEPSR {
        STEPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Step Counter Direction"]
    #[inline]
    pub fn stepd(&self) -> STEPDR {
        STEPDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Sign Signal from Carrier Generator"]
    #[inline]
    pub fn sgncg(&self) -> SGNCGR {
        SGNCGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 118489088 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Carrier Generator Operating Mode"]
    #[inline]
    pub fn cgmod(&mut self) -> _CGMODW {
        _CGMODW { w: self }
    }
    #[doc = "Bit 2 - Bit-Reverse PWM Generation"]
    #[inline]
    pub fn brev(&mut self) -> _BREVW {
        _BREVW { w: self }
    }
    #[doc = "Bit 3 - Signal Polarity"]
    #[inline]
    pub fn sigpol(&mut self) -> _SIGPOLW {
        _SIGPOLW { w: self }
    }
    #[doc = "Bits 4:7 - Divider Factor for the PWM Pattern Signal Generator"]
    #[inline]
    pub fn divcg(&mut self) -> _DIVCGW {
        _DIVCGW { w: self }
    }
}
