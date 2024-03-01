#[doc = "Register `CLKCALCONST` reader"]
pub type R = crate::R<ClkcalconstSpec>;
#[doc = "Register `CLKCALCONST` writer"]
pub type W = crate::W<ClkcalconstSpec>;
#[doc = "Field `CALIBCONST` reader - Clock Calibration Constant Value"]
pub type CalibconstR = crate::FieldReader;
#[doc = "Field `CALIBCONST` writer - Clock Calibration Constant Value"]
pub type CalibconstW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Clock Calibration Constant Value"]
    #[inline(always)]
    pub fn calibconst(&self) -> CalibconstR {
        CalibconstR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Calibration Constant Value"]
    #[inline(always)]
    #[must_use]
    pub fn calibconst(&mut self) -> CalibconstW<ClkcalconstSpec> {
        CalibconstW::new(self, 0)
    }
}
#[doc = "Clock Calibration Constant Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcalconst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcalconst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkcalconstSpec;
impl crate::RegisterSpec for ClkcalconstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkcalconst::R`](R) reader structure"]
impl crate::Readable for ClkcalconstSpec {}
#[doc = "`write(|w| ..)` method takes [`clkcalconst::W`](W) writer structure"]
impl crate::Writable for ClkcalconstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCALCONST to value 0"]
impl crate::Resettable for ClkcalconstSpec {
    const RESET_VALUE: u32 = 0;
}
