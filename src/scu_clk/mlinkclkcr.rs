#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MLINKCLKCR {
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
#[doc = r" Value of the field"]
pub struct SYSDIVR {
    bits: u8,
}
impl SYSDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SYSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSSELR {
    #[doc = "fOFIclock"]
    VALUE1,
    #[doc = "fPLLclock"]
    VALUE2,
}
impl SYSSELR {
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
            SYSSELR::VALUE1 => false,
            SYSSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYSSELR {
        match value {
            false => SYSSELR::VALUE1,
            true => SYSSELR::VALUE2,
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
#[doc = "Possible values of the field `CPUDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPUDIVR {
    #[doc = "fCPU=fSYS"]
    VALUE1,
    #[doc = "fCPU=fSYS/ 2"]
    VALUE2,
}
impl CPUDIVR {
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
            CPUDIVR::VALUE1 => false,
            CPUDIVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPUDIVR {
        match value {
            false => CPUDIVR::VALUE1,
            true => CPUDIVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CPUDIVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CPUDIVR::VALUE2
    }
}
#[doc = "Possible values of the field `PBDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBDIVR {
    #[doc = "fPERIPH=fCPU"]
    VALUE1,
    #[doc = "fPERIPH=fCPU/ 2"]
    VALUE2,
}
impl PBDIVR {
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
            PBDIVR::VALUE1 => false,
            PBDIVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PBDIVR {
        match value {
            false => PBDIVR::VALUE1,
            true => PBDIVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PBDIVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PBDIVR::VALUE2
    }
}
#[doc = "Possible values of the field `CCUDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCUDIVR {
    #[doc = "fCCU=fSYS"]
    VALUE1,
    #[doc = "fCCU=fSYS/ 2"]
    VALUE2,
}
impl CCUDIVR {
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
            CCUDIVR::VALUE1 => false,
            CCUDIVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCUDIVR {
        match value {
            false => CCUDIVR::VALUE1,
            true => CCUDIVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CCUDIVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CCUDIVR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct WDTDIVR {
    bits: u8,
}
impl WDTDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WDTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTSELR {
    #[doc = "fOFIclock"]
    VALUE1,
    #[doc = "fSTDBYclock"]
    VALUE2,
    #[doc = "fPLLclock"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WDTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WDTSELR::VALUE1 => 0,
            WDTSELR::VALUE2 => 1,
            WDTSELR::VALUE3 => 2,
            WDTSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WDTSELR {
        match value {
            0 => WDTSELR::VALUE1,
            1 => WDTSELR::VALUE2,
            2 => WDTSELR::VALUE3,
            i => WDTSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WDTSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WDTSELR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == WDTSELR::VALUE3
    }
}
#[doc = r" Value of the field"]
pub struct EBUDIVR {
    bits: u8,
}
impl EBUDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SYSDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYSSEL`"]
pub enum SYSSELW {
    #[doc = "fOFIclock"]
    VALUE1,
    #[doc = "fPLLclock"]
    VALUE2,
}
impl SYSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSSELW::VALUE1 => false,
            SYSSELW::VALUE2 => true,
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
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "fOFIclock"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SYSSELW::VALUE1)
    }
    #[doc = "fPLLclock"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SYSSELW::VALUE2)
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
#[doc = "Values that can be written to the field `CPUDIV`"]
pub enum CPUDIVW {
    #[doc = "fCPU=fSYS"]
    VALUE1,
    #[doc = "fCPU=fSYS/ 2"]
    VALUE2,
}
impl CPUDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPUDIVW::VALUE1 => false,
            CPUDIVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPUDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CPUDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPUDIVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "fCPU=fSYS"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CPUDIVW::VALUE1)
    }
    #[doc = "fCPU=fSYS/ 2"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CPUDIVW::VALUE2)
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
#[doc = "Values that can be written to the field `PBDIV`"]
pub enum PBDIVW {
    #[doc = "fPERIPH=fCPU"]
    VALUE1,
    #[doc = "fPERIPH=fCPU/ 2"]
    VALUE2,
}
impl PBDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PBDIVW::VALUE1 => false,
            PBDIVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PBDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _PBDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PBDIVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "fPERIPH=fCPU"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PBDIVW::VALUE1)
    }
    #[doc = "fPERIPH=fCPU/ 2"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PBDIVW::VALUE2)
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
#[doc = "Values that can be written to the field `CCUDIV`"]
pub enum CCUDIVW {
    #[doc = "fCCU=fSYS"]
    VALUE1,
    #[doc = "fCCU=fSYS/ 2"]
    VALUE2,
}
impl CCUDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCUDIVW::VALUE1 => false,
            CCUDIVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCUDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CCUDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCUDIVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "fCCU=fSYS"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCUDIVW::VALUE1)
    }
    #[doc = "fCCU=fSYS/ 2"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCUDIVW::VALUE2)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WDTDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WDTSEL`"]
pub enum WDTSELW {
    #[doc = "fOFIclock"]
    VALUE1,
    #[doc = "fSTDBYclock"]
    VALUE2,
    #[doc = "fPLLclock"]
    VALUE3,
}
impl WDTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WDTSELW::VALUE1 => 0,
            WDTSELW::VALUE2 => 1,
            WDTSELW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "fOFIclock"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WDTSELW::VALUE1)
    }
    #[doc = "fSTDBYclock"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WDTSELW::VALUE2)
    }
    #[doc = "fPLLclock"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(WDTSELW::VALUE3)
    }
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
#[doc = r" Proxy"]
pub struct _EBUDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _EBUDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 26;
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
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline]
    pub fn sysdiv(&self) -> SYSDIVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYSDIVR { bits }
    }
    #[doc = "Bit 8 - System Clock Selection Value"]
    #[inline]
    pub fn syssel(&self) -> SYSSELR {
        SYSSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - CPU Clock Divider Enable"]
    #[inline]
    pub fn cpudiv(&self) -> CPUDIVR {
        CPUDIVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - PB Clock Divider Enable"]
    #[inline]
    pub fn pbdiv(&self) -> PBDIVR {
        PBDIVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - CCU Clock Divider Enable"]
    #[inline]
    pub fn ccudiv(&self) -> CCUDIVR {
        CCUDIVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:23 - WDT Clock Divider Value"]
    #[inline]
    pub fn wdtdiv(&self) -> WDTDIVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WDTDIVR { bits }
    }
    #[doc = "Bits 24:25 - WDT Clock Selection Value"]
    #[inline]
    pub fn wdtsel(&self) -> WDTSELR {
        WDTSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:31 - EBU Clock Divider Value"]
    #[inline]
    pub fn ebudiv(&self) -> EBUDIVR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EBUDIVR { bits }
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
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline]
    pub fn sysdiv(&mut self) -> _SYSDIVW {
        _SYSDIVW { w: self }
    }
    #[doc = "Bit 8 - System Clock Selection Value"]
    #[inline]
    pub fn syssel(&mut self) -> _SYSSELW {
        _SYSSELW { w: self }
    }
    #[doc = "Bit 10 - CPU Clock Divider Enable"]
    #[inline]
    pub fn cpudiv(&mut self) -> _CPUDIVW {
        _CPUDIVW { w: self }
    }
    #[doc = "Bit 12 - PB Clock Divider Enable"]
    #[inline]
    pub fn pbdiv(&mut self) -> _PBDIVW {
        _PBDIVW { w: self }
    }
    #[doc = "Bit 14 - CCU Clock Divider Enable"]
    #[inline]
    pub fn ccudiv(&mut self) -> _CCUDIVW {
        _CCUDIVW { w: self }
    }
    #[doc = "Bits 16:23 - WDT Clock Divider Value"]
    #[inline]
    pub fn wdtdiv(&mut self) -> _WDTDIVW {
        _WDTDIVW { w: self }
    }
    #[doc = "Bits 24:25 - WDT Clock Selection Value"]
    #[inline]
    pub fn wdtsel(&mut self) -> _WDTSELW {
        _WDTSELW { w: self }
    }
    #[doc = "Bits 26:31 - EBU Clock Divider Value"]
    #[inline]
    pub fn ebudiv(&mut self) -> _EBUDIVW {
        _EBUDIVW { w: self }
    }
}
