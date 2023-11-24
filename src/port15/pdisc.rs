#[doc = "Register `PDISC` reader"]
pub type R = crate::R<PDISC_SPEC>;
#[doc = "Register `PDISC` writer"]
pub type W = crate::W<PDISC_SPEC>;
#[doc = "Field `PDIS2` reader - Pad Disable for Port 15 Pin 2"]
pub type PDIS2_R = crate::BitReader<PDIS2_A>;
#[doc = "Pad Disable for Port 15 Pin 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS2_A {
    #[doc = "0: Pad is enabled, digital input selected."]
    VALUE1 = 0,
    #[doc = "1: Pad is disabled, ADC 2 analog input 2."]
    VALUE2 = 1,
}
impl From<PDIS2_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS2_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS2_A {
        match self.bits {
            false => PDIS2_A::VALUE1,
            true => PDIS2_A::VALUE2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS2_A::VALUE1
    }
    #[doc = "Pad is disabled, ADC 2 analog input 2."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS2_A::VALUE2
    }
}
#[doc = "Field `PDIS2` writer - Pad Disable for Port 15 Pin 2"]
pub type PDIS2_W<'a, REG> = crate::BitWriter<'a, REG, PDIS2_A>;
impl<'a, REG> PDIS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS2_A::VALUE1)
    }
    #[doc = "Pad is disabled, ADC 2 analog input 2."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS2_A::VALUE2)
    }
}
#[doc = "Field `PDIS3` reader - Pad Disable for Port 15 Pin 3"]
pub type PDIS3_R = crate::BitReader<PDIS3_A>;
#[doc = "Pad Disable for Port 15 Pin 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS3_A {
    #[doc = "0: Pad is enabled, digital input selected."]
    VALUE1 = 0,
    #[doc = "1: Pad is disabled, ADC 2 analog input 3."]
    VALUE2 = 1,
}
impl From<PDIS3_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS3_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS3_A {
        match self.bits {
            false => PDIS3_A::VALUE1,
            true => PDIS3_A::VALUE2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS3_A::VALUE1
    }
    #[doc = "Pad is disabled, ADC 2 analog input 3."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS3_A::VALUE2
    }
}
#[doc = "Field `PDIS3` writer - Pad Disable for Port 15 Pin 3"]
pub type PDIS3_W<'a, REG> = crate::BitWriter<'a, REG, PDIS3_A>;
impl<'a, REG> PDIS3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS3_A::VALUE1)
    }
    #[doc = "Pad is disabled, ADC 2 analog input 3."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS3_A::VALUE2)
    }
}
#[doc = "Field `PDIS4` reader - Pad Disable for Port 15 Pin 4"]
pub type PDIS4_R = crate::BitReader<PDIS4_A>;
#[doc = "Pad Disable for Port 15 Pin 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS4_A {
    #[doc = "0: Pad is enabled, digital input selected."]
    VALUE1 = 0,
    #[doc = "1: Pad is disabled, ADC 2 analog input 4."]
    VALUE2 = 1,
}
impl From<PDIS4_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS4_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS4_A {
        match self.bits {
            false => PDIS4_A::VALUE1,
            true => PDIS4_A::VALUE2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS4_A::VALUE1
    }
    #[doc = "Pad is disabled, ADC 2 analog input 4."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS4_A::VALUE2
    }
}
#[doc = "Field `PDIS4` writer - Pad Disable for Port 15 Pin 4"]
pub type PDIS4_W<'a, REG> = crate::BitWriter<'a, REG, PDIS4_A>;
impl<'a, REG> PDIS4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS4_A::VALUE1)
    }
    #[doc = "Pad is disabled, ADC 2 analog input 4."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS4_A::VALUE2)
    }
}
#[doc = "Field `PDIS5` reader - Pad Disable for Port 15 Pin 5"]
pub type PDIS5_R = crate::BitReader<PDIS5_A>;
#[doc = "Pad Disable for Port 15 Pin 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS5_A {
    #[doc = "0: Pad is enabled, digital input selected."]
    VALUE1 = 0,
    #[doc = "1: Pad is disabled, ADC 2 analog input 5."]
    VALUE2 = 1,
}
impl From<PDIS5_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS5_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS5_A {
        match self.bits {
            false => PDIS5_A::VALUE1,
            true => PDIS5_A::VALUE2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS5_A::VALUE1
    }
    #[doc = "Pad is disabled, ADC 2 analog input 5."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS5_A::VALUE2
    }
}
#[doc = "Field `PDIS5` writer - Pad Disable for Port 15 Pin 5"]
pub type PDIS5_W<'a, REG> = crate::BitWriter<'a, REG, PDIS5_A>;
impl<'a, REG> PDIS5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS5_A::VALUE1)
    }
    #[doc = "Pad is disabled, ADC 2 analog input 5."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS5_A::VALUE2)
    }
}
#[doc = "Field `PDIS6` reader - Pad Disable for Port 15 Pin 6"]
pub type PDIS6_R = crate::BitReader<PDIS6_A>;
#[doc = "Pad Disable for Port 15 Pin 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS6_A {
    #[doc = "0: Pad is enabled, digital input selected."]
    VALUE1 = 0,
    #[doc = "1: Pad is disabled, ADC 2 analog input 6."]
    VALUE2 = 1,
}
impl From<PDIS6_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS6_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS6_A {
        match self.bits {
            false => PDIS6_A::VALUE1,
            true => PDIS6_A::VALUE2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS6_A::VALUE1
    }
    #[doc = "Pad is disabled, ADC 2 analog input 6."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS6_A::VALUE2
    }
}
#[doc = "Field `PDIS6` writer - Pad Disable for Port 15 Pin 6"]
pub type PDIS6_W<'a, REG> = crate::BitWriter<'a, REG, PDIS6_A>;
impl<'a, REG> PDIS6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS6_A::VALUE1)
    }
    #[doc = "Pad is disabled, ADC 2 analog input 6."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS6_A::VALUE2)
    }
}
#[doc = "Field `PDIS7` reader - Pad Disable for Port 15 Pin 7"]
pub type PDIS7_R = crate::BitReader<PDIS7_A>;
#[doc = "Pad Disable for Port 15 Pin 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS7_A {
    #[doc = "0: Pad is enabled, digital input selected."]
    VALUE1 = 0,
    #[doc = "1: Pad is disabled, ADC 2 analog input 7."]
    VALUE2 = 1,
}
impl From<PDIS7_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS7_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS7_A {
        match self.bits {
            false => PDIS7_A::VALUE1,
            true => PDIS7_A::VALUE2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS7_A::VALUE1
    }
    #[doc = "Pad is disabled, ADC 2 analog input 7."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS7_A::VALUE2
    }
}
#[doc = "Field `PDIS7` writer - Pad Disable for Port 15 Pin 7"]
pub type PDIS7_W<'a, REG> = crate::BitWriter<'a, REG, PDIS7_A>;
impl<'a, REG> PDIS7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS7_A::VALUE1)
    }
    #[doc = "Pad is disabled, ADC 2 analog input 7."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS7_A::VALUE2)
    }
}
#[doc = "Field `PDIS8` reader - Pad Disable for Port 15 Pin 8"]
pub type PDIS8_R = crate::BitReader<PDIS8_A>;
#[doc = "Pad Disable for Port 15 Pin 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS8_A {
    #[doc = "0: Pad is enabled, digital input selected."]
    VALUE1 = 0,
    #[doc = "1: Pad is disabled, ADC 3 analog input 0."]
    VALUE2 = 1,
}
impl From<PDIS8_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS8_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS8_A {
        match self.bits {
            false => PDIS8_A::VALUE1,
            true => PDIS8_A::VALUE2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS8_A::VALUE1
    }
    #[doc = "Pad is disabled, ADC 3 analog input 0."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS8_A::VALUE2
    }
}
#[doc = "Field `PDIS8` writer - Pad Disable for Port 15 Pin 8"]
pub type PDIS8_W<'a, REG> = crate::BitWriter<'a, REG, PDIS8_A>;
impl<'a, REG> PDIS8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS8_A::VALUE1)
    }
    #[doc = "Pad is disabled, ADC 3 analog input 0."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS8_A::VALUE2)
    }
}
#[doc = "Field `PDIS9` reader - Pad Disable for Port 15 Pin 9"]
pub type PDIS9_R = crate::BitReader<PDIS9_A>;
#[doc = "Pad Disable for Port 15 Pin 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS9_A {
    #[doc = "0: Pad is enabled, digital input selected."]
    VALUE1 = 0,
    #[doc = "1: Pad is disabled, ADC 3 analog input 1."]
    VALUE2 = 1,
}
impl From<PDIS9_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS9_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS9_A {
        match self.bits {
            false => PDIS9_A::VALUE1,
            true => PDIS9_A::VALUE2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS9_A::VALUE1
    }
    #[doc = "Pad is disabled, ADC 3 analog input 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS9_A::VALUE2
    }
}
#[doc = "Field `PDIS9` writer - Pad Disable for Port 15 Pin 9"]
pub type PDIS9_W<'a, REG> = crate::BitWriter<'a, REG, PDIS9_A>;
impl<'a, REG> PDIS9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS9_A::VALUE1)
    }
    #[doc = "Pad is disabled, ADC 3 analog input 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS9_A::VALUE2)
    }
}
#[doc = "Field `PDIS12` reader - Pad Disable for Port 15 Pin 12"]
pub type PDIS12_R = crate::BitReader<PDIS12_A>;
#[doc = "Pad Disable for Port 15 Pin 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS12_A {
    #[doc = "0: Pad is enabled, digital input selected."]
    VALUE1 = 0,
    #[doc = "1: Pad is disabled, ADC 3 analog input 4."]
    VALUE2 = 1,
}
impl From<PDIS12_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS12_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS12_A {
        match self.bits {
            false => PDIS12_A::VALUE1,
            true => PDIS12_A::VALUE2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS12_A::VALUE1
    }
    #[doc = "Pad is disabled, ADC 3 analog input 4."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS12_A::VALUE2
    }
}
#[doc = "Field `PDIS12` writer - Pad Disable for Port 15 Pin 12"]
pub type PDIS12_W<'a, REG> = crate::BitWriter<'a, REG, PDIS12_A>;
impl<'a, REG> PDIS12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS12_A::VALUE1)
    }
    #[doc = "Pad is disabled, ADC 3 analog input 4."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS12_A::VALUE2)
    }
}
#[doc = "Field `PDIS13` reader - Pad Disable for Port 15 Pin 13"]
pub type PDIS13_R = crate::BitReader<PDIS13_A>;
#[doc = "Pad Disable for Port 15 Pin 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS13_A {
    #[doc = "0: Pad is enabled, digital input selected."]
    VALUE1 = 0,
    #[doc = "1: Pad is disabled, ADC 3 analog input 5."]
    VALUE2 = 1,
}
impl From<PDIS13_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS13_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS13_A {
        match self.bits {
            false => PDIS13_A::VALUE1,
            true => PDIS13_A::VALUE2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS13_A::VALUE1
    }
    #[doc = "Pad is disabled, ADC 3 analog input 5."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS13_A::VALUE2
    }
}
#[doc = "Field `PDIS13` writer - Pad Disable for Port 15 Pin 13"]
pub type PDIS13_W<'a, REG> = crate::BitWriter<'a, REG, PDIS13_A>;
impl<'a, REG> PDIS13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS13_A::VALUE1)
    }
    #[doc = "Pad is disabled, ADC 3 analog input 5."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS13_A::VALUE2)
    }
}
#[doc = "Field `PDIS14` reader - Pad Disable for Port 15 Pin 14"]
pub type PDIS14_R = crate::BitReader<PDIS14_A>;
#[doc = "Pad Disable for Port 15 Pin 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS14_A {
    #[doc = "0: Pad is enabled, digital input selected."]
    VALUE1 = 0,
    #[doc = "1: Pad is disabled, ADC 3 analog input 6."]
    VALUE2 = 1,
}
impl From<PDIS14_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS14_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS14_A {
        match self.bits {
            false => PDIS14_A::VALUE1,
            true => PDIS14_A::VALUE2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS14_A::VALUE1
    }
    #[doc = "Pad is disabled, ADC 3 analog input 6."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS14_A::VALUE2
    }
}
#[doc = "Field `PDIS14` writer - Pad Disable for Port 15 Pin 14"]
pub type PDIS14_W<'a, REG> = crate::BitWriter<'a, REG, PDIS14_A>;
impl<'a, REG> PDIS14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS14_A::VALUE1)
    }
    #[doc = "Pad is disabled, ADC 3 analog input 6."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS14_A::VALUE2)
    }
}
#[doc = "Field `PDIS15` reader - Pad Disable for Port 15 Pin 15"]
pub type PDIS15_R = crate::BitReader<PDIS15_A>;
#[doc = "Pad Disable for Port 15 Pin 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDIS15_A {
    #[doc = "0: Pad is enabled, digital input selected."]
    VALUE1 = 0,
    #[doc = "1: Pad is disabled, ADC 3 analog input 7."]
    VALUE2 = 1,
}
impl From<PDIS15_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS15_A) -> Self {
        variant as u8 != 0
    }
}
impl PDIS15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIS15_A {
        match self.bits {
            false => PDIS15_A::VALUE1,
            true => PDIS15_A::VALUE2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS15_A::VALUE1
    }
    #[doc = "Pad is disabled, ADC 3 analog input 7."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS15_A::VALUE2
    }
}
#[doc = "Field `PDIS15` writer - Pad Disable for Port 15 Pin 15"]
pub type PDIS15_W<'a, REG> = crate::BitWriter<'a, REG, PDIS15_A>;
impl<'a, REG> PDIS15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS15_A::VALUE1)
    }
    #[doc = "Pad is disabled, ADC 3 analog input 7."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PDIS15_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 2 - Pad Disable for Port 15 Pin 2"]
    #[inline(always)]
    pub fn pdis2(&self) -> PDIS2_R {
        PDIS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pad Disable for Port 15 Pin 3"]
    #[inline(always)]
    pub fn pdis3(&self) -> PDIS3_R {
        PDIS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pad Disable for Port 15 Pin 4"]
    #[inline(always)]
    pub fn pdis4(&self) -> PDIS4_R {
        PDIS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pad Disable for Port 15 Pin 5"]
    #[inline(always)]
    pub fn pdis5(&self) -> PDIS5_R {
        PDIS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pad Disable for Port 15 Pin 6"]
    #[inline(always)]
    pub fn pdis6(&self) -> PDIS6_R {
        PDIS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pad Disable for Port 15 Pin 7"]
    #[inline(always)]
    pub fn pdis7(&self) -> PDIS7_R {
        PDIS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pad Disable for Port 15 Pin 8"]
    #[inline(always)]
    pub fn pdis8(&self) -> PDIS8_R {
        PDIS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pad Disable for Port 15 Pin 9"]
    #[inline(always)]
    pub fn pdis9(&self) -> PDIS9_R {
        PDIS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Pad Disable for Port 15 Pin 12"]
    #[inline(always)]
    pub fn pdis12(&self) -> PDIS12_R {
        PDIS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pad Disable for Port 15 Pin 13"]
    #[inline(always)]
    pub fn pdis13(&self) -> PDIS13_R {
        PDIS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pad Disable for Port 15 Pin 14"]
    #[inline(always)]
    pub fn pdis14(&self) -> PDIS14_R {
        PDIS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pad Disable for Port 15 Pin 15"]
    #[inline(always)]
    pub fn pdis15(&self) -> PDIS15_R {
        PDIS15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Pad Disable for Port 15 Pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn pdis2(&mut self) -> PDIS2_W<PDISC_SPEC> {
        PDIS2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pad Disable for Port 15 Pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn pdis3(&mut self) -> PDIS3_W<PDISC_SPEC> {
        PDIS3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pad Disable for Port 15 Pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn pdis4(&mut self) -> PDIS4_W<PDISC_SPEC> {
        PDIS4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pad Disable for Port 15 Pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn pdis5(&mut self) -> PDIS5_W<PDISC_SPEC> {
        PDIS5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pad Disable for Port 15 Pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn pdis6(&mut self) -> PDIS6_W<PDISC_SPEC> {
        PDIS6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Pad Disable for Port 15 Pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn pdis7(&mut self) -> PDIS7_W<PDISC_SPEC> {
        PDIS7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Pad Disable for Port 15 Pin 8"]
    #[inline(always)]
    #[must_use]
    pub fn pdis8(&mut self) -> PDIS8_W<PDISC_SPEC> {
        PDIS8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Pad Disable for Port 15 Pin 9"]
    #[inline(always)]
    #[must_use]
    pub fn pdis9(&mut self) -> PDIS9_W<PDISC_SPEC> {
        PDIS9_W::new(self, 9)
    }
    #[doc = "Bit 12 - Pad Disable for Port 15 Pin 12"]
    #[inline(always)]
    #[must_use]
    pub fn pdis12(&mut self) -> PDIS12_W<PDISC_SPEC> {
        PDIS12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Pad Disable for Port 15 Pin 13"]
    #[inline(always)]
    #[must_use]
    pub fn pdis13(&mut self) -> PDIS13_W<PDISC_SPEC> {
        PDIS13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Pad Disable for Port 15 Pin 14"]
    #[inline(always)]
    #[must_use]
    pub fn pdis14(&mut self) -> PDIS14_W<PDISC_SPEC> {
        PDIS14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Pad Disable for Port 15 Pin 15"]
    #[inline(always)]
    #[must_use]
    pub fn pdis15(&mut self) -> PDIS15_W<PDISC_SPEC> {
        PDIS15_W::new(self, 15)
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
#[doc = "Port 15 Pin Function Decision Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdisc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdisc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDISC_SPEC;
impl crate::RegisterSpec for PDISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdisc::R`](R) reader structure"]
impl crate::Readable for PDISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pdisc::W`](W) writer structure"]
impl crate::Writable for PDISC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDISC to value 0"]
impl crate::Resettable for PDISC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
