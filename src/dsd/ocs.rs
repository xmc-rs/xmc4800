#[doc = "Register `OCS` reader"]
pub type R = crate::R<OcsSpec>;
#[doc = "Register `OCS` writer"]
pub type W = crate::W<OcsSpec>;
#[doc = "OCDS Suspend Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sus {
    #[doc = "0: Will not suspend"]
    Value1 = 0,
    #[doc = "1: Hard suspend: Clock is switched off immediately."]
    Value2 = 1,
    #[doc = "2: Soft suspend channel 0"]
    Value3 = 2,
    #[doc = "3: Soft suspend channel 1"]
    Value4 = 3,
    #[doc = "5: Soft suspend channel 3"]
    Value5 = 5,
}
impl From<Sus> for u8 {
    #[inline(always)]
    fn from(variant: Sus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sus {
    type Ux = u8;
}
impl crate::IsEnum for Sus {}
#[doc = "Field `SUS` reader - OCDS Suspend Control"]
pub type SusR = crate::FieldReader<Sus>;
impl SusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sus> {
        match self.bits {
            0 => Some(Sus::Value1),
            1 => Some(Sus::Value2),
            2 => Some(Sus::Value3),
            3 => Some(Sus::Value4),
            5 => Some(Sus::Value5),
            _ => None,
        }
    }
    #[doc = "Will not suspend"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sus::Value1
    }
    #[doc = "Hard suspend: Clock is switched off immediately."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sus::Value2
    }
    #[doc = "Soft suspend channel 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Sus::Value3
    }
    #[doc = "Soft suspend channel 1"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Sus::Value4
    }
    #[doc = "Soft suspend channel 3"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Sus::Value5
    }
}
#[doc = "Field `SUS` writer - OCDS Suspend Control"]
pub type SusW<'a, REG> = crate::FieldWriter<'a, REG, 4, Sus>;
impl<'a, REG> SusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Will not suspend"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sus::Value1)
    }
    #[doc = "Hard suspend: Clock is switched off immediately."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sus::Value2)
    }
    #[doc = "Soft suspend channel 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Sus::Value3)
    }
    #[doc = "Soft suspend channel 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Sus::Value4)
    }
    #[doc = "Soft suspend channel 3"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Sus::Value5)
    }
}
#[doc = "Field `SUS_P` writer - SUS Write Protection"]
pub type SusPW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Suspend State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sussta {
    #[doc = "0: Module is not (yet) suspended"]
    Value1 = 0,
    #[doc = "1: Module is suspended"]
    Value2 = 1,
}
impl From<Sussta> for bool {
    #[inline(always)]
    fn from(variant: Sussta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSSTA` reader - Suspend State"]
pub type SusstaR = crate::BitReader<Sussta>;
impl SusstaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sussta {
        match self.bits {
            false => Sussta::Value1,
            true => Sussta::Value2,
        }
    }
    #[doc = "Module is not (yet) suspended"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sussta::Value1
    }
    #[doc = "Module is suspended"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sussta::Value2
    }
}
impl R {
    #[doc = "Bits 24:27 - OCDS Suspend Control"]
    #[inline(always)]
    pub fn sus(&self) -> SusR {
        SusR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Suspend State"]
    #[inline(always)]
    pub fn sussta(&self) -> SusstaR {
        SusstaR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:27 - OCDS Suspend Control"]
    #[inline(always)]
    #[must_use]
    pub fn sus(&mut self) -> SusW<OcsSpec> {
        SusW::new(self, 24)
    }
    #[doc = "Bit 28 - SUS Write Protection"]
    #[inline(always)]
    #[must_use]
    pub fn sus_p(&mut self) -> SusPW<OcsSpec> {
        SusPW::new(self, 28)
    }
}
#[doc = "OCDS Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OcsSpec;
impl crate::RegisterSpec for OcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ocs::R`](R) reader structure"]
impl crate::Readable for OcsSpec {}
#[doc = "`write(|w| ..)` method takes [`ocs::W`](W) writer structure"]
impl crate::Writable for OcsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OCS to value 0"]
impl crate::Resettable for OcsSpec {
    const RESET_VALUE: u32 = 0;
}
