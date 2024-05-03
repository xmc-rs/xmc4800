#[doc = "Register `CCUCON` reader"]
pub type R = crate::R<CCUCON_SPEC>;
#[doc = "Register `CCUCON` writer"]
pub type W = crate::W<CCUCON_SPEC>;
#[doc = "Global Start Control CCU40\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSC40_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<GSC40_A> for bool {
    #[inline(always)]
    fn from(variant: GSC40_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GSC40` reader - Global Start Control CCU40"]
pub type GSC40_R = crate::BitReader<GSC40_A>;
impl GSC40_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GSC40_A {
        match self.bits {
            false => GSC40_A::VALUE1,
            true => GSC40_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GSC40_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GSC40_A::VALUE2
    }
}
#[doc = "Field `GSC40` writer - Global Start Control CCU40"]
pub type GSC40_W<'a, REG> = crate::BitWriter<'a, REG, GSC40_A>;
impl<'a, REG> GSC40_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(GSC40_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(GSC40_A::VALUE2)
    }
}
#[doc = "Global Start Control CCU41\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSC41_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<GSC41_A> for bool {
    #[inline(always)]
    fn from(variant: GSC41_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GSC41` reader - Global Start Control CCU41"]
pub type GSC41_R = crate::BitReader<GSC41_A>;
impl GSC41_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GSC41_A {
        match self.bits {
            false => GSC41_A::VALUE1,
            true => GSC41_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GSC41_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GSC41_A::VALUE2
    }
}
#[doc = "Field `GSC41` writer - Global Start Control CCU41"]
pub type GSC41_W<'a, REG> = crate::BitWriter<'a, REG, GSC41_A>;
impl<'a, REG> GSC41_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(GSC41_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(GSC41_A::VALUE2)
    }
}
#[doc = "Global Start Control CCU42\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSC42_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<GSC42_A> for bool {
    #[inline(always)]
    fn from(variant: GSC42_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GSC42` reader - Global Start Control CCU42"]
pub type GSC42_R = crate::BitReader<GSC42_A>;
impl GSC42_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GSC42_A {
        match self.bits {
            false => GSC42_A::VALUE1,
            true => GSC42_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GSC42_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GSC42_A::VALUE2
    }
}
#[doc = "Field `GSC42` writer - Global Start Control CCU42"]
pub type GSC42_W<'a, REG> = crate::BitWriter<'a, REG, GSC42_A>;
impl<'a, REG> GSC42_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(GSC42_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(GSC42_A::VALUE2)
    }
}
#[doc = "Global Start Control CCU43\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSC43_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<GSC43_A> for bool {
    #[inline(always)]
    fn from(variant: GSC43_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GSC43` reader - Global Start Control CCU43"]
pub type GSC43_R = crate::BitReader<GSC43_A>;
impl GSC43_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GSC43_A {
        match self.bits {
            false => GSC43_A::VALUE1,
            true => GSC43_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GSC43_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GSC43_A::VALUE2
    }
}
#[doc = "Field `GSC43` writer - Global Start Control CCU43"]
pub type GSC43_W<'a, REG> = crate::BitWriter<'a, REG, GSC43_A>;
impl<'a, REG> GSC43_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(GSC43_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(GSC43_A::VALUE2)
    }
}
#[doc = "Global Start Control CCU80\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSC80_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<GSC80_A> for bool {
    #[inline(always)]
    fn from(variant: GSC80_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GSC80` reader - Global Start Control CCU80"]
pub type GSC80_R = crate::BitReader<GSC80_A>;
impl GSC80_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GSC80_A {
        match self.bits {
            false => GSC80_A::VALUE1,
            true => GSC80_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GSC80_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GSC80_A::VALUE2
    }
}
#[doc = "Field `GSC80` writer - Global Start Control CCU80"]
pub type GSC80_W<'a, REG> = crate::BitWriter<'a, REG, GSC80_A>;
impl<'a, REG> GSC80_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(GSC80_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(GSC80_A::VALUE2)
    }
}
#[doc = "Global Start Control CCU81\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSC81_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<GSC81_A> for bool {
    #[inline(always)]
    fn from(variant: GSC81_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GSC81` reader - Global Start Control CCU81"]
pub type GSC81_R = crate::BitReader<GSC81_A>;
impl GSC81_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GSC81_A {
        match self.bits {
            false => GSC81_A::VALUE1,
            true => GSC81_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GSC81_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GSC81_A::VALUE2
    }
}
#[doc = "Field `GSC81` writer - Global Start Control CCU81"]
pub type GSC81_W<'a, REG> = crate::BitWriter<'a, REG, GSC81_A>;
impl<'a, REG> GSC81_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(GSC81_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(GSC81_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Global Start Control CCU40"]
    #[inline(always)]
    pub fn gsc40(&self) -> GSC40_R {
        GSC40_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Global Start Control CCU41"]
    #[inline(always)]
    pub fn gsc41(&self) -> GSC41_R {
        GSC41_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global Start Control CCU42"]
    #[inline(always)]
    pub fn gsc42(&self) -> GSC42_R {
        GSC42_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global Start Control CCU43"]
    #[inline(always)]
    pub fn gsc43(&self) -> GSC43_R {
        GSC43_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Global Start Control CCU80"]
    #[inline(always)]
    pub fn gsc80(&self) -> GSC80_R {
        GSC80_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Start Control CCU81"]
    #[inline(always)]
    pub fn gsc81(&self) -> GSC81_R {
        GSC81_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Start Control CCU40"]
    #[inline(always)]
    #[must_use]
    pub fn gsc40(&mut self) -> GSC40_W<CCUCON_SPEC> {
        GSC40_W::new(self, 0)
    }
    #[doc = "Bit 1 - Global Start Control CCU41"]
    #[inline(always)]
    #[must_use]
    pub fn gsc41(&mut self) -> GSC41_W<CCUCON_SPEC> {
        GSC41_W::new(self, 1)
    }
    #[doc = "Bit 2 - Global Start Control CCU42"]
    #[inline(always)]
    #[must_use]
    pub fn gsc42(&mut self) -> GSC42_W<CCUCON_SPEC> {
        GSC42_W::new(self, 2)
    }
    #[doc = "Bit 3 - Global Start Control CCU43"]
    #[inline(always)]
    #[must_use]
    pub fn gsc43(&mut self) -> GSC43_W<CCUCON_SPEC> {
        GSC43_W::new(self, 3)
    }
    #[doc = "Bit 8 - Global Start Control CCU80"]
    #[inline(always)]
    #[must_use]
    pub fn gsc80(&mut self) -> GSC80_W<CCUCON_SPEC> {
        GSC80_W::new(self, 8)
    }
    #[doc = "Bit 9 - Global Start Control CCU81"]
    #[inline(always)]
    #[must_use]
    pub fn gsc81(&mut self) -> GSC81_W<CCUCON_SPEC> {
        GSC81_W::new(self, 9)
    }
}
#[doc = "CCU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccucon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccucon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCUCON_SPEC;
impl crate::RegisterSpec for CCUCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccucon::R`](R) reader structure"]
impl crate::Readable for CCUCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccucon::W`](W) writer structure"]
impl crate::Writable for CCUCON_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCUCON to value 0"]
impl crate::Resettable for CCUCON_SPEC {
    const RESET_VALUE: u32 = 0;
}
