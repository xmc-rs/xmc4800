#[doc = "Register `CGATSTAT3` reader"]
pub type R = crate::R<CGATSTAT3_SPEC>;
#[doc = "EBU Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type EBU_R = crate::BitReader<EBU_A>;
impl EBU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EBU_A {
        match self.bits {
            false => EBU_A::VALUE1,
            true => EBU_A::VALUE2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EBU_A::VALUE1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EBU_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 2 - EBU Gating Status"]
    #[inline(always)]
    pub fn ebu(&self) -> EBU_R {
        EBU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Peripheral 3 Clock Gating Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgatstat3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CGATSTAT3_SPEC;
impl crate::RegisterSpec for CGATSTAT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgatstat3::R`](R) reader structure"]
impl crate::Readable for CGATSTAT3_SPEC {}
#[doc = "`reset()` method sets CGATSTAT3 to value 0"]
impl crate::Resettable for CGATSTAT3_SPEC {
    const RESET_VALUE: u32 = 0;
}
