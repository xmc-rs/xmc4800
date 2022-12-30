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
#[doc = "Field `CH0RUN` reader - Channel 0 Run Control"]
pub type CH0RUN_R = crate::BitReader<CH0RUN_A>;
#[doc = "Channel 0 Run Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CH0RUN_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `CH0RUN` writer - Channel 0 Run Control"]
pub type CH0RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLOBRC_SPEC, CH0RUN_A, O>;
impl<'a, const O: u8> CH0RUN_W<'a, O> {
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
}
#[doc = "Field `CH1RUN` reader - Channel 1 Run Control"]
pub type CH1RUN_R = crate::BitReader<CH1RUN_A>;
#[doc = "Channel 1 Run Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CH1RUN_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `CH1RUN` writer - Channel 1 Run Control"]
pub type CH1RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLOBRC_SPEC, CH1RUN_A, O>;
impl<'a, const O: u8> CH1RUN_W<'a, O> {
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
}
#[doc = "Field `CH2RUN` reader - Channel 2 Run Control"]
pub type CH2RUN_R = crate::BitReader<CH2RUN_A>;
#[doc = "Channel 2 Run Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CH2RUN_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `CH2RUN` writer - Channel 2 Run Control"]
pub type CH2RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLOBRC_SPEC, CH2RUN_A, O>;
impl<'a, const O: u8> CH2RUN_W<'a, O> {
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
}
#[doc = "Field `CH3RUN` reader - Channel 3 Run Control"]
pub type CH3RUN_R = crate::BitReader<CH3RUN_A>;
#[doc = "Channel 3 Run Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CH3RUN_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `CH3RUN` writer - Channel 3 Run Control"]
pub type CH3RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLOBRC_SPEC, CH3RUN_A, O>;
impl<'a, const O: u8> CH3RUN_W<'a, O> {
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
}
impl R {
    #[doc = "Bit 0 - Channel 0 Run Control"]
    #[inline(always)]
    pub fn ch0run(&self) -> CH0RUN_R {
        CH0RUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Run Control"]
    #[inline(always)]
    pub fn ch1run(&self) -> CH1RUN_R {
        CH1RUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Run Control"]
    #[inline(always)]
    pub fn ch2run(&self) -> CH2RUN_R {
        CH2RUN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Run Control"]
    #[inline(always)]
    pub fn ch3run(&self) -> CH3RUN_R {
        CH3RUN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Run Control"]
    #[inline(always)]
    #[must_use]
    pub fn ch0run(&mut self) -> CH0RUN_W<0> {
        CH0RUN_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Run Control"]
    #[inline(always)]
    #[must_use]
    pub fn ch1run(&mut self) -> CH1RUN_W<1> {
        CH1RUN_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Run Control"]
    #[inline(always)]
    #[must_use]
    pub fn ch2run(&mut self) -> CH2RUN_W<2> {
        CH2RUN_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Run Control"]
    #[inline(always)]
    #[must_use]
    pub fn ch3run(&mut self) -> CH3RUN_W<3> {
        CH3RUN_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GLOBRC to value 0"]
impl crate::Resettable for GLOBRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
