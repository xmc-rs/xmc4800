#[doc = "Register `MASKDSTTRAN` reader"]
pub type R = crate::R<MASKDSTTRAN_SPEC>;
#[doc = "Register `MASKDSTTRAN` writer"]
pub type W = crate::W<MASKDSTTRAN_SPEC>;
#[doc = "Field `CH0` reader - Mask bit for channel 0"]
pub type CH0_R = crate::BitReader<CH0_A>;
#[doc = "Mask bit for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0_A {
    #[doc = "0: masked"]
    VALUE1 = 0,
    #[doc = "1: unmasked"]
    VALUE2 = 1,
}
impl From<CH0_A> for bool {
    #[inline(always)]
    fn from(variant: CH0_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0_A {
        match self.bits {
            false => CH0_A::VALUE1,
            true => CH0_A::VALUE2,
        }
    }
    #[doc = "masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH0_A::VALUE1
    }
    #[doc = "unmasked"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH0_A::VALUE2
    }
}
#[doc = "Field `CH0` writer - Mask bit for channel 0"]
pub type CH0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH0_A>;
impl<'a, REG, const O: u8> CH0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH0_A::VALUE1)
    }
    #[doc = "unmasked"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH0_A::VALUE2)
    }
}
#[doc = "Field `CH1` reader - Mask bit for channel 1"]
pub type CH1_R = crate::BitReader<CH1_A>;
#[doc = "Mask bit for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1_A {
    #[doc = "0: masked"]
    VALUE1 = 0,
    #[doc = "1: unmasked"]
    VALUE2 = 1,
}
impl From<CH1_A> for bool {
    #[inline(always)]
    fn from(variant: CH1_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1_A {
        match self.bits {
            false => CH1_A::VALUE1,
            true => CH1_A::VALUE2,
        }
    }
    #[doc = "masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH1_A::VALUE1
    }
    #[doc = "unmasked"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH1_A::VALUE2
    }
}
#[doc = "Field `CH1` writer - Mask bit for channel 1"]
pub type CH1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH1_A>;
impl<'a, REG, const O: u8> CH1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH1_A::VALUE1)
    }
    #[doc = "unmasked"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH1_A::VALUE2)
    }
}
#[doc = "Field `CH2` reader - Mask bit for channel 2"]
pub type CH2_R = crate::BitReader<CH2_A>;
#[doc = "Mask bit for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2_A {
    #[doc = "0: masked"]
    VALUE1 = 0,
    #[doc = "1: unmasked"]
    VALUE2 = 1,
}
impl From<CH2_A> for bool {
    #[inline(always)]
    fn from(variant: CH2_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2_A {
        match self.bits {
            false => CH2_A::VALUE1,
            true => CH2_A::VALUE2,
        }
    }
    #[doc = "masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH2_A::VALUE1
    }
    #[doc = "unmasked"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH2_A::VALUE2
    }
}
#[doc = "Field `CH2` writer - Mask bit for channel 2"]
pub type CH2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH2_A>;
impl<'a, REG, const O: u8> CH2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH2_A::VALUE1)
    }
    #[doc = "unmasked"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH2_A::VALUE2)
    }
}
#[doc = "Field `CH3` reader - Mask bit for channel 3"]
pub type CH3_R = crate::BitReader<CH3_A>;
#[doc = "Mask bit for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3_A {
    #[doc = "0: masked"]
    VALUE1 = 0,
    #[doc = "1: unmasked"]
    VALUE2 = 1,
}
impl From<CH3_A> for bool {
    #[inline(always)]
    fn from(variant: CH3_A) -> Self {
        variant as u8 != 0
    }
}
impl CH3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3_A {
        match self.bits {
            false => CH3_A::VALUE1,
            true => CH3_A::VALUE2,
        }
    }
    #[doc = "masked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH3_A::VALUE1
    }
    #[doc = "unmasked"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH3_A::VALUE2
    }
}
#[doc = "Field `CH3` writer - Mask bit for channel 3"]
pub type CH3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH3_A>;
impl<'a, REG, const O: u8> CH3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "masked"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH3_A::VALUE1)
    }
    #[doc = "unmasked"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH3_A::VALUE2)
    }
}
#[doc = "Write enable for mask bit of channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH0_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH0_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH0` writer - Write enable for mask bit of channel 0"]
pub type WE_CH0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WE_CH0_AW>;
impl<'a, REG, const O: u8> WE_CH0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH0_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH0_AW::VALUE2)
    }
}
#[doc = "Write enable for mask bit of channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH1_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH1_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH1` writer - Write enable for mask bit of channel 1"]
pub type WE_CH1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WE_CH1_AW>;
impl<'a, REG, const O: u8> WE_CH1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH1_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH1_AW::VALUE2)
    }
}
#[doc = "Write enable for mask bit of channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH2_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH2_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH2` writer - Write enable for mask bit of channel 2"]
pub type WE_CH2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WE_CH2_AW>;
impl<'a, REG, const O: u8> WE_CH2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH2_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH2_AW::VALUE2)
    }
}
#[doc = "Write enable for mask bit of channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH3_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH3_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH3` writer - Write enable for mask bit of channel 3"]
pub type WE_CH3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WE_CH3_AW>;
impl<'a, REG, const O: u8> WE_CH3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH3_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WE_CH3_AW::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Mask bit for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask bit for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<MASKDSTTRAN_SPEC, 0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Mask bit for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<MASKDSTTRAN_SPEC, 1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - Mask bit for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<MASKDSTTRAN_SPEC, 2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - Mask bit for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<MASKDSTTRAN_SPEC, 3> {
        CH3_W::new(self)
    }
    #[doc = "Bit 8 - Write enable for mask bit of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch0(&mut self) -> WE_CH0_W<MASKDSTTRAN_SPEC, 8> {
        WE_CH0_W::new(self)
    }
    #[doc = "Bit 9 - Write enable for mask bit of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch1(&mut self) -> WE_CH1_W<MASKDSTTRAN_SPEC, 9> {
        WE_CH1_W::new(self)
    }
    #[doc = "Bit 10 - Write enable for mask bit of channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch2(&mut self) -> WE_CH2_W<MASKDSTTRAN_SPEC, 10> {
        WE_CH2_W::new(self)
    }
    #[doc = "Bit 11 - Write enable for mask bit of channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch3(&mut self) -> WE_CH3_W<MASKDSTTRAN_SPEC, 11> {
        WE_CH3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Mask for Raw IntBlock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maskdsttran::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maskdsttran::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MASKDSTTRAN_SPEC;
impl crate::RegisterSpec for MASKDSTTRAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maskdsttran::R`](R) reader structure"]
impl crate::Readable for MASKDSTTRAN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maskdsttran::W`](W) writer structure"]
impl crate::Writable for MASKDSTTRAN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MASKDSTTRAN to value 0"]
impl crate::Resettable for MASKDSTTRAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
