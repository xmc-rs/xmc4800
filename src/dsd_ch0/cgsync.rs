#[doc = "Register `CGSYNC` reader"]
pub type R = crate::R<CGSYNC_SPEC>;
#[doc = "Register `CGSYNC` writer"]
pub type W = crate::W<CGSYNC_SPEC>;
#[doc = "Field `SDCOUNT` reader - Sign Delay Counter"]
pub type SDCOUNT_R = crate::FieldReader;
#[doc = "Field `SDCAP` reader - Sign Delay Capture Value"]
pub type SDCAP_R = crate::FieldReader;
#[doc = "Field `SDPOS` reader - Sign Delay Value for Positive Halfwave"]
pub type SDPOS_R = crate::FieldReader;
#[doc = "Field `SDPOS` writer - Sign Delay Value for Positive Halfwave"]
pub type SDPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SDNEG` reader - Sign Delay Value for Negative Halfwave"]
pub type SDNEG_R = crate::FieldReader;
#[doc = "Field `SDNEG` writer - Sign Delay Value for Negative Halfwave"]
pub type SDNEG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
    #[must_use]
    pub fn sdpos(&mut self) -> SDPOS_W<CGSYNC_SPEC> {
        SDPOS_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Sign Delay Value for Negative Halfwave"]
    #[inline(always)]
    #[must_use]
    pub fn sdneg(&mut self) -> SDNEG_W<CGSYNC_SPEC> {
        SDNEG_W::new(self, 24)
    }
}
#[doc = "Carrier Generator Synchronization Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgsync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgsync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CGSYNC_SPEC;
impl crate::RegisterSpec for CGSYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgsync::R`](R) reader structure"]
impl crate::Readable for CGSYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cgsync::W`](W) writer structure"]
impl crate::Writable for CGSYNC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CGSYNC to value 0"]
impl crate::Resettable for CGSYNC_SPEC {
    const RESET_VALUE: u32 = 0;
}
