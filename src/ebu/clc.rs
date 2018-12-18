#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLC {
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
#[doc = "Possible values of the field `DISR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISRR {
    #[doc = "EBU disable is not requested"]
    VALUE1,
    #[doc = "EBU disable is requested"]
    VALUE2,
}
impl DISRR {
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
            DISRR::VALUE1 => false,
            DISRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISRR {
        match value {
            false => DISRR::VALUE1,
            true => DISRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DISRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DISRR::VALUE2
    }
}
#[doc = "Possible values of the field `DISS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISSR {
    #[doc = "EBU is enabled (default after reset)"]
    VALUE1,
    #[doc = "EBU is disabled"]
    VALUE2,
}
impl DISSR {
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
            DISSR::VALUE1 => false,
            DISSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISSR {
        match value {
            false => DISSR::VALUE1,
            true => DISSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DISSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DISSR::VALUE2
    }
}
#[doc = "Possible values of the field `SYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCR {
    #[doc = "request EBU to run asynchronously to AHB bus clock and use separate clock source"]
    VALUE1,
    #[doc = "request EBU to run synchronously to ARM processor (default after reset)"]
    VALUE2,
}
impl SYNCR {
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
            SYNCR::VALUE1 => false,
            SYNCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCR {
        match value {
            false => SYNCR::VALUE1,
            true => SYNCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SYNCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SYNCR::VALUE2
    }
}
#[doc = "Possible values of the field `DIV2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV2R {
    #[doc = "standard clocking mode. clock input selected by SYNC bitfield (default after reset)."]
    VALUE1,
    #[doc = "request EBU to run off AHB bus clock divided by 2."]
    VALUE2,
}
impl DIV2R {
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
            DIV2R::VALUE1 => false,
            DIV2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIV2R {
        match value {
            false => DIV2R::VALUE1,
            true => DIV2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIV2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIV2R::VALUE2
    }
}
#[doc = "Possible values of the field `EBUDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBUDIVR {
    #[doc = "request EBU to run off input clock (default after reset)"]
    VALUE1,
    #[doc = "request EBU to run off input clock divided by 2"]
    VALUE2,
    #[doc = "request EBU to run off input clock divided by 3"]
    VALUE3,
    #[doc = "request EBU to run off input clock divided by 4"]
    VALUE4,
}
impl EBUDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EBUDIVR::VALUE1 => 0,
            EBUDIVR::VALUE2 => 1,
            EBUDIVR::VALUE3 => 2,
            EBUDIVR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EBUDIVR {
        match value {
            0 => EBUDIVR::VALUE1,
            1 => EBUDIVR::VALUE2,
            2 => EBUDIVR::VALUE3,
            3 => EBUDIVR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EBUDIVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EBUDIVR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EBUDIVR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EBUDIVR::VALUE4
    }
}
#[doc = "Possible values of the field `SYNCACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCACKR {
    #[doc = "the EBU is asynchronous to the AHB bus clock and is using a separate clock source"]
    VALUE1,
    #[doc = "EBU is synchronous to the AHB bus clock (default after reset)"]
    VALUE2,
}
impl SYNCACKR {
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
            SYNCACKR::VALUE1 => false,
            SYNCACKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCACKR {
        match value {
            false => SYNCACKR::VALUE1,
            true => SYNCACKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SYNCACKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SYNCACKR::VALUE2
    }
}
#[doc = "Possible values of the field `DIV2ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV2ACKR {
    #[doc = "EBU is using standard clocking mode. clock input selected by SYNC bitfield (default after reset)."]
    VALUE1,
    #[doc = "EBU is running off AHB bus clock divided by 2."]
    VALUE2,
}
impl DIV2ACKR {
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
            DIV2ACKR::VALUE1 => false,
            DIV2ACKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIV2ACKR {
        match value {
            false => DIV2ACKR::VALUE1,
            true => DIV2ACKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIV2ACKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIV2ACKR::VALUE2
    }
}
#[doc = "Possible values of the field `EBUDIVACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBUDIVACKR {
    #[doc = "EBU is running off input clock (default after reset)"]
    VALUE1,
    #[doc = "EBU is running off input clock divided by 2"]
    VALUE2,
    #[doc = "EBU is running off input clock divided by 3"]
    VALUE3,
    #[doc = "EBU is running off input clock divided by 4"]
    VALUE4,
}
impl EBUDIVACKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EBUDIVACKR::VALUE1 => 0,
            EBUDIVACKR::VALUE2 => 1,
            EBUDIVACKR::VALUE3 => 2,
            EBUDIVACKR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EBUDIVACKR {
        match value {
            0 => EBUDIVACKR::VALUE1,
            1 => EBUDIVACKR::VALUE2,
            2 => EBUDIVACKR::VALUE3,
            3 => EBUDIVACKR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EBUDIVACKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EBUDIVACKR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EBUDIVACKR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EBUDIVACKR::VALUE4
    }
}
#[doc = "Values that can be written to the field `DISR`"]
pub enum DISRW {
    #[doc = "EBU disable is not requested"]
    VALUE1,
    #[doc = "EBU disable is requested"]
    VALUE2,
}
impl DISRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISRW::VALUE1 => false,
            DISRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISRW<'a> {
    w: &'a mut W,
}
impl<'a> _DISRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EBU disable is not requested"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DISRW::VALUE1)
    }
    #[doc = "EBU disable is requested"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DISRW::VALUE2)
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
#[doc = "Values that can be written to the field `SYNC`"]
pub enum SYNCW {
    #[doc = "request EBU to run asynchronously to AHB bus clock and use separate clock source"]
    VALUE1,
    #[doc = "request EBU to run synchronously to ARM processor (default after reset)"]
    VALUE2,
}
impl SYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNCW::VALUE1 => false,
            SYNCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "request EBU to run asynchronously to AHB bus clock and use separate clock source"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SYNCW::VALUE1)
    }
    #[doc = "request EBU to run synchronously to ARM processor (default after reset)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SYNCW::VALUE2)
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
#[doc = "Values that can be written to the field `DIV2`"]
pub enum DIV2W {
    #[doc = "standard clocking mode. clock input selected by SYNC bitfield (default after reset)."]
    VALUE1,
    #[doc = "request EBU to run off AHB bus clock divided by 2."]
    VALUE2,
}
impl DIV2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIV2W::VALUE1 => false,
            DIV2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIV2W<'a> {
    w: &'a mut W,
}
impl<'a> _DIV2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIV2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "standard clocking mode. clock input selected by SYNC bitfield (default after reset)."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIV2W::VALUE1)
    }
    #[doc = "request EBU to run off AHB bus clock divided by 2."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIV2W::VALUE2)
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
#[doc = "Values that can be written to the field `EBUDIV`"]
pub enum EBUDIVW {
    #[doc = "request EBU to run off input clock (default after reset)"]
    VALUE1,
    #[doc = "request EBU to run off input clock divided by 2"]
    VALUE2,
    #[doc = "request EBU to run off input clock divided by 3"]
    VALUE3,
    #[doc = "request EBU to run off input clock divided by 4"]
    VALUE4,
}
impl EBUDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EBUDIVW::VALUE1 => 0,
            EBUDIVW::VALUE2 => 1,
            EBUDIVW::VALUE3 => 2,
            EBUDIVW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EBUDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _EBUDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EBUDIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "request EBU to run off input clock (default after reset)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EBUDIVW::VALUE1)
    }
    #[doc = "request EBU to run off input clock divided by 2"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EBUDIVW::VALUE2)
    }
    #[doc = "request EBU to run off input clock divided by 3"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(EBUDIVW::VALUE3)
    }
    #[doc = "request EBU to run off input clock divided by 4"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(EBUDIVW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - EBU Disable Request Bit"]
    #[inline]
    pub fn disr(&self) -> DISRR {
        DISRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - EBU Disable Status Bit"]
    #[inline]
    pub fn diss(&self) -> DISSR {
        DISSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - EBU Clocking Mode"]
    #[inline]
    pub fn sync(&self) -> SYNCR {
        SYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - DIV2 Clocking Mode"]
    #[inline]
    pub fn div2(&self) -> DIV2R {
        DIV2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 18:19 - EBU Clock Divide Ratio"]
    #[inline]
    pub fn ebudiv(&self) -> EBUDIVR {
        EBUDIVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - EBU Clocking Mode Status"]
    #[inline]
    pub fn syncack(&self) -> SYNCACKR {
        SYNCACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - DIV2 Clocking Mode Status"]
    #[inline]
    pub fn div2ack(&self) -> DIV2ACKR {
        DIV2ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 22:23 - EBU Clock Divide Ratio Status"]
    #[inline]
    pub fn ebudivack(&self) -> EBUDIVACKR {
        EBUDIVACKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1114112 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - EBU Disable Request Bit"]
    #[inline]
    pub fn disr(&mut self) -> _DISRW {
        _DISRW { w: self }
    }
    #[doc = "Bit 16 - EBU Clocking Mode"]
    #[inline]
    pub fn sync(&mut self) -> _SYNCW {
        _SYNCW { w: self }
    }
    #[doc = "Bit 17 - DIV2 Clocking Mode"]
    #[inline]
    pub fn div2(&mut self) -> _DIV2W {
        _DIV2W { w: self }
    }
    #[doc = "Bits 18:19 - EBU Clock Divide Ratio"]
    #[inline]
    pub fn ebudiv(&mut self) -> _EBUDIVW {
        _EBUDIVW { w: self }
    }
}
