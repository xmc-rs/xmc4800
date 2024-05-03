#[doc = "Register `MCMF` reader"]
pub type R = crate::R<MCMF_SPEC>;
#[doc = "Multi-Channel Pattern update status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type MSS_R = crate::BitReader<MSS_A>;
impl MSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSS_A {
        match self.bits {
            false => MSS_A::VALUE1,
            true => MSS_A::VALUE2,
        }
    }
    #[doc = "Update of the Multi-Channel pattern is set"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MSS_A::VALUE1
    }
    #[doc = "Update of the Multi-Channel pattern is not set"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSS_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Multi-Channel Pattern update status"]
    #[inline(always)]
    pub fn mss(&self) -> MSS_R {
        MSS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Multi-Channel Pattern Control flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcmf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCMF_SPEC;
impl crate::RegisterSpec for MCMF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcmf::R`](R) reader structure"]
impl crate::Readable for MCMF_SPEC {}
#[doc = "`reset()` method sets MCMF to value 0"]
impl crate::Resettable for MCMF_SPEC {
    const RESET_VALUE: u32 = 0;
}
