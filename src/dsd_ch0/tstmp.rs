#[doc = "Reader of register TSTMP"]
pub type R = crate::R<u32, super::TSTMP>;
#[doc = "Reader of field `RESULT`"]
pub type RESULT_R = crate::R<u16, u16>;
#[doc = "Reader of field `CFMDCNT`"]
pub type CFMDCNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `NVALCNT`"]
pub type NVALCNT_R = crate::R<u8, u8>;
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
