#[doc = "Register `CGATSET3` writer"]
pub type W = crate::W<CGATSET3_SPEC>;
#[doc = "EBU Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EBU_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<EBU_A> for bool {
    #[inline(always)]
    fn from(variant: EBU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBU` writer - EBU Gating Set"]
pub type EBU_W<'a, REG> = crate::BitWriter<'a, REG, EBU_A>;
impl<'a, REG> EBU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EBU_A::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EBU_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 2 - EBU Gating Set"]
    #[inline(always)]
    pub fn ebu(&mut self) -> EBU_W<CGATSET3_SPEC> {
        EBU_W::new(self, 2)
    }
}
#[doc = "Peripheral 3 Clock Gating Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgatset3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CGATSET3_SPEC;
impl crate::RegisterSpec for CGATSET3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cgatset3::W`](W) writer structure"]
impl crate::Writable for CGATSET3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CGATSET3 to value 0"]
impl crate::Resettable for CGATSET3_SPEC {
    const RESET_VALUE: u32 = 0;
}
