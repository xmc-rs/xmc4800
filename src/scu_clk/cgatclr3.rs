#[doc = "Register `CGATCLR3` writer"]
pub type W = crate::W<Cgatclr3Spec>;
#[doc = "EBU Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ebu {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Disable gating"]
    Value2 = 1,
}
impl From<Ebu> for bool {
    #[inline(always)]
    fn from(variant: Ebu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBU` writer - EBU Gating Clear"]
pub type EbuW<'a, REG> = crate::BitWriter<'a, REG, Ebu>;
impl<'a, REG> EbuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ebu::Value1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ebu::Value2)
    }
}
impl W {
    #[doc = "Bit 2 - EBU Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ebu(&mut self) -> EbuW<Cgatclr3Spec> {
        EbuW::new(self, 2)
    }
}
#[doc = "Peripheral 3 Clock Gating Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatclr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cgatclr3Spec;
impl crate::RegisterSpec for Cgatclr3Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cgatclr3::W`](W) writer structure"]
impl crate::Writable for Cgatclr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CGATCLR3 to value 0"]
impl crate::Resettable for Cgatclr3Spec {
    const RESET_VALUE: u32 = 0;
}
