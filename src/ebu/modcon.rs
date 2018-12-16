#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODCON {
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
pub struct STSR {
    bits: bool,
}
impl STSR {
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
#[doc = r" Value of the field"]
pub struct LCKABRTR {
    bits: bool,
}
impl LCKABRTR {
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
#[doc = "Possible values of the field `SDTRI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDTRIR {
    #[doc = "SDRAM control signals are driven by the EBU when the EBU does not own the external bus. SDRAM cannot be shared."]
    VALUE1,
    #[doc = "SDRAM control signals are tri-stated by the EBU when the EBU does not own the external bus. The SDRAM can be shared."]
    VALUE2,
}
impl SDTRIR {
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
            SDTRIR::VALUE1 => false,
            SDTRIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDTRIR {
        match value {
            false => SDTRIR::VALUE1,
            true => SDTRIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SDTRIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SDTRIR::VALUE2
    }
}
#[doc = "Possible values of the field `EXTLOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTLOCKR {
    #[doc = "External bus is not locked after the EBU gains ownership"]
    VALUE1,
    #[doc = "External bus is locked after the EBU gains ownership"]
    VALUE2,
}
impl EXTLOCKR {
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
            EXTLOCKR::VALUE1 => false,
            EXTLOCKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXTLOCKR {
        match value {
            false => EXTLOCKR::VALUE1,
            true => EXTLOCKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EXTLOCKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EXTLOCKR::VALUE2
    }
}
#[doc = "Possible values of the field `ARBSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBSYNCR {
    #[doc = "Arbitration inputs are synchronous"]
    VALUE1,
    #[doc = "Arbitration inputs are asynchronous"]
    VALUE2,
}
impl ARBSYNCR {
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
            ARBSYNCR::VALUE1 => false,
            ARBSYNCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARBSYNCR {
        match value {
            false => ARBSYNCR::VALUE1,
            true => ARBSYNCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ARBSYNCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ARBSYNCR::VALUE2
    }
}
#[doc = "Possible values of the field `ARBMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBMODER {
    #[doc = "No Bus arbitration mode selected"]
    VALUE1,
    #[doc = "Arbiter Mode arbitration mode selected"]
    VALUE2,
    #[doc = "Participant arbitration mode selected"]
    VALUE3,
    #[doc = "Sole Master arbitration mode selected"]
    VALUE4,
}
impl ARBMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ARBMODER::VALUE1 => 0,
            ARBMODER::VALUE2 => 1,
            ARBMODER::VALUE3 => 2,
            ARBMODER::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ARBMODER {
        match value {
            0 => ARBMODER::VALUE1,
            1 => ARBMODER::VALUE2,
            2 => ARBMODER::VALUE3,
            3 => ARBMODER::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ARBMODER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ARBMODER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ARBMODER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == ARBMODER::VALUE4
    }
}
#[doc = "Possible values of the field `TIMEOUTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUTCR {
    #[doc = "Time-out is disabled."]
    VALUE1,
    #[doc = "Time-out is generated after 1 8 clock cycles."]
    VALUE2,
    #[doc = "Time-out is generated after 255 8 clock cycles."]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TIMEOUTCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMEOUTCR::VALUE1 => 0,
            TIMEOUTCR::VALUE2 => 1,
            TIMEOUTCR::VALUE3 => 255,
            TIMEOUTCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMEOUTCR {
        match value {
            0 => TIMEOUTCR::VALUE1,
            1 => TIMEOUTCR::VALUE2,
            255 => TIMEOUTCR::VALUE3,
            i => TIMEOUTCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TIMEOUTCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TIMEOUTCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == TIMEOUTCR::VALUE3
    }
}
#[doc = r" Value of the field"]
pub struct LOCKTIMEOUTR {
    bits: u8,
}
impl LOCKTIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GLOBALCSR {
    bits: u8,
}
impl GLOBALCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ACCSINHR {
    bits: bool,
}
impl ACCSINHR {
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
#[doc = r" Value of the field"]
pub struct ACCSINHACKR {
    bits: bool,
}
impl ACCSINHACKR {
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
#[doc = "Possible values of the field `ALE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALER {
    #[doc = "Output is ADV"]
    VALUE1,
    #[doc = "Output is ALE"]
    VALUE2,
}
impl ALER {
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
            ALER::VALUE1 => false,
            ALER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALER {
        match value {
            false => ALER::VALUE1,
            true => ALER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ALER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ALER::VALUE2
    }
}
#[doc = "Values that can be written to the field `SDTRI`"]
pub enum SDTRIW {
    #[doc = "SDRAM control signals are driven by the EBU when the EBU does not own the external bus. SDRAM cannot be shared."]
    VALUE1,
    #[doc = "SDRAM control signals are tri-stated by the EBU when the EBU does not own the external bus. The SDRAM can be shared."]
    VALUE2,
}
impl SDTRIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDTRIW::VALUE1 => false,
            SDTRIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDTRIW<'a> {
    w: &'a mut W,
}
impl<'a> _SDTRIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDTRIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SDRAM control signals are driven by the EBU when the EBU does not own the external bus. SDRAM cannot be shared."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SDTRIW::VALUE1)
    }
    #[doc = "SDRAM control signals are tri-stated by the EBU when the EBU does not own the external bus. The SDRAM can be shared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SDTRIW::VALUE2)
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
#[doc = "Values that can be written to the field `EXTLOCK`"]
pub enum EXTLOCKW {
    #[doc = "External bus is not locked after the EBU gains ownership"]
    VALUE1,
    #[doc = "External bus is locked after the EBU gains ownership"]
    VALUE2,
}
impl EXTLOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTLOCKW::VALUE1 => false,
            EXTLOCKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTLOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTLOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External bus is not locked after the EBU gains ownership"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXTLOCKW::VALUE1)
    }
    #[doc = "External bus is locked after the EBU gains ownership"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXTLOCKW::VALUE2)
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
#[doc = "Values that can be written to the field `ARBSYNC`"]
pub enum ARBSYNCW {
    #[doc = "Arbitration inputs are synchronous"]
    VALUE1,
    #[doc = "Arbitration inputs are asynchronous"]
    VALUE2,
}
impl ARBSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARBSYNCW::VALUE1 => false,
            ARBSYNCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARBSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _ARBSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARBSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Arbitration inputs are synchronous"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARBSYNCW::VALUE1)
    }
    #[doc = "Arbitration inputs are asynchronous"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARBSYNCW::VALUE2)
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
#[doc = "Values that can be written to the field `ARBMODE`"]
pub enum ARBMODEW {
    #[doc = "No Bus arbitration mode selected"]
    VALUE1,
    #[doc = "Arbiter Mode arbitration mode selected"]
    VALUE2,
    #[doc = "Participant arbitration mode selected"]
    VALUE3,
    #[doc = "Sole Master arbitration mode selected"]
    VALUE4,
}
impl ARBMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ARBMODEW::VALUE1 => 0,
            ARBMODEW::VALUE2 => 1,
            ARBMODEW::VALUE3 => 2,
            ARBMODEW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARBMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ARBMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARBMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No Bus arbitration mode selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARBMODEW::VALUE1)
    }
    #[doc = "Arbiter Mode arbitration mode selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARBMODEW::VALUE2)
    }
    #[doc = "Participant arbitration mode selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ARBMODEW::VALUE3)
    }
    #[doc = "Sole Master arbitration mode selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(ARBMODEW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMEOUTC`"]
