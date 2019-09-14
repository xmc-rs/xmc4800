#[doc = "Reader of register LSTDSTREG"]
pub type R = crate::R<u32, super::LSTDSTREG>;
#[doc = "Writer for register LSTDSTREG"]
pub type W = crate::W<u32, super::LSTDSTREG>;
#[doc = "Register LSTDSTREG `reset()`'s with value 0"]
impl crate::ResetValue for super::LSTDSTREG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Destination last transaction request write enable for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WE_CH0_AW {
    #[doc = "0: write disabled"]
    VALUE1,
    #[doc = "1: write enabled"]
    VALUE2,
}
impl From<WE_CH0_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH0_AW) -> Self {
        match variant {
            WE_CH0_AW::VALUE1 => false,
            WE_CH0_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `WE_CH0`"]
pub struct WE_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_CH0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WE_CH0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH0_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH0_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Destination last transaction request write enable for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WE_CH1_AW {
    #[doc = "0: write disabled"]
    VALUE1,
    #[doc = "1: write enabled"]
    VALUE2,
}
impl From<WE_CH1_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH1_AW) -> Self {
        match variant {
            WE_CH1_AW::VALUE1 => false,
            WE_CH1_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `WE_CH1`"]
pub struct WE_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_CH1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WE_CH1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH1_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH1_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Destination last transaction request write enable for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WE_CH2_AW {
    #[doc = "0: write disabled"]
    VALUE1,
    #[doc = "1: write enabled"]
    VALUE2,
}
impl From<WE_CH2_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH2_AW) -> Self {
        match variant {
            WE_CH2_AW::VALUE1 => false,
            WE_CH2_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `WE_CH2`"]
pub struct WE_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_CH2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WE_CH2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH2_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH2_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Destination last transaction request write enable for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WE_CH3_AW {
    #[doc = "0: write disabled"]
    VALUE1,
    #[doc = "1: write enabled"]
    VALUE2,
}
impl From<WE_CH3_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH3_AW) -> Self {
        match variant {
            WE_CH3_AW::VALUE1 => false,
            WE_CH3_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `WE_CH3`"]
pub struct WE_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_CH3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WE_CH3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH3_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH3_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Destination last request for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1,
    #[doc = "1: Last transaction in current block"]
    VALUE2,
}
impl From<CH0_A> for bool {
    #[inline(always)]
    fn from(variant: CH0_A) -> Self {
        match variant {
            CH0_A::VALUE1 => false,
            CH0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CH0`"]
pub type CH0_R = crate::R<bool, CH0_A>;
impl CH0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0_A {
        match self.bits {
            false => CH0_A::VALUE1,
            true => CH0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH0_A::VALUE2
    }
}
#[doc = "Write proxy for field `CH0`"]
pub struct CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH0_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH0_A::VALUE2)
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
#[doc = "Destination last request for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1,
    #[doc = "1: Last transaction in current block"]
    VALUE2,
}
impl From<CH1_A> for bool {
    #[inline(always)]
    fn from(variant: CH1_A) -> Self {
        match variant {
            CH1_A::VALUE1 => false,
            CH1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CH1`"]
pub type CH1_R = crate::R<bool, CH1_A>;
impl CH1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1_A {
        match self.bits {
            false => CH1_A::VALUE1,
            true => CH1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH1_A::VALUE2
    }
}
#[doc = "Write proxy for field `CH1`"]
pub struct CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH1_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH1_A::VALUE2)
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
#[doc = "Destination last request for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1,
    #[doc = "1: Last transaction in current block"]
    VALUE2,
}
impl From<CH2_A> for bool {
    #[inline(always)]
    fn from(variant: CH2_A) -> Self {
        match variant {
            CH2_A::VALUE1 => false,
            CH2_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CH2`"]
pub type CH2_R = crate::R<bool, CH2_A>;
impl CH2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2_A {
        match self.bits {
            false => CH2_A::VALUE1,
            true => CH2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH2_A::VALUE2
    }
}
#[doc = "Write proxy for field `CH2`"]
pub struct CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH2_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH2_A::VALUE2)
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
#[doc = "Destination last request for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1,
    #[doc = "1: Last transaction in current block"]
    VALUE2,
}
impl From<CH3_A> for bool {
    #[inline(always)]
    fn from(variant: CH3_A) -> Self {
        match variant {
            CH3_A::VALUE1 => false,
            CH3_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CH3`"]
pub type CH3_R = crate::R<bool, CH3_A>;
impl CH3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3_A {
        match self.bits {
            false => CH3_A::VALUE1,
            true => CH3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH3_A::VALUE2
    }
}
#[doc = "Write proxy for field `CH3`"]
pub struct CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH3_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH3_A::VALUE2)
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
    #[doc = "Bit 0 - Destination last request for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Destination last request for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Destination last request for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Destination last request for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Destination last transaction request write enable for channel 0"]
    #[inline(always)]
    pub fn we_ch0(&mut self) -> WE_CH0_W {
        WE_CH0_W { w: self }
    }
    #[doc = "Bit 9 - Destination last transaction request write enable for channel 1"]
    #[inline(always)]
    pub fn we_ch1(&mut self) -> WE_CH1_W {
        WE_CH1_W { w: self }
    }
    #[doc = "Bit 10 - Destination last transaction request write enable for channel 2"]
    #[inline(always)]
    pub fn we_ch2(&mut self) -> WE_CH2_W {
        WE_CH2_W { w: self }
    }
    #[doc = "Bit 11 - Destination last transaction request write enable for channel 3"]
    #[inline(always)]
    pub fn we_ch3(&mut self) -> WE_CH3_W {
        WE_CH3_W { w: self }
    }
    #[doc = "Bit 0 - Destination last request for channel 0"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W {
        CH0_W { w: self }
    }
    #[doc = "Bit 1 - Destination last request for channel 1"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W {
        CH1_W { w: self }
    }
    #[doc = "Bit 2 - Destination last request for channel 2"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W {
        CH2_W { w: self }
    }
    #[doc = "Bit 3 - Destination last request for channel 3"]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W {
        CH3_W { w: self }
    }
}
