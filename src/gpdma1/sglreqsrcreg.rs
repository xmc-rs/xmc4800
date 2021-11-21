#[doc = "Register `SGLREQSRCREG` reader"]
pub struct R(crate::R<SGLREQSRCREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SGLREQSRCREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SGLREQSRCREG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SGLREQSRCREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SGLREQSRCREG` writer"]
pub struct W(crate::W<SGLREQSRCREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SGLREQSRCREG_SPEC>;
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
impl From<crate::W<SGLREQSRCREG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SGLREQSRCREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Source request write enable for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WE_CH0_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH0_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH0` writer - Source request write enable for channel 0"]
pub struct WE_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_CH0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WE_CH0_AW) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Source request write enable for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WE_CH1_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH1_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH1` writer - Source request write enable for channel 1"]
pub struct WE_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_CH1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WE_CH1_AW) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Source request write enable for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WE_CH2_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH2_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH2` writer - Source request write enable for channel 2"]
pub struct WE_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_CH2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WE_CH2_AW) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Source request write enable for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WE_CH3_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH3_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH3` writer - Source request write enable for channel 3"]
pub struct WE_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_CH3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WE_CH3_AW) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `CH0` reader - Source request for channel 0"]
pub struct CH0_R(crate::FieldReader<bool, bool>);
impl CH0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0` writer - Source request for channel 0"]
pub struct CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_W<'a> {
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
#[doc = "Field `CH1` reader - Source request for channel 1"]
pub struct CH1_R(crate::FieldReader<bool, bool>);
impl CH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1` writer - Source request for channel 1"]
pub struct CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_W<'a> {
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
#[doc = "Field `CH2` reader - Source request for channel 2"]
pub struct CH2_R(crate::FieldReader<bool, bool>);
impl CH2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2` writer - Source request for channel 2"]
pub struct CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_W<'a> {
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
#[doc = "Field `CH3` reader - Source request for channel 3"]
pub struct CH3_R(crate::FieldReader<bool, bool>);
impl CH3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3` writer - Source request for channel 3"]
pub struct CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_W<'a> {
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
    #[doc = "Bit 0 - Source request for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Source request for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Source request for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Source request for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Source request write enable for channel 0"]
    #[inline(always)]
    pub fn we_ch0(&mut self) -> WE_CH0_W {
        WE_CH0_W { w: self }
    }
    #[doc = "Bit 9 - Source request write enable for channel 1"]
    #[inline(always)]
    pub fn we_ch1(&mut self) -> WE_CH1_W {
        WE_CH1_W { w: self }
    }
    #[doc = "Bit 10 - Source request write enable for channel 2"]
    #[inline(always)]
    pub fn we_ch2(&mut self) -> WE_CH2_W {
        WE_CH2_W { w: self }
    }
    #[doc = "Bit 11 - Source request write enable for channel 3"]
    #[inline(always)]
    pub fn we_ch3(&mut self) -> WE_CH3_W {
        WE_CH3_W { w: self }
    }
    #[doc = "Bit 0 - Source request for channel 0"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W {
        CH0_W { w: self }
    }
    #[doc = "Bit 1 - Source request for channel 1"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W {
        CH1_W { w: self }
    }
    #[doc = "Bit 2 - Source request for channel 2"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W {
        CH2_W { w: self }
    }
    #[doc = "Bit 3 - Source request for channel 3"]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W {
        CH3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Single Source Transaction Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sglreqsrcreg](index.html) module"]
pub struct SGLREQSRCREG_SPEC;
impl crate::RegisterSpec for SGLREQSRCREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sglreqsrcreg::R](R) reader structure"]
impl crate::Readable for SGLREQSRCREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sglreqsrcreg::W](W) writer structure"]
impl crate::Writable for SGLREQSRCREG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SGLREQSRCREG to value 0"]
impl crate::Resettable for SGLREQSRCREG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
