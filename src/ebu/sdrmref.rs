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
#[doc = "Field `REFRESHC` reader - Refresh counter period"]
pub type REFRESHC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFRESHC` writer - Refresh counter period"]
pub type REFRESHC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRMREF_SPEC, u8, u8, 6, O>;
#[doc = "Field `REFRESHR` reader - Number of refresh commands"]
pub type REFRESHR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFRESHR` writer - Number of refresh commands"]
pub type REFRESHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRMREF_SPEC, u8, u8, 3, O>;
#[doc = "Field `SELFREXST` reader - Self Refresh Exit Status."]
pub type SELFREXST_R = crate::BitReader<bool>;
#[doc = "Field `SELFREX` reader - Self Refresh Exit (Power Up)."]
pub type SELFREX_R = crate::BitReader<bool>;
#[doc = "Field `SELFREX` writer - Self Refresh Exit (Power Up)."]
pub type SELFREX_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDRMREF_SPEC, bool, O>;
#[doc = "Field `SELFRENST` reader - Self Refresh Entry Status."]
pub type SELFRENST_R = crate::BitReader<bool>;
#[doc = "Field `SELFREN` reader - Self Refresh Entry"]
pub type SELFREN_R = crate::BitReader<bool>;
#[doc = "Field `SELFREN` writer - Self Refresh Entry"]
pub type SELFREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDRMREF_SPEC, bool, O>;
#[doc = "Field `AUTOSELFR` reader - Automatic Self Refresh"]
pub type AUTOSELFR_R = crate::BitReader<bool>;
#[doc = "Field `AUTOSELFR` writer - Automatic Self Refresh"]
pub type AUTOSELFR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDRMREF_SPEC, bool, O>;
#[doc = "Field `ERFSHC` reader - Extended Refresh Counter Period"]
pub type ERFSHC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERFSHC` writer - Extended Refresh Counter Period"]
pub type ERFSHC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRMREF_SPEC, u8, u8, 2, O>;
#[doc = "Field `SELFREX_DLY` reader - Self Refresh Exit Delay"]
pub type SELFREX_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELFREX_DLY` writer - Self Refresh Exit Delay"]
pub type SELFREX_DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRMREF_SPEC, u8, u8, 8, O>;
#[doc = "Field `ARFSH` reader - Auto Refresh on Self refresh Exit"]
pub type ARFSH_R = crate::BitReader<bool>;
#[doc = "Field `ARFSH` writer - Auto Refresh on Self refresh Exit"]
pub type ARFSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDRMREF_SPEC, bool, O>;
#[doc = "Field `RES_DLY` reader - Delay on Power Down Exit"]
pub type RES_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RES_DLY` writer - Delay on Power Down Exit"]
pub type RES_DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRMREF_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:5 - Refresh counter period"]
    #[inline(always)]
    pub fn refreshc(&self) -> REFRESHC_R {
        REFRESHC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:8 - Number of refresh commands"]
    #[inline(always)]
    pub fn refreshr(&self) -> REFRESHR_R {
        REFRESHR_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - Self Refresh Exit Status."]
    #[inline(always)]
    pub fn selfrexst(&self) -> SELFREXST_R {
        SELFREXST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Self Refresh Exit (Power Up)."]
    #[inline(always)]
    pub fn selfrex(&self) -> SELFREX_R {
        SELFREX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Self Refresh Entry Status."]
    #[inline(always)]
    pub fn selfrenst(&self) -> SELFRENST_R {
        SELFRENST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Self Refresh Entry"]
    #[inline(always)]
    pub fn selfren(&self) -> SELFREN_R {
        SELFREN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Automatic Self Refresh"]
    #[inline(always)]
    pub fn autoselfr(&self) -> AUTOSELFR_R {
        AUTOSELFR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Extended Refresh Counter Period"]
    #[inline(always)]
    pub fn erfshc(&self) -> ERFSHC_R {
        ERFSHC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Self Refresh Exit Delay"]
    #[inline(always)]
    pub fn selfrex_dly(&self) -> SELFREX_DLY_R {
        SELFREX_DLY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Auto Refresh on Self refresh Exit"]
    #[inline(always)]
    pub fn arfsh(&self) -> ARFSH_R {
        ARFSH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Delay on Power Down Exit"]
    #[inline(always)]
    pub fn res_dly(&self) -> RES_DLY_R {
        RES_DLY_R::new(((self.bits >> 25) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Refresh counter period"]
    #[inline(always)]
    #[must_use]
    pub fn refreshc(&mut self) -> REFRESHC_W<0> {
        REFRESHC_W::new(self)
    }
    #[doc = "Bits 6:8 - Number of refresh commands"]
    #[inline(always)]
    #[must_use]
    pub fn refreshr(&mut self) -> REFRESHR_W<6> {
        REFRESHR_W::new(self)
    }
    #[doc = "Bit 10 - Self Refresh Exit (Power Up)."]
    #[inline(always)]
    #[must_use]
    pub fn selfrex(&mut self) -> SELFREX_W<10> {
        SELFREX_W::new(self)
    }
    #[doc = "Bit 12 - Self Refresh Entry"]
    #[inline(always)]
    #[must_use]
    pub fn selfren(&mut self) -> SELFREN_W<12> {
        SELFREN_W::new(self)
    }
    #[doc = "Bit 13 - Automatic Self Refresh"]
    #[inline(always)]
    #[must_use]
    pub fn autoselfr(&mut self) -> AUTOSELFR_W<13> {
        AUTOSELFR_W::new(self)
    }
    #[doc = "Bits 14:15 - Extended Refresh Counter Period"]
    #[inline(always)]
    #[must_use]
    pub fn erfshc(&mut self) -> ERFSHC_W<14> {
        ERFSHC_W::new(self)
    }
    #[doc = "Bits 16:23 - Self Refresh Exit Delay"]
    #[inline(always)]
    #[must_use]
    pub fn selfrex_dly(&mut self) -> SELFREX_DLY_W<16> {
        SELFREX_DLY_W::new(self)
    }
    #[doc = "Bit 24 - Auto Refresh on Self refresh Exit"]
    #[inline(always)]
    #[must_use]
    pub fn arfsh(&mut self) -> ARFSH_W<24> {
        ARFSH_W::new(self)
    }
    #[doc = "Bits 25:27 - Delay on Power Down Exit"]
    #[inline(always)]
    #[must_use]
    pub fn res_dly(&mut self) -> RES_DLY_W<25> {
        RES_DLY_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDRMREF to value 0"]
impl crate::Resettable for SDRMREF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
