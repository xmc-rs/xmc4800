#[doc = "Register `BOUNDSEL` reader"]
pub type R = crate::R<BoundselSpec>;
#[doc = "Register `BOUNDSEL` writer"]
pub type W = crate::W<BoundselSpec>;
#[doc = "Field `BOUNDARYL` reader - Lower Boundary Value for Limit Checking"]
pub type BoundarylR = crate::FieldReader<u16>;
#[doc = "Field `BOUNDARYL` writer - Lower Boundary Value for Limit Checking"]
pub type BoundarylW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BOUNDARYU` reader - Upper Boundary Value for Limit Checking"]
pub type BoundaryuR = crate::FieldReader<u16>;
#[doc = "Field `BOUNDARYU` writer - Upper Boundary Value for Limit Checking"]
pub type BoundaryuW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Lower Boundary Value for Limit Checking"]
    #[inline(always)]
    pub fn boundaryl(&self) -> BoundarylR {
        BoundarylR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Upper Boundary Value for Limit Checking"]
    #[inline(always)]
    pub fn boundaryu(&self) -> BoundaryuR {
        BoundaryuR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower Boundary Value for Limit Checking"]
    #[inline(always)]
    #[must_use]
    pub fn boundaryl(&mut self) -> BoundarylW<BoundselSpec> {
        BoundarylW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Upper Boundary Value for Limit Checking"]
    #[inline(always)]
    #[must_use]
    pub fn boundaryu(&mut self) -> BoundaryuW<BoundselSpec> {
        BoundaryuW::new(self, 16)
    }
}
#[doc = "Boundary Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boundsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boundsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BoundselSpec;
impl crate::RegisterSpec for BoundselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boundsel::R`](R) reader structure"]
impl crate::Readable for BoundselSpec {}
#[doc = "`write(|w| ..)` method takes [`boundsel::W`](W) writer structure"]
impl crate::Writable for BoundselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOUNDSEL to value 0"]
impl crate::Resettable for BoundselSpec {
    const RESET_VALUE: u32 = 0;
}
