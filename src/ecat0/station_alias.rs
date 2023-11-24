#[doc = "Register `STATION_ALIAS` reader"]
pub type R = crate::R<STATION_ALIAS_SPEC>;
#[doc = "Register `STATION_ALIAS` writer"]
pub type W = crate::W<STATION_ALIAS_SPEC>;
#[doc = "Field `ALIAS_ADDR` reader - Alias Address used for node addressing(FPxx commands)"]
pub type ALIAS_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `ALIAS_ADDR` writer - Alias Address used for node addressing(FPxx commands)"]
pub type ALIAS_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Alias Address used for node addressing(FPxx commands)"]
    #[inline(always)]
    pub fn alias_addr(&self) -> ALIAS_ADDR_R {
        ALIAS_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Alias Address used for node addressing(FPxx commands)"]
    #[inline(always)]
    #[must_use]
    pub fn alias_addr(&mut self) -> ALIAS_ADDR_W<STATION_ALIAS_SPEC> {
        ALIAS_ADDR_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configured Station Alias\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`station_alias::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`station_alias::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATION_ALIAS_SPEC;
impl crate::RegisterSpec for STATION_ALIAS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`station_alias::R`](R) reader structure"]
impl crate::Readable for STATION_ALIAS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`station_alias::W`](W) writer structure"]
impl crate::Writable for STATION_ALIAS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATION_ALIAS to value 0"]
impl crate::Resettable for STATION_ALIAS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
