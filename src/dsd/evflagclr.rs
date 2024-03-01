#[doc = "Register `EVFLAGCLR` writer"]
pub type W = crate::W<EvflagclrSpec>;
#[doc = "Result Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resec0 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear bit RESEVx"]
    Value2 = 1,
}
impl From<Resec0> for bool {
    #[inline(always)]
    fn from(variant: Resec0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEC0` writer - Result Event Clear"]
pub type Resec0W<'a, REG> = crate::BitWriter<'a, REG, Resec0>;
impl<'a, REG> Resec0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Resec0::Value1)
    }
    #[doc = "Clear bit RESEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Resec0::Value2)
    }
}
#[doc = "Result Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resec1 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear bit RESEVx"]
    Value2 = 1,
}
impl From<Resec1> for bool {
    #[inline(always)]
    fn from(variant: Resec1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEC1` writer - Result Event Clear"]
pub type Resec1W<'a, REG> = crate::BitWriter<'a, REG, Resec1>;
impl<'a, REG> Resec1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Resec1::Value1)
    }
    #[doc = "Clear bit RESEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Resec1::Value2)
    }
}
#[doc = "Result Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resec2 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear bit RESEVx"]
    Value2 = 1,
}
impl From<Resec2> for bool {
    #[inline(always)]
    fn from(variant: Resec2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEC2` writer - Result Event Clear"]
pub type Resec2W<'a, REG> = crate::BitWriter<'a, REG, Resec2>;
impl<'a, REG> Resec2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Resec2::Value1)
    }
    #[doc = "Clear bit RESEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Resec2::Value2)
    }
}
#[doc = "Result Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resec3 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear bit RESEVx"]
    Value2 = 1,
}
impl From<Resec3> for bool {
    #[inline(always)]
    fn from(variant: Resec3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEC3` writer - Result Event Clear"]
pub type Resec3W<'a, REG> = crate::BitWriter<'a, REG, Resec3>;
impl<'a, REG> Resec3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Resec3::Value1)
    }
    #[doc = "Clear bit RESEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Resec3::Value2)
    }
}
#[doc = "Alarm Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alec0 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear bit ALEVx"]
    Value2 = 1,
}
impl From<Alec0> for bool {
    #[inline(always)]
    fn from(variant: Alec0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEC0` writer - Alarm Event Clear"]
pub type Alec0W<'a, REG> = crate::BitWriter<'a, REG, Alec0>;
impl<'a, REG> Alec0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Alec0::Value1)
    }
    #[doc = "Clear bit ALEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Alec0::Value2)
    }
}
#[doc = "Alarm Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alec1 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear bit ALEVx"]
    Value2 = 1,
}
impl From<Alec1> for bool {
    #[inline(always)]
    fn from(variant: Alec1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEC1` writer - Alarm Event Clear"]
pub type Alec1W<'a, REG> = crate::BitWriter<'a, REG, Alec1>;
impl<'a, REG> Alec1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Alec1::Value1)
    }
    #[doc = "Clear bit ALEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Alec1::Value2)
    }
}
#[doc = "Alarm Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alec2 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear bit ALEVx"]
    Value2 = 1,
}
impl From<Alec2> for bool {
    #[inline(always)]
    fn from(variant: Alec2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEC2` writer - Alarm Event Clear"]
pub type Alec2W<'a, REG> = crate::BitWriter<'a, REG, Alec2>;
impl<'a, REG> Alec2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Alec2::Value1)
    }
    #[doc = "Clear bit ALEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Alec2::Value2)
    }
}
#[doc = "Alarm Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alec3 {
    #[doc = "0: No action"]
    Value1 = 0,
    #[doc = "1: Clear bit ALEVx"]
    Value2 = 1,
}
impl From<Alec3> for bool {
    #[inline(always)]
    fn from(variant: Alec3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEC3` writer - Alarm Event Clear"]
pub type Alec3W<'a, REG> = crate::BitWriter<'a, REG, Alec3>;
impl<'a, REG> Alec3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Alec3::Value1)
    }
    #[doc = "Clear bit ALEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Alec3::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Result Event Clear"]
    #[inline(always)]
    #[must_use]
    pub fn resec0(&mut self) -> Resec0W<EvflagclrSpec> {
        Resec0W::new(self, 0)
    }
    #[doc = "Bit 1 - Result Event Clear"]
    #[inline(always)]
    #[must_use]
    pub fn resec1(&mut self) -> Resec1W<EvflagclrSpec> {
        Resec1W::new(self, 1)
    }
    #[doc = "Bit 2 - Result Event Clear"]
    #[inline(always)]
    #[must_use]
    pub fn resec2(&mut self) -> Resec2W<EvflagclrSpec> {
        Resec2W::new(self, 2)
    }
    #[doc = "Bit 3 - Result Event Clear"]
    #[inline(always)]
    #[must_use]
    pub fn resec3(&mut self) -> Resec3W<EvflagclrSpec> {
        Resec3W::new(self, 3)
    }
    #[doc = "Bit 16 - Alarm Event Clear"]
    #[inline(always)]
    #[must_use]
    pub fn alec0(&mut self) -> Alec0W<EvflagclrSpec> {
        Alec0W::new(self, 16)
    }
    #[doc = "Bit 17 - Alarm Event Clear"]
    #[inline(always)]
    #[must_use]
    pub fn alec1(&mut self) -> Alec1W<EvflagclrSpec> {
        Alec1W::new(self, 17)
    }
    #[doc = "Bit 18 - Alarm Event Clear"]
    #[inline(always)]
    #[must_use]
    pub fn alec2(&mut self) -> Alec2W<EvflagclrSpec> {
        Alec2W::new(self, 18)
    }
    #[doc = "Bit 19 - Alarm Event Clear"]
    #[inline(always)]
    #[must_use]
    pub fn alec3(&mut self) -> Alec3W<EvflagclrSpec> {
        Alec3W::new(self, 19)
    }
}
#[doc = "Event Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evflagclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvflagclrSpec;
impl crate::RegisterSpec for EvflagclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`evflagclr::W`](W) writer structure"]
impl crate::Writable for EvflagclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVFLAGCLR to value 0"]
impl crate::Resettable for EvflagclrSpec {
    const RESET_VALUE: u32 = 0;
}
