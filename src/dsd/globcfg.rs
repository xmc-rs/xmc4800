#[doc = "Register `GLOBCFG` reader"]
pub type R = crate::R<GLOBCFG_SPEC>;
#[doc = "Register `GLOBCFG` writer"]
pub type W = crate::W<GLOBCFG_SPEC>;
#[doc = "Field `MCSEL` reader - Modulator Clock Select"]
pub type MCSEL_R = crate::FieldReader<MCSEL_A>;
#[doc = "Modulator Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCSEL_A {
    #[doc = "0: Internal clock off, no source selected"]
    VALUE1 = 0,
    #[doc = "1: fDSD"]
    VALUE2 = 1,
}
impl From<MCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MCSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCSEL_A {
    type Ux = u8;
}
impl MCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCSEL_A> {
        match self.bits {
            0 => Some(MCSEL_A::VALUE1),
            1 => Some(MCSEL_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Internal clock off, no source selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MCSEL_A::VALUE1
    }
    #[doc = "fDSD"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCSEL_A::VALUE2
    }
}
#[doc = "Field `MCSEL` writer - Modulator Clock Select"]
pub type MCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MCSEL_A>;
impl<'a, REG> MCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal clock off, no source selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MCSEL_A::VALUE1)
    }
    #[doc = "fDSD"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MCSEL_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:2 - Modulator Clock Select"]
    #[inline(always)]
    pub fn mcsel(&self) -> MCSEL_R {
        MCSEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Modulator Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn mcsel(&mut self) -> MCSEL_W<GLOBCFG_SPEC> {
        MCSEL_W::new(self, 0)
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
#[doc = "Global Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GLOBCFG_SPEC;
impl crate::RegisterSpec for GLOBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`globcfg::R`](R) reader structure"]
impl crate::Readable for GLOBCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`globcfg::W`](W) writer structure"]
impl crate::Writable for GLOBCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GLOBCFG to value 0"]
impl crate::Resettable for GLOBCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
