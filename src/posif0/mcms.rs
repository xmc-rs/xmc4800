#[doc = "Register `MCMS` writer"]
pub type W = crate::W<MCMS_SPEC>;
#[doc = "Field `MNPS` writer - Multi-Channel Pattern Update Enable Set"]
pub type MNPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STHR` writer - Hall Pattern Shadow Transfer Request"]
pub type STHR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STMR` writer - Multi-Channel Shadow Transfer Request"]
pub type STMR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Multi-Channel Pattern Update Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn mnps(&mut self) -> MNPS_W<MCMS_SPEC> {
        MNPS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Hall Pattern Shadow Transfer Request"]
    #[inline(always)]
    #[must_use]
    pub fn sthr(&mut self) -> STHR_W<MCMS_SPEC> {
        STHR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Multi-Channel Shadow Transfer Request"]
    #[inline(always)]
    #[must_use]
    pub fn stmr(&mut self) -> STMR_W<MCMS_SPEC> {
        STMR_W::new(self, 2)
    }
}
#[doc = "Multi-Channel Pattern Control set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcms::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCMS_SPEC;
impl crate::RegisterSpec for MCMS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mcms::W`](W) writer structure"]
impl crate::Writable for MCMS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCMS to value 0"]
impl crate::Resettable for MCMS_SPEC {
    const RESET_VALUE: u32 = 0;
}
