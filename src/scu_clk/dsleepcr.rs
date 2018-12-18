#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DSLEEPCR {
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
#[doc = "Possible values of the field `SYSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSSELR {
    #[doc = "fOFI clock"]
    VALUE1,
    #[doc = "fPLL clock"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SYSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSSELR::VALUE1 => 0,
            SYSSELR::VALUE2 => 1,
            SYSSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYSSELR {
        match value {
            0 => SYSSELR::VALUE1,
            1 => SYSSELR::VALUE2,
            i => SYSSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SYSSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SYSSELR::VALUE2
    }
}
#[doc = "Possible values of the field `FPDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPDNR {
    #[doc = "Flash power down module"]
    VALUE1,
    #[doc = "No effect"]
    VALUE2,
}
impl FPDNR {
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
            FPDNR::VALUE1 => true,
            FPDNR::VALUE2 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FPDNR {
        match value {
            true => FPDNR::VALUE1,
            false => FPDNR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FPDNR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FPDNR::VALUE2
    }
}
#[doc = "Possible values of the field `PLLPDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLPDNR {
    #[doc = "Switch off main PLL"]
    VALUE1,
    #[doc = "No effect"]
    VALUE2,
}
impl PLLPDNR {
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
            PLLPDNR::VALUE1 => true,
            PLLPDNR::VALUE2 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLPDNR {
        match value {
            true => PLLPDNR::VALUE1,
            false => PLLPDNR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PLLPDNR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PLLPDNR::VALUE2
    }
}
#[doc = "Possible values of the field `VCOPDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOPDNR {
    #[doc = "Switch off VCO of main PLL"]
    VALUE1,
    #[doc = "No effect"]
    VALUE2,
}
impl VCOPDNR {
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
            VCOPDNR::VALUE1 => true,
            VCOPDNR::VALUE2 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VCOPDNR {
        match value {
            true => VCOPDNR::VALUE1,
            false => VCOPDNR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VCOPDNR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VCOPDNR::VALUE2
    }
}
#[doc = "Possible values of the field `USBCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBCRR {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl USBCRR {
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
            USBCRR::VALUE1 => false,
            USBCRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBCRR {
        match value {
            false => USBCRR::VALUE1,
            true => USBCRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == USBCRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == USBCRR::VALUE2
    }
}
#[doc = "Possible values of the field `MMCCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMCCRR {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl MMCCRR {
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
            MMCCRR::VALUE1 => false,
            MMCCRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MMCCRR {
        match value {
            false => MMCCRR::VALUE1,
            true => MMCCRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MMCCRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MMCCRR::VALUE2
    }
}
#[doc = "Possible values of the field `ETH0CR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETH0CRR {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl ETH0CRR {
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
            ETH0CRR::VALUE1 => false,
            ETH0CRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ETH0CRR {
        match value {
            false => ETH0CRR::VALUE1,
            true => ETH0CRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ETH0CRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ETH0CRR::VALUE2
    }
}
#[doc = "Possible values of the field `EBUCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBUCRR {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl EBUCRR {
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
            EBUCRR::VALUE1 => false,
            EBUCRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EBUCRR {
        match value {
            false => EBUCRR::VALUE1,
            true => EBUCRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EBUCRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EBUCRR::VALUE2
    }
}
#[doc = "Possible values of the field `CCUCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCUCRR {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl CCUCRR {
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
            CCUCRR::VALUE1 => false,
            CCUCRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCUCRR {
        match value {
            false => CCUCRR::VALUE1,
            true => CCUCRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CCUCRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CCUCRR::VALUE2
    }
}
#[doc = "Possible values of the field `WDTCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTCRR {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl WDTCRR {
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
            WDTCRR::VALUE1 => false,
            WDTCRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDTCRR {
        match value {
            false => WDTCRR::VALUE1,
            true => WDTCRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WDTCRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WDTCRR::VALUE2
    }
}
#[doc = "Values that can be written to the field `SYSSEL`"]
pub enum SYSSELW {
    #[doc = "fOFI clock"]
    VALUE1,
    #[doc = "fPLL clock"]
    VALUE2,
}
impl SYSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSSELW::VALUE1 => 0,
            SYSSELW::VALUE2 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "fOFI clock"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SYSSELW::VALUE1)
    }
    #[doc = "fPLL clock"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SYSSELW::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FPDN`"]
