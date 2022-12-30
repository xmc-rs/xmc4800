#[doc = "Register `DC_RCV_TIME_PORT1` reader"]
pub struct R(crate::R<DC_RCV_TIME_PORT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_RCV_TIME_PORT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_RCV_TIME_PORT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_RCV_TIME_PORT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOCAL_TIME_P1` reader - Local time of the beginning of a frame"]
pub type LOCAL_TIME_P1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Local time of the beginning of a frame"]
    #[inline(always)]
    pub fn local_time_p1(&self) -> LOCAL_TIME_P1_R {
        LOCAL_TIME_P1_R::new(self.bits)
    }
}
#[doc = "Receive Time Port 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_rcv_time_port1](index.html) module"]
pub struct DC_RCV_TIME_PORT1_SPEC;
impl crate::RegisterSpec for DC_RCV_TIME_PORT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_rcv_time_port1::R](R) reader structure"]
impl crate::Readable for DC_RCV_TIME_PORT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DC_RCV_TIME_PORT1 to value 0"]
impl crate::Resettable for DC_RCV_TIME_PORT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
