#[doc = "Register `PSUS` reader"]
pub struct R(crate::R<PSUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSUS` writer"]
pub struct W(crate::W<PSUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSUS_SPEC>;
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
impl From<crate::W<PSUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QSUS` reader - Quadrature Mode Suspend Config"]
pub type QSUS_R = crate::FieldReader<u8, QSUS_A>;
#[doc = "Quadrature Mode Suspend Config\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QSUS_A {
    #[doc = "0: Suspend request ignored"]
    VALUE1 = 0,
    #[doc = "1: Stop immediately"]
    VALUE2 = 1,
    #[doc = "2: Suspend in the next index occurrence"]
    VALUE3 = 2,
    #[doc = "3: Suspend in the next phase (PhaseA or PhaseB) occurrence"]
    VALUE4 = 3,
}
impl From<QSUS_A> for u8 {
    #[inline(always)]
    fn from(variant: QSUS_A) -> Self {
        variant as _
    }
}
impl QSUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QSUS_A {
        match self.bits {
            0 => QSUS_A::VALUE1,
            1 => QSUS_A::VALUE2,
            2 => QSUS_A::VALUE3,
            3 => QSUS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == QSUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == QSUS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == QSUS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == QSUS_A::VALUE4
    }
}
#[doc = "Field `QSUS` writer - Quadrature Mode Suspend Config"]
pub type QSUS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PSUS_SPEC, u8, QSUS_A, 2, O>;
impl<'a, const O: u8> QSUS_W<'a, O> {
    #[doc = "Suspend request ignored"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(QSUS_A::VALUE1)
    }
    #[doc = "Stop immediately"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(QSUS_A::VALUE2)
    }
    #[doc = "Suspend in the next index occurrence"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(QSUS_A::VALUE3)
    }
    #[doc = "Suspend in the next phase (PhaseA or PhaseB) occurrence"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(QSUS_A::VALUE4)
    }
}
#[doc = "Field `MSUS` reader - Multi-Channel Mode Suspend Config"]
pub type MSUS_R = crate::FieldReader<u8, MSUS_A>;
#[doc = "Multi-Channel Mode Suspend Config\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSUS_A {
    #[doc = "0: Suspend request ignored"]
    VALUE1 = 0,
    #[doc = "1: Stop immediately. Multi-Channel pattern is not set to the reset value."]
    VALUE2 = 1,
    #[doc = "2: Stop immediately. Multi-Channel pattern is set to the reset value."]
    VALUE3 = 2,
    #[doc = "3: Suspend with the synchronization of the PWM signal. Multi-Channel pattern is set to the reset value at the same time of the synchronization."]
    VALUE4 = 3,
}
impl From<MSUS_A> for u8 {
    #[inline(always)]
    fn from(variant: MSUS_A) -> Self {
        variant as _
    }
}
impl MSUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSUS_A {
        match self.bits {
            0 => MSUS_A::VALUE1,
            1 => MSUS_A::VALUE2,
            2 => MSUS_A::VALUE3,
            3 => MSUS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MSUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSUS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MSUS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MSUS_A::VALUE4
    }
}
#[doc = "Field `MSUS` writer - Multi-Channel Mode Suspend Config"]
pub type MSUS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PSUS_SPEC, u8, MSUS_A, 2, O>;
impl<'a, const O: u8> MSUS_W<'a, O> {
    #[doc = "Suspend request ignored"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSUS_A::VALUE1)
    }
    #[doc = "Stop immediately. Multi-Channel pattern is not set to the reset value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSUS_A::VALUE2)
    }
    #[doc = "Stop immediately. Multi-Channel pattern is set to the reset value."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(MSUS_A::VALUE3)
    }
    #[doc = "Suspend with the synchronization of the PWM signal. Multi-Channel pattern is set to the reset value at the same time of the synchronization."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(MSUS_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Quadrature Mode Suspend Config"]
    #[inline(always)]
    pub fn qsus(&self) -> QSUS_R {
        QSUS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Multi-Channel Mode Suspend Config"]
    #[inline(always)]
    pub fn msus(&self) -> MSUS_R {
        MSUS_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Quadrature Mode Suspend Config"]
    #[inline(always)]
    #[must_use]
    pub fn qsus(&mut self) -> QSUS_W<0> {
        QSUS_W::new(self)
    }
    #[doc = "Bits 2:3 - Multi-Channel Mode Suspend Config"]
    #[inline(always)]
    #[must_use]
    pub fn msus(&mut self) -> MSUS_W<2> {
        MSUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "POSIF Suspend Config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psus](index.html) module"]
pub struct PSUS_SPEC;
impl crate::RegisterSpec for PSUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psus::R](R) reader structure"]
impl crate::Readable for PSUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psus::W](W) writer structure"]
impl crate::Writable for PSUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSUS to value 0"]
impl crate::Resettable for PSUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
