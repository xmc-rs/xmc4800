#[doc = "Register `BLOCK_COUNT` reader"]
pub type R = crate::R<BLOCK_COUNT_SPEC>;
#[doc = "Register `BLOCK_COUNT` writer"]
pub type W = crate::W<BLOCK_COUNT_SPEC>;
#[doc = "Field `BLOCK_COUNT` reader - Blocks Count for Current Transfer"]
pub type BLOCK_COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `BLOCK_COUNT` writer - Blocks Count for Current Transfer"]
pub type BLOCK_COUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Blocks Count for Current Transfer"]
    #[inline(always)]
    pub fn block_count(&self) -> BLOCK_COUNT_R {
        BLOCK_COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Blocks Count for Current Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn block_count(&mut self) -> BLOCK_COUNT_W<BLOCK_COUNT_SPEC, 0> {
        BLOCK_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Block Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_count::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_count::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLOCK_COUNT_SPEC;
impl crate::RegisterSpec for BLOCK_COUNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`block_count::R`](R) reader structure"]
impl crate::Readable for BLOCK_COUNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`block_count::W`](W) writer structure"]
impl crate::Writable for BLOCK_COUNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLOCK_COUNT to value 0"]
impl crate::Resettable for BLOCK_COUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
