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
#[doc = "Field `RFEN` reader - Rectification Enable"]
pub type RFEN_R = crate::BitReader<RFEN_A>;
#[doc = "Rectification Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RFEN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == RFEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RFEN_A::VALUE2
    }
}
#[doc = "Field `RFEN` writer - Rectification Enable"]
pub type RFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RECTCFG_SPEC, RFEN_A, O>;
impl<'a, const O: u8> RFEN_W<'a, O> {
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
}
#[doc = "Field `SSRC` reader - Sign Source"]
pub type SSRC_R = crate::FieldReader<u8, SSRC_A>;
#[doc = "Sign Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SSRC_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SSRC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SSRC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SSRC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SSRC_A::VALUE4
    }
}
#[doc = "Field `SSRC` writer - Sign Source"]
pub type SSRC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, RECTCFG_SPEC, u8, SSRC_A, 2, O>;
impl<'a, const O: u8> SSRC_W<'a, O> {
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
}
#[doc = "Field `SDVAL` reader - Valid Flag"]
pub type SDVAL_R = crate::BitReader<SDVAL_A>;
#[doc = "Valid Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SDVAL_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SDVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SDVAL_A::VALUE2
    }
}
#[doc = "Field `SGNCS` reader - Selected Carrier Sign Signal"]
pub type SGNCS_R = crate::BitReader<SGNCS_A>;
#[doc = "Selected Carrier Sign Signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SGNCS_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SGNCS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SGNCS_A::VALUE2
    }
}
#[doc = "Field `SGND` reader - Sign Signal Delayed"]
pub type SGND_R = crate::BitReader<SGND_A>;
#[doc = "Sign Signal Delayed\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SGND_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SGND_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SGND_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Rectification Enable"]
    #[inline(always)]
    pub fn rfen(&self) -> RFEN_R {
        RFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Sign Source"]
    #[inline(always)]
    pub fn ssrc(&self) -> SSRC_R {
        SSRC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 15 - Valid Flag"]
    #[inline(always)]
    pub fn sdval(&self) -> SDVAL_R {
        SDVAL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 30 - Selected Carrier Sign Signal"]
    #[inline(always)]
    pub fn sgncs(&self) -> SGNCS_R {
        SGNCS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Sign Signal Delayed"]
    #[inline(always)]
    pub fn sgnd(&self) -> SGND_R {
        SGND_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rectification Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfen(&mut self) -> RFEN_W<0> {
        RFEN_W::new(self)
    }
    #[doc = "Bits 4:5 - Sign Source"]
    #[inline(always)]
    #[must_use]
    pub fn ssrc(&mut self) -> SSRC_W<4> {
        SSRC_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RECTCFG to value 0x8000_0000"]
impl crate::Resettable for RECTCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
