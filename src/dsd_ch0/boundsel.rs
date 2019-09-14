#[doc = "Reader of register BOUNDSEL"]
pub type R = crate::R<u32, super::BOUNDSEL>;
#[doc = "Writer for register BOUNDSEL"]
pub type W = crate::W<u32, super::BOUNDSEL>;
#[doc = "Register BOUNDSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::BOUNDSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BOUNDARYL`"]
pub type BOUNDARYL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BOUNDARYL`"]
pub struct BOUNDARYL_W<'a> {
    w: &'a mut W,
}
impl<'a> BOUNDARYL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `BOUNDARYU`"]
pub type BOUNDARYU_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BOUNDARYU`"]
pub struct BOUNDARYU_W<'a> {
    w: &'a mut W,
}
impl<'a> BOUNDARYU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Lower Boundary Value for Limit Checking"]
    #[inline(always)]
    pub fn boundaryl(&self) -> BOUNDARYL_R {
        BOUNDARYL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Upper Boundary Value for Limit Checking"]
    #[inline(always)]
    pub fn boundaryu(&self) -> BOUNDARYU_R {
        BOUNDARYU_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower Boundary Value for Limit Checking"]
    #[inline(always)]
    pub fn boundaryl(&mut self) -> BOUNDARYL_W {
        BOUNDARYL_W { w: self }
    }
    #[doc = "Bits 16:31 - Upper Boundary Value for Limit Checking"]
    #[inline(always)]
    pub fn boundaryu(&mut self) -> BOUNDARYU_W {
        BOUNDARYU_W { w: self }
    }
}
