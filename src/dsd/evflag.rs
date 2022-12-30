#[doc = "Register `EVFLAG` reader"]
pub struct R(crate::R<EVFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVFLAG` writer"]
pub struct W(crate::W<EVFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVFLAG_SPEC>;
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
impl From<crate::W<EVFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESEV0` reader - Result Event"]
pub type RESEV0_R = crate::BitReader<RESEV0_A>;
#[doc = "Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESEV0_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: A new result has been stored in register RESMx"]
    VALUE2 = 1,
}
impl From<RESEV0_A> for bool {
    #[inline(always)]
    fn from(variant: RESEV0_A) -> Self {
        variant as u8 != 0
    }
}
impl RESEV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESEV0_A {
        match self.bits {
            false => RESEV0_A::VALUE1,
            true => RESEV0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESEV0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESEV0_A::VALUE2
    }
}
#[doc = "Field `RESEV0` writer - Result Event"]
pub type RESEV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, RESEV0_A, O>;
impl<'a, const O: u8> RESEV0_W<'a, O> {
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESEV0_A::VALUE1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESEV0_A::VALUE2)
    }
}
#[doc = "Field `RESEV1` reader - Result Event"]
pub type RESEV1_R = crate::BitReader<RESEV1_A>;
#[doc = "Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESEV1_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: A new result has been stored in register RESMx"]
    VALUE2 = 1,
}
impl From<RESEV1_A> for bool {
    #[inline(always)]
    fn from(variant: RESEV1_A) -> Self {
        variant as u8 != 0
    }
}
impl RESEV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESEV1_A {
        match self.bits {
            false => RESEV1_A::VALUE1,
            true => RESEV1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESEV1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESEV1_A::VALUE2
    }
}
#[doc = "Field `RESEV1` writer - Result Event"]
pub type RESEV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, RESEV1_A, O>;
impl<'a, const O: u8> RESEV1_W<'a, O> {
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESEV1_A::VALUE1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESEV1_A::VALUE2)
    }
}
#[doc = "Field `RESEV2` reader - Result Event"]
pub type RESEV2_R = crate::BitReader<RESEV2_A>;
#[doc = "Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESEV2_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: A new result has been stored in register RESMx"]
    VALUE2 = 1,
}
impl From<RESEV2_A> for bool {
    #[inline(always)]
    fn from(variant: RESEV2_A) -> Self {
        variant as u8 != 0
    }
}
impl RESEV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESEV2_A {
        match self.bits {
            false => RESEV2_A::VALUE1,
            true => RESEV2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESEV2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESEV2_A::VALUE2
    }
}
#[doc = "Field `RESEV2` writer - Result Event"]
pub type RESEV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, RESEV2_A, O>;
impl<'a, const O: u8> RESEV2_W<'a, O> {
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESEV2_A::VALUE1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESEV2_A::VALUE2)
    }
}
#[doc = "Field `RESEV3` reader - Result Event"]
pub type RESEV3_R = crate::BitReader<RESEV3_A>;
#[doc = "Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESEV3_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: A new result has been stored in register RESMx"]
    VALUE2 = 1,
}
impl From<RESEV3_A> for bool {
    #[inline(always)]
    fn from(variant: RESEV3_A) -> Self {
        variant as u8 != 0
    }
}
impl RESEV3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESEV3_A {
        match self.bits {
            false => RESEV3_A::VALUE1,
            true => RESEV3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESEV3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESEV3_A::VALUE2
    }
}
#[doc = "Field `RESEV3` writer - Result Event"]
pub type RESEV3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, RESEV3_A, O>;
impl<'a, const O: u8> RESEV3_W<'a, O> {
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESEV3_A::VALUE1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESEV3_A::VALUE2)
    }
}
#[doc = "Field `ALEV0` reader - Alarm Event"]
pub type ALEV0_R = crate::BitReader<ALEV0_A>;
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALEV0_A {
    #[doc = "0: No alarm event"]
    VALUE1 = 0,
    #[doc = "1: An alarm event has occurred"]
    VALUE2 = 1,
}
impl From<ALEV0_A> for bool {
    #[inline(always)]
    fn from(variant: ALEV0_A) -> Self {
        variant as u8 != 0
    }
}
impl ALEV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALEV0_A {
        match self.bits {
            false => ALEV0_A::VALUE1,
            true => ALEV0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV0_A::VALUE2
    }
}
#[doc = "Field `ALEV0` writer - Alarm Event"]
pub type ALEV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, ALEV0_A, O>;
impl<'a, const O: u8> ALEV0_W<'a, O> {
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV0_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV0_A::VALUE2)
    }
}
#[doc = "Field `ALEV1` reader - Alarm Event"]
pub type ALEV1_R = crate::BitReader<ALEV1_A>;
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALEV1_A {
    #[doc = "0: No alarm event"]
    VALUE1 = 0,
    #[doc = "1: An alarm event has occurred"]
    VALUE2 = 1,
}
impl From<ALEV1_A> for bool {
    #[inline(always)]
    fn from(variant: ALEV1_A) -> Self {
        variant as u8 != 0
    }
}
impl ALEV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALEV1_A {
        match self.bits {
            false => ALEV1_A::VALUE1,
            true => ALEV1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV1_A::VALUE2
    }
}
#[doc = "Field `ALEV1` writer - Alarm Event"]
pub type ALEV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, ALEV1_A, O>;
impl<'a, const O: u8> ALEV1_W<'a, O> {
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV1_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV1_A::VALUE2)
    }
}
#[doc = "Field `ALEV2` reader - Alarm Event"]
pub type ALEV2_R = crate::BitReader<ALEV2_A>;
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALEV2_A {
    #[doc = "0: No alarm event"]
    VALUE1 = 0,
    #[doc = "1: An alarm event has occurred"]
    VALUE2 = 1,
}
impl From<ALEV2_A> for bool {
    #[inline(always)]
    fn from(variant: ALEV2_A) -> Self {
        variant as u8 != 0
    }
}
impl ALEV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALEV2_A {
        match self.bits {
            false => ALEV2_A::VALUE1,
            true => ALEV2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV2_A::VALUE2
    }
}
#[doc = "Field `ALEV2` writer - Alarm Event"]
pub type ALEV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, ALEV2_A, O>;
impl<'a, const O: u8> ALEV2_W<'a, O> {
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV2_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV2_A::VALUE2)
    }
}
#[doc = "Field `ALEV3` reader - Alarm Event"]
pub type ALEV3_R = crate::BitReader<ALEV3_A>;
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALEV3_A {
    #[doc = "0: No alarm event"]
    VALUE1 = 0,
    #[doc = "1: An alarm event has occurred"]
    VALUE2 = 1,
}
impl From<ALEV3_A> for bool {
    #[inline(always)]
    fn from(variant: ALEV3_A) -> Self {
        variant as u8 != 0
    }
}
impl ALEV3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALEV3_A {
        match self.bits {
            false => ALEV3_A::VALUE1,
            true => ALEV3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV3_A::VALUE2
    }
}
#[doc = "Field `ALEV3` writer - Alarm Event"]
pub type ALEV3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, ALEV3_A, O>;
impl<'a, const O: u8> ALEV3_W<'a, O> {
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV3_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV3_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Result Event"]
    #[inline(always)]
    pub fn resev0(&self) -> RESEV0_R {
        RESEV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Result Event"]
    #[inline(always)]
    pub fn resev1(&self) -> RESEV1_R {
        RESEV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Result Event"]
    #[inline(always)]
    pub fn resev2(&self) -> RESEV2_R {
        RESEV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Result Event"]
    #[inline(always)]
    pub fn resev3(&self) -> RESEV3_R {
        RESEV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Alarm Event"]
    #[inline(always)]
    pub fn alev0(&self) -> ALEV0_R {
        ALEV0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Alarm Event"]
    #[inline(always)]
    pub fn alev1(&self) -> ALEV1_R {
        ALEV1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Alarm Event"]
    #[inline(always)]
    pub fn alev2(&self) -> ALEV2_R {
        ALEV2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Alarm Event"]
    #[inline(always)]
    pub fn alev3(&self) -> ALEV3_R {
        ALEV3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Result Event"]
    #[inline(always)]
    #[must_use]
    pub fn resev0(&mut self) -> RESEV0_W<0> {
        RESEV0_W::new(self)
    }
    #[doc = "Bit 1 - Result Event"]
    #[inline(always)]
    #[must_use]
    pub fn resev1(&mut self) -> RESEV1_W<1> {
        RESEV1_W::new(self)
    }
    #[doc = "Bit 2 - Result Event"]
    #[inline(always)]
    #[must_use]
    pub fn resev2(&mut self) -> RESEV2_W<2> {
        RESEV2_W::new(self)
    }
    #[doc = "Bit 3 - Result Event"]
    #[inline(always)]
    #[must_use]
    pub fn resev3(&mut self) -> RESEV3_W<3> {
        RESEV3_W::new(self)
    }
    #[doc = "Bit 16 - Alarm Event"]
    #[inline(always)]
    #[must_use]
    pub fn alev0(&mut self) -> ALEV0_W<16> {
        ALEV0_W::new(self)
    }
    #[doc = "Bit 17 - Alarm Event"]
    #[inline(always)]
    #[must_use]
    pub fn alev1(&mut self) -> ALEV1_W<17> {
        ALEV1_W::new(self)
    }
    #[doc = "Bit 18 - Alarm Event"]
    #[inline(always)]
    #[must_use]
    pub fn alev2(&mut self) -> ALEV2_W<18> {
        ALEV2_W::new(self)
    }
    #[doc = "Bit 19 - Alarm Event"]
    #[inline(always)]
    #[must_use]
    pub fn alev3(&mut self) -> ALEV3_W<19> {
        ALEV3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evflag](index.html) module"]
pub struct EVFLAG_SPEC;
impl crate::RegisterSpec for EVFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evflag::R](R) reader structure"]
impl crate::Readable for EVFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evflag::W](W) writer structure"]
impl crate::Writable for EVFLAG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVFLAG to value 0"]
impl crate::Resettable for EVFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
