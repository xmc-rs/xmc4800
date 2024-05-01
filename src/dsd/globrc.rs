#[doc = "Register `GLOBRC` reader"]
pub type R = crate::R<GlobrcSpec>;
#[doc = "Register `GLOBRC` writer"]
pub type W = crate::W<GlobrcSpec>;
#[doc = "Channel 0 Run Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0run {
    #[doc = "0: Stop channel x"]
    Value1 = 0,
    #[doc = "1: Demodulator channel x is enabled and runs"]
    Value2 = 1,
}
impl From<Ch0run> for bool {
    #[inline(always)]
    fn from(variant: Ch0run) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0RUN` reader - Channel 0 Run Control"]
pub type Ch0runR = crate::BitReader<Ch0run>;
impl Ch0runR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0run {
        match self.bits {
            false => Ch0run::Value1,
            true => Ch0run::Value2,
        }
    }
    #[doc = "Stop channel x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ch0run::Value1
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ch0run::Value2
    }
}
#[doc = "Field `CH0RUN` writer - Channel 0 Run Control"]
pub type Ch0runW<'a, REG> = crate::BitWriter<'a, REG, Ch0run>;
impl<'a, REG> Ch0runW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop channel x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0run::Value1)
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0run::Value2)
    }
}
#[doc = "Channel 1 Run Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1run {
    #[doc = "0: Stop channel x"]
    Value1 = 0,
    #[doc = "1: Demodulator channel x is enabled and runs"]
    Value2 = 1,
}
impl From<Ch1run> for bool {
    #[inline(always)]
    fn from(variant: Ch1run) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1RUN` reader - Channel 1 Run Control"]
pub type Ch1runR = crate::BitReader<Ch1run>;
impl Ch1runR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1run {
        match self.bits {
            false => Ch1run::Value1,
            true => Ch1run::Value2,
        }
    }
    #[doc = "Stop channel x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ch1run::Value1
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ch1run::Value2
    }
}
#[doc = "Field `CH1RUN` writer - Channel 1 Run Control"]
pub type Ch1runW<'a, REG> = crate::BitWriter<'a, REG, Ch1run>;
impl<'a, REG> Ch1runW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop channel x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1run::Value1)
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1run::Value2)
    }
}
#[doc = "Channel 2 Run Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2run {
    #[doc = "0: Stop channel x"]
    Value1 = 0,
    #[doc = "1: Demodulator channel x is enabled and runs"]
    Value2 = 1,
}
impl From<Ch2run> for bool {
    #[inline(always)]
    fn from(variant: Ch2run) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2RUN` reader - Channel 2 Run Control"]
pub type Ch2runR = crate::BitReader<Ch2run>;
impl Ch2runR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2run {
        match self.bits {
            false => Ch2run::Value1,
            true => Ch2run::Value2,
        }
    }
    #[doc = "Stop channel x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ch2run::Value1
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ch2run::Value2
    }
}
#[doc = "Field `CH2RUN` writer - Channel 2 Run Control"]
pub type Ch2runW<'a, REG> = crate::BitWriter<'a, REG, Ch2run>;
impl<'a, REG> Ch2runW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop channel x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2run::Value1)
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2run::Value2)
    }
}
#[doc = "Channel 3 Run Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3run {
    #[doc = "0: Stop channel x"]
    Value1 = 0,
    #[doc = "1: Demodulator channel x is enabled and runs"]
    Value2 = 1,
}
impl From<Ch3run> for bool {
    #[inline(always)]
    fn from(variant: Ch3run) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3RUN` reader - Channel 3 Run Control"]
pub type Ch3runR = crate::BitReader<Ch3run>;
impl Ch3runR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3run {
        match self.bits {
            false => Ch3run::Value1,
            true => Ch3run::Value2,
        }
    }
    #[doc = "Stop channel x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ch3run::Value1
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ch3run::Value2
    }
}
#[doc = "Field `CH3RUN` writer - Channel 3 Run Control"]
pub type Ch3runW<'a, REG> = crate::BitWriter<'a, REG, Ch3run>;
impl<'a, REG> Ch3runW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop channel x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3run::Value1)
    }
    #[doc = "Demodulator channel x is enabled and runs"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3run::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Run Control"]
    #[inline(always)]
    pub fn ch0run(&self) -> Ch0runR {
        Ch0runR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Run Control"]
    #[inline(always)]
    pub fn ch1run(&self) -> Ch1runR {
        Ch1runR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Run Control"]
    #[inline(always)]
    pub fn ch2run(&self) -> Ch2runR {
        Ch2runR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Run Control"]
    #[inline(always)]
    pub fn ch3run(&self) -> Ch3runR {
        Ch3runR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Run Control"]
    #[inline(always)]
    #[must_use]
    pub fn ch0run(&mut self) -> Ch0runW<GlobrcSpec> {
        Ch0runW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Run Control"]
    #[inline(always)]
    #[must_use]
    pub fn ch1run(&mut self) -> Ch1runW<GlobrcSpec> {
        Ch1runW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Run Control"]
    #[inline(always)]
    #[must_use]
    pub fn ch2run(&mut self) -> Ch2runW<GlobrcSpec> {
        Ch2runW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Run Control"]
    #[inline(always)]
    #[must_use]
    pub fn ch3run(&mut self) -> Ch3runW<GlobrcSpec> {
        Ch3runW::new(self, 3)
    }
}
#[doc = "Global Run Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlobrcSpec;
impl crate::RegisterSpec for GlobrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`globrc::R`](R) reader structure"]
impl crate::Readable for GlobrcSpec {}
#[doc = "`write(|w| ..)` method takes [`globrc::W`](W) writer structure"]
impl crate::Writable for GlobrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOBRC to value 0"]
impl crate::Resettable for GlobrcSpec {
    const RESET_VALUE: u32 = 0;
}
