#[doc = "Register `CGATCLR3` writer"]
pub type W = crate::W<CGATCLR3_SPEC>;
#[doc = "EBU Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EBU_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<EBU_AW> for bool {
    #[inline(always)]
    fn from(variant: EBU_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBU` writer - EBU Gating Clear"]
pub type EBU_W<'a, REG> = crate::BitWriter<'a, REG, EBU_AW>;
impl<'a, REG> EBU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EBU_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EBU_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 2 - EBU Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ebu(&mut self) -> EBU_W<CGATCLR3_SPEC> {
        EBU_W::new(self, 2)
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
#[doc = "Peripheral 3 Clock Gating Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatclr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CGATCLR3_SPEC;
impl crate::RegisterSpec for CGATCLR3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cgatclr3::W`](W) writer structure"]
impl crate::Writable for CGATCLR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CGATCLR3 to value 0"]
impl crate::Resettable for CGATCLR3_SPEC {
    const RESET_VALUE: u32 = 0;
}
