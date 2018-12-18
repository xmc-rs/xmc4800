#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BUSRCON0 {
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
#[doc = "Possible values of the field `FETBLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FETBLENR {
    #[doc = "1 data access (default after reset)."]
    VALUE1,
    #[doc = "2 data accesses."]
    VALUE2,
    #[doc = "4 data accesses."]
    VALUE3,
    #[doc = "8 data accesses."]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FETBLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FETBLENR::VALUE1 => 0,
            FETBLENR::VALUE2 => 1,
            FETBLENR::VALUE3 => 2,
            FETBLENR::VALUE4 => 3,
            FETBLENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FETBLENR {
        match value {
            0 => FETBLENR::VALUE1,
            1 => FETBLENR::VALUE2,
            2 => FETBLENR::VALUE3,
            3 => FETBLENR::VALUE4,
            i => FETBLENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FETBLENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FETBLENR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == FETBLENR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == FETBLENR::VALUE4
    }
}
#[doc = "Possible values of the field `FBBMSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FBBMSELR {
    #[doc = "Burst buffer length defined by value in FETBLEN (default after reset)."]
    VALUE1,
    #[doc = "Continuous mode. All data required for transaction is transferred in a single burst."]
    VALUE2,
}
impl FBBMSELR {
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
            FBBMSELR::VALUE1 => false,
            FBBMSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FBBMSELR {
        match value {
            false => FBBMSELR::VALUE1,
            true => FBBMSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FBBMSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FBBMSELR::VALUE2
    }
}
#[doc = "Possible values of the field `BFSSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFSSSR {
    #[doc = "Two stages of synchronization used. (maximum margin)"]
    VALUE1,
    #[doc = "One stage of synchronization used. (minimum latency)"]
    VALUE2,
}
impl BFSSSR {
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
            BFSSSR::VALUE1 => false,
            BFSSSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFSSSR {
        match value {
            false => BFSSSR::VALUE1,
            true => BFSSSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFSSSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFSSSR::VALUE2
    }
}
#[doc = "Possible values of the field `FDBKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDBKENR {
    #[doc = "BFCLK feedback not used."]
    VALUE1,
    #[doc = "Incoming data and control signals (from the Burst FLASH device) are re-synchronized to the BFCLKI input."]
    VALUE2,
}
impl FDBKENR {
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
            FDBKENR::VALUE1 => false,
            FDBKENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FDBKENR {
        match value {
            false => FDBKENR::VALUE1,
            true => FDBKENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FDBKENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FDBKENR::VALUE2
    }
}
#[doc = "Possible values of the field `BFCMSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFCMSELR {
    #[doc = "Burst Flash Clock runs continuously with values selected by this register"]
    VALUE1,
    #[doc = "Burst Flash Clock is disabled between accesses"]
    VALUE2,
}
impl BFCMSELR {
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
            BFCMSELR::VALUE1 => false,
            BFCMSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFCMSELR {
        match value {
            false => BFCMSELR::VALUE1,
            true => BFCMSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFCMSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFCMSELR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct NAAR {
    bits: bool,
}
impl NAAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `ECSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECSER {
    #[doc = "CS is delayed."]
    VALUE1,
    #[doc = "CS is not delayed."]
    VALUE2,
}
impl ECSER {
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
            ECSER::VALUE1 => false,
            ECSER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECSER {
        match value {
            false => ECSER::VALUE1,
            true => ECSER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ECSER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ECSER::VALUE2
    }
}
#[doc = "Possible values of the field `EBSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBSER {
    #[doc = "ADV is delayed."]
    VALUE1,
    #[doc = "ADV is not delayed."]
    VALUE2,
}
impl EBSER {
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
            EBSER::VALUE1 => false,
            EBSER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EBSER {
        match value {
            false => EBSER::VALUE1,
            true => EBSER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EBSER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EBSER::VALUE2
    }
}
#[doc = "Possible values of the field `DBA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBAR {
    #[doc = "Memory Controller automatically re-aligns any non-aligned synchronous burst access so that data can be fetched from the device in a single burst transaction."]
    VALUE1,
    #[doc = "Memory Controller always starts any burst access to a synchronous burst device at the address specified by the AHB request. Any required address wrapping must be automatically provided by the Burst FLASH device."]
    VALUE2,
}
impl DBAR {
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
            DBAR::VALUE1 => false,
            DBAR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBAR {
        match value {
            false => DBAR::VALUE1,
            true => DBAR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DBAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DBAR::VALUE2
    }
}
#[doc = "Possible values of the field `WAITINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITINVR {
    #[doc = "input at WAIT pin is active low (default after reset)."]
    VALUE1,
    #[doc = "input at WAIT pin is active high."]
    VALUE2,
}
impl WAITINVR {
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
            WAITINVR::VALUE1 => false,
            WAITINVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAITINVR {
        match value {
            false => WAITINVR::VALUE1,
            true => WAITINVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WAITINVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WAITINVR::VALUE2
    }
}
#[doc = "Possible values of the field `BCGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCGENR {
    #[doc = "Byte control signals follow chip select timing."]
    VALUE1,
    #[doc = "Byte control signals follow control signal timing (RD, RD/WR) (default after reset)."]
    VALUE2,
    #[doc = "Byte control signals follow write enable signal timing (RD/WR only)."]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BCGENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BCGENR::VALUE1 => 0,
            BCGENR::VALUE2 => 1,
            BCGENR::VALUE3 => 2,
            BCGENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BCGENR {
        match value {
            0 => BCGENR::VALUE1,
            1 => BCGENR::VALUE2,
            2 => BCGENR::VALUE3,
            i => BCGENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BCGENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BCGENR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == BCGENR::VALUE3
    }
}
#[doc = r" Value of the field"]
pub struct PORTWR {
    bits: u8,
}
impl PORTWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WAITR {
    bits: u8,
}
impl WAITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `AAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AAPR {
    #[doc = "Clock is enabled at beginning of access."]
    VALUE1,
    #[doc = "Clock is enabled at after address phase."]
    VALUE2,
}
impl AAPR {
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
            AAPR::VALUE1 => false,
            AAPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AAPR {
        match value {
            false => AAPR::VALUE1,
            true => AAPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AAPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AAPR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct AGENR {
    bits: u8,
}
impl AGENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `FETBLEN`"]
pub enum FETBLENW {
    #[doc = "1 data access (default after reset)."]
    VALUE1,
    #[doc = "2 data accesses."]
    VALUE2,
    #[doc = "4 data accesses."]
    VALUE3,
    #[doc = "8 data accesses."]
    VALUE4,
}
impl FETBLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FETBLENW::VALUE1 => 0,
            FETBLENW::VALUE2 => 1,
            FETBLENW::VALUE3 => 2,
            FETBLENW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FETBLENW<'a> {
    w: &'a mut W,
}
impl<'a> _FETBLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FETBLENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 data access (default after reset)."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FETBLENW::VALUE1)
    }
    #[doc = "2 data accesses."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FETBLENW::VALUE2)
    }
    #[doc = "4 data accesses."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(FETBLENW::VALUE3)
    }
    #[doc = "8 data accesses."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(FETBLENW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FBBMSEL`"]
pub enum FBBMSELW {
    #[doc = "Burst buffer length defined by value in FETBLEN (default after reset)."]
    VALUE1,
    #[doc = "Continuous mode. All data required for transaction is transferred in a single burst."]
    VALUE2,
}
impl FBBMSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FBBMSELW::VALUE1 => false,
            FBBMSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FBBMSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FBBMSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FBBMSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Burst buffer length defined by value in FETBLEN (default after reset)."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FBBMSELW::VALUE1)
    }
    #[doc = "Continuous mode. All data required for transaction is transferred in a single burst."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FBBMSELW::VALUE2)
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
#[doc = "Values that can be written to the field `BFSSS`"]
pub enum BFSSSW {
    #[doc = "Two stages of synchronization used. (maximum margin)"]
    VALUE1,
    #[doc = "One stage of synchronization used. (minimum latency)"]
    VALUE2,
}
impl BFSSSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFSSSW::VALUE1 => false,
            BFSSSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFSSSW<'a> {
    w: &'a mut W,
}
impl<'a> _BFSSSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFSSSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Two stages of synchronization used. (maximum margin)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFSSSW::VALUE1)
    }
    #[doc = "One stage of synchronization used. (minimum latency)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFSSSW::VALUE2)
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
#[doc = "Values that can be written to the field `FDBKEN`"]
pub enum FDBKENW {
    #[doc = "BFCLK feedback not used."]
    VALUE1,
    #[doc = "Incoming data and control signals (from the Burst FLASH device) are re-synchronized to the BFCLKI input."]
    VALUE2,
}
impl FDBKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FDBKENW::VALUE1 => false,
            FDBKENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FDBKENW<'a> {
    w: &'a mut W,
}
impl<'a> _FDBKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FDBKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "BFCLK feedback not used."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FDBKENW::VALUE1)
    }
    #[doc = "Incoming data and control signals (from the Burst FLASH device) are re-synchronized to the BFCLKI input."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FDBKENW::VALUE2)
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
#[doc = "Values that can be written to the field `BFCMSEL`"]
pub enum BFCMSELW {
    #[doc = "Burst Flash Clock runs continuously with values selected by this register"]
    VALUE1,
    #[doc = "Burst Flash Clock is disabled between accesses"]
    VALUE2,
}
impl BFCMSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFCMSELW::VALUE1 => false,
            BFCMSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFCMSELW<'a> {
    w: &'a mut W,
}
impl<'a> _BFCMSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFCMSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Burst Flash Clock runs continuously with values selected by this register"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFCMSELW::VALUE1)
    }
    #[doc = "Burst Flash Clock is disabled between accesses"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFCMSELW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _NAAW<'a> {
    w: &'a mut W,
}
impl<'a> _NAAW<'a> {
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
#[doc = "Values that can be written to the field `ECSE`"]
pub enum ECSEW {
    #[doc = "CS is delayed."]
    VALUE1,
    #[doc = "CS is not delayed."]
    VALUE2,
}
impl ECSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECSEW::VALUE1 => false,
            ECSEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECSEW<'a> {
    w: &'a mut W,
}
impl<'a> _ECSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CS is delayed."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECSEW::VALUE1)
    }
    #[doc = "CS is not delayed."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECSEW::VALUE2)
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
#[doc = "Values that can be written to the field `EBSE`"]
pub enum EBSEW {
    #[doc = "ADV is delayed."]
    VALUE1,
    #[doc = "ADV is not delayed."]
    VALUE2,
}
impl EBSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EBSEW::VALUE1 => false,
            EBSEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EBSEW<'a> {
    w: &'a mut W,
}
impl<'a> _EBSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EBSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADV is delayed."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EBSEW::VALUE1)
    }
    #[doc = "ADV is not delayed."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EBSEW::VALUE2)
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
#[doc = "Values that can be written to the field `DBA`"]
pub enum DBAW {
    #[doc = "Memory Controller automatically re-aligns any non-aligned synchronous burst access so that data can be fetched from the device in a single burst transaction."]
    VALUE1,
    #[doc = "Memory Controller always starts any burst access to a synchronous burst device at the address specified by the AHB request. Any required address wrapping must be automatically provided by the Burst FLASH device."]
    VALUE2,
}
impl DBAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBAW::VALUE1 => false,
            DBAW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBAW<'a> {
    w: &'a mut W,
}
impl<'a> _DBAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Memory Controller automatically re-aligns any non-aligned synchronous burst access so that data can be fetched from the device in a single burst transaction."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DBAW::VALUE1)
    }
    #[doc = "Memory Controller always starts any burst access to a synchronous burst device at the address specified by the AHB request. Any required address wrapping must be automatically provided by the Burst FLASH device."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DBAW::VALUE2)
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
#[doc = "Values that can be written to the field `WAITINV`"]
pub enum WAITINVW {
    #[doc = "input at WAIT pin is active low (default after reset)."]
    VALUE1,
    #[doc = "input at WAIT pin is active high."]
    VALUE2,
}
impl WAITINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAITINVW::VALUE1 => false,
            WAITINVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAITINVW<'a> {
    w: &'a mut W,
}
impl<'a> _WAITINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAITINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "input at WAIT pin is active low (default after reset)."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAITINVW::VALUE1)
    }
    #[doc = "input at WAIT pin is active high."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAITINVW::VALUE2)
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
#[doc = "Values that can be written to the field `BCGEN`"]
pub enum BCGENW {
    #[doc = "Byte control signals follow chip select timing."]
    VALUE1,
    #[doc = "Byte control signals follow control signal timing (RD, RD/WR) (default after reset)."]
    VALUE2,
    #[doc = "Byte control signals follow write enable signal timing (RD/WR only)."]
    VALUE3,
}
impl BCGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BCGENW::VALUE1 => 0,
            BCGENW::VALUE2 => 1,
            BCGENW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCGENW<'a> {
    w: &'a mut W,
}
impl<'a> _BCGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCGENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Byte control signals follow chip select timing."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BCGENW::VALUE1)
    }
    #[doc = "Byte control signals follow control signal timing (RD, RD/WR) (default after reset)."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BCGENW::VALUE2)
    }
    #[doc = "Byte control signals follow write enable signal timing (RD/WR only)."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(BCGENW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PORTWW<'a> {
    w: &'a mut W,
}
impl<'a> _PORTWW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WAITW<'a> {
    w: &'a mut W,
}
impl<'a> _WAITW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AAP`"]
pub enum AAPW {
    #[doc = "Clock is enabled at beginning of access."]
    VALUE1,
    #[doc = "Clock is enabled at after address phase."]
    VALUE2,
}
impl AAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AAPW::VALUE1 => false,
            AAPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AAPW<'a> {
    w: &'a mut W,
}
impl<'a> _AAPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AAPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock is enabled at beginning of access."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AAPW::VALUE1)
    }
    #[doc = "Clock is enabled at after address phase."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AAPW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _AGENW<'a> {
    w: &'a mut W,
}
impl<'a> _AGENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:2 - Burst Length for Synchronous Burst"]
    #[inline]
    pub fn fetblen(&self) -> FETBLENR {
        FETBLENR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Synchronous burst buffer mode select"]
    #[inline]
    pub fn fbbmsel(&self) -> FBBMSELR {
        FBBMSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Read Single Stage Synchronization:"]
    #[inline]
    pub fn bfsss(&self) -> BFSSSR {
        BFSSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Burst FLASH Clock Feedback Enable"]
    #[inline]
    pub fn fdbken(&self) -> FDBKENR {
        FDBKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Burst Flash Clock Mode Select"]
    #[inline]
    pub fn bfcmsel(&self) -> BFCMSELR {
        BFCMSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable flash non-array access workaround"]
    #[inline]
    pub fn naa(&self) -> NAAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NAAR { bits }
    }
    #[doc = "Bit 16 - Early Chip Select for Synchronous Burst"]
    #[inline]
    pub fn ecse(&self) -> ECSER {
        ECSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Early Burst Signal Enable for Synchronous Burst"]
    #[inline]
    pub fn ebse(&self) -> EBSER {
        EBSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Disable Burst Address Wrapping"]
    #[inline]
    pub fn dba(&self) -> DBAR {
        DBAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Reversed polarity at WAIT"]
    #[inline]
    pub fn waitinv(&self) -> WAITINVR {
        WAITINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 20:21 - Byte Control Signal Control"]
    #[inline]
    pub fn bcgen(&self) -> BCGENR {
        BCGENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Device Addressing Mode"]
    #[inline]
    pub fn portw(&self) -> PORTWR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PORTWR { bits }
    }
    #[doc = "Bits 24:25 - External Wait Control: 0=OFF (default after reset)., 1=Asynchronous input at WAIT., 2=Synchronous input at WAIT., 3=reserved., 0=OFF (default after reset)., 1=Wait for page load (Early WAIT)., 2=Wait for page load (WAIT with data)., 3=Abort and retry access.,"]
    #[inline]
    pub fn wait(&self) -> WAITR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WAITR { bits }
    }
    #[doc = "Bit 26 - Asynchronous Address phase:"]
    #[inline]
    pub fn aap(&self) -> AAPR {
        AAPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 28:31 - Device Type for Region"]
    #[inline]
    pub fn agen(&self) -> AGENR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AGENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 13828160 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Burst Length for Synchronous Burst"]
    #[inline]
    pub fn fetblen(&mut self) -> _FETBLENW {
        _FETBLENW { w: self }
    }
    #[doc = "Bit 3 - Synchronous burst buffer mode select"]
    #[inline]
    pub fn fbbmsel(&mut self) -> _FBBMSELW {
        _FBBMSELW { w: self }
    }
    #[doc = "Bit 4 - Read Single Stage Synchronization:"]
    #[inline]
    pub fn bfsss(&mut self) -> _BFSSSW {
        _BFSSSW { w: self }
    }
    #[doc = "Bit 5 - Burst FLASH Clock Feedback Enable"]
    #[inline]
    pub fn fdbken(&mut self) -> _FDBKENW {
        _FDBKENW { w: self }
    }
    #[doc = "Bit 6 - Burst Flash Clock Mode Select"]
    #[inline]
    pub fn bfcmsel(&mut self) -> _BFCMSELW {
        _BFCMSELW { w: self }
    }
    #[doc = "Bit 7 - Enable flash non-array access workaround"]
    #[inline]
    pub fn naa(&mut self) -> _NAAW {
        _NAAW { w: self }
    }
    #[doc = "Bit 16 - Early Chip Select for Synchronous Burst"]
    #[inline]
    pub fn ecse(&mut self) -> _ECSEW {
        _ECSEW { w: self }
    }
    #[doc = "Bit 17 - Early Burst Signal Enable for Synchronous Burst"]
    #[inline]
    pub fn ebse(&mut self) -> _EBSEW {
        _EBSEW { w: self }
    }
    #[doc = "Bit 18 - Disable Burst Address Wrapping"]
    #[inline]
    pub fn dba(&mut self) -> _DBAW {
        _DBAW { w: self }
    }
    #[doc = "Bit 19 - Reversed polarity at WAIT"]
    #[inline]
    pub fn waitinv(&mut self) -> _WAITINVW {
        _WAITINVW { w: self }
    }
    #[doc = "Bits 20:21 - Byte Control Signal Control"]
    #[inline]
    pub fn bcgen(&mut self) -> _BCGENW {
        _BCGENW { w: self }
    }
    #[doc = "Bits 22:23 - Device Addressing Mode"]
    #[inline]
    pub fn portw(&mut self) -> _PORTWW {
        _PORTWW { w: self }
    }
    #[doc = "Bits 24:25 - External Wait Control: 0=OFF (default after reset)., 1=Asynchronous input at WAIT., 2=Synchronous input at WAIT., 3=reserved., 0=OFF (default after reset)., 1=Wait for page load (Early WAIT)., 2=Wait for page load (WAIT with data)., 3=Abort and retry access.,"]
    #[inline]
    pub fn wait(&mut self) -> _WAITW {
        _WAITW { w: self }
    }
    #[doc = "Bit 26 - Asynchronous Address phase:"]
    #[inline]
    pub fn aap(&mut self) -> _AAPW {
        _AAPW { w: self }
    }
    #[doc = "Bits 28:31 - Device Type for Region"]
    #[inline]
    pub fn agen(&mut self) -> _AGENW {
        _AGENW { w: self }
    }
}
