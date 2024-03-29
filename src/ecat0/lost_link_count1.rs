#[doc = "Register `LOST_LINK_COUNT1` reader"]
pub type R = crate::R<LostLinkCount1Spec>;
#[doc = "Field `LL_COUNTER` reader - Lost Link counter of Port p"]
pub type LlCounterR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Lost Link counter of Port p"]
    #[inline(always)]
    pub fn ll_counter(&self) -> LlCounterR {
        LlCounterR::new(self.bits)
    }
}
#[doc = "Lost Link Counter Port 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lost_link_count1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LostLinkCount1Spec;
impl crate::RegisterSpec for LostLinkCount1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lost_link_count1::R`](R) reader structure"]
impl crate::Readable for LostLinkCount1Spec {}
#[doc = "`reset()` method sets LOST_LINK_COUNT1 to value 0"]
impl crate::Resettable for LostLinkCount1Spec {
    const RESET_VALUE: u8 = 0;
}
