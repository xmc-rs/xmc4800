#[doc = "Reader of register REMOTE_WAKE_UP_FRAME_FILTER"]
pub type R = crate::R<u32, super::REMOTE_WAKE_UP_FRAME_FILTER>;
#[doc = "Writer for register REMOTE_WAKE_UP_FRAME_FILTER"]
pub type W = crate::W<u32, super::REMOTE_WAKE_UP_FRAME_FILTER>;
#[doc = "Register REMOTE_WAKE_UP_FRAME_FILTER `reset()`'s with value 0"]
impl crate::ResetValue for super::REMOTE_WAKE_UP_FRAME_FILTER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WKUPFRMFTR`"]
pub type WKUPFRMFTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WKUPFRMFTR`"]
pub struct WKUPFRMFTR_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPFRMFTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Remote Wake-Up Frame Filter"]
    #[inline(always)]
    pub fn wkupfrmftr(&self) -> WKUPFRMFTR_R {
        WKUPFRMFTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Remote Wake-Up Frame Filter"]
    #[inline(always)]
    pub fn wkupfrmftr(&mut self) -> WKUPFRMFTR_W {
        WKUPFRMFTR_W { w: self }
    }
}
