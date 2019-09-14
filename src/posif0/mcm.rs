#[doc = "Reader of register MCM"]
pub type R = crate::R<u32, super::MCM>;
#[doc = "Reader of field `MCMP`"]
pub type MCMP_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Multi-Channel Pattern"]
    #[inline(always)]
    pub fn mcmp(&self) -> MCMP_R {
        MCMP_R::new((self.bits & 0xffff) as u16)
    }
}
