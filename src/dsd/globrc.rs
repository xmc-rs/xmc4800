#[doc = "Register `GLOBRC` reader"]
pub struct R(crate::R<GLOBRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLOBRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLOBRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLOBRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLOBRC` writer"]
pub struct W(crate::W<GLOBRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLOBRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GLOBRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLOBRC_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `CH0RUN` reader - Channel 0 Run Control"]
pub struct CH0RUN_R(crate::FieldReader<bool, CH0RUN_A>);
impl CH0RUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0RUN_R(crate::FieldReader::new(bits))
    }
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
        **self == CH0RUN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CH0RUN_A::VALUE2
    }
}
impl core::ops::Deref for CH0RUN_R {
    type Target = crate::FieldReader<bool, CH0RUN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0RUN` writer - Channel 0 Run Control"]
pub struct CH0RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0RUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0RUN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
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
#[doc = "Field `CH1RUN` reader - Channel 1 Run Control"]
pub struct CH1RUN_R(crate::FieldReader<bool, CH1RUN_A>);
impl CH1RUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1RUN_R(crate::FieldReader::new(bits))
    }
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
        **self == CH1RUN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CH1RUN_A::VALUE2
    }
}
impl core::ops::Deref for CH1RUN_R {
    type Target = crate::FieldReader<bool, CH1RUN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1RUN` writer - Channel 1 Run Control"]
pub struct CH1RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1RUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1RUN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
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
#[doc = "Field `CH2RUN` reader - Channel 2 Run Control"]
pub struct CH2RUN_R(crate::FieldReader<bool, CH2RUN_A>);
impl CH2RUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2RUN_R(crate::FieldReader::new(bits))
    }
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
        **self == CH2RUN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CH2RUN_A::VALUE2
    }
}
impl core::ops::Deref for CH2RUN_R {
    type Target = crate::FieldReader<bool, CH2RUN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2RUN` writer - Channel 2 Run Control"]
pub struct CH2RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2RUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2RUN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
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
#[doc = "Field `CH3RUN` reader - Channel 3 Run Control"]
pub struct CH3RUN_R(crate::FieldReader<bool, CH3RUN_A>);
impl CH3RUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH3RUN_R(crate::FieldReader::new(bits))
    }
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
        **self == CH3RUN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CH3RUN_A::VALUE2
    }
}
impl core::ops::Deref for CH3RUN_R {
    type Target = crate::FieldReader<bool, CH3RUN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3RUN` writer - Channel 3 Run Control"]
pub struct CH3RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3RUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3RUN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Run Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globrc](index.html) module"]
pub struct GLOBRC_SPEC;
impl crate::RegisterSpec for GLOBRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [globrc::R](R) reader structure"]
impl crate::Readable for GLOBRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [globrc::W](W) writer structure"]
impl crate::Writable for GLOBRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GLOBRC to value 0"]
impl crate::Resettable for GLOBRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
