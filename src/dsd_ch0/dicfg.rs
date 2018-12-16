#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DICFG {
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
#[doc = "Possible values of the field `DSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSRCR {
    #[doc = "Disconnected"]
    VALUE1,
    #[doc = "External, from input A, direct"]
    VALUE2,
    #[doc = "External, from input A, inverted"]
    VALUE3,
    #[doc = "External, from input B, direct"]
    VALUE4,
    #[doc = "External, from input B, inverted"]
    VALUE5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DSRCR::VALUE1 => 0,
            DSRCR::VALUE2 => 2,
            DSRCR::VALUE3 => 3,
            DSRCR::VALUE4 => 4,
            DSRCR::VALUE5 => 5,
            DSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DSRCR {
        match value {
            0 => DSRCR::VALUE1,
            2 => DSRCR::VALUE2,
            3 => DSRCR::VALUE3,
            4 => DSRCR::VALUE4,
            5 => DSRCR::VALUE5,
            i => DSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DSRCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DSRCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DSRCR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == DSRCR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == DSRCR::VALUE5
    }
}
#[doc = "Possible values of the field `ITRMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITRMODER {
    #[doc = "No integration trigger, integrator bypassed, INTEN = 0 all the time"]
    VALUE1,
    #[doc = "Trigger event upon a falling edge"]
    VALUE2,
    #[doc = "Trigger event upon a rising edge"]
    VALUE3,
    #[doc = "No trigger, integrator active all the time, INTEN = 1 all the time"]
    VALUE4,
}
impl ITRMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ITRMODER::VALUE1 => 0,
            ITRMODER::VALUE2 => 1,
            ITRMODER::VALUE3 => 2,
            ITRMODER::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ITRMODER {
        match value {
            0 => ITRMODER::VALUE1,
            1 => ITRMODER::VALUE2,
            2 => ITRMODER::VALUE3,
            3 => ITRMODER::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ITRMODER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ITRMODER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ITRMODER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == ITRMODER::VALUE4
    }
}
#[doc = "Possible values of the field `TSTRMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTRMODER {
    #[doc = "No timestamp trigger"]
    VALUE1,
    #[doc = "Trigger event upon a falling edge"]
    VALUE2,
    #[doc = "Trigger event upon a rising edge"]
    VALUE3,
    #[doc = "Trigger event upon each edge"]
    VALUE4,
}
impl TSTRMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSTRMODER::VALUE1 => 0,
            TSTRMODER::VALUE2 => 1,
            TSTRMODER::VALUE3 => 2,
            TSTRMODER::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSTRMODER {
        match value {
            0 => TSTRMODER::VALUE1,
            1 => TSTRMODER::VALUE2,
            2 => TSTRMODER::VALUE3,
            3 => TSTRMODER::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TSTRMODER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TSTRMODER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == TSTRMODER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == TSTRMODER::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct TRSELR {
    bits: u8,
}
impl TRSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSRCR {
    #[doc = "External, from input A"]
    VALUE2,
    #[doc = "External, from input B"]
    VALUE3,
    #[doc = "External, from input C"]
    VALUE4,
    #[doc = "External, from input D"]
    VALUE5,
    #[doc = "Internal clock"]
    VALUE6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSRCR::VALUE2 => 1,
            CSRCR::VALUE3 => 2,
            CSRCR::VALUE4 => 3,
            CSRCR::VALUE5 => 4,
            CSRCR::VALUE6 => 15,
            CSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSRCR {
        match value {
            1 => CSRCR::VALUE2,
            2 => CSRCR::VALUE3,
            3 => CSRCR::VALUE4,
            4 => CSRCR::VALUE5,
            15 => CSRCR::VALUE6,
            i => CSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CSRCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CSRCR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CSRCR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == CSRCR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == CSRCR::VALUE6
    }
}
#[doc = "Possible values of the field `STROBE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STROBER {
    #[doc = "No data strobe"]
    VALUE1,
    #[doc = "Direct clock, a sample trigger is generated at each rising clock edge"]
    VALUE2,
    #[doc = "Direct clock, a sample trigger is generated at each falling clock edge"]
    VALUE3,
    #[doc = "Double data, a sample trigger is generated at each rising and falling clock edge"]
    VALUE4,
    #[doc = "Double clock, a sample trigger is generated at every 2nd rising clock edge"]
    VALUE6,
    #[doc = "Double clock, a sample trigger is generated at every 2nd falling clock edge"]
    VALUE7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STROBER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STROBER::VALUE1 => 0,
            STROBER::VALUE2 => 1,
            STROBER::VALUE3 => 2,
            STROBER::VALUE4 => 3,
            STROBER::VALUE6 => 5,
            STROBER::VALUE7 => 6,
            STROBER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STROBER {
        match value {
            0 => STROBER::VALUE1,
            1 => STROBER::VALUE2,
            2 => STROBER::VALUE3,
            3 => STROBER::VALUE4,
            5 => STROBER::VALUE6,
            6 => STROBER::VALUE7,
            i => STROBER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STROBER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STROBER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == STROBER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == STROBER::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == STROBER::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == STROBER::VALUE7
    }
}
#[doc = "Values that can be written to the field `DSRC`"]
pub enum DSRCW {
    #[doc = "Disconnected"]
    VALUE1,
    #[doc = "External, from input A, direct"]
    VALUE2,
    #[doc = "External, from input A, inverted"]
    VALUE3,
    #[doc = "External, from input B, direct"]
    VALUE4,
    #[doc = "External, from input B, inverted"]
    VALUE5,
}
impl DSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DSRCW::VALUE1 => 0,
            DSRCW::VALUE2 => 2,
            DSRCW::VALUE3 => 3,
            DSRCW::VALUE4 => 4,
            DSRCW::VALUE5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _DSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disconnected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DSRCW::VALUE1)
    }
    #[doc = "External, from input A, direct"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DSRCW::VALUE2)
    }
    #[doc = "External, from input A, inverted"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DSRCW::VALUE3)
    }
    #[doc = "External, from input B, direct"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(DSRCW::VALUE4)
    }
    #[doc = "External, from input B, inverted"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(DSRCW::VALUE5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DSWC`"]
