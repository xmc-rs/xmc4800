#[doc = "Register `EBUCLKCR` reader"]
pub type R = crate::R<EbuclkcrSpec>;
#[doc = "Register `EBUCLKCR` writer"]
pub type W = crate::W<EbuclkcrSpec>;
#[doc = "Field `EBUDIV` reader - EBU Clock Divider Value"]
pub type EbudivR = crate::FieldReader;
#[doc = "Field `EBUDIV` writer - EBU Clock Divider Value"]
pub type EbudivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - EBU Clock Divider Value"]
    #[inline(always)]
    pub fn ebudiv(&self) -> EbudivR {
        EbudivR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - EBU Clock Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn ebudiv(&mut self) -> EbudivW<EbuclkcrSpec> {
        EbudivW::new(self, 0)
    }
}
#[doc = "EBU Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ebuclkcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ebuclkcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EbuclkcrSpec;
impl crate::RegisterSpec for EbuclkcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ebuclkcr::R`](R) reader structure"]
impl crate::Readable for EbuclkcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ebuclkcr::W`](W) writer structure"]
impl crate::Writable for EbuclkcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EBUCLKCR to value 0"]
impl crate::Resettable for EbuclkcrSpec {
    const RESET_VALUE: u32 = 0;
}
