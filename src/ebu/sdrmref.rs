#[doc = "Register `SDRMREF` reader"]
pub type R = crate::R<SDRMREF_SPEC>;
#[doc = "Register `SDRMREF` writer"]
pub type W = crate::W<SDRMREF_SPEC>;
#[doc = "Field `REFRESHC` reader - Refresh counter period"]
pub type REFRESHC_R = crate::FieldReader;
#[doc = "Field `REFRESHC` writer - Refresh counter period"]
pub type REFRESHC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REFRESHR` reader - Number of refresh commands"]
pub type REFRESHR_R = crate::FieldReader;
#[doc = "Field `REFRESHR` writer - Number of refresh commands"]
pub type REFRESHR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SELFREXST` reader - Self Refresh Exit Status."]
pub type SELFREXST_R = crate::BitReader;
#[doc = "Field `SELFREX` reader - Self Refresh Exit (Power Up)."]
pub type SELFREX_R = crate::BitReader;
#[doc = "Field `SELFREX` writer - Self Refresh Exit (Power Up)."]
pub type SELFREX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELFRENST` reader - Self Refresh Entry Status."]
pub type SELFRENST_R = crate::BitReader;
#[doc = "Field `SELFREN` reader - Self Refresh Entry"]
pub type SELFREN_R = crate::BitReader;
#[doc = "Field `SELFREN` writer - Self Refresh Entry"]
pub type SELFREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOSELFR` reader - Automatic Self Refresh"]
pub type AUTOSELFR_R = crate::BitReader;
#[doc = "Field `AUTOSELFR` writer - Automatic Self Refresh"]
pub type AUTOSELFR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERFSHC` reader - Extended Refresh Counter Period"]
pub type ERFSHC_R = crate::FieldReader;
#[doc = "Field `ERFSHC` writer - Extended Refresh Counter Period"]
pub type ERFSHC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SELFREX_DLY` reader - Self Refresh Exit Delay"]
pub type SELFREX_DLY_R = crate::FieldReader;
#[doc = "Field `SELFREX_DLY` writer - Self Refresh Exit Delay"]
pub type SELFREX_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ARFSH` reader - Auto Refresh on Self refresh Exit"]
pub type ARFSH_R = crate::BitReader;
#[doc = "Field `ARFSH` writer - Auto Refresh on Self refresh Exit"]
pub type ARFSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES_DLY` reader - Delay on Power Down Exit"]
pub type RES_DLY_R = crate::FieldReader;
#[doc = "Field `RES_DLY` writer - Delay on Power Down Exit"]
pub type RES_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
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
    pub fn refreshc(&mut self) -> REFRESHC_W<SDRMREF_SPEC> {
        REFRESHC_W::new(self, 0)
    }
    #[doc = "Bits 6:8 - Number of refresh commands"]
    #[inline(always)]
    #[must_use]
    pub fn refreshr(&mut self) -> REFRESHR_W<SDRMREF_SPEC> {
        REFRESHR_W::new(self, 6)
    }
    #[doc = "Bit 10 - Self Refresh Exit (Power Up)."]
    #[inline(always)]
    #[must_use]
    pub fn selfrex(&mut self) -> SELFREX_W<SDRMREF_SPEC> {
        SELFREX_W::new(self, 10)
    }
    #[doc = "Bit 12 - Self Refresh Entry"]
    #[inline(always)]
    #[must_use]
    pub fn selfren(&mut self) -> SELFREN_W<SDRMREF_SPEC> {
        SELFREN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Automatic Self Refresh"]
    #[inline(always)]
    #[must_use]
    pub fn autoselfr(&mut self) -> AUTOSELFR_W<SDRMREF_SPEC> {
        AUTOSELFR_W::new(self, 13)
    }
    #[doc = "Bits 14:15 - Extended Refresh Counter Period"]
    #[inline(always)]
    #[must_use]
    pub fn erfshc(&mut self) -> ERFSHC_W<SDRMREF_SPEC> {
        ERFSHC_W::new(self, 14)
    }
    #[doc = "Bits 16:23 - Self Refresh Exit Delay"]
    #[inline(always)]
    #[must_use]
    pub fn selfrex_dly(&mut self) -> SELFREX_DLY_W<SDRMREF_SPEC> {
        SELFREX_DLY_W::new(self, 16)
    }
    #[doc = "Bit 24 - Auto Refresh on Self refresh Exit"]
    #[inline(always)]
    #[must_use]
    pub fn arfsh(&mut self) -> ARFSH_W<SDRMREF_SPEC> {
        ARFSH_W::new(self, 24)
    }
    #[doc = "Bits 25:27 - Delay on Power Down Exit"]
    #[inline(always)]
    #[must_use]
    pub fn res_dly(&mut self) -> RES_DLY_W<SDRMREF_SPEC> {
        RES_DLY_W::new(self, 25)
    }
}
#[doc = "EBU SDRAM Refresh Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdrmref::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdrmref::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDRMREF_SPEC;
impl crate::RegisterSpec for SDRMREF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdrmref::R`](R) reader structure"]
impl crate::Readable for SDRMREF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdrmref::W`](W) writer structure"]
impl crate::Writable for SDRMREF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDRMREF to value 0"]
impl crate::Resettable for SDRMREF_SPEC {
    const RESET_VALUE: u32 = 0;
}
