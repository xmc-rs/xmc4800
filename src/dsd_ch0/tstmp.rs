#[doc = "Register `TSTMP` reader"]
pub type R = crate::R<TstmpSpec>;
#[doc = "Field `RESULT` reader - Result of most recent conversion"]
pub type ResultR = crate::FieldReader<u16>;
#[doc = "Field `CFMDCNT` reader - CIC Filter (Main Chain) Decimation Counter"]
pub type CfmdcntR = crate::FieldReader;
#[doc = "Field `NVALCNT` reader - Number of Values Counted"]
pub type NvalcntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Result of most recent conversion"]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - CIC Filter (Main Chain) Decimation Counter"]
    #[inline(always)]
    pub fn cfmdcnt(&self) -> CfmdcntR {
        CfmdcntR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:29 - Number of Values Counted"]
    #[inline(always)]
    pub fn nvalcnt(&self) -> NvalcntR {
        NvalcntR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[doc = "Time-Stamp Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tstmp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TstmpSpec;
impl crate::RegisterSpec for TstmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tstmp::R`](R) reader structure"]
impl crate::Readable for TstmpSpec {}
#[doc = "`reset()` method sets TSTMP to value 0"]
impl crate::Resettable for TstmpSpec {
    const RESET_VALUE: u32 = 0;
}
