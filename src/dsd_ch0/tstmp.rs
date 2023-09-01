#[doc = "Register `TSTMP` reader"]
pub type R = crate::R<TSTMP_SPEC>;
#[doc = "Field `RESULT` reader - Result of most recent conversion"]
pub type RESULT_R = crate::FieldReader<u16>;
#[doc = "Field `CFMDCNT` reader - CIC Filter (Main Chain) Decimation Counter"]
pub type CFMDCNT_R = crate::FieldReader;
#[doc = "Field `NVALCNT` reader - Number of Values Counted"]
pub type NVALCNT_R = crate::FieldReader;
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
#[doc = "Time-Stamp Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tstmp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSTMP_SPEC;
impl crate::RegisterSpec for TSTMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tstmp::R`](R) reader structure"]
impl crate::Readable for TSTMP_SPEC {}
#[doc = "`reset()` method sets TSTMP to value 0"]
impl crate::Resettable for TSTMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
