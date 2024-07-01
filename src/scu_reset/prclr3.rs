#[doc = "Register `PRCLR3` writer"]
pub type W = crate::W<PRCLR3_SPEC>;
#[doc = "EBU Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EBURS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<EBURS_A> for bool {
    #[inline(always)]
    fn from(variant: EBURS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBURS` writer - EBU Reset Assert"]
pub type EBURS_W<'a, REG> = crate::BitWriter<'a, REG, EBURS_A>;
impl<'a, REG> EBURS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EBURS_A::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EBURS_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 2 - EBU Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn eburs(&mut self) -> EBURS_W<PRCLR3_SPEC> {
        EBURS_W::new(self, 2)
    }
}
#[doc = "RCU Peripheral 3 Reset Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prclr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRCLR3_SPEC;
impl crate::RegisterSpec for PRCLR3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prclr3::W`](W) writer structure"]
impl crate::Writable for PRCLR3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRCLR3 to value 0"]
impl crate::Resettable for PRCLR3_SPEC {
    const RESET_VALUE: u32 = 0;
}
