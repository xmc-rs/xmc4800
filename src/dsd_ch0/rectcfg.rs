#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RECTCFG {
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
#[doc = "Possible values of the field `RFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFENR {
    #[doc = "No rectification, data not altered"]
    VALUE1,
    #[doc = "Data are rectified according to SGND"]
    VALUE2,
}
impl RFENR {
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
            RFENR::VALUE1 => false,
            RFENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFENR {
        match value {
            false => RFENR::VALUE1,
            true => RFENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RFENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RFENR::VALUE2
    }
}
#[doc = "Possible values of the field `SSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSRCR {
    #[doc = "On-chip carrier generator"]
    VALUE1,
    #[doc = "Sign of result of next channel"]
    VALUE2,
    #[doc = "External sign signal A"]
    VALUE3,
    #[doc = "External sign signal B"]
    VALUE4,
}
impl SSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SSRCR::VALUE1 => 0,
            SSRCR::VALUE2 => 1,
            SSRCR::VALUE3 => 2,
            SSRCR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SSRCR {
        match value {
            0 => SSRCR::VALUE1,
            1 => SSRCR::VALUE2,
            2 => SSRCR::VALUE3,
            3 => SSRCR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SSRCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SSRCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SSRCR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SSRCR::VALUE4
    }
}
#[doc = "Possible values of the field `SDVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDVALR {
    #[doc = "No new result available"]
    VALUE1,
    #[doc = "Bitfield SDCAP has been updated with a new captured value and has not yet been read"]
    VALUE2,
}
impl SDVALR {
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
            SDVALR::VALUE1 => false,
            SDVALR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDVALR {
        match value {
            false => SDVALR::VALUE1,
            true => SDVALR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SDVALR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SDVALR::VALUE2
    }
}
#[doc = "Possible values of the field `SGNCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SGNCSR {
    #[doc = "Positive values"]
    VALUE1,
    #[doc = "Negative values"]
    VALUE2,
}
impl SGNCSR {
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
            SGNCSR::VALUE1 => false,
            SGNCSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SGNCSR {
        match value {
            false => SGNCSR::VALUE1,
            true => SGNCSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SGNCSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SGNCSR::VALUE2
    }
}
#[doc = "Possible values of the field `SGND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SGNDR {
    #[doc = "Positive values"]
    VALUE1,
    #[doc = "Negative values"]
    VALUE2,
}
impl SGNDR {
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
            SGNDR::VALUE1 => false,
            SGNDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SGNDR {
        match value {
            false => SGNDR::VALUE1,
            true => SGNDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SGNDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SGNDR::VALUE2
    }
}
#[doc = "Values that can be written to the field `RFEN`"]
pub enum RFENW {
    #[doc = "No rectification, data not altered"]
    VALUE1,
    #[doc = "Data are rectified according to SGND"]
    VALUE2,
}
impl RFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RFENW::VALUE1 => false,
            RFENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFENW<'a> {
    w: &'a mut W,
}
impl<'a> _RFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No rectification, data not altered"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RFENW::VALUE1)
    }
    #[doc = "Data are rectified according to SGND"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RFENW::VALUE2)
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
#[doc = "Values that can be written to the field `SSRC`"]
pub enum SSRCW {
    #[doc = "On-chip carrier generator"]
    VALUE1,
    #[doc = "Sign of result of next channel"]
    VALUE2,
    #[doc = "External sign signal A"]
    VALUE3,
    #[doc = "External sign signal B"]
    VALUE4,
}
impl SSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SSRCW::VALUE1 => 0,
            SSRCW::VALUE2 => 1,
            SSRCW::VALUE3 => 2,
            SSRCW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _SSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "On-chip carrier generator"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SSRCW::VALUE1)
    }
    #[doc = "Sign of result of next channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SSRCW::VALUE2)
    }
    #[doc = "External sign signal A"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SSRCW::VALUE3)
    }
    #[doc = "External sign signal B"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(SSRCW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 0 - Rectification Enable"]
    #[inline]
    pub fn rfen(&self) -> RFENR {
        RFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Sign Source"]
    #[inline]
    pub fn ssrc(&self) -> SSRCR {
        SSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Valid Flag"]
    #[inline]
    pub fn sdval(&self) -> SDVALR {
        SDVALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Selected Carrier Sign Signal"]
    #[inline]
    pub fn sgncs(&self) -> SGNCSR {
        SGNCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Sign Signal Delayed"]
    #[inline]
    pub fn sgnd(&self) -> SGNDR {
        SGNDR::_from({
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
        W { bits: 2147483648 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Rectification Enable"]
    #[inline]
    pub fn rfen(&mut self) -> _RFENW {
        _RFENW { w: self }
    }
    #[doc = "Bits 4:5 - Sign Source"]
    #[inline]
    pub fn ssrc(&mut self) -> _SSRCW {
        _SSRCW { w: self }
    }
}
