#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PSUS {
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
#[doc = "Possible values of the field `QSUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QSUSR {
    #[doc = "Suspend request ignored"]
    VALUE1,
    #[doc = "Stop immediately"]
    VALUE2,
    #[doc = "Suspend in the next index occurrence"]
    VALUE3,
    #[doc = "Suspend in the next phase (PhaseA or PhaseB) occurrence"]
    VALUE4,
}
impl QSUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            QSUSR::VALUE1 => 0,
            QSUSR::VALUE2 => 1,
            QSUSR::VALUE3 => 2,
            QSUSR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> QSUSR {
        match value {
            0 => QSUSR::VALUE1,
            1 => QSUSR::VALUE2,
            2 => QSUSR::VALUE3,
            3 => QSUSR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == QSUSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == QSUSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == QSUSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == QSUSR::VALUE4
    }
}
#[doc = "Possible values of the field `MSUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSUSR {
    #[doc = "Suspend request ignored"]
    VALUE1,
    #[doc = "Stop immediately. Multi-Channel pattern is not set to the reset value."]
    VALUE2,
    #[doc = "Stop immediately. Multi-Channel pattern is set to the reset value."]
    VALUE3,
    #[doc = "Suspend with the synchronization of the PWM signal. Multi-Channel pattern is set to the reset value at the same time of the synchronization."]
    VALUE4,
}
impl MSUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSUSR::VALUE1 => 0,
            MSUSR::VALUE2 => 1,
            MSUSR::VALUE3 => 2,
            MSUSR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSUSR {
        match value {
            0 => MSUSR::VALUE1,
            1 => MSUSR::VALUE2,
            2 => MSUSR::VALUE3,
            3 => MSUSR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MSUSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MSUSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == MSUSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == MSUSR::VALUE4
    }
}
#[doc = "Values that can be written to the field `QSUS`"]
pub enum QSUSW {
    #[doc = "Suspend request ignored"]
    VALUE1,
    #[doc = "Stop immediately"]
    VALUE2,
    #[doc = "Suspend in the next index occurrence"]
    VALUE3,
    #[doc = "Suspend in the next phase (PhaseA or PhaseB) occurrence"]
    VALUE4,
}
impl QSUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            QSUSW::VALUE1 => 0,
            QSUSW::VALUE2 => 1,
            QSUSW::VALUE3 => 2,
            QSUSW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QSUSW<'a> {
    w: &'a mut W,
}
impl<'a> _QSUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QSUSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Suspend request ignored"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(QSUSW::VALUE1)
    }
    #[doc = "Stop immediately"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(QSUSW::VALUE2)
    }
    #[doc = "Suspend in the next index occurrence"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(QSUSW::VALUE3)
    }
    #[doc = "Suspend in the next phase (PhaseA or PhaseB) occurrence"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(QSUSW::VALUE4)
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
#[doc = "Values that can be written to the field `MSUS`"]
pub enum MSUSW {
    #[doc = "Suspend request ignored"]
    VALUE1,
    #[doc = "Stop immediately. Multi-Channel pattern is not set to the reset value."]
    VALUE2,
    #[doc = "Stop immediately. Multi-Channel pattern is set to the reset value."]
    VALUE3,
    #[doc = "Suspend with the synchronization of the PWM signal. Multi-Channel pattern is set to the reset value at the same time of the synchronization."]
    VALUE4,
}
impl MSUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSUSW::VALUE1 => 0,
            MSUSW::VALUE2 => 1,
            MSUSW::VALUE3 => 2,
            MSUSW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSUSW<'a> {
    w: &'a mut W,
}
impl<'a> _MSUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSUSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Suspend request ignored"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSUSW::VALUE1)
    }
    #[doc = "Stop immediately. Multi-Channel pattern is not set to the reset value."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSUSW::VALUE2)
    }
    #[doc = "Stop immediately. Multi-Channel pattern is set to the reset value."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(MSUSW::VALUE3)
    }
    #[doc = "Suspend with the synchronization of the PWM signal. Multi-Channel pattern is set to the reset value at the same time of the synchronization."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(MSUSW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
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
    #[doc = "Bits 0:1 - Quadrature Mode Suspend Config"]
    #[inline]
    pub fn qsus(&self) -> QSUSR {
        QSUSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Multi-Channel Mode Suspend Config"]
    #[inline]
    pub fn msus(&self) -> MSUSR {
        MSUSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
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
    #[doc = "Bits 0:1 - Quadrature Mode Suspend Config"]
    #[inline]
    pub fn qsus(&mut self) -> _QSUSW {
        _QSUSW { w: self }
    }
    #[doc = "Bits 2:3 - Multi-Channel Mode Suspend Config"]
    #[inline]
    pub fn msus(&mut self) -> _MSUSW {
        _MSUSW { w: self }
    }
}
