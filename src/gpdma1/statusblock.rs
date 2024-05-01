#[doc = "Register `STATUSBLOCK` reader"]
pub type R = crate::R<StatusblockSpec>;
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
#[doc = "IntBlock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusblock::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusblockSpec;
impl crate::RegisterSpec for StatusblockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusblock::R`](R) reader structure"]
impl crate::Readable for StatusblockSpec {}
#[doc = "`reset()` method sets STATUSBLOCK to value 0"]
impl crate::Resettable for StatusblockSpec {
    const RESET_VALUE: u32 = 0;
}
