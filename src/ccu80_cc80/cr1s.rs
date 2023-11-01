#[doc = "Register `CR1S` reader"]
pub type R = crate::R<CR1S_SPEC>;
#[doc = "Register `CR1S` writer"]
pub type W = crate::W<CR1S_SPEC>;
#[doc = "Field `CR1S` reader - Shadow Compare Register for Channel 1"]
pub type CR1S_R = crate::FieldReader<u16>;
#[doc = "Field `CR1S` writer - Shadow Compare Register for Channel 1"]
pub type CR1S_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Shadow Compare Register for Channel 1"]
    #[inline(always)]
    pub fn cr1s(&self) -> CR1S_R {
        CR1S_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow Compare Register for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr1s(&mut self) -> CR1S_W<CR1S_SPEC, 0> {
        CR1S_W::new(self)
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
#[doc = "Channel 1 Compare Shadow Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1S_SPEC;
impl crate::RegisterSpec for CR1S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1s::R`](R) reader structure"]
impl crate::Readable for CR1S_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1s::W`](W) writer structure"]
impl crate::Writable for CR1S_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1S to value 0"]
impl crate::Resettable for CR1S_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
