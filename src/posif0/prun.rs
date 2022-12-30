#[doc = "Register `PRUN` reader"]
pub struct R(crate::R<PRUN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRUN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRUN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRUN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RB` reader - Run Bit"]
pub type RB_R = crate::BitReader<RB_A>;
#[doc = "Run Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RB_A {
    #[doc = "0: IDLE"]
    VALUE1 = 0,
    #[doc = "1: Running"]
    VALUE2 = 1,
}
impl From<RB_A> for bool {
    #[inline(always)]
    fn from(variant: RB_A) -> Self {
        variant as u8 != 0
    }
}
impl RB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RB_A {
        match self.bits {
            false => RB_A::VALUE1,
            true => RB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RB_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Run Bit"]
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new((self.bits & 1) != 0)
    }
}
#[doc = "POSIF Run Bit Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prun](index.html) module"]
pub struct PRUN_SPEC;
impl crate::RegisterSpec for PRUN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prun::R](R) reader structure"]
impl crate::Readable for PRUN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRUN to value 0"]
impl crate::Resettable for PRUN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
