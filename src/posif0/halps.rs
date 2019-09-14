#[doc = "Reader of register HALPS"]
pub type R = crate::R<u32, super::HALPS>;
#[doc = "Writer for register HALPS"]
pub type W = crate::W<u32, super::HALPS>;
#[doc = "Register HALPS `reset()`'s with value 0"]
impl crate::ResetValue for super::HALPS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HCPS`"]
pub type HCPS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HCPS`"]
pub struct HCPS_W<'a> {
    w: &'a mut W,
}
impl<'a> HCPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `HEPS`"]
pub type HEPS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HEPS`"]
pub struct HEPS_W<'a> {
    w: &'a mut W,
}
impl<'a> HEPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Shadow Hall Current Pattern"]
    #[inline(always)]
    pub fn hcps(&self) -> HCPS_R {
        HCPS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Shadow Hall expected Pattern"]
    #[inline(always)]
    pub fn heps(&self) -> HEPS_R {
        HEPS_R::new(((self.bits >> 3) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Shadow Hall Current Pattern"]
    #[inline(always)]
    pub fn hcps(&mut self) -> HCPS_W {
        HCPS_W { w: self }
    }
    #[doc = "Bits 3:5 - Shadow Hall expected Pattern"]
    #[inline(always)]
    pub fn heps(&mut self) -> HEPS_W {
        HEPS_W { w: self }
    }
}
