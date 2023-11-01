#[doc = "Register `PRSET3` writer"]
pub type W = crate::W<PRSET3_SPEC>;
#[doc = "EBU Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EBURS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<EBURS_AW> for bool {
    #[inline(always)]
    fn from(variant: EBURS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBURS` writer - EBU Reset Assert"]
pub type EBURS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EBURS_AW>;
impl<'a, REG, const O: u8> EBURS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EBURS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EBURS_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 2 - EBU Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn eburs(&mut self) -> EBURS_W<PRSET3_SPEC, 2> {
        EBURS_W::new(self)
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
#[doc = "RCU Peripheral 3 Reset Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prset3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRSET3_SPEC;
impl crate::RegisterSpec for PRSET3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prset3::W`](W) writer structure"]
impl crate::Writable for PRSET3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSET3 to value 0"]
impl crate::Resettable for PRSET3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
