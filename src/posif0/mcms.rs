#[doc = "Register `MCMS` writer"]
pub type W = crate::W<McmsSpec>;
#[doc = "Field `MNPS` writer - Multi-Channel Pattern Update Enable Set"]
pub type MnpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STHR` writer - Hall Pattern Shadow Transfer Request"]
pub type SthrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STMR` writer - Multi-Channel Shadow Transfer Request"]
pub type StmrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Multi-Channel Pattern Update Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn mnps(&mut self) -> MnpsW<McmsSpec> {
        MnpsW::new(self, 0)
    }
    #[doc = "Bit 1 - Hall Pattern Shadow Transfer Request"]
    #[inline(always)]
    #[must_use]
    pub fn sthr(&mut self) -> SthrW<McmsSpec> {
        SthrW::new(self, 1)
    }
    #[doc = "Bit 2 - Multi-Channel Shadow Transfer Request"]
    #[inline(always)]
    #[must_use]
    pub fn stmr(&mut self) -> StmrW<McmsSpec> {
        StmrW::new(self, 2)
    }
}
#[doc = "Multi-Channel Pattern Control set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcms::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McmsSpec;
impl crate::RegisterSpec for McmsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mcms::W`](W) writer structure"]
impl crate::Writable for McmsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCMS to value 0"]
impl crate::Resettable for McmsSpec {
    const RESET_VALUE: u32 = 0;
}
