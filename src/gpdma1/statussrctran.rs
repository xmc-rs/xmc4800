#[doc = "Register `STATUSSRCTRAN` reader"]
pub type R = crate::R<StatussrctranSpec>;
#[doc = "Field `CH0` reader - Interrupt Status for channel 0"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH1` reader - Interrupt Status for channel 1"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH2` reader - Interrupt Status for channel 2"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH3` reader - Interrupt Status for channel 3"]
pub type Ch3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "IntSrcTran Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statussrctran::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatussrctranSpec;
impl crate::RegisterSpec for StatussrctranSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statussrctran::R`](R) reader structure"]
impl crate::Readable for StatussrctranSpec {}
#[doc = "`reset()` method sets STATUSSRCTRAN to value 0"]
impl crate::Resettable for StatussrctranSpec {
    const RESET_VALUE: u32 = 0;
}
