#[doc = "Register `GLOBRC` reader"]
pub type R = crate::R<GLOBRC_SPEC>;
#[doc = "Register `GLOBRC` writer"]
pub type W = crate::W<GLOBRC_SPEC>;
#[doc = "Channel 0 Run Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0RUN_A {
    #[doc = "0: Stop channel x"]
    VALUE1 = 0,
    #[doc = "1: Demodulator channel x is enabled and runs"]
    VALUE2 = 1,
}
impl From<CH0RUN_A> for bool {
    #[inline(always)]
    fn from(variant: CH0RUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0RUN` reader - Channel 0 Run Control"]
pub type CH0RUN_R = crate::BitReader<CH0RUN_A>;
impl CH0RUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH0RUN_A {
        match self.bits {
            false => CH0RUN_A::VALUE1,
            true => CH0RUN_A::VALUE2,
        }
    }
    #[doc = "Stop channel x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH0RUN_A::VALUE1
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH0RUN_A::VALUE2
    }
}
#[doc = "Field `CH0RUN` writer - Channel 0 Run Control"]
pub type CH0RUN_W<'a, REG> = crate::BitWriter<'a, REG, CH0RUN_A>;
impl<'a, REG> CH0RUN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop channel x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH0RUN_A::VALUE1)
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH0RUN_A::VALUE2)
    }
}
#[doc = "Channel 1 Run Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1RUN_A {
    #[doc = "0: Stop channel x"]
    VALUE1 = 0,
    #[doc = "1: Demodulator channel x is enabled and runs"]
    VALUE2 = 1,
}
impl From<CH1RUN_A> for bool {
    #[inline(always)]
    fn from(variant: CH1RUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1RUN` reader - Channel 1 Run Control"]
pub type CH1RUN_R = crate::BitReader<CH1RUN_A>;
impl CH1RUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH1RUN_A {
        match self.bits {
            false => CH1RUN_A::VALUE1,
            true => CH1RUN_A::VALUE2,
        }
    }
    #[doc = "Stop channel x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH1RUN_A::VALUE1
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH1RUN_A::VALUE2
    }
}
#[doc = "Field `CH1RUN` writer - Channel 1 Run Control"]
pub type CH1RUN_W<'a, REG> = crate::BitWriter<'a, REG, CH1RUN_A>;
impl<'a, REG> CH1RUN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop channel x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH1RUN_A::VALUE1)
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH1RUN_A::VALUE2)
    }
}
#[doc = "Channel 2 Run Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2RUN_A {
    #[doc = "0: Stop channel x"]
    VALUE1 = 0,
    #[doc = "1: Demodulator channel x is enabled and runs"]
    VALUE2 = 1,
}
impl From<CH2RUN_A> for bool {
    #[inline(always)]
    fn from(variant: CH2RUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2RUN` reader - Channel 2 Run Control"]
pub type CH2RUN_R = crate::BitReader<CH2RUN_A>;
impl CH2RUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH2RUN_A {
        match self.bits {
            false => CH2RUN_A::VALUE1,
            true => CH2RUN_A::VALUE2,
        }
    }
    #[doc = "Stop channel x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH2RUN_A::VALUE1
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH2RUN_A::VALUE2
    }
}
#[doc = "Field `CH2RUN` writer - Channel 2 Run Control"]
pub type CH2RUN_W<'a, REG> = crate::BitWriter<'a, REG, CH2RUN_A>;
impl<'a, REG> CH2RUN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop channel x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH2RUN_A::VALUE1)
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH2RUN_A::VALUE2)
    }
}
#[doc = "Channel 3 Run Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3RUN_A {
    #[doc = "0: Stop channel x"]
    VALUE1 = 0,
    #[doc = "1: Demodulator channel x is enabled and runs"]
    VALUE2 = 1,
}
impl From<CH3RUN_A> for bool {
    #[inline(always)]
    fn from(variant: CH3RUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3RUN` reader - Channel 3 Run Control"]
pub type CH3RUN_R = crate::BitReader<CH3RUN_A>;
impl CH3RUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH3RUN_A {
        match self.bits {
            false => CH3RUN_A::VALUE1,
            true => CH3RUN_A::VALUE2,
        }
    }
    #[doc = "Stop channel x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH3RUN_A::VALUE1
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH3RUN_A::VALUE2
    }
}
#[doc = "Field `CH3RUN` writer - Channel 3 Run Control"]
pub type CH3RUN_W<'a, REG> = crate::BitWriter<'a, REG, CH3RUN_A>;
impl<'a, REG> CH3RUN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop channel x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH3RUN_A::VALUE1)
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH3RUN_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Run Control"]
    #[inline(always)]
    pub fn ch0run(&self) -> CH0RUN_R {
        CH0RUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Run Control"]
    #[inline(always)]
    pub fn ch1run(&self) -> CH1RUN_R {
        CH1RUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Run Control"]
    #[inline(always)]
    pub fn ch2run(&self) -> CH2RUN_R {
        CH2RUN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Run Control"]
    #[inline(always)]
    pub fn ch3run(&self) -> CH3RUN_R {
        CH3RUN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Run Control"]
    #[inline(always)]
    pub fn ch0run(&mut self) -> CH0RUN_W<GLOBRC_SPEC> {
        CH0RUN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Run Control"]
    #[inline(always)]
    pub fn ch1run(&mut self) -> CH1RUN_W<GLOBRC_SPEC> {
        CH1RUN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Run Control"]
    #[inline(always)]
    pub fn ch2run(&mut self) -> CH2RUN_W<GLOBRC_SPEC> {
        CH2RUN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Run Control"]
    #[inline(always)]
    pub fn ch3run(&mut self) -> CH3RUN_W<GLOBRC_SPEC> {
        CH3RUN_W::new(self, 3)
    }
}
#[doc = "Global Run Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`globrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`globrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GLOBRC_SPEC;
impl crate::RegisterSpec for GLOBRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`globrc::R`](R) reader structure"]
impl crate::Readable for GLOBRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`globrc::W`](W) writer structure"]
impl crate::Writable for GLOBRC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOBRC to value 0"]
impl crate::Resettable for GLOBRC_SPEC {
    const RESET_VALUE: u32 = 0;
}
