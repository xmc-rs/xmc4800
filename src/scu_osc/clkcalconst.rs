#[doc = "Reader of register CLKCALCONST"]
pub type R = crate::R<u32, super::CLKCALCONST>;
#[doc = "Writer for register CLKCALCONST"]
pub type W = crate::W<u32, super::CLKCALCONST>;
#[doc = "Register CLKCALCONST `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKCALCONST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALIBCONST`"]
pub type CALIBCONST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CALIBCONST`"]
pub struct CALIBCONST_W<'a> {
    w: &'a mut W,
}
impl<'a> CALIBCONST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Clock Calibration Constant Value"]
    #[inline(always)]
    pub fn calibconst(&self) -> CALIBCONST_R {
        CALIBCONST_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Calibration Constant Value"]
    #[inline(always)]
    pub fn calibconst(&mut self) -> CALIBCONST_W {
        CALIBCONST_W { w: self }
    }
}