pub enum FPDNW {
    #[doc = "Flash power down module"]
    VALUE1,
    #[doc = "No effect"]
    VALUE2,
}
impl FPDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FPDNW::VALUE1 => true,
            FPDNW::VALUE2 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FPDNW<'a> {
    w: &'a mut W,
}
impl<'a> _FPDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FPDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flash power down module"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FPDNW::VALUE1)
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FPDNW::VALUE2)
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
#[doc = "Values that can be written to the field `PLLPDN`"]
pub enum PLLPDNW {
    #[doc = "Switch off main PLL"]
    VALUE1,
    #[doc = "No effect"]
    VALUE2,
}
impl PLLPDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLPDNW::VALUE1 => true,
            PLLPDNW::VALUE2 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLPDNW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLPDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLPDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Switch off main PLL"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PLLPDNW::VALUE1)
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PLLPDNW::VALUE2)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VCOPDN`"]
pub enum VCOPDNW {
    #[doc = "Switch off VCO of main PLL"]
    VALUE1,
    #[doc = "No effect"]
    VALUE2,
}
impl VCOPDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VCOPDNW::VALUE1 => true,
            VCOPDNW::VALUE2 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VCOPDNW<'a> {
    w: &'a mut W,
}
impl<'a> _VCOPDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VCOPDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Switch off VCO of main PLL"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VCOPDNW::VALUE1)
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VCOPDNW::VALUE2)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USBCR`"]
pub enum USBCRW {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl USBCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBCRW::VALUE1 => false,
            USBCRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBCRW<'a> {
    w: &'a mut W,
}
impl<'a> _USBCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(USBCRW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(USBCRW::VALUE2)
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
#[doc = "Values that can be written to the field `MMCCR`"]
pub enum MMCCRW {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl MMCCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MMCCRW::VALUE1 => false,
            MMCCRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMCCRW<'a> {
    w: &'a mut W,
}
impl<'a> _MMCCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MMCCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MMCCRW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MMCCRW::VALUE2)
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
#[doc = "Values that can be written to the field `ETH0CR`"]
pub enum ETH0CRW {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl ETH0CRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ETH0CRW::VALUE1 => false,
            ETH0CRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETH0CRW<'a> {
    w: &'a mut W,
}
impl<'a> _ETH0CRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETH0CRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ETH0CRW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ETH0CRW::VALUE2)
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
#[doc = "Values that can be written to the field `EBUCR`"]
pub enum EBUCRW {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl EBUCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EBUCRW::VALUE1 => false,
            EBUCRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EBUCRW<'a> {
    w: &'a mut W,
}
impl<'a> _EBUCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EBUCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EBUCRW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EBUCRW::VALUE2)
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
#[doc = "Values that can be written to the field `CCUCR`"]
pub enum CCUCRW {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl CCUCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCUCRW::VALUE1 => false,
            CCUCRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCUCRW<'a> {
    w: &'a mut W,
}
impl<'a> _CCUCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCUCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCUCRW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCUCRW::VALUE2)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WDTCR`"]
pub enum WDTCRW {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl WDTCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDTCRW::VALUE1 => false,
            WDTCRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDTCRW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WDTCRW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WDTCRW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - System Clock Selection Value"]
    #[inline]
    pub fn syssel(&self) -> SYSSELR {
        SYSSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Flash Power Down"]
    #[inline]
    pub fn fpdn(&self) -> FPDNR {
        FPDNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - PLL Power Down"]
    #[inline]
    pub fn pllpdn(&self) -> PLLPDNR {
        PLLPDNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - VCO Power Down"]
    #[inline]
    pub fn vcopdn(&self) -> VCOPDNR {
        VCOPDNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - USB Clock Control"]
    #[inline]
    pub fn usbcr(&self) -> USBCRR {
        USBCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - MMC Clock Control"]
    #[inline]
    pub fn mmccr(&self) -> MMCCRR {
        MMCCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Ethernet Clock Control"]
    #[inline]
    pub fn eth0cr(&self) -> ETH0CRR {
        ETH0CRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - EBU Clock Control"]
    #[inline]
    pub fn ebucr(&self) -> EBUCRR {
        EBUCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - CCU Clock Control"]
    #[inline]
    pub fn ccucr(&self) -> CCUCRR {
        CCUCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - WDT Clock Control"]
    #[inline]
    pub fn wdtcr(&self) -> WDTCRR {
        WDTCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:1 - System Clock Selection Value"]
    #[inline]
    pub fn syssel(&mut self) -> _SYSSELW {
        _SYSSELW { w: self }
    }
    #[doc = "Bit 11 - Flash Power Down"]
    #[inline]
    pub fn fpdn(&mut self) -> _FPDNW {
        _FPDNW { w: self }
    }
    #[doc = "Bit 12 - PLL Power Down"]
    #[inline]
    pub fn pllpdn(&mut self) -> _PLLPDNW {
        _PLLPDNW { w: self }
    }
    #[doc = "Bit 13 - VCO Power Down"]
    #[inline]
    pub fn vcopdn(&mut self) -> _VCOPDNW {
        _VCOPDNW { w: self }
    }
    #[doc = "Bit 16 - USB Clock Control"]
    #[inline]
    pub fn usbcr(&mut self) -> _USBCRW {
        _USBCRW { w: self }
    }
    #[doc = "Bit 17 - MMC Clock Control"]
    #[inline]
    pub fn mmccr(&mut self) -> _MMCCRW {
        _MMCCRW { w: self }
    }
    #[doc = "Bit 18 - Ethernet Clock Control"]
    #[inline]
    pub fn eth0cr(&mut self) -> _ETH0CRW {
        _ETH0CRW { w: self }
    }
    #[doc = "Bit 19 - EBU Clock Control"]
    #[inline]
    pub fn ebucr(&mut self) -> _EBUCRW {
        _EBUCRW { w: self }
    }
    #[doc = "Bit 20 - CCU Clock Control"]
    #[inline]
    pub fn ccucr(&mut self) -> _CCUCRW {
        _CCUCRW { w: self }
    }
    #[doc = "Bit 21 - WDT Clock Control"]
    #[inline]
    pub fn wdtcr(&mut self) -> _WDTCRW {
        _WDTCRW { w: self }
    }
}
