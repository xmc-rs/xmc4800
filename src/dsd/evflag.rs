#[doc = "Register `EVFLAG` reader"]
pub type R = crate::R<EVFLAG_SPEC>;
#[doc = "Register `EVFLAG` writer"]
pub type W = crate::W<EVFLAG_SPEC>;
#[doc = "Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESEV0_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: A new result has been stored in register RESMx"]
    VALUE2 = 1,
}
impl From<RESEV0_A> for bool {
    #[inline(always)]
    fn from(variant: RESEV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEV0` reader - Result Event"]
pub type RESEV0_R = crate::BitReader<RESEV0_A>;
impl RESEV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESEV0_A {
        match self.bits {
            false => RESEV0_A::VALUE1,
            true => RESEV0_A::VALUE2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESEV0_A::VALUE1
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESEV0_A::VALUE2
    }
}
#[doc = "Field `RESEV0` writer - Result Event"]
pub type RESEV0_W<'a, REG> = crate::BitWriter<'a, REG, RESEV0_A>;
impl<'a, REG> RESEV0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RESEV0_A::VALUE1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RESEV0_A::VALUE2)
    }
}
#[doc = "Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESEV1_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: A new result has been stored in register RESMx"]
    VALUE2 = 1,
}
impl From<RESEV1_A> for bool {
    #[inline(always)]
    fn from(variant: RESEV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEV1` reader - Result Event"]
pub type RESEV1_R = crate::BitReader<RESEV1_A>;
impl RESEV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESEV1_A {
        match self.bits {
            false => RESEV1_A::VALUE1,
            true => RESEV1_A::VALUE2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESEV1_A::VALUE1
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESEV1_A::VALUE2
    }
}
#[doc = "Field `RESEV1` writer - Result Event"]
pub type RESEV1_W<'a, REG> = crate::BitWriter<'a, REG, RESEV1_A>;
impl<'a, REG> RESEV1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RESEV1_A::VALUE1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RESEV1_A::VALUE2)
    }
}
#[doc = "Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESEV2_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: A new result has been stored in register RESMx"]
    VALUE2 = 1,
}
impl From<RESEV2_A> for bool {
    #[inline(always)]
    fn from(variant: RESEV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEV2` reader - Result Event"]
pub type RESEV2_R = crate::BitReader<RESEV2_A>;
impl RESEV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESEV2_A {
        match self.bits {
            false => RESEV2_A::VALUE1,
            true => RESEV2_A::VALUE2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESEV2_A::VALUE1
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESEV2_A::VALUE2
    }
}
#[doc = "Field `RESEV2` writer - Result Event"]
pub type RESEV2_W<'a, REG> = crate::BitWriter<'a, REG, RESEV2_A>;
impl<'a, REG> RESEV2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RESEV2_A::VALUE1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RESEV2_A::VALUE2)
    }
}
#[doc = "Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESEV3_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: A new result has been stored in register RESMx"]
    VALUE2 = 1,
}
impl From<RESEV3_A> for bool {
    #[inline(always)]
    fn from(variant: RESEV3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEV3` reader - Result Event"]
pub type RESEV3_R = crate::BitReader<RESEV3_A>;
impl RESEV3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESEV3_A {
        match self.bits {
            false => RESEV3_A::VALUE1,
            true => RESEV3_A::VALUE2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESEV3_A::VALUE1
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESEV3_A::VALUE2
    }
}
#[doc = "Field `RESEV3` writer - Result Event"]
pub type RESEV3_W<'a, REG> = crate::BitWriter<'a, REG, RESEV3_A>;
impl<'a, REG> RESEV3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RESEV3_A::VALUE1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RESEV3_A::VALUE2)
    }
}
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALEV0_A {
    #[doc = "0: No alarm event"]
    VALUE1 = 0,
    #[doc = "1: An alarm event has occurred"]
    VALUE2 = 1,
}
impl From<ALEV0_A> for bool {
    #[inline(always)]
    fn from(variant: ALEV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEV0` reader - Alarm Event"]
pub type ALEV0_R = crate::BitReader<ALEV0_A>;
impl ALEV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALEV0_A {
        match self.bits {
            false => ALEV0_A::VALUE1,
            true => ALEV0_A::VALUE2,
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV0_A::VALUE1
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV0_A::VALUE2
    }
}
#[doc = "Field `ALEV0` writer - Alarm Event"]
pub type ALEV0_W<'a, REG> = crate::BitWriter<'a, REG, ALEV0_A>;
impl<'a, REG> ALEV0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV0_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV0_A::VALUE2)
    }
}
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALEV1_A {
    #[doc = "0: No alarm event"]
    VALUE1 = 0,
    #[doc = "1: An alarm event has occurred"]
    VALUE2 = 1,
}
impl From<ALEV1_A> for bool {
    #[inline(always)]
    fn from(variant: ALEV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEV1` reader - Alarm Event"]
pub type ALEV1_R = crate::BitReader<ALEV1_A>;
impl ALEV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALEV1_A {
        match self.bits {
            false => ALEV1_A::VALUE1,
            true => ALEV1_A::VALUE2,
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV1_A::VALUE1
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV1_A::VALUE2
    }
}
#[doc = "Field `ALEV1` writer - Alarm Event"]
pub type ALEV1_W<'a, REG> = crate::BitWriter<'a, REG, ALEV1_A>;
impl<'a, REG> ALEV1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV1_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV1_A::VALUE2)
    }
}
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALEV2_A {
    #[doc = "0: No alarm event"]
    VALUE1 = 0,
    #[doc = "1: An alarm event has occurred"]
    VALUE2 = 1,
}
impl From<ALEV2_A> for bool {
    #[inline(always)]
    fn from(variant: ALEV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEV2` reader - Alarm Event"]
pub type ALEV2_R = crate::BitReader<ALEV2_A>;
impl ALEV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALEV2_A {
        match self.bits {
            false => ALEV2_A::VALUE1,
            true => ALEV2_A::VALUE2,
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV2_A::VALUE1
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV2_A::VALUE2
    }
}
#[doc = "Field `ALEV2` writer - Alarm Event"]
pub type ALEV2_W<'a, REG> = crate::BitWriter<'a, REG, ALEV2_A>;
impl<'a, REG> ALEV2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV2_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV2_A::VALUE2)
    }
}
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALEV3_A {
    #[doc = "0: No alarm event"]
    VALUE1 = 0,
    #[doc = "1: An alarm event has occurred"]
    VALUE2 = 1,
}
impl From<ALEV3_A> for bool {
    #[inline(always)]
    fn from(variant: ALEV3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEV3` reader - Alarm Event"]
pub type ALEV3_R = crate::BitReader<ALEV3_A>;
impl ALEV3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALEV3_A {
        match self.bits {
            false => ALEV3_A::VALUE1,
            true => ALEV3_A::VALUE2,
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV3_A::VALUE1
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV3_A::VALUE2
    }
}
#[doc = "Field `ALEV3` writer - Alarm Event"]
pub type ALEV3_W<'a, REG> = crate::BitWriter<'a, REG, ALEV3_A>;
impl<'a, REG> ALEV3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV3_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV3_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Result Event"]
    #[inline(always)]
    pub fn resev0(&self) -> RESEV0_R {
        RESEV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Result Event"]
    #[inline(always)]
    pub fn resev1(&self) -> RESEV1_R {
        RESEV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Result Event"]
    #[inline(always)]
    pub fn resev2(&self) -> RESEV2_R {
        RESEV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Result Event"]
    #[inline(always)]
    pub fn resev3(&self) -> RESEV3_R {
        RESEV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Alarm Event"]
    #[inline(always)]
    pub fn alev0(&self) -> ALEV0_R {
        ALEV0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Alarm Event"]
    #[inline(always)]
    pub fn alev1(&self) -> ALEV1_R {
        ALEV1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Alarm Event"]
    #[inline(always)]
    pub fn alev2(&self) -> ALEV2_R {
        ALEV2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Alarm Event"]
    #[inline(always)]
    pub fn alev3(&self) -> ALEV3_R {
        ALEV3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Result Event"]
    #[inline(always)]
    pub fn resev0(&mut self) -> RESEV0_W<EVFLAG_SPEC> {
        RESEV0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Result Event"]
    #[inline(always)]
    pub fn resev1(&mut self) -> RESEV1_W<EVFLAG_SPEC> {
        RESEV1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Result Event"]
    #[inline(always)]
    pub fn resev2(&mut self) -> RESEV2_W<EVFLAG_SPEC> {
        RESEV2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Result Event"]
    #[inline(always)]
    pub fn resev3(&mut self) -> RESEV3_W<EVFLAG_SPEC> {
        RESEV3_W::new(self, 3)
    }
    #[doc = "Bit 16 - Alarm Event"]
    #[inline(always)]
    pub fn alev0(&mut self) -> ALEV0_W<EVFLAG_SPEC> {
        ALEV0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Alarm Event"]
    #[inline(always)]
    pub fn alev1(&mut self) -> ALEV1_W<EVFLAG_SPEC> {
        ALEV1_W::new(self, 17)
    }
    #[doc = "Bit 18 - Alarm Event"]
    #[inline(always)]
    pub fn alev2(&mut self) -> ALEV2_W<EVFLAG_SPEC> {
        ALEV2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Alarm Event"]
    #[inline(always)]
    pub fn alev3(&mut self) -> ALEV3_W<EVFLAG_SPEC> {
        ALEV3_W::new(self, 19)
    }
}
#[doc = "Event Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`evflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVFLAG_SPEC;
impl crate::RegisterSpec for EVFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evflag::R`](R) reader structure"]
impl crate::Readable for EVFLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evflag::W`](W) writer structure"]
impl crate::Writable for EVFLAG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVFLAG to value 0"]
impl crate::Resettable for EVFLAG_SPEC {
    const RESET_VALUE: u32 = 0;
}
