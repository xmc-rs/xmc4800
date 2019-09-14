#[doc = "Reader of register SRSEL1"]
pub type R = crate::R<u32, super::SRSEL1>;
#[doc = "Writer for register SRSEL1"]
pub type W = crate::W<u32, super::SRSEL1>;
#[doc = "Register SRSEL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SRSEL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RS8`"]
pub type RS8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RS8`"]
pub struct RS8_W<'a> {
    w: &'a mut W,
}
impl<'a> RS8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `RS9`"]
pub type RS9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RS9`"]
pub struct RS9_W<'a> {
    w: &'a mut W,
}
impl<'a> RS9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `RS10`"]
pub type RS10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RS10`"]
pub struct RS10_W<'a> {
    w: &'a mut W,
}
impl<'a> RS10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RS11`"]
pub type RS11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RS11`"]
pub struct RS11_W<'a> {
    w: &'a mut W,
}
impl<'a> RS11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Request Source for Line 8"]
    #[inline(always)]
    pub fn rs8(&self) -> RS8_R {
        RS8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Request Source for Line 9"]
    #[inline(always)]
    pub fn rs9(&self) -> RS9_R {
        RS9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Request Source for Line 10"]
    #[inline(always)]
    pub fn rs10(&self) -> RS10_R {
        RS10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Request Source for Line 11"]
    #[inline(always)]
    pub fn rs11(&self) -> RS11_R {
        RS11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Request Source for Line 8"]
    #[inline(always)]
    pub fn rs8(&mut self) -> RS8_W {
        RS8_W { w: self }
    }
    #[doc = "Bits 4:7 - Request Source for Line 9"]
    #[inline(always)]
    pub fn rs9(&mut self) -> RS9_W {
        RS9_W { w: self }
    }
    #[doc = "Bits 8:11 - Request Source for Line 10"]
    #[inline(always)]
    pub fn rs10(&mut self) -> RS10_W {
        RS10_W { w: self }
    }
    #[doc = "Bits 12:15 - Request Source for Line 11"]
    #[inline(always)]
    pub fn rs11(&mut self) -> RS11_W {
        RS11_W { w: self }
    }
}
