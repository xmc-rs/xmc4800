#[doc = "Register `SGLREQSRCREG` reader"]
pub type R = crate::R<SGLREQSRCREG_SPEC>;
#[doc = "Register `SGLREQSRCREG` writer"]
pub type W = crate::W<SGLREQSRCREG_SPEC>;
#[doc = "Field `CH0` reader - Source request for channel 0"]
pub type CH0_R = crate::BitReader;
#[doc = "Field `CH0` writer - Source request for channel 0"]
pub type CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - Source request for channel 1"]
pub type CH1_R = crate::BitReader;
#[doc = "Field `CH1` writer - Source request for channel 1"]
pub type CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - Source request for channel 2"]
pub type CH2_R = crate::BitReader;
#[doc = "Field `CH2` writer - Source request for channel 2"]
pub type CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - Source request for channel 3"]
pub type CH3_R = crate::BitReader;
#[doc = "Field `CH3` writer - Source request for channel 3"]
pub type CH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Source request write enable for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH0_A {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH0_A> for bool {
    #[inline(always)]
    fn from(variant: WE_CH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH0` writer - Source request write enable for channel 0"]
pub type WE_CH0_W<'a, REG> = crate::BitWriter<'a, REG, WE_CH0_A>;
impl<'a, REG> WE_CH0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH0_A::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH0_A::VALUE2)
    }
}
#[doc = "Source request write enable for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH1_A {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH1_A> for bool {
    #[inline(always)]
    fn from(variant: WE_CH1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH1` writer - Source request write enable for channel 1"]
pub type WE_CH1_W<'a, REG> = crate::BitWriter<'a, REG, WE_CH1_A>;
impl<'a, REG> WE_CH1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH1_A::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH1_A::VALUE2)
    }
}
#[doc = "Source request write enable for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH2_A {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH2_A> for bool {
    #[inline(always)]
    fn from(variant: WE_CH2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH2` writer - Source request write enable for channel 2"]
pub type WE_CH2_W<'a, REG> = crate::BitWriter<'a, REG, WE_CH2_A>;
impl<'a, REG> WE_CH2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH2_A::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH2_A::VALUE2)
    }
}
#[doc = "Source request write enable for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH3_A {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH3_A> for bool {
    #[inline(always)]
    fn from(variant: WE_CH3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH3` writer - Source request write enable for channel 3"]
pub type WE_CH3_W<'a, REG> = crate::BitWriter<'a, REG, WE_CH3_A>;
impl<'a, REG> WE_CH3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH3_A::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH3_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Source request for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Source request for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Source request for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Source request for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source request for channel 0"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W<SGLREQSRCREG_SPEC> {
        CH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Source request for channel 1"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W<SGLREQSRCREG_SPEC> {
        CH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Source request for channel 2"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W<SGLREQSRCREG_SPEC> {
        CH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Source request for channel 3"]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W<SGLREQSRCREG_SPEC> {
        CH3_W::new(self, 3)
    }
    #[doc = "Bit 8 - Source request write enable for channel 0"]
    #[inline(always)]
    pub fn we_ch0(&mut self) -> WE_CH0_W<SGLREQSRCREG_SPEC> {
        WE_CH0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Source request write enable for channel 1"]
    #[inline(always)]
    pub fn we_ch1(&mut self) -> WE_CH1_W<SGLREQSRCREG_SPEC> {
        WE_CH1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Source request write enable for channel 2"]
    #[inline(always)]
    pub fn we_ch2(&mut self) -> WE_CH2_W<SGLREQSRCREG_SPEC> {
        WE_CH2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Source request write enable for channel 3"]
    #[inline(always)]
    pub fn we_ch3(&mut self) -> WE_CH3_W<SGLREQSRCREG_SPEC> {
        WE_CH3_W::new(self, 11)
    }
}
#[doc = "Single Source Transaction Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sglreqsrcreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sglreqsrcreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SGLREQSRCREG_SPEC;
impl crate::RegisterSpec for SGLREQSRCREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sglreqsrcreg::R`](R) reader structure"]
impl crate::Readable for SGLREQSRCREG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sglreqsrcreg::W`](W) writer structure"]
impl crate::Writable for SGLREQSRCREG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SGLREQSRCREG to value 0"]
impl crate::Resettable for SGLREQSRCREG_SPEC {
    const RESET_VALUE: u32 = 0;
}
