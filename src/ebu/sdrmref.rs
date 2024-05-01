#[doc = "Register `SDRMREF` reader"]
pub type R = crate::R<SdrmrefSpec>;
#[doc = "Register `SDRMREF` writer"]
pub type W = crate::W<SdrmrefSpec>;
#[doc = "Field `REFRESHC` reader - Refresh counter period"]
pub type RefreshcR = crate::FieldReader;
#[doc = "Field `REFRESHC` writer - Refresh counter period"]
pub type RefreshcW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REFRESHR` reader - Number of refresh commands"]
pub type RefreshrR = crate::FieldReader;
#[doc = "Field `REFRESHR` writer - Number of refresh commands"]
pub type RefreshrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SELFREXST` reader - Self Refresh Exit Status."]
pub type SelfrexstR = crate::BitReader;
#[doc = "Field `SELFREX` reader - Self Refresh Exit (Power Up)."]
pub type SelfrexR = crate::BitReader;
#[doc = "Field `SELFREX` writer - Self Refresh Exit (Power Up)."]
pub type SelfrexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELFRENST` reader - Self Refresh Entry Status."]
pub type SelfrenstR = crate::BitReader;
#[doc = "Field `SELFREN` reader - Self Refresh Entry"]
pub type SelfrenR = crate::BitReader;
#[doc = "Field `SELFREN` writer - Self Refresh Entry"]
pub type SelfrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOSELFR` reader - Automatic Self Refresh"]
pub type AutoselfrR = crate::BitReader;
#[doc = "Field `AUTOSELFR` writer - Automatic Self Refresh"]
pub type AutoselfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERFSHC` reader - Extended Refresh Counter Period"]
pub type ErfshcR = crate::FieldReader;
#[doc = "Field `ERFSHC` writer - Extended Refresh Counter Period"]
pub type ErfshcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SELFREX_DLY` reader - Self Refresh Exit Delay"]
pub type SelfrexDlyR = crate::FieldReader;
#[doc = "Field `SELFREX_DLY` writer - Self Refresh Exit Delay"]
pub type SelfrexDlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ARFSH` reader - Auto Refresh on Self refresh Exit"]
pub type ArfshR = crate::BitReader;
#[doc = "Field `ARFSH` writer - Auto Refresh on Self refresh Exit"]
pub type ArfshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES_DLY` reader - Delay on Power Down Exit"]
pub type ResDlyR = crate::FieldReader;
#[doc = "Field `RES_DLY` writer - Delay on Power Down Exit"]
pub type ResDlyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5 - Refresh counter period"]
    #[inline(always)]
    pub fn refreshc(&self) -> RefreshcR {
        RefreshcR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:8 - Number of refresh commands"]
    #[inline(always)]
    pub fn refreshr(&self) -> RefreshrR {
        RefreshrR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - Self Refresh Exit Status."]
    #[inline(always)]
    pub fn selfrexst(&self) -> SelfrexstR {
        SelfrexstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Self Refresh Exit (Power Up)."]
    #[inline(always)]
    pub fn selfrex(&self) -> SelfrexR {
        SelfrexR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Self Refresh Entry Status."]
    #[inline(always)]
    pub fn selfrenst(&self) -> SelfrenstR {
        SelfrenstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Self Refresh Entry"]
    #[inline(always)]
    pub fn selfren(&self) -> SelfrenR {
        SelfrenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Automatic Self Refresh"]
    #[inline(always)]
    pub fn autoselfr(&self) -> AutoselfrR {
        AutoselfrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Extended Refresh Counter Period"]
    #[inline(always)]
    pub fn erfshc(&self) -> ErfshcR {
        ErfshcR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Self Refresh Exit Delay"]
    #[inline(always)]
    pub fn selfrex_dly(&self) -> SelfrexDlyR {
        SelfrexDlyR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Auto Refresh on Self refresh Exit"]
    #[inline(always)]
    pub fn arfsh(&self) -> ArfshR {
        ArfshR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Delay on Power Down Exit"]
    #[inline(always)]
    pub fn res_dly(&self) -> ResDlyR {
        ResDlyR::new(((self.bits >> 25) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Refresh counter period"]
    #[inline(always)]
    #[must_use]
    pub fn refreshc(&mut self) -> RefreshcW<SdrmrefSpec> {
        RefreshcW::new(self, 0)
    }
    #[doc = "Bits 6:8 - Number of refresh commands"]
    #[inline(always)]
    #[must_use]
    pub fn refreshr(&mut self) -> RefreshrW<SdrmrefSpec> {
        RefreshrW::new(self, 6)
    }
    #[doc = "Bit 10 - Self Refresh Exit (Power Up)."]
    #[inline(always)]
    #[must_use]
    pub fn selfrex(&mut self) -> SelfrexW<SdrmrefSpec> {
        SelfrexW::new(self, 10)
    }
    #[doc = "Bit 12 - Self Refresh Entry"]
    #[inline(always)]
    #[must_use]
    pub fn selfren(&mut self) -> SelfrenW<SdrmrefSpec> {
        SelfrenW::new(self, 12)
    }
    #[doc = "Bit 13 - Automatic Self Refresh"]
    #[inline(always)]
    #[must_use]
    pub fn autoselfr(&mut self) -> AutoselfrW<SdrmrefSpec> {
        AutoselfrW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Extended Refresh Counter Period"]
    #[inline(always)]
    #[must_use]
    pub fn erfshc(&mut self) -> ErfshcW<SdrmrefSpec> {
        ErfshcW::new(self, 14)
    }
    #[doc = "Bits 16:23 - Self Refresh Exit Delay"]
    #[inline(always)]
    #[must_use]
    pub fn selfrex_dly(&mut self) -> SelfrexDlyW<SdrmrefSpec> {
        SelfrexDlyW::new(self, 16)
    }
    #[doc = "Bit 24 - Auto Refresh on Self refresh Exit"]
    #[inline(always)]
    #[must_use]
    pub fn arfsh(&mut self) -> ArfshW<SdrmrefSpec> {
        ArfshW::new(self, 24)
    }
    #[doc = "Bits 25:27 - Delay on Power Down Exit"]
    #[inline(always)]
    #[must_use]
    pub fn res_dly(&mut self) -> ResDlyW<SdrmrefSpec> {
        ResDlyW::new(self, 25)
    }
}
#[doc = "EBU SDRAM Refresh Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdrmref::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdrmref::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdrmrefSpec;
impl crate::RegisterSpec for SdrmrefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdrmref::R`](R) reader structure"]
impl crate::Readable for SdrmrefSpec {}
#[doc = "`write(|w| ..)` method takes [`sdrmref::W`](W) writer structure"]
impl crate::Writable for SdrmrefSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDRMREF to value 0"]
impl crate::Resettable for SdrmrefSpec {
    const RESET_VALUE: u32 = 0;
}
