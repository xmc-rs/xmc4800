#[doc = "Register `HALP` reader"]
pub type R = crate::R<HALP_SPEC>;
#[doc = "Field `HCP` reader - Hall Current Pattern"]
pub type HCP_R = crate::FieldReader;
#[doc = "Field `HEP` reader - Hall Expected Pattern"]
pub type HEP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Hall Current Pattern"]
    #[inline(always)]
    pub fn hcp(&self) -> HCP_R {
        HCP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Hall Expected Pattern"]
    #[inline(always)]
    pub fn hep(&self) -> HEP_R {
        HEP_R::new(((self.bits >> 3) & 7) as u8)
    }
}
#[doc = "Hall Sensor Patterns\n\nYou can [`read`](crate::Reg::read) this register and get [`halp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HALP_SPEC;
impl crate::RegisterSpec for HALP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`halp::R`](R) reader structure"]
impl crate::Readable for HALP_SPEC {}
#[doc = "`reset()` method sets HALP to value 0"]
impl crate::Resettable for HALP_SPEC {
    const RESET_VALUE: u32 = 0;
}
