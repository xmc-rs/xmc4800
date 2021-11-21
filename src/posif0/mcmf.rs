#[doc = "Register `MCMF` reader"]
pub struct R(crate::R<MCMF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCMF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCMF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCMF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Multi-Channel Pattern update status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSS_A {
    #[doc = "0: Update of the Multi-Channel pattern is set"]
    VALUE1 = 0,
    #[doc = "1: Update of the Multi-Channel pattern is not set"]
    VALUE2 = 1,
}
impl From<MSS_A> for bool {
    #[inline(always)]
    fn from(variant: MSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSS` reader - Multi-Channel Pattern update status"]
pub struct MSS_R(crate::FieldReader<bool, MSS_A>);
impl MSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSS_A {
        match self.bits {
            false => MSS_A::VALUE1,
            true => MSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MSS_A::VALUE2
    }
}
impl core::ops::Deref for MSS_R {
    type Target = crate::FieldReader<bool, MSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Multi-Channel Pattern update status"]
    #[inline(always)]
    pub fn mss(&self) -> MSS_R {
        MSS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Multi-Channel Pattern Control flag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcmf](index.html) module"]
pub struct MCMF_SPEC;
impl crate::RegisterSpec for MCMF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcmf::R](R) reader structure"]
impl crate::Readable for MCMF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MCMF to value 0"]
impl crate::Resettable for MCMF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
