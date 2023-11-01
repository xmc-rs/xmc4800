#[doc = "Register `FPCS` reader"]
pub type R = crate::R<FPCS_SPEC>;
#[doc = "Register `FPCS` writer"]
pub type W = crate::W<FPCS_SPEC>;
#[doc = "Field `PCMP` reader - Floating Prescaler Shadow Compare Value"]
pub type PCMP_R = crate::FieldReader;
#[doc = "Field `PCMP` writer - Floating Prescaler Shadow Compare Value"]
pub type PCMP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Floating Prescaler Shadow Compare Value"]
    #[inline(always)]
    pub fn pcmp(&self) -> PCMP_R {
        PCMP_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Floating Prescaler Shadow Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn pcmp(&mut self) -> PCMP_W<FPCS_SPEC, 0> {
        PCMP_W::new(self)
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
#[doc = "Floating Prescaler Shadow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPCS_SPEC;
impl crate::RegisterSpec for FPCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpcs::R`](R) reader structure"]
impl crate::Readable for FPCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fpcs::W`](W) writer structure"]
impl crate::Writable for FPCS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPCS to value 0"]
impl crate::Resettable for FPCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
