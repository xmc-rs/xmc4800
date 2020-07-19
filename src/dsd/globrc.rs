#[doc = "Reader of register GLOBRC"]
pub type R = crate::R<u32, super::GLOBRC>;
#[doc = "Writer for register GLOBRC"]
pub type W = crate::W<u32, super::GLOBRC>;
#[doc = "Register GLOBRC `reset()`'s with value 0"]
impl crate::ResetValue for super::GLOBRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel 0 Run Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0RUN_A {
    #[doc = "0: Stop channel x"]
    VALUE1 = 0,
    #[doc = "1: Demodulator channel x is enabled and runs"]
    VALUE2 = 1,
}
impl From<CH0RUN_A> for bool {
    #[inline(always)]
    fn from(variant: CH0RUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH0RUN`"]
pub type CH0RUN_R = crate::R<bool, CH0RUN_A>;
impl CH0RUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0RUN_A {
        match self.bits {
            false => CH0RUN_A::VALUE1,
            true => CH0RUN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH0RUN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH0RUN_A::VALUE2
    }
}
#[doc = "Write proxy for field `CH0RUN`"]
pub struct CH0RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0RUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0RUN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stop channel x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH0RUN_A::VALUE1)
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH0RUN_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Channel 1 Run Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1RUN_A {
    #[doc = "0: Stop channel x"]
    VALUE1 = 0,
    #[doc = "1: Demodulator channel x is enabled and runs"]
    VALUE2 = 1,
}
impl From<CH1RUN_A> for bool {
    #[inline(always)]
    fn from(variant: CH1RUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH1RUN`"]
pub type CH1RUN_R = crate::R<bool, CH1RUN_A>;
impl CH1RUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1RUN_A {
        match self.bits {
            false => CH1RUN_A::VALUE1,
            true => CH1RUN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH1RUN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH1RUN_A::VALUE2
    }
}
#[doc = "Write proxy for field `CH1RUN`"]
pub struct CH1RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1RUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1RUN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stop channel x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH1RUN_A::VALUE1)
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH1RUN_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Channel 2 Run Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2RUN_A {
    #[doc = "0: Stop channel x"]
    VALUE1 = 0,
    #[doc = "1: Demodulator channel x is enabled and runs"]
    VALUE2 = 1,
}
impl From<CH2RUN_A> for bool {
    #[inline(always)]
    fn from(variant: CH2RUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH2RUN`"]
pub type CH2RUN_R = crate::R<bool, CH2RUN_A>;
impl CH2RUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2RUN_A {
        match self.bits {
            false => CH2RUN_A::VALUE1,
            true => CH2RUN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH2RUN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH2RUN_A::VALUE2
    }
}
#[doc = "Write proxy for field `CH2RUN`"]
pub struct CH2RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2RUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2RUN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stop channel x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH2RUN_A::VALUE1)
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH2RUN_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Channel 3 Run Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3RUN_A {
    #[doc = "0: Stop channel x"]
    VALUE1 = 0,
    #[doc = "1: Demodulator channel x is enabled and runs"]
    VALUE2 = 1,
}
impl From<CH3RUN_A> for bool {
    #[inline(always)]
    fn from(variant: CH3RUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH3RUN`"]
pub type CH3RUN_R = crate::R<bool, CH3RUN_A>;
impl CH3RUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3RUN_A {
        match self.bits {
            false => CH3RUN_A::VALUE1,
            true => CH3RUN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH3RUN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH3RUN_A::VALUE2
    }
}
#[doc = "Write proxy for field `CH3RUN`"]
pub struct CH3RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3RUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3RUN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stop channel x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH3RUN_A::VALUE1)
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH3RUN_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Run Control"]
    #[inline(always)]
    pub fn ch0run(&self) -> CH0RUN_R {
        CH0RUN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Run Control"]
    #[inline(always)]
    pub fn ch1run(&self) -> CH1RUN_R {
        CH1RUN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Run Control"]
    #[inline(always)]
    pub fn ch2run(&self) -> CH2RUN_R {
        CH2RUN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Run Control"]
    #[inline(always)]
    pub fn ch3run(&self) -> CH3RUN_R {
        CH3RUN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Run Control"]
    #[inline(always)]
    pub fn ch0run(&mut self) -> CH0RUN_W {
        CH0RUN_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Run Control"]
    #[inline(always)]
    pub fn ch1run(&mut self) -> CH1RUN_W {
        CH1RUN_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Run Control"]
    #[inline(always)]
    pub fn ch2run(&mut self) -> CH2RUN_W {
        CH2RUN_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Run Control"]
    #[inline(always)]
    pub fn ch3run(&mut self) -> CH3RUN_W {
        CH3RUN_W { w: self }
    }
}
