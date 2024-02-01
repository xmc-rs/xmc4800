#[doc = "Register `CLEARTFR` writer"]
pub type W = crate::W<CLEARTFR_SPEC>;
#[doc = "Clear Interrupt Status and Raw Status for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH0_AW> for bool {
    #[inline(always)]
    fn from(variant: CH0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` writer - Clear Interrupt Status and Raw Status for channel 0"]
pub type CH0_W<'a, REG> = crate::BitWriter<'a, REG, CH0_AW>;
impl<'a, REG> CH0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH0_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH0_AW::VALUE2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH1_AW> for bool {
    #[inline(always)]
    fn from(variant: CH1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1` writer - Clear Interrupt Status and Raw Status for channel 1"]
pub type CH1_W<'a, REG> = crate::BitWriter<'a, REG, CH1_AW>;
impl<'a, REG> CH1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH1_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH1_AW::VALUE2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH2_AW> for bool {
    #[inline(always)]
    fn from(variant: CH2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2` writer - Clear Interrupt Status and Raw Status for channel 2"]
pub type CH2_W<'a, REG> = crate::BitWriter<'a, REG, CH2_AW>;
impl<'a, REG> CH2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH2_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH2_AW::VALUE2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH3_AW> for bool {
    #[inline(always)]
    fn from(variant: CH3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3` writer - Clear Interrupt Status and Raw Status for channel 3"]
pub type CH3_W<'a, REG> = crate::BitWriter<'a, REG, CH3_AW>;
impl<'a, REG> CH3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH3_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH3_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Interrupt Status and Raw Status for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<CLEARTFR_SPEC> {
        CH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Interrupt Status and Raw Status for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<CLEARTFR_SPEC> {
        CH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Interrupt Status and Raw Status for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<CLEARTFR_SPEC> {
        CH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Interrupt Status and Raw Status for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<CLEARTFR_SPEC> {
        CH3_W::new(self, 3)
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
#[doc = "IntTfr Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cleartfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLEARTFR_SPEC;
impl crate::RegisterSpec for CLEARTFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cleartfr::W`](W) writer structure"]
impl crate::Writable for CLEARTFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLEARTFR to value 0"]
impl crate::Resettable for CLEARTFR_SPEC {
    const RESET_VALUE: u32 = 0;
}
