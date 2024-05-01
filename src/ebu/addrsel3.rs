#[doc = "Register `ADDRSEL3` reader"]
pub type R = crate::R<Addrsel3Spec>;
#[doc = "Register `ADDRSEL3` writer"]
pub type W = crate::W<Addrsel3Spec>;
#[doc = "Memory Region Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Regenab {
    #[doc = "0: Memory region is disabled (default after reset)."]
    Value1 = 0,
    #[doc = "1: Memory region is enabled."]
    Value2 = 1,
}
impl From<Regenab> for bool {
    #[inline(always)]
    fn from(variant: Regenab) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGENAB` reader - Memory Region Enable"]
pub type RegenabR = crate::BitReader<Regenab>;
impl RegenabR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Regenab {
        match self.bits {
            false => Regenab::Value1,
            true => Regenab::Value2,
        }
    }
    #[doc = "Memory region is disabled (default after reset)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Regenab::Value1
    }
    #[doc = "Memory region is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Regenab::Value2
    }
}
#[doc = "Field `REGENAB` writer - Memory Region Enable"]
pub type RegenabW<'a, REG> = crate::BitWriter<'a, REG, Regenab>;
impl<'a, REG> RegenabW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memory region is disabled (default after reset)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Regenab::Value1)
    }
    #[doc = "Memory region is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Regenab::Value2)
    }
}
#[doc = "Alternate Region Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Altenab {
    #[doc = "0: Memory region is disabled (default after reset)."]
    Value1 = 0,
    #[doc = "1: Memory region is enabled."]
    Value2 = 1,
}
impl From<Altenab> for bool {
    #[inline(always)]
    fn from(variant: Altenab) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALTENAB` reader - Alternate Region Enable"]
pub type AltenabR = crate::BitReader<Altenab>;
impl AltenabR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Altenab {
        match self.bits {
            false => Altenab::Value1,
            true => Altenab::Value2,
        }
    }
    #[doc = "Memory region is disabled (default after reset)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Altenab::Value1
    }
    #[doc = "Memory region is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Altenab::Value2
    }
}
#[doc = "Field `ALTENAB` writer - Alternate Region Enable"]
pub type AltenabW<'a, REG> = crate::BitWriter<'a, REG, Altenab>;
impl<'a, REG> AltenabW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memory region is disabled (default after reset)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Altenab::Value1)
    }
    #[doc = "Memory region is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Altenab::Value2)
    }
}
#[doc = "Memory Region Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wprot {
    #[doc = "0: Region is enabled for write accesses"]
    Value1 = 0,
    #[doc = "1: Region is write protected."]
    Value2 = 1,
}
impl From<Wprot> for bool {
    #[inline(always)]
    fn from(variant: Wprot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPROT` reader - Memory Region Write Protect"]
pub type WprotR = crate::BitReader<Wprot>;
impl WprotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wprot {
        match self.bits {
            false => Wprot::Value1,
            true => Wprot::Value2,
        }
    }
    #[doc = "Region is enabled for write accesses"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wprot::Value1
    }
    #[doc = "Region is write protected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wprot::Value2
    }
}
#[doc = "Field `WPROT` writer - Memory Region Write Protect"]
pub type WprotW<'a, REG> = crate::BitWriter<'a, REG, Wprot>;
impl<'a, REG> WprotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Region is enabled for write accesses"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wprot::Value1)
    }
    #[doc = "Region is write protected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wprot::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Memory Region Enable"]
    #[inline(always)]
    pub fn regenab(&self) -> RegenabR {
        RegenabR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alternate Region Enable"]
    #[inline(always)]
    pub fn altenab(&self) -> AltenabR {
        AltenabR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Memory Region Write Protect"]
    #[inline(always)]
    pub fn wprot(&self) -> WprotR {
        WprotR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Memory Region Enable"]
    #[inline(always)]
    #[must_use]
    pub fn regenab(&mut self) -> RegenabW<Addrsel3Spec> {
        RegenabW::new(self, 0)
    }
    #[doc = "Bit 1 - Alternate Region Enable"]
    #[inline(always)]
    #[must_use]
    pub fn altenab(&mut self) -> AltenabW<Addrsel3Spec> {
        AltenabW::new(self, 1)
    }
    #[doc = "Bit 2 - Memory Region Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn wprot(&mut self) -> WprotW<Addrsel3Spec> {
        WprotW::new(self, 2)
    }
}
#[doc = "EBU Address Select Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addrsel3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addrsel3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addrsel3Spec;
impl crate::RegisterSpec for Addrsel3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addrsel3::R`](R) reader structure"]
impl crate::Readable for Addrsel3Spec {}
#[doc = "`write(|w| ..)` method takes [`addrsel3::W`](W) writer structure"]
impl crate::Writable for Addrsel3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDRSEL3 to value 0"]
impl crate::Resettable for Addrsel3Spec {
    const RESET_VALUE: u32 = 0;
}
