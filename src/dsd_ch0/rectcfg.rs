#[doc = "Register `RECTCFG` reader"]
pub struct R(crate::R<RECTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECTCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RECTCFG` writer"]
pub struct W(crate::W<RECTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RECTCFG_SPEC>;
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
impl From<crate::W<RECTCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RECTCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Rectification Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFEN_A {
    #[doc = "0: No rectification, data not altered"]
    VALUE1 = 0,
    #[doc = "1: Data are rectified according to SGND"]
    VALUE2 = 1,
}
impl From<RFEN_A> for bool {
    #[inline(always)]
    fn from(variant: RFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFEN` reader - Rectification Enable"]
pub struct RFEN_R(crate::FieldReader<bool, RFEN_A>);
impl RFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFEN_A {
        match self.bits {
            false => RFEN_A::VALUE1,
            true => RFEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RFEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RFEN_A::VALUE2
    }
}
impl core::ops::Deref for RFEN_R {
    type Target = crate::FieldReader<bool, RFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFEN` writer - Rectification Enable"]
pub struct RFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No rectification, data not altered"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RFEN_A::VALUE1)
    }
    #[doc = "Data are rectified according to SGND"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RFEN_A::VALUE2)
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
#[doc = "Sign Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSRC_A {
    #[doc = "0: On-chip carrier generator"]
    VALUE1 = 0,
    #[doc = "1: Sign of result of next channel"]
    VALUE2 = 1,
    #[doc = "2: External sign signal A"]
    VALUE3 = 2,
    #[doc = "3: External sign signal B"]
    VALUE4 = 3,
}
impl From<SSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SSRC` reader - Sign Source"]
pub struct SSRC_R(crate::FieldReader<u8, SSRC_A>);
impl SSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSRC_A {
        match self.bits {
            0 => SSRC_A::VALUE1,
            1 => SSRC_A::VALUE2,
            2 => SSRC_A::VALUE3,
            3 => SSRC_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SSRC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SSRC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SSRC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == SSRC_A::VALUE4
    }
}
impl core::ops::Deref for SSRC_R {
    type Target = crate::FieldReader<u8, SSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSRC` writer - Sign Source"]
pub struct SSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "On-chip carrier generator"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SSRC_A::VALUE1)
    }
    #[doc = "Sign of result of next channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SSRC_A::VALUE2)
    }
    #[doc = "External sign signal A"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SSRC_A::VALUE3)
    }
    #[doc = "External sign signal B"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SSRC_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Valid Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDVAL_A {
    #[doc = "0: No new result available"]
    VALUE1 = 0,
    #[doc = "1: Bitfield SDCAP has been updated with a new captured value and has not yet been read"]
    VALUE2 = 1,
}
impl From<SDVAL_A> for bool {
    #[inline(always)]
    fn from(variant: SDVAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDVAL` reader - Valid Flag"]
pub struct SDVAL_R(crate::FieldReader<bool, SDVAL_A>);
impl SDVAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDVAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDVAL_A {
        match self.bits {
            false => SDVAL_A::VALUE1,
            true => SDVAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SDVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SDVAL_A::VALUE2
    }
}
impl core::ops::Deref for SDVAL_R {
    type Target = crate::FieldReader<bool, SDVAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Selected Carrier Sign Signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SGNCS_A {
    #[doc = "0: Positive values"]
    VALUE1 = 0,
    #[doc = "1: Negative values"]
    VALUE2 = 1,
}
impl From<SGNCS_A> for bool {
    #[inline(always)]
    fn from(variant: SGNCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SGNCS` reader - Selected Carrier Sign Signal"]
pub struct SGNCS_R(crate::FieldReader<bool, SGNCS_A>);
impl SGNCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SGNCS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGNCS_A {
        match self.bits {
            false => SGNCS_A::VALUE1,
            true => SGNCS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SGNCS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SGNCS_A::VALUE2
    }
}
impl core::ops::Deref for SGNCS_R {
    type Target = crate::FieldReader<bool, SGNCS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sign Signal Delayed\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SGND_A {
    #[doc = "0: Positive values"]
    VALUE1 = 0,
    #[doc = "1: Negative values"]
    VALUE2 = 1,
}
impl From<SGND_A> for bool {
    #[inline(always)]
    fn from(variant: SGND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SGND` reader - Sign Signal Delayed"]
pub struct SGND_R(crate::FieldReader<bool, SGND_A>);
impl SGND_R {
    pub(crate) fn new(bits: bool) -> Self {
        SGND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGND_A {
        match self.bits {
            false => SGND_A::VALUE1,
            true => SGND_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SGND_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SGND_A::VALUE2
    }
}
impl core::ops::Deref for SGND_R {
    type Target = crate::FieldReader<bool, SGND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Rectification Enable"]
    #[inline(always)]
    pub fn rfen(&self) -> RFEN_R {
        RFEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Sign Source"]
    #[inline(always)]
    pub fn ssrc(&self) -> SSRC_R {
        SSRC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Valid Flag"]
    #[inline(always)]
    pub fn sdval(&self) -> SDVAL_R {
        SDVAL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Selected Carrier Sign Signal"]
    #[inline(always)]
    pub fn sgncs(&self) -> SGNCS_R {
        SGNCS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Sign Signal Delayed"]
    #[inline(always)]
    pub fn sgnd(&self) -> SGND_R {
        SGND_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rectification Enable"]
    #[inline(always)]
    pub fn rfen(&mut self) -> RFEN_W {
        RFEN_W { w: self }
    }
    #[doc = "Bits 4:5 - Sign Source"]
    #[inline(always)]
    pub fn ssrc(&mut self) -> SSRC_W {
        SSRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rectification Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rectcfg](index.html) module"]
pub struct RECTCFG_SPEC;
impl crate::RegisterSpec for RECTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rectcfg::R](R) reader structure"]
impl crate::Readable for RECTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rectcfg::W](W) writer structure"]
impl crate::Writable for RECTCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RECTCFG to value 0x8000_0000"]
impl crate::Resettable for RECTCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
