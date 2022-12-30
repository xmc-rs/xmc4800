#[doc = "Register `STATUSBLOCK` reader"]
pub struct R(crate::R<STATUSBLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSBLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSBLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSBLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH0` reader - Interrupt Status for channel 0"]
pub type CH0_R = crate::BitReader<bool>;
#[doc = "Field `CH1` reader - Interrupt Status for channel 1"]
pub type CH1_R = crate::BitReader<bool>;
#[doc = "Field `CH2` reader - Interrupt Status for channel 2"]
pub type CH2_R = crate::BitReader<bool>;
#[doc = "Field `CH3` reader - Interrupt Status for channel 3"]
pub type CH3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Interrupt Status for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "IntBlock Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusblock](index.html) module"]
pub struct STATUSBLOCK_SPEC;
impl crate::RegisterSpec for STATUSBLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statusblock::R](R) reader structure"]
impl crate::Readable for STATUSBLOCK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSBLOCK to value 0"]
impl crate::Resettable for STATUSBLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