pub enum TIMEOUTCW {
    #[doc = "Time-out is disabled."]
    VALUE1,
    #[doc = "Time-out is generated after 1 8 clock cycles."]
    VALUE2,
    #[doc = "Time-out is generated after 255 8 clock cycles."]
    VALUE3,
}
impl TIMEOUTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMEOUTCW::VALUE1 => 0,
            TIMEOUTCW::VALUE2 => 1,
            TIMEOUTCW::VALUE3 => 255,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMEOUTCW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMEOUTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMEOUTCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Time-out is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TIMEOUTCW::VALUE1)
    }
    #[doc = "Time-out is generated after 1 8 clock cycles."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TIMEOUTCW::VALUE2)
    }
    #[doc = "Time-out is generated after 255 8 clock cycles."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(TIMEOUTCW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOCKTIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKTIMEOUTW<'a> {
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
#[doc = r" Proxy"]
pub struct _GLOBALCSW<'a> {
    w: &'a mut W,
}
impl<'a> _GLOBALCSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACCSINHW<'a> {
    w: &'a mut W,
}
impl<'a> _ACCSINHW<'a> {
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
#[doc = "Values that can be written to the field `ALE`"]
pub enum ALEW {
    #[doc = "Output is ADV"]
    VALUE1,
    #[doc = "Output is ALE"]
    VALUE2,
}
impl ALEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALEW::VALUE1 => false,
            ALEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALEW<'a> {
    w: &'a mut W,
}
impl<'a> _ALEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Output is ADV"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEW::VALUE1)
    }
    #[doc = "Output is ALE"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEW::VALUE2)
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
    #[doc = "Bit 0 - Memory Status Bit"]
    #[inline]
    pub fn sts(&self) -> STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STSR { bits }
    }
    #[doc = "Bit 1 - Lock Abort"]
    #[inline]
    pub fn lckabrt(&self) -> LCKABRTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LCKABRTR { bits }
    }
    #[doc = "Bit 2 - SDRAM Tristate"]
    #[inline]
    pub fn sdtri(&self) -> SDTRIR {
        SDTRIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - External Bus Lock Control"]
    #[inline]
    pub fn extlock(&self) -> EXTLOCKR {
        EXTLOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Arbitration Signal Synchronization Control"]
    #[inline]
    pub fn arbsync(&self) -> ARBSYNCR {
        ARBSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:7 - Arbitration Mode Selection"]
    #[inline]
    pub fn arbmode(&self) -> ARBMODER {
        ARBMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Bus Time-out Control"]
    #[inline]
    pub fn timeoutc(&self) -> TIMEOUTCR {
        TIMEOUTCR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Lock Timeout Counter Preload"]
    #[inline]
    pub fn locktimeout(&self) -> LOCKTIMEOUTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LOCKTIMEOUTR { bits }
    }
    #[doc = "Bits 24:27 - Global Chip Select Enable"]
    #[inline]
    pub fn globalcs(&self) -> GLOBALCSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GLOBALCSR { bits }
    }
    #[doc = "Bit 28 - Access Inhibit request"]
    #[inline]
    pub fn accsinh(&self) -> ACCSINHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACCSINHR { bits }
    }
    #[doc = "Bit 29 - Access inhibit acknowledge"]
    #[inline]
    pub fn accsinhack(&self) -> ACCSINHACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACCSINHACKR { bits }
    }
    #[doc = "Bit 31 - ALE Mode"]
    #[inline]
    pub fn ale(&self) -> ALER {
        ALER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 32 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - SDRAM Tristate"]
    #[inline]
    pub fn sdtri(&mut self) -> _SDTRIW {
        _SDTRIW { w: self }
    }
    #[doc = "Bit 4 - External Bus Lock Control"]
    #[inline]
    pub fn extlock(&mut self) -> _EXTLOCKW {
        _EXTLOCKW { w: self }
    }
    #[doc = "Bit 5 - Arbitration Signal Synchronization Control"]
    #[inline]
    pub fn arbsync(&mut self) -> _ARBSYNCW {
        _ARBSYNCW { w: self }
    }
    #[doc = "Bits 6:7 - Arbitration Mode Selection"]
    #[inline]
    pub fn arbmode(&mut self) -> _ARBMODEW {
        _ARBMODEW { w: self }
    }
    #[doc = "Bits 8:15 - Bus Time-out Control"]
    #[inline]
    pub fn timeoutc(&mut self) -> _TIMEOUTCW {
        _TIMEOUTCW { w: self }
    }
    #[doc = "Bits 16:23 - Lock Timeout Counter Preload"]
    #[inline]
    pub fn locktimeout(&mut self) -> _LOCKTIMEOUTW {
        _LOCKTIMEOUTW { w: self }
    }
    #[doc = "Bits 24:27 - Global Chip Select Enable"]
    #[inline]
    pub fn globalcs(&mut self) -> _GLOBALCSW {
        _GLOBALCSW { w: self }
    }
    #[doc = "Bit 28 - Access Inhibit request"]
    #[inline]
    pub fn accsinh(&mut self) -> _ACCSINHW {
        _ACCSINHW { w: self }
    }
    #[doc = "Bit 31 - ALE Mode"]
    #[inline]
    pub fn ale(&mut self) -> _ALEW {
        _ALEW { w: self }
    }
}
