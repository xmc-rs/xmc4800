#[doc = "Reader of register HALP"]
pub type R = crate::R<u32, super::HALP>;
#[doc = "Reader of field `HCP`"]
pub type HCP_R = crate::R<u8, u8>;
#[doc = "Reader of field `HEP`"]
pub type HEP_R = crate::R<u8, u8>;
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
