#[doc = "Register `FDR` reader"]
pub type R = crate::R<FdrSpec>;
#[doc = "Register `FDR` writer"]
pub type W = crate::W<FdrSpec>;
#[doc = "Field `STEP` reader - Step Value"]
pub type StepR = crate::FieldReader<u16>;
#[doc = "Field `STEP` writer - Step Value"]
pub type StepW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Divider Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dm {
    #[doc = "0: The divider is switched off, fFD = 0."]
    Value1 = 0,
    #[doc = "1: Normal divider mode selected."]
    Value2 = 1,
    #[doc = "2: Fractional divider mode selected."]
    Value3 = 2,
    #[doc = "3: The divider is switched off, fFD = 0."]
    Value4 = 3,
}
impl From<Dm> for u8 {
    #[inline(always)]
    fn from(variant: Dm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dm {
    type Ux = u8;
}
#[doc = "Field `DM` reader - Divider Mode"]
pub type DmR = crate::FieldReader<Dm>;
impl DmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dm {
        match self.bits {
            0 => Dm::Value1,
            1 => Dm::Value2,
            2 => Dm::Value3,
            3 => Dm::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "The divider is switched off, fFD = 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dm::Value1
    }
    #[doc = "Normal divider mode selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dm::Value2
    }
    #[doc = "Fractional divider mode selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Dm::Value3
    }
    #[doc = "The divider is switched off, fFD = 0."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Dm::Value4
    }
}
#[doc = "Field `DM` writer - Divider Mode"]
pub type DmW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Dm>;
impl<'a, REG> DmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The divider is switched off, fFD = 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dm::Value1)
    }
    #[doc = "Normal divider mode selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dm::Value2)
    }
    #[doc = "Fractional divider mode selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Dm::Value3)
    }
    #[doc = "The divider is switched off, fFD = 0."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Dm::Value4)
    }
}
#[doc = "Field `RESULT` reader - Result Value"]
pub type ResultR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Step Value"]
    #[inline(always)]
    pub fn step(&self) -> StepR {
        StepR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 14:15 - Divider Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DmR {
        DmR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Result Value"]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Step Value"]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> StepW<FdrSpec> {
        StepW::new(self, 0)
    }
    #[doc = "Bits 14:15 - Divider Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DmW<FdrSpec> {
        DmW::new(self, 14)
    }
}
#[doc = "Fractional Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdrSpec;
impl crate::RegisterSpec for FdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdr::R`](R) reader structure"]
impl crate::Readable for FdrSpec {}
#[doc = "`write(|w| ..)` method takes [`fdr::W`](W) writer structure"]
impl crate::Writable for FdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDR to value 0"]
impl crate::Resettable for FdrSpec {
    const RESET_VALUE: u32 = 0;
}
