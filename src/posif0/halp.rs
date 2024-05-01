#[doc = "Register `HALP` reader"]
pub type R = crate::R<HalpSpec>;
#[doc = "Field `HCP` reader - Hall Current Pattern"]
pub type HcpR = crate::FieldReader;
#[doc = "Field `HEP` reader - Hall Expected Pattern"]
pub type HepR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Hall Current Pattern"]
    #[inline(always)]
    pub fn hcp(&self) -> HcpR {
        HcpR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Hall Expected Pattern"]
    #[inline(always)]
    pub fn hep(&self) -> HepR {
        HepR::new(((self.bits >> 3) & 7) as u8)
    }
}
#[doc = "Hall Sensor Patterns\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`halp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HalpSpec;
impl crate::RegisterSpec for HalpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`halp::R`](R) reader structure"]
impl crate::Readable for HalpSpec {}
#[doc = "`reset()` method sets HALP to value 0"]
impl crate::Resettable for HalpSpec {
    const RESET_VALUE: u32 = 0;
}
