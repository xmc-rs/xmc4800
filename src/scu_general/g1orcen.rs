#[doc = "Register `G1ORCEN` reader"]
pub type R = crate::R<G1orcenSpec>;
#[doc = "Register `G1ORCEN` writer"]
pub type W = crate::W<G1orcenSpec>;
#[doc = "Enable Out of Range Comparator, Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enorc6 {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Enorc6> for bool {
    #[inline(always)]
    fn from(variant: Enorc6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENORC6` reader - Enable Out of Range Comparator, Channel 6"]
pub type Enorc6R = crate::BitReader<Enorc6>;
impl Enorc6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enorc6 {
        match self.bits {
            false => Enorc6::Value1,
            true => Enorc6::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Enorc6::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Enorc6::Value2
    }
}
#[doc = "Field `ENORC6` writer - Enable Out of Range Comparator, Channel 6"]
pub type Enorc6W<'a, REG> = crate::BitWriter<'a, REG, Enorc6>;
impl<'a, REG> Enorc6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Enorc6::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Enorc6::Value2)
    }
}
#[doc = "Enable Out of Range Comparator, Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enorc7 {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Enorc7> for bool {
    #[inline(always)]
    fn from(variant: Enorc7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENORC7` reader - Enable Out of Range Comparator, Channel 7"]
pub type Enorc7R = crate::BitReader<Enorc7>;
impl Enorc7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enorc7 {
        match self.bits {
            false => Enorc7::Value1,
            true => Enorc7::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Enorc7::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Enorc7::Value2
    }
}
#[doc = "Field `ENORC7` writer - Enable Out of Range Comparator, Channel 7"]
pub type Enorc7W<'a, REG> = crate::BitWriter<'a, REG, Enorc7>;
impl<'a, REG> Enorc7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Enorc7::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Enorc7::Value2)
    }
}
impl R {
    #[doc = "Bit 6 - Enable Out of Range Comparator, Channel 6"]
    #[inline(always)]
    pub fn enorc6(&self) -> Enorc6R {
        Enorc6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Out of Range Comparator, Channel 7"]
    #[inline(always)]
    pub fn enorc7(&self) -> Enorc7R {
        Enorc7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Enable Out of Range Comparator, Channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn enorc6(&mut self) -> Enorc6W<G1orcenSpec> {
        Enorc6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Out of Range Comparator, Channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn enorc7(&mut self) -> Enorc7W<G1orcenSpec> {
        Enorc7W::new(self, 7)
    }
}
#[doc = "Out of Range Comparator Enable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g1orcen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`g1orcen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct G1orcenSpec;
impl crate::RegisterSpec for G1orcenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`g1orcen::R`](R) reader structure"]
impl crate::Readable for G1orcenSpec {}
#[doc = "`write(|w| ..)` method takes [`g1orcen::W`](W) writer structure"]
impl crate::Writable for G1orcenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets G1ORCEN to value 0"]
impl crate::Resettable for G1orcenSpec {
    const RESET_VALUE: u32 = 0;
}
