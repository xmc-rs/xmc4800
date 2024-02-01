#[doc = "Register `BOUNDSEL` reader"]
pub type R = crate::R<BOUNDSEL_SPEC>;
#[doc = "Register `BOUNDSEL` writer"]
pub type W = crate::W<BOUNDSEL_SPEC>;
#[doc = "Field `BOUNDARYL` reader - Lower Boundary Value for Limit Checking"]
pub type BOUNDARYL_R = crate::FieldReader<u16>;
#[doc = "Field `BOUNDARYL` writer - Lower Boundary Value for Limit Checking"]
pub type BOUNDARYL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BOUNDARYU` reader - Upper Boundary Value for Limit Checking"]
pub type BOUNDARYU_R = crate::FieldReader<u16>;
#[doc = "Field `BOUNDARYU` writer - Upper Boundary Value for Limit Checking"]
pub type BOUNDARYU_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Lower Boundary Value for Limit Checking"]
    #[inline(always)]
    pub fn boundaryl(&self) -> BOUNDARYL_R {
        BOUNDARYL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Upper Boundary Value for Limit Checking"]
    #[inline(always)]
    pub fn boundaryu(&self) -> BOUNDARYU_R {
        BOUNDARYU_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower Boundary Value for Limit Checking"]
    #[inline(always)]
    #[must_use]
    pub fn boundaryl(&mut self) -> BOUNDARYL_W<BOUNDSEL_SPEC> {
        BOUNDARYL_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Upper Boundary Value for Limit Checking"]
    #[inline(always)]
    #[must_use]
    pub fn boundaryu(&mut self) -> BOUNDARYU_W<BOUNDSEL_SPEC> {
        BOUNDARYU_W::new(self, 16)
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
#[doc = "Boundary Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boundsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boundsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOUNDSEL_SPEC;
impl crate::RegisterSpec for BOUNDSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boundsel::R`](R) reader structure"]
impl crate::Readable for BOUNDSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`boundsel::W`](W) writer structure"]
impl crate::Writable for BOUNDSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOUNDSEL to value 0"]
impl crate::Resettable for BOUNDSEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
