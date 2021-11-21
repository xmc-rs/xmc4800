#[doc = "Register `MCM` reader"]
pub struct R(crate::R<MCM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MCMP` reader - Multi-Channel Pattern"]
pub struct MCMP_R(crate::FieldReader<u16, u16>);
impl MCMP_R {
    pub(crate) fn new(bits: u16) -> Self {
        MCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCMP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Multi-Channel Pattern"]
    #[inline(always)]
    pub fn mcmp(&self) -> MCMP_R {
        MCMP_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Multi-Channel Pattern\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcm](index.html) module"]
pub struct MCM_SPEC;
impl crate::RegisterSpec for MCM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcm::R](R) reader structure"]
impl crate::Readable for MCM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MCM to value 0"]
impl crate::Resettable for MCM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
