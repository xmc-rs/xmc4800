#[doc = "Register `PSUS` reader"]
pub type R = crate::R<PSUS_SPEC>;
#[doc = "Register `PSUS` writer"]
pub type W = crate::W<PSUS_SPEC>;
#[doc = "Quadrature Mode Suspend Config\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QSUS_A {
    #[doc = "0: Suspend request ignored"]
    VALUE1 = 0,
    #[doc = "1: Stop immediately"]
    VALUE2 = 1,
    #[doc = "2: Suspend in the next index occurrence"]
    VALUE3 = 2,
    #[doc = "3: Suspend in the next phase (PhaseA or PhaseB) occurrence"]
    VALUE4 = 3,
}
impl From<QSUS_A> for u8 {
    #[inline(always)]
    fn from(variant: QSUS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for QSUS_A {
    type Ux = u8;
}
impl crate::IsEnum for QSUS_A {}
#[doc = "Field `QSUS` reader - Quadrature Mode Suspend Config"]
pub type QSUS_R = crate::FieldReader<QSUS_A>;
impl QSUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> QSUS_A {
        match self.bits {
            0 => QSUS_A::VALUE1,
            1 => QSUS_A::VALUE2,
            2 => QSUS_A::VALUE3,
            3 => QSUS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Suspend request ignored"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == QSUS_A::VALUE1
    }
    #[doc = "Stop immediately"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == QSUS_A::VALUE2
    }
    #[doc = "Suspend in the next index occurrence"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == QSUS_A::VALUE3
    }
    #[doc = "Suspend in the next phase (PhaseA or PhaseB) occurrence"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == QSUS_A::VALUE4
    }
}
#[doc = "Field `QSUS` writer - Quadrature Mode Suspend Config"]
pub type QSUS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, QSUS_A, crate::Safe>;
impl<'a, REG> QSUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Suspend request ignored"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(QSUS_A::VALUE1)
    }
    #[doc = "Stop immediately"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(QSUS_A::VALUE2)
    }
    #[doc = "Suspend in the next index occurrence"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(QSUS_A::VALUE3)
    }
    #[doc = "Suspend in the next phase (PhaseA or PhaseB) occurrence"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(QSUS_A::VALUE4)
    }
}
#[doc = "Multi-Channel Mode Suspend Config\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSUS_A {
    #[doc = "0: Suspend request ignored"]
    VALUE1 = 0,
    #[doc = "1: Stop immediately. Multi-Channel pattern is not set to the reset value."]
    VALUE2 = 1,
    #[doc = "2: Stop immediately. Multi-Channel pattern is set to the reset value."]
    VALUE3 = 2,
    #[doc = "3: Suspend with the synchronization of the PWM signal. Multi-Channel pattern is set to the reset value at the same time of the synchronization."]
    VALUE4 = 3,
}
impl From<MSUS_A> for u8 {
    #[inline(always)]
    fn from(variant: MSUS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MSUS_A {
    type Ux = u8;
}
impl crate::IsEnum for MSUS_A {}
#[doc = "Field `MSUS` reader - Multi-Channel Mode Suspend Config"]
pub type MSUS_R = crate::FieldReader<MSUS_A>;
impl MSUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSUS_A {
        match self.bits {
            0 => MSUS_A::VALUE1,
            1 => MSUS_A::VALUE2,
            2 => MSUS_A::VALUE3,
            3 => MSUS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Suspend request ignored"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MSUS_A::VALUE1
    }
    #[doc = "Stop immediately. Multi-Channel pattern is not set to the reset value."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSUS_A::VALUE2
    }
    #[doc = "Stop immediately. Multi-Channel pattern is set to the reset value."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MSUS_A::VALUE3
    }
    #[doc = "Suspend with the synchronization of the PWM signal. Multi-Channel pattern is set to the reset value at the same time of the synchronization."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MSUS_A::VALUE4
    }
}
#[doc = "Field `MSUS` writer - Multi-Channel Mode Suspend Config"]
pub type MSUS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MSUS_A, crate::Safe>;
impl<'a, REG> MSUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Suspend request ignored"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MSUS_A::VALUE1)
    }
    #[doc = "Stop immediately. Multi-Channel pattern is not set to the reset value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MSUS_A::VALUE2)
    }
    #[doc = "Stop immediately. Multi-Channel pattern is set to the reset value."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(MSUS_A::VALUE3)
    }
    #[doc = "Suspend with the synchronization of the PWM signal. Multi-Channel pattern is set to the reset value at the same time of the synchronization."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(MSUS_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Quadrature Mode Suspend Config"]
    #[inline(always)]
    pub fn qsus(&self) -> QSUS_R {
        QSUS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Multi-Channel Mode Suspend Config"]
    #[inline(always)]
    pub fn msus(&self) -> MSUS_R {
        MSUS_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Quadrature Mode Suspend Config"]
    #[inline(always)]
    #[must_use]
    pub fn qsus(&mut self) -> QSUS_W<PSUS_SPEC> {
        QSUS_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Multi-Channel Mode Suspend Config"]
    #[inline(always)]
    #[must_use]
    pub fn msus(&mut self) -> MSUS_W<PSUS_SPEC> {
        MSUS_W::new(self, 2)
    }
}
#[doc = "POSIF Suspend Config\n\nYou can [`read`](crate::Reg::read) this register and get [`psus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSUS_SPEC;
impl crate::RegisterSpec for PSUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psus::R`](R) reader structure"]
impl crate::Readable for PSUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psus::W`](W) writer structure"]
impl crate::Writable for PSUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSUS to value 0"]
impl crate::Resettable for PSUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
