#[doc = "Register `WUB` reader"]
pub type R = crate::R<WUB_SPEC>;
#[doc = "Register `WUB` writer"]
pub type W = crate::W<WUB_SPEC>;
#[doc = "Field `WUB` reader - Window Upper Bound"]
pub type WUB_R = crate::FieldReader<u32>;
#[doc = "Field `WUB` writer - Window Upper Bound"]
pub type WUB_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Window Upper Bound"]
    #[inline(always)]
    pub fn wub(&self) -> WUB_R {
        WUB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Window Upper Bound"]
    #[inline(always)]
    #[must_use]
    pub fn wub(&mut self) -> WUB_W<WUB_SPEC> {
        WUB_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "WDT Window Upper Bound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wub::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wub::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WUB_SPEC;
impl crate::RegisterSpec for WUB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wub::R`](R) reader structure"]
impl crate::Readable for WUB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wub::W`](W) writer structure"]
impl crate::Writable for WUB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WUB to value 0xffff_ffff"]
impl crate::Resettable for WUB_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