pub enum DSWCW {
    #[doc = "No write access to data parameters"]
    VALUE1,
    #[doc = "Bitfield DSRC can be written"]
    VALUE2,
}
impl DSWCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DSWCW::VALUE1 => false,
            DSWCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSWCW<'a> {
    w: &'a mut W,
}
impl<'a> _DSWCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSWCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No write access to data parameters"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DSWCW::VALUE1)
    }
    #[doc = "Bitfield DSRC can be written"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DSWCW::VALUE2)
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
#[doc = "Values that can be written to the field `ITRMODE`"]
pub enum ITRMODEW {
    #[doc = "No integration trigger, integrator bypassed, INTEN = 0 all the time"]
    VALUE1,
    #[doc = "Trigger event upon a falling edge"]
    VALUE2,
    #[doc = "Trigger event upon a rising edge"]
    VALUE3,
    #[doc = "No trigger, integrator active all the time, INTEN = 1 all the time"]
    VALUE4,
}
impl ITRMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ITRMODEW::VALUE1 => 0,
            ITRMODEW::VALUE2 => 1,
            ITRMODEW::VALUE3 => 2,
            ITRMODEW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ITRMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ITRMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ITRMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No integration trigger, integrator bypassed, INTEN = 0 all the time"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ITRMODEW::VALUE1)
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ITRMODEW::VALUE2)
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ITRMODEW::VALUE3)
    }
    #[doc = "No trigger, integrator active all the time, INTEN = 1 all the time"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(ITRMODEW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TSTRMODE`"]
pub enum TSTRMODEW {
    #[doc = "No timestamp trigger"]
    VALUE1,
    #[doc = "Trigger event upon a falling edge"]
    VALUE2,
    #[doc = "Trigger event upon a rising edge"]
    VALUE3,
    #[doc = "Trigger event upon each edge"]
    VALUE4,
}
impl TSTRMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSTRMODEW::VALUE1 => 0,
            TSTRMODEW::VALUE2 => 1,
            TSTRMODEW::VALUE3 => 2,
            TSTRMODEW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSTRMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTRMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSTRMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No timestamp trigger"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSTRMODEW::VALUE1)
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSTRMODEW::VALUE2)
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(TSTRMODEW::VALUE3)
    }
    #[doc = "Trigger event upon each edge"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(TSTRMODEW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRWC`"]
pub enum TRWCW {
    #[doc = "No write access to trigger parameters"]
    VALUE1,
    #[doc = "Bitfields TRSEL, TSTRMODE, ITRMODE can be written"]
    VALUE2,
}
impl TRWCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRWCW::VALUE1 => false,
            TRWCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRWCW<'a> {
    w: &'a mut W,
}
impl<'a> _TRWCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRWCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No write access to trigger parameters"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRWCW::VALUE1)
    }
    #[doc = "Bitfields TRSEL, TSTRMODE, ITRMODE can be written"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRWCW::VALUE2)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSRC`"]
pub enum CSRCW {
    #[doc = "External, from input A"]
    VALUE2,
    #[doc = "External, from input B"]
    VALUE3,
    #[doc = "External, from input C"]
    VALUE4,
    #[doc = "External, from input D"]
    VALUE5,
    #[doc = "Internal clock"]
    VALUE6,
}
impl CSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSRCW::VALUE2 => 1,
            CSRCW::VALUE3 => 2,
            CSRCW::VALUE4 => 3,
            CSRCW::VALUE5 => 4,
            CSRCW::VALUE6 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "External, from input A"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CSRCW::VALUE2)
    }
    #[doc = "External, from input B"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CSRCW::VALUE3)
    }
    #[doc = "External, from input C"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CSRCW::VALUE4)
    }
    #[doc = "External, from input D"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(CSRCW::VALUE5)
    }
    #[doc = "Internal clock"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(CSRCW::VALUE6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STROBE`"]
