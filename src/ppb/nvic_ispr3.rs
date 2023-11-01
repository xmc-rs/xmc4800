#[doc = "Register `NVIC_ISPR3` reader"]
pub type R = crate::R<NVIC_ISPR3_SPEC>;
#[doc = "Register `NVIC_ISPR3` writer"]
pub type W = crate::W<NVIC_ISPR3_SPEC>;
#[doc = "Field `SETPEND` reader - Interrupt set-pending bits."]
pub type SETPEND_R = crate::FieldReader<SETPEND_A>;
#[doc = "Interrupt set-pending bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SETPEND_A {
    #[doc = "0: interrupt is not pending"]
    VALUE3 = 0,
    #[doc = "1: interrupt is pending."]
    VALUE4 = 1,
}
impl From<SETPEND_A> for u32 {
    #[inline(always)]
    fn from(variant: SETPEND_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SETPEND_A {
    type Ux = u32;
}
impl SETPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SETPEND_A> {
        match self.bits {
            0 => Some(SETPEND_A::VALUE3),
            1 => Some(SETPEND_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "interrupt is not pending"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SETPEND_A::VALUE3
    }
    #[doc = "interrupt is pending."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SETPEND_A::VALUE4
    }
}
#[doc = "Field `SETPEND` writer - Interrupt set-pending bits."]
pub type SETPEND_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, SETPEND_A>;
impl<'a, REG, const O: u8> SETPEND_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "interrupt is not pending"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(SETPEND_A::VALUE3)
    }
    #[doc = "interrupt is pending."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(SETPEND_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt set-pending bits."]
    #[inline(always)]
    pub fn setpend(&self) -> SETPEND_R {
        SETPEND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt set-pending bits."]
    #[inline(always)]
    #[must_use]
    pub fn setpend(&mut self) -> SETPEND_W<NVIC_ISPR3_SPEC, 0> {
        SETPEND_W::new(self)
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
#[doc = "Interrupt Set-pending Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ispr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ispr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_ISPR3_SPEC;
impl crate::RegisterSpec for NVIC_ISPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ispr3::R`](R) reader structure"]
impl crate::Readable for NVIC_ISPR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_ispr3::W`](W) writer structure"]
impl crate::Writable for NVIC_ISPR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_ISPR3 to value 0"]
impl crate::Resettable for NVIC_ISPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
