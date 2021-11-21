#[doc = "Register `EVFLAGCLR` writer"]
pub struct W(crate::W<EVFLAGCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVFLAGCLR_SPEC>;
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
impl From<crate::W<EVFLAGCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVFLAGCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Result Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESEC0_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit RESEVx"]
    VALUE2 = 1,
}
impl From<RESEC0_AW> for bool {
    #[inline(always)]
    fn from(variant: RESEC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEC0` writer - Result Event Clear"]
pub struct RESEC0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESEC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESEC0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESEC0_AW::VALUE1)
    }
    #[doc = "Clear bit RESEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESEC0_AW::VALUE2)
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
#[doc = "Result Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESEC1_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit RESEVx"]
    VALUE2 = 1,
}
impl From<RESEC1_AW> for bool {
    #[inline(always)]
    fn from(variant: RESEC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEC1` writer - Result Event Clear"]
pub struct RESEC1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESEC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESEC1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESEC1_AW::VALUE1)
    }
    #[doc = "Clear bit RESEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESEC1_AW::VALUE2)
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
#[doc = "Result Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESEC2_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit RESEVx"]
    VALUE2 = 1,
}
impl From<RESEC2_AW> for bool {
    #[inline(always)]
    fn from(variant: RESEC2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEC2` writer - Result Event Clear"]
pub struct RESEC2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESEC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESEC2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESEC2_AW::VALUE1)
    }
    #[doc = "Clear bit RESEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESEC2_AW::VALUE2)
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
#[doc = "Result Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESEC3_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit RESEVx"]
    VALUE2 = 1,
}
impl From<RESEC3_AW> for bool {
    #[inline(always)]
    fn from(variant: RESEC3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEC3` writer - Result Event Clear"]
pub struct RESEC3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESEC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESEC3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESEC3_AW::VALUE1)
    }
    #[doc = "Clear bit RESEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESEC3_AW::VALUE2)
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
#[doc = "Alarm Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEC0_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit ALEVx"]
    VALUE2 = 1,
}
impl From<ALEC0_AW> for bool {
    #[inline(always)]
    fn from(variant: ALEC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEC0` writer - Alarm Event Clear"]
pub struct ALEC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ALEC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALEC0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEC0_AW::VALUE1)
    }
    #[doc = "Clear bit ALEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEC0_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Alarm Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEC1_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit ALEVx"]
    VALUE2 = 1,
}
impl From<ALEC1_AW> for bool {
    #[inline(always)]
    fn from(variant: ALEC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEC1` writer - Alarm Event Clear"]
pub struct ALEC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ALEC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALEC1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEC1_AW::VALUE1)
    }
    #[doc = "Clear bit ALEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEC1_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Alarm Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEC2_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit ALEVx"]
    VALUE2 = 1,
}
impl From<ALEC2_AW> for bool {
    #[inline(always)]
    fn from(variant: ALEC2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEC2` writer - Alarm Event Clear"]
pub struct ALEC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ALEC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALEC2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEC2_AW::VALUE1)
    }
    #[doc = "Clear bit ALEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEC2_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Alarm Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEC3_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit ALEVx"]
    VALUE2 = 1,
}
impl From<ALEC3_AW> for bool {
    #[inline(always)]
    fn from(variant: ALEC3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEC3` writer - Alarm Event Clear"]
pub struct ALEC3_W<'a> {
    w: &'a mut W,
}
impl<'a> ALEC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALEC3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEC3_AW::VALUE1)
    }
    #[doc = "Clear bit ALEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEC3_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Result Event Clear"]
    #[inline(always)]
    pub fn resec0(&mut self) -> RESEC0_W {
        RESEC0_W { w: self }
    }
    #[doc = "Bit 1 - Result Event Clear"]
    #[inline(always)]
    pub fn resec1(&mut self) -> RESEC1_W {
        RESEC1_W { w: self }
    }
    #[doc = "Bit 2 - Result Event Clear"]
    #[inline(always)]
    pub fn resec2(&mut self) -> RESEC2_W {
        RESEC2_W { w: self }
    }
    #[doc = "Bit 3 - Result Event Clear"]
    #[inline(always)]
    pub fn resec3(&mut self) -> RESEC3_W {
        RESEC3_W { w: self }
    }
    #[doc = "Bit 16 - Alarm Event Clear"]
    #[inline(always)]
    pub fn alec0(&mut self) -> ALEC0_W {
        ALEC0_W { w: self }
    }
    #[doc = "Bit 17 - Alarm Event Clear"]
    #[inline(always)]
    pub fn alec1(&mut self) -> ALEC1_W {
        ALEC1_W { w: self }
    }
    #[doc = "Bit 18 - Alarm Event Clear"]
    #[inline(always)]
    pub fn alec2(&mut self) -> ALEC2_W {
        ALEC2_W { w: self }
    }
    #[doc = "Bit 19 - Alarm Event Clear"]
    #[inline(always)]
    pub fn alec3(&mut self) -> ALEC3_W {
        ALEC3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evflagclr](index.html) module"]
pub struct EVFLAGCLR_SPEC;
impl crate::RegisterSpec for EVFLAGCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [evflagclr::W](W) writer structure"]
impl crate::Writable for EVFLAGCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVFLAGCLR to value 0"]
impl crate::Resettable for EVFLAGCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
