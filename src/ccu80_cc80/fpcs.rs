#[doc = "Reader of register FPCS"]
pub type R = crate::R<u32, super::FPCS>;
#[doc = "Writer for register FPCS"]
pub type W = crate::W<u32, super::FPCS>;
#[doc = "Register FPCS `reset()`'s with value 0"]
impl crate::ResetValue for super::FPCS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCMP`"]
pub type PCMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCMP`"]
pub struct PCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PCMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Floating Prescaler Shadow Compare Value"]
    #[inline(always)]
    pub fn pcmp(&self) -> PCMP_R {
        PCMP_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Floating Prescaler Shadow Compare Value"]
    #[inline(always)]
    pub fn pcmp(&mut self) -> PCMP_W {
        PCMP_W { w: self }
    }
}
