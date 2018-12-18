#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDRMCON {
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
#[doc = "Possible values of the field `SDCMSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDCMSELR {
    #[doc = "clock disabled between accesses"]
    VALUE1,
    #[doc = "clock continuously runs"]
    VALUE2,
}
impl SDCMSELR {
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
            SDCMSELR::VALUE1 => true,
            SDCMSELR::VALUE2 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDCMSELR {
        match value {
            true => SDCMSELR::VALUE1,
            false => SDCMSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SDCMSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SDCMSELR::VALUE2
    }
}
#[doc = "Possible values of the field `PWR_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_MODER {
    #[doc = "precharge before clock stop (default after reset)"]
    VALUE1,
    #[doc = "auto-precharge before clock stop"]
    VALUE2,
    #[doc = "active power down (stop clock without precharge)"]
    VALUE3,
    #[doc = "clock stop power down"]
    VALUE4,
}
impl PWR_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWR_MODER::VALUE1 => 0,
            PWR_MODER::VALUE2 => 1,
            PWR_MODER::VALUE3 => 2,
            PWR_MODER::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWR_MODER {
        match value {
            0 => PWR_MODER::VALUE1,
            1 => PWR_MODER::VALUE2,
            2 => PWR_MODER::VALUE3,
            3 => PWR_MODER::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PWR_MODER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PWR_MODER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PWR_MODER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PWR_MODER::VALUE4
    }
}
#[doc = "Possible values of the field `CLKDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKDISR {
    #[doc = "clock enabled"]
    VALUE1,
    #[doc = "clock disabled"]
    VALUE2,
}
impl CLKDISR {
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
            CLKDISR::VALUE1 => false,
            CLKDISR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKDISR {
        match value {
            false => CLKDISR::VALUE1,
            true => CLKDISR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CLKDISR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CLKDISR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct CRCER {
    bits: u8,
}
impl CRCER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BANKM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANKMR {
    #[doc = "Address bit 21 to 20"]
    VALUE2,
    #[doc = "Address bit 22 to 21"]
    VALUE3,
    #[doc = "Address bit 23 to 22"]
    VALUE4,
    #[doc = "Address bit 24 to 23"]
    VALUE5,
    #[doc = "Address bit 25 to 24"]
    VALUE6,
    #[doc = "Address bit 26 to 25"]
    VALUE7,
    #[doc = "Address bit 26"]
    VALUE8,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BANKMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BANKMR::VALUE2 => 1,
            BANKMR::VALUE3 => 2,
            BANKMR::VALUE4 => 3,
            BANKMR::VALUE5 => 4,
            BANKMR::VALUE6 => 5,
            BANKMR::VALUE7 => 6,
            BANKMR::VALUE8 => 7,
            BANKMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BANKMR {
        match value {
            1 => BANKMR::VALUE2,
            2 => BANKMR::VALUE3,
            3 => BANKMR::VALUE4,
            4 => BANKMR::VALUE5,
            5 => BANKMR::VALUE6,
            6 => BANKMR::VALUE7,
            7 => BANKMR::VALUE8,
            i => BANKMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BANKMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == BANKMR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == BANKMR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == BANKMR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == BANKMR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == BANKMR::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == BANKMR::VALUE8
    }
}
#[doc = "Possible values of the field `ROWM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROWMR {
    #[doc = "Address bit 26 to 9"]
    VALUE2,
    #[doc = "Address bit 26 to 10"]
    VALUE3,
    #[doc = "Address bit 26 to 11"]
    VALUE4,
    #[doc = "Address bit 26 to 12"]
    VALUE5,
    #[doc = "Address bit 26 to 13"]
    VALUE6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ROWMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ROWMR::VALUE2 => 1,
            ROWMR::VALUE3 => 2,
            ROWMR::VALUE4 => 3,
            ROWMR::VALUE5 => 4,
            ROWMR::VALUE6 => 5,
            ROWMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ROWMR {
        match value {
            1 => ROWMR::VALUE2,
            2 => ROWMR::VALUE3,
            3 => ROWMR::VALUE4,
            4 => ROWMR::VALUE5,
            5 => ROWMR::VALUE6,
            i => ROWMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ROWMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ROWMR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == ROWMR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == ROWMR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == ROWMR::VALUE6
    }
}
#[doc = r" Value of the field"]
pub struct CRCR {
    bits: u8,
}
impl CRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CRCDR {
    bits: u8,
}
impl CRCDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `AWIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWIDTHR {
    #[doc = "do not use"]
    VALUE1,
    #[doc = "Address(8:0)"]
    VALUE2,
    #[doc = "Address(9:0)"]
    VALUE3,
    #[doc = "Address(10:0)"]
    VALUE4,
}
impl AWIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AWIDTHR::VALUE1 => 0,
            AWIDTHR::VALUE2 => 1,
            AWIDTHR::VALUE3 => 2,
            AWIDTHR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AWIDTHR {
        match value {
            0 => AWIDTHR::VALUE1,
            1 => AWIDTHR::VALUE2,
            2 => AWIDTHR::VALUE3,
            3 => AWIDTHR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AWIDTHR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AWIDTHR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == AWIDTHR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == AWIDTHR::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct CRPR {
    bits: u8,
}
impl CRPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CRSCR {
    bits: u8,
}
impl CRSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CRFSHR {
    bits: u8,
}
impl CRFSHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CRASR {
    bits: u8,
}
impl CRASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `SDCMSEL`"]
pub enum SDCMSELW {
    #[doc = "clock disabled between accesses"]
    VALUE1,
    #[doc = "clock continuously runs"]
    VALUE2,
}
impl SDCMSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDCMSELW::VALUE1 => true,
            SDCMSELW::VALUE2 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDCMSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SDCMSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDCMSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "clock disabled between accesses"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SDCMSELW::VALUE1)
    }
    #[doc = "clock continuously runs"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SDCMSELW::VALUE2)
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
#[doc = "Values that can be written to the field `PWR_MODE`"]
pub enum PWR_MODEW {
    #[doc = "precharge before clock stop (default after reset)"]
    VALUE1,
    #[doc = "auto-precharge before clock stop"]
    VALUE2,
    #[doc = "active power down (stop clock without precharge)"]
    VALUE3,
    #[doc = "clock stop power down"]
    VALUE4,
}
impl PWR_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWR_MODEW::VALUE1 => 0,
            PWR_MODEW::VALUE2 => 1,
            PWR_MODEW::VALUE3 => 2,
            PWR_MODEW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWR_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "precharge before clock stop (default after reset)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PWR_MODEW::VALUE1)
    }
    #[doc = "auto-precharge before clock stop"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PWR_MODEW::VALUE2)
    }
    #[doc = "active power down (stop clock without precharge)"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PWR_MODEW::VALUE3)
    }
    #[doc = "clock stop power down"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PWR_MODEW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKDIS`"]
pub enum CLKDISW {
    #[doc = "clock enabled"]
    VALUE1,
    #[doc = "clock disabled"]
    VALUE2,
}
impl CLKDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKDISW::VALUE1 => false,
            CLKDISW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKDISW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "clock enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLKDISW::VALUE1)
    }
    #[doc = "clock disabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CLKDISW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _CRCEW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BANKM`"]
pub enum BANKMW {
    #[doc = "Address bit 21 to 20"]
    VALUE2,
    #[doc = "Address bit 22 to 21"]
    VALUE3,
    #[doc = "Address bit 23 to 22"]
    VALUE4,
    #[doc = "Address bit 24 to 23"]
    VALUE5,
    #[doc = "Address bit 25 to 24"]
    VALUE6,
    #[doc = "Address bit 26 to 25"]
    VALUE7,
    #[doc = "Address bit 26"]
    VALUE8,
}
impl BANKMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BANKMW::VALUE2 => 1,
            BANKMW::VALUE3 => 2,
            BANKMW::VALUE4 => 3,
            BANKMW::VALUE5 => 4,
            BANKMW::VALUE6 => 5,
            BANKMW::VALUE7 => 6,
            BANKMW::VALUE8 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BANKMW<'a> {
    w: &'a mut W,
}
impl<'a> _BANKMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BANKMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Address bit 21 to 20"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BANKMW::VALUE2)
    }
    #[doc = "Address bit 22 to 21"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(BANKMW::VALUE3)
    }
    #[doc = "Address bit 23 to 22"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(BANKMW::VALUE4)
    }
    #[doc = "Address bit 24 to 23"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(BANKMW::VALUE5)
    }
    #[doc = "Address bit 25 to 24"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(BANKMW::VALUE6)
    }
    #[doc = "Address bit 26 to 25"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(BANKMW::VALUE7)
    }
    #[doc = "Address bit 26"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(BANKMW::VALUE8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ROWM`"]
