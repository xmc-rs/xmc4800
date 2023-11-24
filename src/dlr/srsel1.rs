#[doc = "Register `SRSEL1` reader"]
pub type R = crate::R<SRSEL1_SPEC>;
#[doc = "Register `SRSEL1` writer"]
pub type W = crate::W<SRSEL1_SPEC>;
#[doc = "Field `RS8` reader - Request Source for Line 8"]
pub type RS8_R = crate::FieldReader;
#[doc = "Field `RS8` writer - Request Source for Line 8"]
pub type RS8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RS9` reader - Request Source for Line 9"]
pub type RS9_R = crate::FieldReader;
#[doc = "Field `RS9` writer - Request Source for Line 9"]
pub type RS9_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RS10` reader - Request Source for Line 10"]
pub type RS10_R = crate::FieldReader;
#[doc = "Field `RS10` writer - Request Source for Line 10"]
pub type RS10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RS11` reader - Request Source for Line 11"]
pub type RS11_R = crate::FieldReader;
#[doc = "Field `RS11` writer - Request Source for Line 11"]
pub type RS11_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Request Source for Line 8"]
    #[inline(always)]
    pub fn rs8(&self) -> RS8_R {
        RS8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Request Source for Line 9"]
    #[inline(always)]
    pub fn rs9(&self) -> RS9_R {
        RS9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Request Source for Line 10"]
    #[inline(always)]
    pub fn rs10(&self) -> RS10_R {
        RS10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Request Source for Line 11"]
    #[inline(always)]
    pub fn rs11(&self) -> RS11_R {
        RS11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Request Source for Line 8"]
    #[inline(always)]
    #[must_use]
    pub fn rs8(&mut self) -> RS8_W<SRSEL1_SPEC> {
        RS8_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Request Source for Line 9"]
    #[inline(always)]
    #[must_use]
    pub fn rs9(&mut self) -> RS9_W<SRSEL1_SPEC> {
        RS9_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Request Source for Line 10"]
    #[inline(always)]
    #[must_use]
    pub fn rs10(&mut self) -> RS10_W<SRSEL1_SPEC> {
        RS10_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Request Source for Line 11"]
    #[inline(always)]
    #[must_use]
    pub fn rs11(&mut self) -> RS11_W<SRSEL1_SPEC> {
        RS11_W::new(self, 12)
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
#[doc = "Service Request Selection 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRSEL1_SPEC;
impl crate::RegisterSpec for SRSEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srsel1::R`](R) reader structure"]
impl crate::Readable for SRSEL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srsel1::W`](W) writer structure"]
impl crate::Writable for SRSEL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRSEL1 to value 0"]
impl crate::Resettable for SRSEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
