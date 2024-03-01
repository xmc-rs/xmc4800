#[doc = "Register `CGSYNC` reader"]
pub type R = crate::R<CgsyncSpec>;
#[doc = "Register `CGSYNC` writer"]
pub type W = crate::W<CgsyncSpec>;
#[doc = "Field `SDCOUNT` reader - Sign Delay Counter"]
pub type SdcountR = crate::FieldReader;
#[doc = "Field `SDCAP` reader - Sign Delay Capture Value"]
pub type SdcapR = crate::FieldReader;
#[doc = "Field `SDPOS` reader - Sign Delay Value for Positive Halfwave"]
pub type SdposR = crate::FieldReader;
#[doc = "Field `SDPOS` writer - Sign Delay Value for Positive Halfwave"]
pub type SdposW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SDNEG` reader - Sign Delay Value for Negative Halfwave"]
pub type SdnegR = crate::FieldReader;
#[doc = "Field `SDNEG` writer - Sign Delay Value for Negative Halfwave"]
pub type SdnegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sign Delay Counter"]
    #[inline(always)]
    pub fn sdcount(&self) -> SdcountR {
        SdcountR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sign Delay Capture Value"]
    #[inline(always)]
    pub fn sdcap(&self) -> SdcapR {
        SdcapR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sign Delay Value for Positive Halfwave"]
    #[inline(always)]
    pub fn sdpos(&self) -> SdposR {
        SdposR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sign Delay Value for Negative Halfwave"]
    #[inline(always)]
    pub fn sdneg(&self) -> SdnegR {
        SdnegR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Sign Delay Value for Positive Halfwave"]
    #[inline(always)]
    #[must_use]
    pub fn sdpos(&mut self) -> SdposW<CgsyncSpec> {
        SdposW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Sign Delay Value for Negative Halfwave"]
    #[inline(always)]
    #[must_use]
    pub fn sdneg(&mut self) -> SdnegW<CgsyncSpec> {
        SdnegW::new(self, 24)
    }
}
#[doc = "Carrier Generator Synchronization Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgsync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgsync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CgsyncSpec;
impl crate::RegisterSpec for CgsyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgsync::R`](R) reader structure"]
impl crate::Readable for CgsyncSpec {}
#[doc = "`write(|w| ..)` method takes [`cgsync::W`](W) writer structure"]
impl crate::Writable for CgsyncSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CGSYNC to value 0"]
impl crate::Resettable for CgsyncSpec {
    const RESET_VALUE: u32 = 0;
}
