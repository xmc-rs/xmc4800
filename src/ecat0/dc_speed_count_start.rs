#[doc = "Register `DC_SPEED_COUNT_START` reader"]
pub type R = crate::R<DC_SPEED_COUNT_START_SPEC>;
#[doc = "Register `DC_SPEED_COUNT_START` writer"]
pub type W = crate::W<DC_SPEED_COUNT_START_SPEC>;
#[doc = "Field `COUNT_START` reader - Bandwidth for adjustment of local copy of System Time"]
pub type COUNT_START_R = crate::FieldReader<u16>;
#[doc = "Field `COUNT_START` writer - Bandwidth for adjustment of local copy of System Time"]
pub type COUNT_START_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Bandwidth for adjustment of local copy of System Time"]
    #[inline(always)]
    pub fn count_start(&self) -> COUNT_START_R {
        COUNT_START_R::new(self.bits & 0x7fff)
    }
}
impl W {
    #[doc = "Bits 0:14 - Bandwidth for adjustment of local copy of System Time"]
    #[inline(always)]
    pub fn count_start(&mut self) -> COUNT_START_W<DC_SPEED_COUNT_START_SPEC> {
        COUNT_START_W::new(self, 0)
    }
}
#[doc = "Speed Counter Start\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_speed_count_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_speed_count_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_SPEED_COUNT_START_SPEC;
impl crate::RegisterSpec for DC_SPEED_COUNT_START_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dc_speed_count_start::R`](R) reader structure"]
impl crate::Readable for DC_SPEED_COUNT_START_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dc_speed_count_start::W`](W) writer structure"]
impl crate::Writable for DC_SPEED_COUNT_START_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DC_SPEED_COUNT_START to value 0x1000"]
impl crate::Resettable for DC_SPEED_COUNT_START_SPEC {
    const RESET_VALUE: u16 = 0x1000;
}
