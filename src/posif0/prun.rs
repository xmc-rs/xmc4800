#[doc = "Register `PRUN` reader"]
pub type R = crate::R<PRUN_SPEC>;
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
    pub const fn variant(&self) -> RB_A {
        match self.bits {
            false => RB_A::VALUE1,
            true => RB_A::VALUE2,
        }
    }
    #[doc = "IDLE"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RB_A::VALUE1
    }
    #[doc = "Running"]
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
#[doc = "POSIF Run Bit Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prun::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRUN_SPEC;
impl crate::RegisterSpec for PRUN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prun::R`](R) reader structure"]
impl crate::Readable for PRUN_SPEC {}
#[doc = "`reset()` method sets PRUN to value 0"]
impl crate::Resettable for PRUN_SPEC {
    const RESET_VALUE: u32 = 0;
}
