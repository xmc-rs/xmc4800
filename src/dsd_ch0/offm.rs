#[doc = "Reader of register OFFM"]
pub type R = crate::R<u32, super::OFFM>;
#[doc = "Writer for register OFFM"]
pub type W = crate::W<u32, super::OFFM>;
#[doc = "Register OFFM `reset()`'s with value 0"]
impl crate::ResetValue for super::OFFM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OFFSET`"]
pub type OFFSET_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OFFSET`"]
pub struct OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Offset Value"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Offset Value"]
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W {
        OFFSET_W { w: self }
    }
}
