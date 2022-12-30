#[doc = "Register `RX_256TO511OCTETS_FRAMES_GOOD_BAD` reader"]
pub struct R(crate::R<RX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX256_511OCTGB` reader - This field indicates the number of received good and bad frames with length between 256 and 511 (inclusive) bytes, exclusive of preamble."]
pub type RX256_511OCTGB_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames with length between 256 and 511 (inclusive) bytes, exclusive of preamble."]
    #[inline(always)]
    pub fn rx256_511octgb(&self) -> RX256_511OCTGB_R {
        RX256_511OCTGB_R::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Good and Bad 256 to 511 Bytes Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_256to511octets_frames_good_bad](index.html) module"]
pub struct RX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC;
impl crate::RegisterSpec for RX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_256to511octets_frames_good_bad::R](R) reader structure"]
impl crate::Readable for RX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_256TO511OCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for RX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
