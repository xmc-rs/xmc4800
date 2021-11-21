#[doc = "Register `CGATSTAT3` reader"]
pub struct R(crate::R<CGATSTAT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGATSTAT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGATSTAT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGATSTAT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "EBU Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBU_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<EBU_A> for bool {
    #[inline(always)]
    fn from(variant: EBU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBU` reader - EBU Gating Status"]
pub struct EBU_R(crate::FieldReader<bool, EBU_A>);
impl EBU_R {
    pub(crate) fn new(bits: bool) -> Self {
        EBU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EBU_A {
        match self.bits {
            false => EBU_A::VALUE1,
            true => EBU_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EBU_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EBU_A::VALUE2
    }
}
impl core::ops::Deref for EBU_R {
    type Target = crate::FieldReader<bool, EBU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 2 - EBU Gating Status"]
    #[inline(always)]
    pub fn ebu(&self) -> EBU_R {
        EBU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "Peripheral 3 Clock Gating Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgatstat3](index.html) module"]
pub struct CGATSTAT3_SPEC;
impl crate::RegisterSpec for CGATSTAT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgatstat3::R](R) reader structure"]
impl crate::Readable for CGATSTAT3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CGATSTAT3 to value 0"]
impl crate::Resettable for CGATSTAT3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
