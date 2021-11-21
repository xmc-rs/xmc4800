#[doc = "Register `SDRMREF` reader"]
pub struct R(crate::R<SDRMREF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDRMREF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDRMREF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDRMREF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDRMREF` writer"]
pub struct W(crate::W<SDRMREF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDRMREF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SDRMREF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDRMREF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RES_DLY` reader - Delay on Power Down Exit"]
pub struct RES_DLY_R(crate::FieldReader<u8, u8>);
impl RES_DLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        RES_DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RES_DLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RES_DLY` writer - Delay on Power Down Exit"]
pub struct RES_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | ((value as u32 & 0x07) << 25);
        self.w
    }
}
#[doc = "Field `ARFSH` reader - Auto Refresh on Self refresh Exit"]
pub struct ARFSH_R(crate::FieldReader<bool, bool>);
impl ARFSH_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARFSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARFSH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARFSH` writer - Auto Refresh on Self refresh Exit"]
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `SELFREX_DLY` reader - Self Refresh Exit Delay"]
pub struct SELFREX_DLY_R(crate::FieldReader<u8, u8>);
impl SELFREX_DLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        SELFREX_DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELFREX_DLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELFREX_DLY` writer - Self Refresh Exit Delay"]
pub struct SELFREX_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> SELFREX_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `ERFSHC` reader - Extended Refresh Counter Period"]
pub struct ERFSHC_R(crate::FieldReader<u8, u8>);
impl ERFSHC_R {
    pub(crate) fn new(bits: u8) -> Self {
        ERFSHC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERFSHC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERFSHC` writer - Extended Refresh Counter Period"]
pub struct ERFSHC_W<'a> {
    w: &'a mut W,
}
impl<'a> ERFSHC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `AUTOSELFR` reader - Automatic Self Refresh"]
pub struct AUTOSELFR_R(crate::FieldReader<bool, bool>);
impl AUTOSELFR_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUTOSELFR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTOSELFR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOSELFR` writer - Automatic Self Refresh"]
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `SELFREN` reader - Self Refresh Entry"]
pub struct SELFREN_R(crate::FieldReader<bool, bool>);
impl SELFREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELFREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELFREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELFREN` writer - Self Refresh Entry"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `SELFRENST` reader - Self Refresh Entry Status."]
pub struct SELFRENST_R(crate::FieldReader<bool, bool>);
impl SELFRENST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELFRENST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELFRENST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELFREX` reader - Self Refresh Exit (Power Up)."]
pub struct SELFREX_R(crate::FieldReader<bool, bool>);
impl SELFREX_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELFREX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELFREX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELFREX` writer - Self Refresh Exit (Power Up)."]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SELFREXST` reader - Self Refresh Exit Status."]
pub struct SELFREXST_R(crate::FieldReader<bool, bool>);
impl SELFREXST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELFREXST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELFREXST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFRESHR` reader - Number of refresh commands"]
pub struct REFRESHR_R(crate::FieldReader<u8, u8>);
impl REFRESHR_R {
    pub(crate) fn new(bits: u8) -> Self {
        REFRESHR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFRESHR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFRESHR` writer - Number of refresh commands"]
pub struct REFRESHR_W<'a> {
    w: &'a mut W,
}
impl<'a> REFRESHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Field `REFRESHC` reader - Refresh counter period"]
pub struct REFRESHC_R(crate::FieldReader<u8, u8>);
impl REFRESHC_R {
    pub(crate) fn new(bits: u8) -> Self {
        REFRESHC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFRESHC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFRESHC` writer - Refresh counter period"]
pub struct REFRESHC_W<'a> {
    w: &'a mut W,
}
impl<'a> REFRESHC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBU SDRAM Refresh Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdrmref](index.html) module"]
pub struct SDRMREF_SPEC;
impl crate::RegisterSpec for SDRMREF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdrmref::R](R) reader structure"]
impl crate::Readable for SDRMREF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdrmref::W](W) writer structure"]
impl crate::Writable for SDRMREF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDRMREF to value 0"]
impl crate::Resettable for SDRMREF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
