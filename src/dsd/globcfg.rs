#[doc = "Register `GLOBCFG` reader"]
pub type R = crate::R<GlobcfgSpec>;
#[doc = "Register `GLOBCFG` writer"]
pub type W = crate::W<GlobcfgSpec>;
#[doc = "Modulator Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mcsel {
    #[doc = "0: Internal clock off, no source selected"]
    Value1 = 0,
    #[doc = "1: fDSD"]
    Value2 = 1,
}
impl From<Mcsel> for u8 {
    #[inline(always)]
    fn from(variant: Mcsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mcsel {
    type Ux = u8;
}
impl crate::IsEnum for Mcsel {}
#[doc = "Field `MCSEL` reader - Modulator Clock Select"]
pub type McselR = crate::FieldReader<Mcsel>;
impl McselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mcsel> {
        match self.bits {
            0 => Some(Mcsel::Value1),
            1 => Some(Mcsel::Value2),
            _ => None,
        }
    }
    #[doc = "Internal clock off, no source selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mcsel::Value1
    }
    #[doc = "fDSD"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mcsel::Value2
    }
}
#[doc = "Field `MCSEL` writer - Modulator Clock Select"]
pub type McselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mcsel>;
impl<'a, REG> McselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal clock off, no source selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mcsel::Value1)
    }
    #[doc = "fDSD"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mcsel::Value2)
    }
}
impl R {
    #[doc = "Bits 0:2 - Modulator Clock Select"]
    #[inline(always)]
    pub fn mcsel(&self) -> McselR {
        McselR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Modulator Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn mcsel(&mut self) -> McselW<GlobcfgSpec> {
        McselW::new(self, 0)
    }
}
#[doc = "Global Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlobcfgSpec;
impl crate::RegisterSpec for GlobcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`globcfg::R`](R) reader structure"]
impl crate::Readable for GlobcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`globcfg::W`](W) writer structure"]
impl crate::Writable for GlobcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOBCFG to value 0"]
impl crate::Resettable for GlobcfgSpec {
    const RESET_VALUE: u32 = 0;
}
