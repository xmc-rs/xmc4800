#[doc = "Register `HALP` reader"]
pub struct R(crate::R<HALP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HALP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HALP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HALP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HCP` reader - Hall Current Pattern"]
pub struct HCP_R(crate::FieldReader<u8, u8>);
impl HCP_R {
    pub(crate) fn new(bits: u8) -> Self {
        HCP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HEP` reader - Hall Expected Pattern"]
pub struct HEP_R(crate::FieldReader<u8, u8>);
impl HEP_R {
    pub(crate) fn new(bits: u8) -> Self {
        HEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HEP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Hall Current Pattern"]
    #[inline(always)]
    pub fn hcp(&self) -> HCP_R {
        HCP_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Hall Expected Pattern"]
    #[inline(always)]
    pub fn hep(&self) -> HEP_R {
        HEP_R::new(((self.bits >> 3) & 0x07) as u8)
    }
}
#[doc = "Hall Sensor Patterns\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [halp](index.html) module"]
pub struct HALP_SPEC;
impl crate::RegisterSpec for HALP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [halp::R](R) reader structure"]
impl crate::Readable for HALP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HALP to value 0"]
impl crate::Resettable for HALP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
