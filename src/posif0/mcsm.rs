#[doc = "Register `MCSM` reader"]
pub type R = crate::R<McsmSpec>;
#[doc = "Register `MCSM` writer"]
pub type W = crate::W<McsmSpec>;
#[doc = "Field `MCMPS` reader - Shadow Multi-Channel Pattern"]
pub type McmpsR = crate::FieldReader<u16>;
#[doc = "Field `MCMPS` writer - Shadow Multi-Channel Pattern"]
pub type McmpsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shadow Multi-Channel Pattern"]
    #[inline(always)]
    pub fn mcmps(&self) -> McmpsR {
        McmpsR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow Multi-Channel Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn mcmps(&mut self) -> McmpsW<McsmSpec> {
        McmpsW::new(self, 0)
    }
}
#[doc = "Multi-Channel Shadow Pattern\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcsm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcsm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McsmSpec;
impl crate::RegisterSpec for McsmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcsm::R`](R) reader structure"]
impl crate::Readable for McsmSpec {}
#[doc = "`write(|w| ..)` method takes [`mcsm::W`](W) writer structure"]
impl crate::Writable for McsmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCSM to value 0"]
impl crate::Resettable for McsmSpec {
    const RESET_VALUE: u32 = 0;
}
