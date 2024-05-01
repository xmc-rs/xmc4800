#[doc = "Register `OFFM` reader"]
pub type R = crate::R<OffmSpec>;
#[doc = "Register `OFFM` writer"]
pub type W = crate::W<OffmSpec>;
#[doc = "Field `OFFSET` reader - Offset Value"]
pub type OffsetR = crate::FieldReader<u16>;
#[doc = "Field `OFFSET` writer - Offset Value"]
pub type OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Offset Value"]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Offset Value"]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OffsetW<OffmSpec> {
        OffsetW::new(self, 0)
    }
}
#[doc = "Offset Register, Main Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`offm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`offm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OffmSpec;
impl crate::RegisterSpec for OffmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`offm::R`](R) reader structure"]
impl crate::Readable for OffmSpec {}
#[doc = "`write(|w| ..)` method takes [`offm::W`](W) writer structure"]
impl crate::Writable for OffmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OFFM to value 0"]
impl crate::Resettable for OffmSpec {
    const RESET_VALUE: u32 = 0;
}
