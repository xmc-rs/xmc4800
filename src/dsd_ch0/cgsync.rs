#[doc = "Reader of register CGSYNC"]
pub type R = crate::R<u32, super::CGSYNC>;
#[doc = "Writer for register CGSYNC"]
pub type W = crate::W<u32, super::CGSYNC>;
#[doc = "Register CGSYNC `reset()`'s with value 0"]
impl crate::ResetValue for super::CGSYNC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDCOUNT`"]
pub type SDCOUNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `SDCAP`"]
pub type SDCAP_R = crate::R<u8, u8>;
#[doc = "Reader of field `SDPOS`"]
pub type SDPOS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDPOS`"]
pub struct SDPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> SDPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SDNEG`"]
pub type SDNEG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDNEG`"]
pub struct SDNEG_W<'a> {
    w: &'a mut W,
}
impl<'a> SDNEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Sign Delay Counter"]
    #[inline(always)]
    pub fn sdcount(&self) -> SDCOUNT_R {
        SDCOUNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sign Delay Capture Value"]
    #[inline(always)]
    pub fn sdcap(&self) -> SDCAP_R {
        SDCAP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sign Delay Value for Positive Halfwave"]
    #[inline(always)]
    pub fn sdpos(&self) -> SDPOS_R {
        SDPOS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sign Delay Value for Negative Halfwave"]
    #[inline(always)]
    pub fn sdneg(&self) -> SDNEG_R {
        SDNEG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Sign Delay Value for Positive Halfwave"]
    #[inline(always)]
    pub fn sdpos(&mut self) -> SDPOS_W {
        SDPOS_W { w: self }
    }
    #[doc = "Bits 24:31 - Sign Delay Value for Negative Halfwave"]
    #[inline(always)]
    pub fn sdneg(&mut self) -> SDNEG_W {
        SDNEG_W { w: self }
    }
}
