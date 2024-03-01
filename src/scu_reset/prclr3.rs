#[doc = "Register `PRCLR3` writer"]
pub type W = crate::W<Prclr3Spec>;
#[doc = "EBU Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eburs {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: De-assert reset"]
    Value2 = 1,
}
impl From<Eburs> for bool {
    #[inline(always)]
    fn from(variant: Eburs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBURS` writer - EBU Reset Assert"]
pub type EbursW<'a, REG> = crate::BitWriter<'a, REG, Eburs>;
impl<'a, REG> EbursW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Eburs::Value1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Eburs::Value2)
    }
}
impl W {
    #[doc = "Bit 2 - EBU Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn eburs(&mut self) -> EbursW<Prclr3Spec> {
        EbursW::new(self, 2)
    }
}
#[doc = "RCU Peripheral 3 Reset Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prclr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prclr3Spec;
impl crate::RegisterSpec for Prclr3Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prclr3::W`](W) writer structure"]
impl crate::Writable for Prclr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRCLR3 to value 0"]
impl crate::Resettable for Prclr3Spec {
    const RESET_VALUE: u32 = 0;
}
