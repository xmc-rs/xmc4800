#[doc = "Reader of register EBUCLKCR"]
pub type R = crate::R<u32, super::EBUCLKCR>;
#[doc = "Writer for register EBUCLKCR"]
pub type W = crate::W<u32, super::EBUCLKCR>;
#[doc = "Register EBUCLKCR `reset()`'s with value 0"]
impl crate::ResetValue for super::EBUCLKCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EBUDIV`"]
pub type EBUDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EBUDIV`"]
pub struct EBUDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> EBUDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - EBU Clock Divider Value"]
    #[inline(always)]
    pub fn ebudiv(&self) -> EBUDIV_R {
        EBUDIV_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - EBU Clock Divider Value"]
    #[inline(always)]
    pub fn ebudiv(&mut self) -> EBUDIV_W {
        EBUDIV_W { w: self }
    }
}
