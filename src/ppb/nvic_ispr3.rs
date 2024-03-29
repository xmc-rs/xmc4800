#[doc = "Register `NVIC_ISPR3` reader"]
pub type R = crate::R<NvicIspr3Spec>;
#[doc = "Register `NVIC_ISPR3` writer"]
pub type W = crate::W<NvicIspr3Spec>;
#[doc = "Interrupt set-pending bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Setpend {
    #[doc = "0: interrupt is not pending"]
    Value3 = 0,
    #[doc = "1: interrupt is pending."]
    Value4 = 1,
}
impl From<Setpend> for u32 {
    #[inline(always)]
    fn from(variant: Setpend) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setpend {
    type Ux = u32;
}
#[doc = "Field `SETPEND` reader - Interrupt set-pending bits."]
pub type SetpendR = crate::FieldReader<Setpend>;
impl SetpendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Setpend> {
        match self.bits {
            0 => Some(Setpend::Value3),
            1 => Some(Setpend::Value4),
            _ => None,
        }
    }
    #[doc = "interrupt is not pending"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Setpend::Value3
    }
    #[doc = "interrupt is pending."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Setpend::Value4
    }
}
#[doc = "Field `SETPEND` writer - Interrupt set-pending bits."]
pub type SetpendW<'a, REG> = crate::FieldWriter<'a, REG, 32, Setpend>;
impl<'a, REG> SetpendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "interrupt is not pending"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Setpend::Value3)
    }
    #[doc = "interrupt is pending."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Setpend::Value4)
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt set-pending bits."]
    #[inline(always)]
    pub fn setpend(&self) -> SetpendR {
        SetpendR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt set-pending bits."]
    #[inline(always)]
    #[must_use]
    pub fn setpend(&mut self) -> SetpendW<NvicIspr3Spec> {
        SetpendW::new(self, 0)
    }
}
#[doc = "Interrupt Set-pending Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ispr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ispr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIspr3Spec;
impl crate::RegisterSpec for NvicIspr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ispr3::R`](R) reader structure"]
impl crate::Readable for NvicIspr3Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_ispr3::W`](W) writer structure"]
impl crate::Writable for NvicIspr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_ISPR3 to value 0"]
impl crate::Resettable for NvicIspr3Spec {
    const RESET_VALUE: u32 = 0;
}
