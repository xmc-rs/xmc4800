#[doc = "Reader of register MCSM"]
pub type R = crate::R<u32, super::MCSM>;
#[doc = "Writer for register MCSM"]
pub type W = crate::W<u32, super::MCSM>;
#[doc = "Register MCSM `reset()`'s with value 0"]
impl crate::ResetValue for super::MCSM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCMPS`"]
pub type MCMPS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MCMPS`"]
pub struct MCMPS_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Shadow Multi-Channel Pattern"]
    #[inline(always)]
    pub fn mcmps(&self) -> MCMPS_R {
        MCMPS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow Multi-Channel Pattern"]
    #[inline(always)]
    pub fn mcmps(&mut self) -> MCMPS_W {
        MCMPS_W { w: self }
    }
}
