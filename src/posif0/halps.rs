#[doc = "Register `HALPS` reader"]
pub type R = crate::R<HalpsSpec>;
#[doc = "Register `HALPS` writer"]
pub type W = crate::W<HalpsSpec>;
#[doc = "Field `HCPS` reader - Shadow Hall Current Pattern"]
pub type HcpsR = crate::FieldReader;
#[doc = "Field `HCPS` writer - Shadow Hall Current Pattern"]
pub type HcpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HEPS` reader - Shadow Hall expected Pattern"]
pub type HepsR = crate::FieldReader;
#[doc = "Field `HEPS` writer - Shadow Hall expected Pattern"]
pub type HepsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Shadow Hall Current Pattern"]
    #[inline(always)]
    pub fn hcps(&self) -> HcpsR {
        HcpsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Shadow Hall expected Pattern"]
    #[inline(always)]
    pub fn heps(&self) -> HepsR {
        HepsR::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Shadow Hall Current Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn hcps(&mut self) -> HcpsW<HalpsSpec> {
        HcpsW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Shadow Hall expected Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn heps(&mut self) -> HepsW<HalpsSpec> {
        HepsW::new(self, 3)
    }
}
#[doc = "Hall Sensor Shadow Patterns\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`halps::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`halps::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HalpsSpec;
impl crate::RegisterSpec for HalpsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`halps::R`](R) reader structure"]
impl crate::Readable for HalpsSpec {}
#[doc = "`write(|w| ..)` method takes [`halps::W`](W) writer structure"]
impl crate::Writable for HalpsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HALPS to value 0"]
impl crate::Resettable for HalpsSpec {
    const RESET_VALUE: u32 = 0;
}
