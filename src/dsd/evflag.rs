#[doc = "Register `EVFLAG` reader"]
pub type R = crate::R<EvflagSpec>;
#[doc = "Register `EVFLAG` writer"]
pub type W = crate::W<EvflagSpec>;
#[doc = "Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resev0 {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: A new result has been stored in register RESMx"]
    Value2 = 1,
}
impl From<Resev0> for bool {
    #[inline(always)]
    fn from(variant: Resev0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEV0` reader - Result Event"]
pub type Resev0R = crate::BitReader<Resev0>;
impl Resev0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resev0 {
        match self.bits {
            false => Resev0::Value1,
            true => Resev0::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Resev0::Value1
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Resev0::Value2
    }
}
#[doc = "Field `RESEV0` writer - Result Event"]
pub type Resev0W<'a, REG> = crate::BitWriter<'a, REG, Resev0>;
impl<'a, REG> Resev0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Resev0::Value1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Resev0::Value2)
    }
}
#[doc = "Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resev1 {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: A new result has been stored in register RESMx"]
    Value2 = 1,
}
impl From<Resev1> for bool {
    #[inline(always)]
    fn from(variant: Resev1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEV1` reader - Result Event"]
pub type Resev1R = crate::BitReader<Resev1>;
impl Resev1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resev1 {
        match self.bits {
            false => Resev1::Value1,
            true => Resev1::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Resev1::Value1
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Resev1::Value2
    }
}
#[doc = "Field `RESEV1` writer - Result Event"]
pub type Resev1W<'a, REG> = crate::BitWriter<'a, REG, Resev1>;
impl<'a, REG> Resev1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Resev1::Value1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Resev1::Value2)
    }
}
#[doc = "Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resev2 {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: A new result has been stored in register RESMx"]
    Value2 = 1,
}
impl From<Resev2> for bool {
    #[inline(always)]
    fn from(variant: Resev2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEV2` reader - Result Event"]
pub type Resev2R = crate::BitReader<Resev2>;
impl Resev2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resev2 {
        match self.bits {
            false => Resev2::Value1,
            true => Resev2::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Resev2::Value1
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Resev2::Value2
    }
}
#[doc = "Field `RESEV2` writer - Result Event"]
pub type Resev2W<'a, REG> = crate::BitWriter<'a, REG, Resev2>;
impl<'a, REG> Resev2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Resev2::Value1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Resev2::Value2)
    }
}
#[doc = "Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resev3 {
    #[doc = "0: No result event"]
    Value1 = 0,
    #[doc = "1: A new result has been stored in register RESMx"]
    Value2 = 1,
}
impl From<Resev3> for bool {
    #[inline(always)]
    fn from(variant: Resev3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEV3` reader - Result Event"]
pub type Resev3R = crate::BitReader<Resev3>;
impl Resev3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resev3 {
        match self.bits {
            false => Resev3::Value1,
            true => Resev3::Value2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Resev3::Value1
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Resev3::Value2
    }
}
#[doc = "Field `RESEV3` writer - Result Event"]
pub type Resev3W<'a, REG> = crate::BitWriter<'a, REG, Resev3>;
impl<'a, REG> Resev3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Resev3::Value1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Resev3::Value2)
    }
}
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alev0 {
    #[doc = "0: No alarm event"]
    Value1 = 0,
    #[doc = "1: An alarm event has occurred"]
    Value2 = 1,
}
impl From<Alev0> for bool {
    #[inline(always)]
    fn from(variant: Alev0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEV0` reader - Alarm Event"]
pub type Alev0R = crate::BitReader<Alev0>;
impl Alev0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alev0 {
        match self.bits {
            false => Alev0::Value1,
            true => Alev0::Value2,
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Alev0::Value1
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Alev0::Value2
    }
}
#[doc = "Field `ALEV0` writer - Alarm Event"]
pub type Alev0W<'a, REG> = crate::BitWriter<'a, REG, Alev0>;
impl<'a, REG> Alev0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Alev0::Value1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Alev0::Value2)
    }
}
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alev1 {
    #[doc = "0: No alarm event"]
    Value1 = 0,
    #[doc = "1: An alarm event has occurred"]
    Value2 = 1,
}
impl From<Alev1> for bool {
    #[inline(always)]
    fn from(variant: Alev1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEV1` reader - Alarm Event"]
pub type Alev1R = crate::BitReader<Alev1>;
impl Alev1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alev1 {
        match self.bits {
            false => Alev1::Value1,
            true => Alev1::Value2,
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Alev1::Value1
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Alev1::Value2
    }
}
#[doc = "Field `ALEV1` writer - Alarm Event"]
pub type Alev1W<'a, REG> = crate::BitWriter<'a, REG, Alev1>;
impl<'a, REG> Alev1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Alev1::Value1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Alev1::Value2)
    }
}
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alev2 {
    #[doc = "0: No alarm event"]
    Value1 = 0,
    #[doc = "1: An alarm event has occurred"]
    Value2 = 1,
}
impl From<Alev2> for bool {
    #[inline(always)]
    fn from(variant: Alev2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEV2` reader - Alarm Event"]
pub type Alev2R = crate::BitReader<Alev2>;
impl Alev2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alev2 {
        match self.bits {
            false => Alev2::Value1,
            true => Alev2::Value2,
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Alev2::Value1
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Alev2::Value2
    }
}
#[doc = "Field `ALEV2` writer - Alarm Event"]
pub type Alev2W<'a, REG> = crate::BitWriter<'a, REG, Alev2>;
impl<'a, REG> Alev2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Alev2::Value1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Alev2::Value2)
    }
}
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alev3 {
    #[doc = "0: No alarm event"]
    Value1 = 0,
    #[doc = "1: An alarm event has occurred"]
    Value2 = 1,
}
impl From<Alev3> for bool {
    #[inline(always)]
    fn from(variant: Alev3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEV3` reader - Alarm Event"]
pub type Alev3R = crate::BitReader<Alev3>;
impl Alev3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alev3 {
        match self.bits {
            false => Alev3::Value1,
            true => Alev3::Value2,
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Alev3::Value1
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Alev3::Value2
    }
}
#[doc = "Field `ALEV3` writer - Alarm Event"]
pub type Alev3W<'a, REG> = crate::BitWriter<'a, REG, Alev3>;
impl<'a, REG> Alev3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Alev3::Value1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Alev3::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Result Event"]
    #[inline(always)]
    pub fn resev0(&self) -> Resev0R {
        Resev0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Result Event"]
    #[inline(always)]
    pub fn resev1(&self) -> Resev1R {
        Resev1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Result Event"]
    #[inline(always)]
    pub fn resev2(&self) -> Resev2R {
        Resev2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Result Event"]
    #[inline(always)]
    pub fn resev3(&self) -> Resev3R {
        Resev3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Alarm Event"]
    #[inline(always)]
    pub fn alev0(&self) -> Alev0R {
        Alev0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Alarm Event"]
    #[inline(always)]
    pub fn alev1(&self) -> Alev1R {
        Alev1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Alarm Event"]
    #[inline(always)]
    pub fn alev2(&self) -> Alev2R {
        Alev2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Alarm Event"]
    #[inline(always)]
    pub fn alev3(&self) -> Alev3R {
        Alev3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Result Event"]
    #[inline(always)]
    #[must_use]
    pub fn resev0(&mut self) -> Resev0W<EvflagSpec> {
        Resev0W::new(self, 0)
    }
    #[doc = "Bit 1 - Result Event"]
    #[inline(always)]
    #[must_use]
    pub fn resev1(&mut self) -> Resev1W<EvflagSpec> {
        Resev1W::new(self, 1)
    }
    #[doc = "Bit 2 - Result Event"]
    #[inline(always)]
    #[must_use]
    pub fn resev2(&mut self) -> Resev2W<EvflagSpec> {
        Resev2W::new(self, 2)
    }
    #[doc = "Bit 3 - Result Event"]
    #[inline(always)]
    #[must_use]
    pub fn resev3(&mut self) -> Resev3W<EvflagSpec> {
        Resev3W::new(self, 3)
    }
    #[doc = "Bit 16 - Alarm Event"]
    #[inline(always)]
    #[must_use]
    pub fn alev0(&mut self) -> Alev0W<EvflagSpec> {
        Alev0W::new(self, 16)
    }
    #[doc = "Bit 17 - Alarm Event"]
    #[inline(always)]
    #[must_use]
    pub fn alev1(&mut self) -> Alev1W<EvflagSpec> {
        Alev1W::new(self, 17)
    }
    #[doc = "Bit 18 - Alarm Event"]
    #[inline(always)]
    #[must_use]
    pub fn alev2(&mut self) -> Alev2W<EvflagSpec> {
        Alev2W::new(self, 18)
    }
    #[doc = "Bit 19 - Alarm Event"]
    #[inline(always)]
    #[must_use]
    pub fn alev3(&mut self) -> Alev3W<EvflagSpec> {
        Alev3W::new(self, 19)
    }
}
#[doc = "Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvflagSpec;
impl crate::RegisterSpec for EvflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evflag::R`](R) reader structure"]
impl crate::Readable for EvflagSpec {}
#[doc = "`write(|w| ..)` method takes [`evflag::W`](W) writer structure"]
impl crate::Writable for EvflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVFLAG to value 0"]
impl crate::Resettable for EvflagSpec {
    const RESET_VALUE: u32 = 0;
}
