#[doc = "Register `TSTMP` reader"]
pub struct R(crate::R<TSTMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSTMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSTMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSTMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESULT` reader - Result of most recent conversion"]
pub struct RESULT_R(crate::FieldReader<u16, u16>);
impl RESULT_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESULT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFMDCNT` reader - CIC Filter (Main Chain) Decimation Counter"]
pub struct CFMDCNT_R(crate::FieldReader<u8, u8>);
impl CFMDCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFMDCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFMDCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVALCNT` reader - Number of Values Counted"]
pub struct NVALCNT_R(crate::FieldReader<u8, u8>);
impl NVALCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        NVALCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVALCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Result of most recent conversion"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - CIC Filter (Main Chain) Decimation Counter"]
    #[inline(always)]
    pub fn cfmdcnt(&self) -> CFMDCNT_R {
        CFMDCNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:29 - Number of Values Counted"]
    #[inline(always)]
    pub fn nvalcnt(&self) -> NVALCNT_R {
        NVALCNT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[doc = "Time-Stamp Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tstmp](index.html) module"]
pub struct TSTMP_SPEC;
impl crate::RegisterSpec for TSTMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tstmp::R](R) reader structure"]
impl crate::Readable for TSTMP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSTMP to value 0"]
impl crate::Resettable for TSTMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