pub enum ROWMW {
    #[doc = "Address bit 26 to 9"]
    VALUE2,
    #[doc = "Address bit 26 to 10"]
    VALUE3,
    #[doc = "Address bit 26 to 11"]
    VALUE4,
    #[doc = "Address bit 26 to 12"]
    VALUE5,
    #[doc = "Address bit 26 to 13"]
    VALUE6,
}
impl ROWMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ROWMW::VALUE2 => 1,
            ROWMW::VALUE3 => 2,
            ROWMW::VALUE4 => 3,
            ROWMW::VALUE5 => 4,
            ROWMW::VALUE6 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROWMW<'a> {
    w: &'a mut W,
}
impl<'a> _ROWMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROWMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Address bit 26 to 9"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ROWMW::VALUE2)
    }
    #[doc = "Address bit 26 to 10"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ROWMW::VALUE3)
    }
    #[doc = "Address bit 26 to 11"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(ROWMW::VALUE4)
    }
    #[doc = "Address bit 26 to 12"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(ROWMW::VALUE5)
    }
    #[doc = "Address bit 26 to 13"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(ROWMW::VALUE6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CRCDW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AWIDTH`"]
pub enum AWIDTHW {
    #[doc = "do not use"]
    VALUE1,
    #[doc = "Address(8:0)"]
    VALUE2,
    #[doc = "Address(9:0)"]
    VALUE3,
    #[doc = "Address(10:0)"]
    VALUE4,
}
impl AWIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AWIDTHW::VALUE1 => 0,
            AWIDTHW::VALUE2 => 1,
            AWIDTHW::VALUE3 => 2,
            AWIDTHW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AWIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _AWIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AWIDTHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "do not use"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AWIDTHW::VALUE1)
    }
    #[doc = "Address(8:0)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AWIDTHW::VALUE2)
    }
    #[doc = "Address(9:0)"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(AWIDTHW::VALUE3)
    }
    #[doc = "Address(10:0)"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(AWIDTHW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CRPW<'a> {
    w: &'a mut W,
}
impl<'a> _CRPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CRSCW<'a> {
    w: &'a mut W,
}
impl<'a> _CRSCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CRFSHW<'a> {
    w: &'a mut W,
}
impl<'a> _CRFSHW<'a> {
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
#[doc = r" Proxy"]
pub struct _CRASW<'a> {
    w: &'a mut W,
}
impl<'a> _CRASW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 31 - SDRAM clock mode select"]
    #[inline]
    pub fn sdcmsel(&self) -> SDCMSELR {
        SDCMSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 29:30 - Power Save Mode used for gated clock mode"]
    #[inline]
    pub fn pwr_mode(&self) -> PWR_MODER {
        PWR_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Disable SDRAM clock output"]
    #[inline]
    pub fn clkdis(&self) -> CLKDISR {
        CLKDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 25:27 - Row cycle time counter extension"]
    #[inline]
    pub fn crce(&self) -> CRCER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CRCER { bits }
    }
    #[doc = "Bits 22:24 - Mask for bank tag"]
    #[inline]
    pub fn bankm(&self) -> BANKMR {
        BANKMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:21 - Mask for row tag"]
    #[inline]
    pub fn rowm(&self) -> ROWMR {
        ROWMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Row cycle time counter"]
    #[inline]
    pub fn crc(&self) -> CRCR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CRCR { bits }
    }
    #[doc = "Bits 14:15 - Row to column delay counter"]
    #[inline]
    pub fn crcd(&self) -> CRCDR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CRCDR { bits }
    }
    #[doc = "Bits 12:13 - Width of column address"]
    #[inline]
    pub fn awidth(&self) -> AWIDTHR {
        AWIDTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Row precharge time counter"]
    #[inline]
    pub fn crp(&self) -> CRPR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CRPR { bits }
    }
    #[doc = "Bits 8:9 - Mode register set-up time"]
    #[inline]
    pub fn crsc(&self) -> CRSCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CRSCR { bits }
    }
    #[doc = "Bits 4:7 - Initialization refresh commands counter"]
    #[inline]
    pub fn crfsh(&self) -> CRFSHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CRFSHR { bits }
    }
    #[doc = "Bits 0:3 - Row to precharge delay counter"]
    #[inline]
    pub fn cras(&self) -> CRASR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CRASR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2147483648 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31 - SDRAM clock mode select"]
    #[inline]
    pub fn sdcmsel(&mut self) -> _SDCMSELW {
        _SDCMSELW { w: self }
    }
    #[doc = "Bits 29:30 - Power Save Mode used for gated clock mode"]
    #[inline]
    pub fn pwr_mode(&mut self) -> _PWR_MODEW {
        _PWR_MODEW { w: self }
    }
    #[doc = "Bit 28 - Disable SDRAM clock output"]
    #[inline]
    pub fn clkdis(&mut self) -> _CLKDISW {
        _CLKDISW { w: self }
    }
    #[doc = "Bits 25:27 - Row cycle time counter extension"]
    #[inline]
    pub fn crce(&mut self) -> _CRCEW {
        _CRCEW { w: self }
    }
    #[doc = "Bits 22:24 - Mask for bank tag"]
    #[inline]
    pub fn bankm(&mut self) -> _BANKMW {
        _BANKMW { w: self }
    }
    #[doc = "Bits 19:21 - Mask for row tag"]
    #[inline]
    pub fn rowm(&mut self) -> _ROWMW {
        _ROWMW { w: self }
    }
    #[doc = "Bits 16:18 - Row cycle time counter"]
    #[inline]
    pub fn crc(&mut self) -> _CRCW {
        _CRCW { w: self }
    }
    #[doc = "Bits 14:15 - Row to column delay counter"]
    #[inline]
    pub fn crcd(&mut self) -> _CRCDW {
        _CRCDW { w: self }
    }
    #[doc = "Bits 12:13 - Width of column address"]
    #[inline]
    pub fn awidth(&mut self) -> _AWIDTHW {
        _AWIDTHW { w: self }
    }
    #[doc = "Bits 10:11 - Row precharge time counter"]
    #[inline]
    pub fn crp(&mut self) -> _CRPW {
        _CRPW { w: self }
    }
    #[doc = "Bits 8:9 - Mode register set-up time"]
    #[inline]
    pub fn crsc(&mut self) -> _CRSCW {
        _CRSCW { w: self }
    }
    #[doc = "Bits 4:7 - Initialization refresh commands counter"]
    #[inline]
    pub fn crfsh(&mut self) -> _CRFSHW {
        _CRFSHW { w: self }
    }
    #[doc = "Bits 0:3 - Row to precharge delay counter"]
    #[inline]
    pub fn cras(&mut self) -> _CRASW {
        _CRASW { w: self }
    }
}