pub enum STROBEW {
    #[doc = "No data strobe"]
    VALUE1,
    #[doc = "Direct clock, a sample trigger is generated at each rising clock edge"]
    VALUE2,
    #[doc = "Direct clock, a sample trigger is generated at each falling clock edge"]
    VALUE3,
    #[doc = "Double data, a sample trigger is generated at each rising and falling clock edge"]
    VALUE4,
    #[doc = "Double clock, a sample trigger is generated at every 2nd rising clock edge"]
    VALUE6,
    #[doc = "Double clock, a sample trigger is generated at every 2nd falling clock edge"]
    VALUE7,
}
impl STROBEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STROBEW::VALUE1 => 0,
            STROBEW::VALUE2 => 1,
            STROBEW::VALUE3 => 2,
            STROBEW::VALUE4 => 3,
            STROBEW::VALUE6 => 5,
            STROBEW::VALUE7 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STROBEW<'a> {
    w: &'a mut W,
}
impl<'a> _STROBEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STROBEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No data strobe"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STROBEW::VALUE1)
    }
    #[doc = "Direct clock, a sample trigger is generated at each rising clock edge"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STROBEW::VALUE2)
    }
    #[doc = "Direct clock, a sample trigger is generated at each falling clock edge"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(STROBEW::VALUE3)
    }
    #[doc = "Double data, a sample trigger is generated at each rising and falling clock edge"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(STROBEW::VALUE4)
    }
    #[doc = "Double clock, a sample trigger is generated at every 2nd rising clock edge"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(STROBEW::VALUE6)
    }
    #[doc = "Double clock, a sample trigger is generated at every 2nd falling clock edge"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(STROBEW::VALUE7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCWC`"]
pub enum SCWCW {
    #[doc = "No write access to strobe/clock parameters"]
    VALUE1,
    #[doc = "Bitfields STROBE, CSRC can be written"]
    VALUE2,
}
impl SCWCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCWCW::VALUE1 => false,
            SCWCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCWCW<'a> {
    w: &'a mut W,
}
impl<'a> _SCWCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCWCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No write access to strobe/clock parameters"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCWCW::VALUE1)
    }
    #[doc = "Bitfields STROBE, CSRC can be written"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCWCW::VALUE2)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:3 - Input Data Source Select"]
    #[inline]
    pub fn dsrc(&self) -> DSRCR {
        DSRCR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Integrator Trigger Mode"]
    #[inline]
    pub fn itrmode(&self) -> ITRMODER {
        ITRMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Timestamp Trigger Mode"]
    #[inline]
    pub fn tstrmode(&self) -> TSTRMODER {
        TSTRMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - Trigger Select"]
    #[inline]
    pub fn trsel(&self) -> TRSELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRSELR { bits }
    }
    #[doc = "Bits 16:19 - Sample Clock Source Select"]
    #[inline]
    pub fn csrc(&self) -> CSRCR {
        CSRCR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Data Strobe Generatoion Mode"]
    #[inline]
    pub fn strobe(&self) -> STROBER {
        STROBER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:3 - Input Data Source Select"]
    #[inline]
    pub fn dsrc(&mut self) -> _DSRCW {
        _DSRCW { w: self }
    }
    #[doc = "Bit 7 - Write Control for Data Selection"]
    #[inline]
    pub fn dswc(&mut self) -> _DSWCW {
        _DSWCW { w: self }
    }
    #[doc = "Bits 8:9 - Integrator Trigger Mode"]
    #[inline]
    pub fn itrmode(&mut self) -> _ITRMODEW {
        _ITRMODEW { w: self }
    }
    #[doc = "Bits 10:11 - Timestamp Trigger Mode"]
    #[inline]
    pub fn tstrmode(&mut self) -> _TSTRMODEW {
        _TSTRMODEW { w: self }
    }
    #[doc = "Bits 12:14 - Trigger Select"]
    #[inline]
    pub fn trsel(&mut self) -> _TRSELW {
        _TRSELW { w: self }
    }
    #[doc = "Bit 15 - Write Control for Trigger Parameters"]
    #[inline]
    pub fn trwc(&mut self) -> _TRWCW {
        _TRWCW { w: self }
    }
    #[doc = "Bits 16:19 - Sample Clock Source Select"]
    #[inline]
    pub fn csrc(&mut self) -> _CSRCW {
        _CSRCW { w: self }
    }
    #[doc = "Bits 20:23 - Data Strobe Generatoion Mode"]
    #[inline]
    pub fn strobe(&mut self) -> _STROBEW {
        _STROBEW { w: self }
    }
    #[doc = "Bit 31 - Write Control for Strobe/Clock Selection"]
    #[inline]
    pub fn scwc(&mut self) -> _SCWCW {
        _SCWCW { w: self }
    }
}
