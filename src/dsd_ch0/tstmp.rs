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
pub type RESULT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CFMDCNT` reader - CIC Filter (Main Chain) Decimation Counter"]
pub type CFMDCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NVALCNT` reader - Number of Values Counted"]
pub type NVALCNT_R = crate::FieldReader<u8, u8>;
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
    const RESET_VALUE: Self::Ux = 0;
}
