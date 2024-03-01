#[doc = "Register `FCFGC` reader"]
pub type R = crate::R<FcfgcSpec>;
#[doc = "Register `FCFGC` writer"]
pub type W = crate::W<FcfgcSpec>;
#[doc = "Field `CFMDF` reader - CIC Filter (Main Chain) Decimation Factor"]
pub type CfmdfR = crate::FieldReader;
#[doc = "Field `CFMDF` writer - CIC Filter (Main Chain) Decimation Factor"]
pub type CfmdfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "CIC Filter (Main Chain) Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfmc {
    #[doc = "0: CIC1"]
    Value1 = 0,
    #[doc = "1: CIC2"]
    Value2 = 1,
    #[doc = "2: CIC3"]
    Value3 = 2,
    #[doc = "3: CICF"]
    Value4 = 3,
}
impl From<Cfmc> for u8 {
    #[inline(always)]
    fn from(variant: Cfmc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfmc {
    type Ux = u8;
}
#[doc = "Field `CFMC` reader - CIC Filter (Main Chain) Configuration"]
pub type CfmcR = crate::FieldReader<Cfmc>;
impl CfmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfmc {
        match self.bits {
            0 => Cfmc::Value1,
            1 => Cfmc::Value2,
            2 => Cfmc::Value3,
            3 => Cfmc::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "CIC1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cfmc::Value1
    }
    #[doc = "CIC2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cfmc::Value2
    }
    #[doc = "CIC3"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cfmc::Value3
    }
    #[doc = "CICF"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cfmc::Value4
    }
}
#[doc = "Field `CFMC` writer - CIC Filter (Main Chain) Configuration"]
pub type CfmcW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Cfmc>;
impl<'a, REG> CfmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CIC1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfmc::Value1)
    }
    #[doc = "CIC2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cfmc::Value2)
    }
    #[doc = "CIC3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cfmc::Value3)
    }
    #[doc = "CICF"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cfmc::Value4)
    }
}
#[doc = "CIC Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfen {
    #[doc = "0: CIC filter disabled and bypassed"]
    Value1 = 0,
    #[doc = "1: Enable CIC filter"]
    Value2 = 1,
}
impl From<Cfen> for bool {
    #[inline(always)]
    fn from(variant: Cfen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFEN` reader - CIC Filter Enable"]
pub type CfenR = crate::BitReader<Cfen>;
impl CfenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfen {
        match self.bits {
            false => Cfen::Value1,
            true => Cfen::Value2,
        }
    }
    #[doc = "CIC filter disabled and bypassed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cfen::Value1
    }
    #[doc = "Enable CIC filter"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cfen::Value2
    }
}
#[doc = "Field `CFEN` writer - CIC Filter Enable"]
pub type CfenW<'a, REG> = crate::BitWriter<'a, REG, Cfen>;
impl<'a, REG> CfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CIC filter disabled and bypassed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfen::Value1)
    }
    #[doc = "Enable CIC filter"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cfen::Value2)
    }
}
#[doc = "Service Request Generation Main Chain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Srgm {
    #[doc = "0: Never, service requests disabled"]
    Value1 = 0,
    #[doc = "3: Always, for each new result value"]
    Value4 = 3,
}
impl From<Srgm> for u8 {
    #[inline(always)]
    fn from(variant: Srgm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Srgm {
    type Ux = u8;
}
#[doc = "Field `SRGM` reader - Service Request Generation Main Chain"]
pub type SrgmR = crate::FieldReader<Srgm>;
impl SrgmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Srgm> {
        match self.bits {
            0 => Some(Srgm::Value1),
            3 => Some(Srgm::Value4),
            _ => None,
        }
    }
    #[doc = "Never, service requests disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Srgm::Value1
    }
    #[doc = "Always, for each new result value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Srgm::Value4
    }
}
#[doc = "Field `SRGM` writer - Service Request Generation Main Chain"]
pub type SrgmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Srgm>;
impl<'a, REG> SrgmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Never, service requests disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Srgm::Value1)
    }
    #[doc = "Always, for each new result value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Srgm::Value4)
    }
}
#[doc = "Field `CFMSV` reader - CIC Filter (Main Chain) Start Value"]
pub type CfmsvR = crate::FieldReader;
#[doc = "Field `CFMSV` writer - CIC Filter (Main Chain) Start Value"]
pub type CfmsvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CFMDCNT` reader - CIC Filter (Main Chain) Decimation Counter"]
pub type CfmdcntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CIC Filter (Main Chain) Decimation Factor"]
    #[inline(always)]
    pub fn cfmdf(&self) -> CfmdfR {
        CfmdfR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - CIC Filter (Main Chain) Configuration"]
    #[inline(always)]
    pub fn cfmc(&self) -> CfmcR {
        CfmcR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - CIC Filter Enable"]
    #[inline(always)]
    pub fn cfen(&self) -> CfenR {
        CfenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Service Request Generation Main Chain"]
    #[inline(always)]
    pub fn srgm(&self) -> SrgmR {
        SrgmR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - CIC Filter (Main Chain) Start Value"]
    #[inline(always)]
    pub fn cfmsv(&self) -> CfmsvR {
        CfmsvR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CIC Filter (Main Chain) Decimation Counter"]
    #[inline(always)]
    pub fn cfmdcnt(&self) -> CfmdcntR {
        CfmdcntR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CIC Filter (Main Chain) Decimation Factor"]
    #[inline(always)]
    #[must_use]
    pub fn cfmdf(&mut self) -> CfmdfW<FcfgcSpec> {
        CfmdfW::new(self, 0)
    }
    #[doc = "Bits 8:9 - CIC Filter (Main Chain) Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn cfmc(&mut self) -> CfmcW<FcfgcSpec> {
        CfmcW::new(self, 8)
    }
    #[doc = "Bit 10 - CIC Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfen(&mut self) -> CfenW<FcfgcSpec> {
        CfenW::new(self, 10)
    }
    #[doc = "Bits 14:15 - Service Request Generation Main Chain"]
    #[inline(always)]
    #[must_use]
    pub fn srgm(&mut self) -> SrgmW<FcfgcSpec> {
        SrgmW::new(self, 14)
    }
    #[doc = "Bits 16:23 - CIC Filter (Main Chain) Start Value"]
    #[inline(always)]
    #[must_use]
    pub fn cfmsv(&mut self) -> CfmsvW<FcfgcSpec> {
        CfmsvW::new(self, 16)
    }
}
#[doc = "Filter Configuration Register, Main CIC Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfgc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfgc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcfgcSpec;
impl crate::RegisterSpec for FcfgcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcfgc::R`](R) reader structure"]
impl crate::Readable for FcfgcSpec {}
#[doc = "`write(|w| ..)` method takes [`fcfgc::W`](W) writer structure"]
impl crate::Writable for FcfgcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCFGC to value 0"]
impl crate::Resettable for FcfgcSpec {
    const RESET_VALUE: u32 = 0;
}
