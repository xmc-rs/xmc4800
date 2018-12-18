#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GLOBRC {
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
#[doc = "Possible values of the field `CH0RUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0RUNR {
    #[doc = "Stop channel x"]
    VALUE1,
    #[doc = "Demodulator channel x is enabled and runs"]
    VALUE2,
}
impl CH0RUNR {
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
            CH0RUNR::VALUE1 => false,
            CH0RUNR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH0RUNR {
        match value {
            false => CH0RUNR::VALUE1,
            true => CH0RUNR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH0RUNR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH0RUNR::VALUE2
    }
}
#[doc = "Possible values of the field `CH1RUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1RUNR {
    #[doc = "Stop channel x"]
    VALUE1,
    #[doc = "Demodulator channel x is enabled and runs"]
    VALUE2,
}
impl CH1RUNR {
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
            CH1RUNR::VALUE1 => false,
            CH1RUNR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH1RUNR {
        match value {
            false => CH1RUNR::VALUE1,
            true => CH1RUNR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH1RUNR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH1RUNR::VALUE2
    }
}
#[doc = "Possible values of the field `CH2RUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2RUNR {
    #[doc = "Stop channel x"]
    VALUE1,
    #[doc = "Demodulator channel x is enabled and runs"]
    VALUE2,
}
impl CH2RUNR {
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
            CH2RUNR::VALUE1 => false,
            CH2RUNR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH2RUNR {
        match value {
            false => CH2RUNR::VALUE1,
            true => CH2RUNR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH2RUNR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH2RUNR::VALUE2
    }
}
#[doc = "Possible values of the field `CH3RUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3RUNR {
    #[doc = "Stop channel x"]
    VALUE1,
    #[doc = "Demodulator channel x is enabled and runs"]
    VALUE2,
}
impl CH3RUNR {
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
            CH3RUNR::VALUE1 => false,
            CH3RUNR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH3RUNR {
        match value {
            false => CH3RUNR::VALUE1,
            true => CH3RUNR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH3RUNR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH3RUNR::VALUE2
    }
}
#[doc = "Values that can be written to the field `CH0RUN`"]
pub enum CH0RUNW {
    #[doc = "Stop channel x"]
    VALUE1,
    #[doc = "Demodulator channel x is enabled and runs"]
    VALUE2,
}
impl CH0RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0RUNW::VALUE1 => false,
            CH0RUNW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH0RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH0RUNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stop channel x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH0RUNW::VALUE1)
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH0RUNW::VALUE2)
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
#[doc = "Values that can be written to the field `CH1RUN`"]
pub enum CH1RUNW {
    #[doc = "Stop channel x"]
    VALUE1,
    #[doc = "Demodulator channel x is enabled and runs"]
    VALUE2,
}
impl CH1RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH1RUNW::VALUE1 => false,
            CH1RUNW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH1RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH1RUNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stop channel x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH1RUNW::VALUE1)
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH1RUNW::VALUE2)
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
#[doc = "Values that can be written to the field `CH2RUN`"]
pub enum CH2RUNW {
    #[doc = "Stop channel x"]
    VALUE1,
    #[doc = "Demodulator channel x is enabled and runs"]
    VALUE2,
}
impl CH2RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH2RUNW::VALUE1 => false,
            CH2RUNW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH2RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH2RUNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stop channel x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH2RUNW::VALUE1)
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH2RUNW::VALUE2)
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
#[doc = "Values that can be written to the field `CH3RUN`"]
pub enum CH3RUNW {
    #[doc = "Stop channel x"]
    VALUE1,
    #[doc = "Demodulator channel x is enabled and runs"]
    VALUE2,
}
impl CH3RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH3RUNW::VALUE1 => false,
            CH3RUNW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH3RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH3RUNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stop channel x"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH3RUNW::VALUE1)
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH3RUNW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Channel 0 Run Control"]
    #[inline]
    pub fn ch0run(&self) -> CH0RUNR {
        CH0RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel 1 Run Control"]
    #[inline]
    pub fn ch1run(&self) -> CH1RUNR {
        CH1RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel 2 Run Control"]
    #[inline]
    pub fn ch2run(&self) -> CH2RUNR {
        CH2RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel 3 Run Control"]
    #[inline]
    pub fn ch3run(&self) -> CH3RUNR {
        CH3RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Channel 0 Run Control"]
    #[inline]
    pub fn ch0run(&mut self) -> _CH0RUNW {
        _CH0RUNW { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Run Control"]
    #[inline]
    pub fn ch1run(&mut self) -> _CH1RUNW {
        _CH1RUNW { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Run Control"]
    #[inline]
    pub fn ch2run(&mut self) -> _CH2RUNW {
        _CH2RUNW { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Run Control"]
    #[inline]
    pub fn ch3run(&mut self) -> _CH3RUNW {
        _CH3RUNW { w: self }
    }
}
