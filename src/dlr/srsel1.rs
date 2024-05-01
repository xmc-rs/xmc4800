#[doc = "Register `SRSEL1` reader"]
pub type R = crate::R<Srsel1Spec>;
#[doc = "Register `SRSEL1` writer"]
pub type W = crate::W<Srsel1Spec>;
#[doc = "Field `RS8` reader - Request Source for Line 8"]
pub type Rs8R = crate::FieldReader;
#[doc = "Field `RS8` writer - Request Source for Line 8"]
pub type Rs8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RS9` reader - Request Source for Line 9"]
pub type Rs9R = crate::FieldReader;
#[doc = "Field `RS9` writer - Request Source for Line 9"]
pub type Rs9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RS10` reader - Request Source for Line 10"]
pub type Rs10R = crate::FieldReader;
#[doc = "Field `RS10` writer - Request Source for Line 10"]
pub type Rs10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RS11` reader - Request Source for Line 11"]
pub type Rs11R = crate::FieldReader;
#[doc = "Field `RS11` writer - Request Source for Line 11"]
pub type Rs11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Request Source for Line 8"]
    #[inline(always)]
    pub fn rs8(&self) -> Rs8R {
        Rs8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Request Source for Line 9"]
    #[inline(always)]
    pub fn rs9(&self) -> Rs9R {
        Rs9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Request Source for Line 10"]
    #[inline(always)]
    pub fn rs10(&self) -> Rs10R {
        Rs10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Request Source for Line 11"]
    #[inline(always)]
    pub fn rs11(&self) -> Rs11R {
        Rs11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Request Source for Line 8"]
    #[inline(always)]
    #[must_use]
    pub fn rs8(&mut self) -> Rs8W<Srsel1Spec> {
        Rs8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Request Source for Line 9"]
    #[inline(always)]
    #[must_use]
    pub fn rs9(&mut self) -> Rs9W<Srsel1Spec> {
        Rs9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Request Source for Line 10"]
    #[inline(always)]
    #[must_use]
    pub fn rs10(&mut self) -> Rs10W<Srsel1Spec> {
        Rs10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Request Source for Line 11"]
    #[inline(always)]
    #[must_use]
    pub fn rs11(&mut self) -> Rs11W<Srsel1Spec> {
        Rs11W::new(self, 12)
    }
}
#[doc = "Service Request Selection 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srsel1Spec;
impl crate::RegisterSpec for Srsel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srsel1::R`](R) reader structure"]
impl crate::Readable for Srsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`srsel1::W`](W) writer structure"]
impl crate::Writable for Srsel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRSEL1 to value 0"]
impl crate::Resettable for Srsel1Spec {
    const RESET_VALUE: u32 = 0;
}
