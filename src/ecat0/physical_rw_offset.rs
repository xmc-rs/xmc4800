#[doc = "Register `PHYSICAL_RW_OFFSET` reader"]
pub type R = crate::R<PHYSICAL_RW_OFFSET_SPEC>;
#[doc = "Field `OFFSET` reader - Offset of R/W Commands (FPRW, APRW) between Read address and Write address"]
pub type OFFSET_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Offset of R/W Commands (FPRW, APRW) between Read address and Write address"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(self.bits)
    }
}
#[doc = "Physical Read/Write Offset\n\nYou can [`read`](crate::Reg::read) this register and get [`physical_rw_offset::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHYSICAL_RW_OFFSET_SPEC;
impl crate::RegisterSpec for PHYSICAL_RW_OFFSET_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`physical_rw_offset::R`](R) reader structure"]
impl crate::Readable for PHYSICAL_RW_OFFSET_SPEC {}
#[doc = "`reset()` method sets PHYSICAL_RW_OFFSET to value 0"]
impl crate::Resettable for PHYSICAL_RW_OFFSET_SPEC {
    const RESET_VALUE: u16 = 0;
}
