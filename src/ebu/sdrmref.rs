#[doc = "Reader of register SDRMREF"]
pub type R = crate::R<u32, super::SDRMREF>;
#[doc = "Writer for register SDRMREF"]
pub type W = crate::W<u32, super::SDRMREF>;
#[doc = "Register SDRMREF `reset()`'s with value 0"]
impl crate::ResetValue for super::SDRMREF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RES_DLY`"]
pub type RES_DLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RES_DLY`"]
pub struct RES_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | (((value as u32) & 0x07) << 25);
        self.w
    }
}
#[doc = "Reader of field `ARFSH`"]
pub type ARFSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARFSH`"]
pub struct ARFSH_W<'a> {
    w: &'a mut W,
}
impl<'a> ARFSH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SELFREX_DLY`"]
pub type SELFREX_DLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SELFREX_DLY`"]
pub struct SELFREX_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> SELFREX_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `ERFSHC`"]
pub type ERFSHC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ERFSHC`"]
pub struct ERFSHC_W<'a> {
    w: &'a mut W,
}
impl<'a> ERFSHC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `AUTOSELFR`"]
pub type AUTOSELFR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOSELFR`"]
pub struct AUTOSELFR_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSELFR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `SELFREN`"]
pub type SELFREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SELFREN`"]
pub struct SELFREN_W<'a> {
    w: &'a mut W,
}
impl<'a> SELFREN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SELFRENST`"]
pub type SELFRENST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SELFREX`"]
pub type SELFREX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SELFREX`"]
pub struct SELFREX_W<'a> {
    w: &'a mut W,
}
impl<'a> SELFREX_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SELFREXST`"]
pub type SELFREXST_R = crate::R<bool, bool>;
#[doc = "Reader of field `REFRESHR`"]
pub type REFRESHR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REFRESHR`"]
pub struct REFRESHR_W<'a> {
    w: &'a mut W,
}
impl<'a> REFRESHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `REFRESHC`"]
pub type REFRESHC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REFRESHC`"]
pub struct REFRESHC_W<'a> {
    w: &'a mut W,
}
impl<'a> REFRESHC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:27 - Delay on Power Down Exit"]
    #[inline(always)]
    pub fn res_dly(&self) -> RES_DLY_R {
        RES_DLY_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bit 24 - Auto Refresh on Self refresh Exit"]
    #[inline(always)]
    pub fn arfsh(&self) -> ARFSH_R {
        ARFSH_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Self Refresh Exit Delay"]
    #[inline(always)]
    pub fn selfrex_dly(&self) -> SELFREX_DLY_R {
        SELFREX_DLY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 14:15 - Extended Refresh Counter Period"]
    #[inline(always)]
    pub fn erfshc(&self) -> ERFSHC_R {
        ERFSHC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 13 - Automatic Self Refresh"]
    #[inline(always)]
    pub fn autoselfr(&self) -> AUTOSELFR_R {
        AUTOSELFR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Self Refresh Entry"]
    #[inline(always)]
    pub fn selfren(&self) -> SELFREN_R {
        SELFREN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Self Refresh Entry Status."]
    #[inline(always)]
    pub fn selfrenst(&self) -> SELFRENST_R {
        SELFRENST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Self Refresh Exit (Power Up)."]
    #[inline(always)]
    pub fn selfrex(&self) -> SELFREX_R {
        SELFREX_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Self Refresh Exit Status."]
    #[inline(always)]
    pub fn selfrexst(&self) -> SELFREXST_R {
        SELFREXST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 6:8 - Number of refresh commands"]
    #[inline(always)]
    pub fn refreshr(&self) -> REFRESHR_R {
        REFRESHR_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 0:5 - Refresh counter period"]
    #[inline(always)]
    pub fn refreshc(&self) -> REFRESHC_R {
        REFRESHC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:27 - Delay on Power Down Exit"]
    #[inline(always)]
    pub fn res_dly(&mut self) -> RES_DLY_W {
        RES_DLY_W { w: self }
    }
    #[doc = "Bit 24 - Auto Refresh on Self refresh Exit"]
    #[inline(always)]
    pub fn arfsh(&mut self) -> ARFSH_W {
        ARFSH_W { w: self }
    }
    #[doc = "Bits 16:23 - Self Refresh Exit Delay"]
    #[inline(always)]
    pub fn selfrex_dly(&mut self) -> SELFREX_DLY_W {
        SELFREX_DLY_W { w: self }
    }
    #[doc = "Bits 14:15 - Extended Refresh Counter Period"]
    #[inline(always)]
    pub fn erfshc(&mut self) -> ERFSHC_W {
        ERFSHC_W { w: self }
    }
    #[doc = "Bit 13 - Automatic Self Refresh"]
    #[inline(always)]
    pub fn autoselfr(&mut self) -> AUTOSELFR_W {
        AUTOSELFR_W { w: self }
    }
    #[doc = "Bit 12 - Self Refresh Entry"]
    #[inline(always)]
    pub fn selfren(&mut self) -> SELFREN_W {
        SELFREN_W { w: self }
    }
    #[doc = "Bit 10 - Self Refresh Exit (Power Up)."]
    #[inline(always)]
    pub fn selfrex(&mut self) -> SELFREX_W {
        SELFREX_W { w: self }
    }
    #[doc = "Bits 6:8 - Number of refresh commands"]
    #[inline(always)]
    pub fn refreshr(&mut self) -> REFRESHR_W {
        REFRESHR_W { w: self }
    }
    #[doc = "Bits 0:5 - Refresh counter period"]
    #[inline(always)]
    pub fn refreshc(&mut self) -> REFRESHC_W {
        REFRESHC_W { w: self }
    }
}
