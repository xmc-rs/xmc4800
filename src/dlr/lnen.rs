#[doc = "Register `LNEN` reader"]
pub type R = crate::R<LNEN_SPEC>;
#[doc = "Register `LNEN` writer"]
pub type W = crate::W<LNEN_SPEC>;
#[doc = "Line 0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LN0_A {
    #[doc = "0: Disables the line"]
    VALUE1 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    VALUE2 = 1,
}
impl From<LN0_A> for bool {
    #[inline(always)]
    fn from(variant: LN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LN0` reader - Line 0 Enable"]
pub type LN0_R = crate::BitReader<LN0_A>;
impl LN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LN0_A {
        match self.bits {
            false => LN0_A::VALUE1,
            true => LN0_A::VALUE2,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LN0_A::VALUE1
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LN0_A::VALUE2
    }
}
#[doc = "Field `LN0` writer - Line 0 Enable"]
pub type LN0_W<'a, REG> = crate::BitWriter<'a, REG, LN0_A>;
impl<'a, REG> LN0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LN0_A::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LN0_A::VALUE2)
    }
}
#[doc = "Line 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LN1_A {
    #[doc = "0: Disables the line"]
    VALUE1 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    VALUE2 = 1,
}
impl From<LN1_A> for bool {
    #[inline(always)]
    fn from(variant: LN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LN1` reader - Line 1 Enable"]
pub type LN1_R = crate::BitReader<LN1_A>;
impl LN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LN1_A {
        match self.bits {
            false => LN1_A::VALUE1,
            true => LN1_A::VALUE2,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LN1_A::VALUE1
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LN1_A::VALUE2
    }
}
#[doc = "Field `LN1` writer - Line 1 Enable"]
pub type LN1_W<'a, REG> = crate::BitWriter<'a, REG, LN1_A>;
impl<'a, REG> LN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LN1_A::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LN1_A::VALUE2)
    }
}
#[doc = "Line 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LN2_A {
    #[doc = "0: Disables the line"]
    VALUE1 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    VALUE2 = 1,
}
impl From<LN2_A> for bool {
    #[inline(always)]
    fn from(variant: LN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LN2` reader - Line 2 Enable"]
pub type LN2_R = crate::BitReader<LN2_A>;
impl LN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LN2_A {
        match self.bits {
            false => LN2_A::VALUE1,
            true => LN2_A::VALUE2,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LN2_A::VALUE1
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LN2_A::VALUE2
    }
}
#[doc = "Field `LN2` writer - Line 2 Enable"]
pub type LN2_W<'a, REG> = crate::BitWriter<'a, REG, LN2_A>;
impl<'a, REG> LN2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LN2_A::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LN2_A::VALUE2)
    }
}
#[doc = "Line 3 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LN3_A {
    #[doc = "0: Disables the line"]
    VALUE1 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    VALUE2 = 1,
}
impl From<LN3_A> for bool {
    #[inline(always)]
    fn from(variant: LN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LN3` reader - Line 3 Enable"]
pub type LN3_R = crate::BitReader<LN3_A>;
impl LN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LN3_A {
        match self.bits {
            false => LN3_A::VALUE1,
            true => LN3_A::VALUE2,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LN3_A::VALUE1
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LN3_A::VALUE2
    }
}
#[doc = "Field `LN3` writer - Line 3 Enable"]
pub type LN3_W<'a, REG> = crate::BitWriter<'a, REG, LN3_A>;
impl<'a, REG> LN3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LN3_A::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LN3_A::VALUE2)
    }
}
#[doc = "Line 4 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LN4_A {
    #[doc = "0: Disables the line"]
    VALUE1 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    VALUE2 = 1,
}
impl From<LN4_A> for bool {
    #[inline(always)]
    fn from(variant: LN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LN4` reader - Line 4 Enable"]
pub type LN4_R = crate::BitReader<LN4_A>;
impl LN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LN4_A {
        match self.bits {
            false => LN4_A::VALUE1,
            true => LN4_A::VALUE2,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LN4_A::VALUE1
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LN4_A::VALUE2
    }
}
#[doc = "Field `LN4` writer - Line 4 Enable"]
pub type LN4_W<'a, REG> = crate::BitWriter<'a, REG, LN4_A>;
impl<'a, REG> LN4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LN4_A::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LN4_A::VALUE2)
    }
}
#[doc = "Line 5 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LN5_A {
    #[doc = "0: Disables the line"]
    VALUE1 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    VALUE2 = 1,
}
impl From<LN5_A> for bool {
    #[inline(always)]
    fn from(variant: LN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LN5` reader - Line 5 Enable"]
pub type LN5_R = crate::BitReader<LN5_A>;
impl LN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LN5_A {
        match self.bits {
            false => LN5_A::VALUE1,
            true => LN5_A::VALUE2,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LN5_A::VALUE1
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LN5_A::VALUE2
    }
}
#[doc = "Field `LN5` writer - Line 5 Enable"]
pub type LN5_W<'a, REG> = crate::BitWriter<'a, REG, LN5_A>;
impl<'a, REG> LN5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LN5_A::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LN5_A::VALUE2)
    }
}
#[doc = "Line 6 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LN6_A {
    #[doc = "0: Disables the line"]
    VALUE1 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    VALUE2 = 1,
}
impl From<LN6_A> for bool {
    #[inline(always)]
    fn from(variant: LN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LN6` reader - Line 6 Enable"]
pub type LN6_R = crate::BitReader<LN6_A>;
impl LN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LN6_A {
        match self.bits {
            false => LN6_A::VALUE1,
            true => LN6_A::VALUE2,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LN6_A::VALUE1
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LN6_A::VALUE2
    }
}
#[doc = "Field `LN6` writer - Line 6 Enable"]
pub type LN6_W<'a, REG> = crate::BitWriter<'a, REG, LN6_A>;
impl<'a, REG> LN6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LN6_A::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LN6_A::VALUE2)
    }
}
#[doc = "Line 7 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LN7_A {
    #[doc = "0: Disables the line"]
    VALUE1 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    VALUE2 = 1,
}
impl From<LN7_A> for bool {
    #[inline(always)]
    fn from(variant: LN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LN7` reader - Line 7 Enable"]
pub type LN7_R = crate::BitReader<LN7_A>;
impl LN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LN7_A {
        match self.bits {
            false => LN7_A::VALUE1,
            true => LN7_A::VALUE2,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LN7_A::VALUE1
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LN7_A::VALUE2
    }
}
#[doc = "Field `LN7` writer - Line 7 Enable"]
pub type LN7_W<'a, REG> = crate::BitWriter<'a, REG, LN7_A>;
impl<'a, REG> LN7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LN7_A::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LN7_A::VALUE2)
    }
}
#[doc = "Line 8 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LN8_A {
    #[doc = "0: Disables the line"]
    VALUE1 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    VALUE2 = 1,
}
impl From<LN8_A> for bool {
    #[inline(always)]
    fn from(variant: LN8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LN8` reader - Line 8 Enable"]
pub type LN8_R = crate::BitReader<LN8_A>;
impl LN8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LN8_A {
        match self.bits {
            false => LN8_A::VALUE1,
            true => LN8_A::VALUE2,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LN8_A::VALUE1
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LN8_A::VALUE2
    }
}
#[doc = "Field `LN8` writer - Line 8 Enable"]
pub type LN8_W<'a, REG> = crate::BitWriter<'a, REG, LN8_A>;
impl<'a, REG> LN8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LN8_A::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LN8_A::VALUE2)
    }
}
#[doc = "Line 9 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LN9_A {
    #[doc = "0: Disables the line"]
    VALUE1 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    VALUE2 = 1,
}
impl From<LN9_A> for bool {
    #[inline(always)]
    fn from(variant: LN9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LN9` reader - Line 9 Enable"]
pub type LN9_R = crate::BitReader<LN9_A>;
impl LN9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LN9_A {
        match self.bits {
            false => LN9_A::VALUE1,
            true => LN9_A::VALUE2,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LN9_A::VALUE1
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LN9_A::VALUE2
    }
}
#[doc = "Field `LN9` writer - Line 9 Enable"]
pub type LN9_W<'a, REG> = crate::BitWriter<'a, REG, LN9_A>;
impl<'a, REG> LN9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LN9_A::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LN9_A::VALUE2)
    }
}
#[doc = "Line 10 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LN10_A {
    #[doc = "0: Disables the line"]
    VALUE1 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    VALUE2 = 1,
}
impl From<LN10_A> for bool {
    #[inline(always)]
    fn from(variant: LN10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LN10` reader - Line 10 Enable"]
pub type LN10_R = crate::BitReader<LN10_A>;
impl LN10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LN10_A {
        match self.bits {
            false => LN10_A::VALUE1,
            true => LN10_A::VALUE2,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LN10_A::VALUE1
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LN10_A::VALUE2
    }
}
#[doc = "Field `LN10` writer - Line 10 Enable"]
pub type LN10_W<'a, REG> = crate::BitWriter<'a, REG, LN10_A>;
impl<'a, REG> LN10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LN10_A::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LN10_A::VALUE2)
    }
}
#[doc = "Line 11 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LN11_A {
    #[doc = "0: Disables the line"]
    VALUE1 = 0,
    #[doc = "1: Enables the line and resets a pending request"]
    VALUE2 = 1,
}
impl From<LN11_A> for bool {
    #[inline(always)]
    fn from(variant: LN11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LN11` reader - Line 11 Enable"]
pub type LN11_R = crate::BitReader<LN11_A>;
impl LN11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LN11_A {
        match self.bits {
            false => LN11_A::VALUE1,
            true => LN11_A::VALUE2,
        }
    }
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LN11_A::VALUE1
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LN11_A::VALUE2
    }
}
#[doc = "Field `LN11` writer - Line 11 Enable"]
pub type LN11_W<'a, REG> = crate::BitWriter<'a, REG, LN11_A>;
impl<'a, REG> LN11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the line"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LN11_A::VALUE1)
    }
    #[doc = "Enables the line and resets a pending request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LN11_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Line 0 Enable"]
    #[inline(always)]
    pub fn ln0(&self) -> LN0_R {
        LN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Line 1 Enable"]
    #[inline(always)]
    pub fn ln1(&self) -> LN1_R {
        LN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Line 2 Enable"]
    #[inline(always)]
    pub fn ln2(&self) -> LN2_R {
        LN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Line 3 Enable"]
    #[inline(always)]
    pub fn ln3(&self) -> LN3_R {
        LN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line 4 Enable"]
    #[inline(always)]
    pub fn ln4(&self) -> LN4_R {
        LN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Line 5 Enable"]
    #[inline(always)]
    pub fn ln5(&self) -> LN5_R {
        LN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Line 6 Enable"]
    #[inline(always)]
    pub fn ln6(&self) -> LN6_R {
        LN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Line 7 Enable"]
    #[inline(always)]
    pub fn ln7(&self) -> LN7_R {
        LN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Line 8 Enable"]
    #[inline(always)]
    pub fn ln8(&self) -> LN8_R {
        LN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Line 9 Enable"]
    #[inline(always)]
    pub fn ln9(&self) -> LN9_R {
        LN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Line 10 Enable"]
    #[inline(always)]
    pub fn ln10(&self) -> LN10_R {
        LN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Line 11 Enable"]
    #[inline(always)]
    pub fn ln11(&self) -> LN11_R {
        LN11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Line 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln0(&mut self) -> LN0_W<LNEN_SPEC> {
        LN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Line 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln1(&mut self) -> LN1_W<LNEN_SPEC> {
        LN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Line 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln2(&mut self) -> LN2_W<LNEN_SPEC> {
        LN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Line 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln3(&mut self) -> LN3_W<LNEN_SPEC> {
        LN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Line 4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln4(&mut self) -> LN4_W<LNEN_SPEC> {
        LN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Line 5 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln5(&mut self) -> LN5_W<LNEN_SPEC> {
        LN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Line 6 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln6(&mut self) -> LN6_W<LNEN_SPEC> {
        LN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Line 7 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln7(&mut self) -> LN7_W<LNEN_SPEC> {
        LN7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Line 8 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln8(&mut self) -> LN8_W<LNEN_SPEC> {
        LN8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Line 9 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln9(&mut self) -> LN9_W<LNEN_SPEC> {
        LN9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Line 10 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln10(&mut self) -> LN10_W<LNEN_SPEC> {
        LN10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Line 11 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ln11(&mut self) -> LN11_W<LNEN_SPEC> {
        LN11_W::new(self, 11)
    }
}
#[doc = "Line Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`lnen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lnen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LNEN_SPEC;
impl crate::RegisterSpec for LNEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lnen::R`](R) reader structure"]
impl crate::Readable for LNEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lnen::W`](W) writer structure"]
impl crate::Writable for LNEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LNEN to value 0"]
impl crate::Resettable for LNEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
