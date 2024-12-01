#[doc = "Register `OFFM` reader"]
pub type R = crate::R<OFFM_SPEC>;
#[doc = "Register `OFFM` writer"]
pub type W = crate::W<OFFM_SPEC>;
#[doc = "Field `OFFSET` reader - Offset Value"]
pub type OFFSET_R = crate::FieldReader<u16>;
#[doc = "Field `OFFSET` writer - Offset Value"]
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Offset Value"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Offset Value"]
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W<OFFM_SPEC> {
        OFFSET_W::new(self, 0)
    }
}
#[doc = "Offset Register, Main Filter\n\nYou can [`read`](crate::Reg::read) this register and get [`offm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OFFM_SPEC;
impl crate::RegisterSpec for OFFM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`offm::R`](R) reader structure"]
impl crate::Readable for OFFM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`offm::W`](W) writer structure"]
impl crate::Writable for OFFM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OFFM to value 0"]
impl crate::Resettable for OFFM_SPEC {
    const RESET_VALUE: u32 = 0;
}
