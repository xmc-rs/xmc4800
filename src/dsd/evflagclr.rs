#[doc = "Register `EVFLAGCLR` writer"]
pub type W = crate::W<EVFLAGCLR_SPEC>;
#[doc = "Result Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESEC0_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit RESEVx"]
    VALUE2 = 1,
}
impl From<RESEC0_AW> for bool {
    #[inline(always)]
    fn from(variant: RESEC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEC0` writer - Result Event Clear"]
pub type RESEC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RESEC0_AW>;
impl<'a, REG, const O: u8> RESEC0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RESEC0_AW::VALUE1)
    }
    #[doc = "Clear bit RESEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RESEC0_AW::VALUE2)
    }
}
#[doc = "Result Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESEC1_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit RESEVx"]
    VALUE2 = 1,
}
impl From<RESEC1_AW> for bool {
    #[inline(always)]
    fn from(variant: RESEC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEC1` writer - Result Event Clear"]
pub type RESEC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RESEC1_AW>;
impl<'a, REG, const O: u8> RESEC1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RESEC1_AW::VALUE1)
    }
    #[doc = "Clear bit RESEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RESEC1_AW::VALUE2)
    }
}
#[doc = "Result Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESEC2_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit RESEVx"]
    VALUE2 = 1,
}
impl From<RESEC2_AW> for bool {
    #[inline(always)]
    fn from(variant: RESEC2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEC2` writer - Result Event Clear"]
pub type RESEC2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RESEC2_AW>;
impl<'a, REG, const O: u8> RESEC2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RESEC2_AW::VALUE1)
    }
    #[doc = "Clear bit RESEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RESEC2_AW::VALUE2)
    }
}
#[doc = "Result Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESEC3_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit RESEVx"]
    VALUE2 = 1,
}
impl From<RESEC3_AW> for bool {
    #[inline(always)]
    fn from(variant: RESEC3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEC3` writer - Result Event Clear"]
pub type RESEC3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RESEC3_AW>;
impl<'a, REG, const O: u8> RESEC3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RESEC3_AW::VALUE1)
    }
    #[doc = "Clear bit RESEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RESEC3_AW::VALUE2)
    }
}
#[doc = "Alarm Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALEC0_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit ALEVx"]
    VALUE2 = 1,
}
impl From<ALEC0_AW> for bool {
    #[inline(always)]
    fn from(variant: ALEC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEC0` writer - Alarm Event Clear"]
pub type ALEC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ALEC0_AW>;
impl<'a, REG, const O: u8> ALEC0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEC0_AW::VALUE1)
    }
    #[doc = "Clear bit ALEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEC0_AW::VALUE2)
    }
}
#[doc = "Alarm Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALEC1_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit ALEVx"]
    VALUE2 = 1,
}
impl From<ALEC1_AW> for bool {
    #[inline(always)]
    fn from(variant: ALEC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEC1` writer - Alarm Event Clear"]
pub type ALEC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ALEC1_AW>;
impl<'a, REG, const O: u8> ALEC1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEC1_AW::VALUE1)
    }
    #[doc = "Clear bit ALEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEC1_AW::VALUE2)
    }
}
#[doc = "Alarm Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALEC2_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit ALEVx"]
    VALUE2 = 1,
}
impl From<ALEC2_AW> for bool {
    #[inline(always)]
    fn from(variant: ALEC2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEC2` writer - Alarm Event Clear"]
pub type ALEC2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ALEC2_AW>;
impl<'a, REG, const O: u8> ALEC2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEC2_AW::VALUE1)
    }
    #[doc = "Clear bit ALEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEC2_AW::VALUE2)
    }
}
#[doc = "Alarm Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALEC3_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit ALEVx"]
    VALUE2 = 1,
}
impl From<ALEC3_AW> for bool {
    #[inline(always)]
    fn from(variant: ALEC3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALEC3` writer - Alarm Event Clear"]
pub type ALEC3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ALEC3_AW>;
impl<'a, REG, const O: u8> ALEC3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEC3_AW::VALUE1)
    }
    #[doc = "Clear bit ALEVx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEC3_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Result Event Clear"]
    #[inline(always)]
    #[must_use]
    pub fn resec0(&mut self) -> RESEC0_W<EVFLAGCLR_SPEC, 0> {
        RESEC0_W::new(self)
    }
    #[doc = "Bit 1 - Result Event Clear"]
    #[inline(always)]
    #[must_use]
    pub fn resec1(&mut self) -> RESEC1_W<EVFLAGCLR_SPEC, 1> {
        RESEC1_W::new(self)
    }
    #[doc = "Bit 2 - Result Event Clear"]
    #[inline(always)]
    #[must_use]
    pub fn resec2(&mut self) -> RESEC2_W<EVFLAGCLR_SPEC, 2> {
        RESEC2_W::new(self)
    }
    #[doc = "Bit 3 - Result Event Clear"]
    #[inline(always)]
    #[must_use]
    pub fn resec3(&mut self) -> RESEC3_W<EVFLAGCLR_SPEC, 3> {
        RESEC3_W::new(self)
    }
    #[doc = "Bit 16 - Alarm Event Clear"]
    #[inline(always)]
    #[must_use]
    pub fn alec0(&mut self) -> ALEC0_W<EVFLAGCLR_SPEC, 16> {
        ALEC0_W::new(self)
    }
    #[doc = "Bit 17 - Alarm Event Clear"]
    #[inline(always)]
    #[must_use]
    pub fn alec1(&mut self) -> ALEC1_W<EVFLAGCLR_SPEC, 17> {
        ALEC1_W::new(self)
    }
    #[doc = "Bit 18 - Alarm Event Clear"]
    #[inline(always)]
    #[must_use]
    pub fn alec2(&mut self) -> ALEC2_W<EVFLAGCLR_SPEC, 18> {
        ALEC2_W::new(self)
    }
    #[doc = "Bit 19 - Alarm Event Clear"]
    #[inline(always)]
    #[must_use]
    pub fn alec3(&mut self) -> ALEC3_W<EVFLAGCLR_SPEC, 19> {
        ALEC3_W::new(self)
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
#[doc = "Event Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evflagclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVFLAGCLR_SPEC;
impl crate::RegisterSpec for EVFLAGCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`evflagclr::W`](W) writer structure"]
impl crate::Writable for EVFLAGCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVFLAGCLR to value 0"]
impl crate::Resettable for EVFLAGCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
