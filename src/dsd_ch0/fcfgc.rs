#[doc = "Register `FCFGC` reader"]
pub type R = crate::R<FCFGC_SPEC>;
#[doc = "Register `FCFGC` writer"]
pub type W = crate::W<FCFGC_SPEC>;
#[doc = "Field `CFMDF` reader - CIC Filter (Main Chain) Decimation Factor"]
pub type CFMDF_R = crate::FieldReader;
#[doc = "Field `CFMDF` writer - CIC Filter (Main Chain) Decimation Factor"]
pub type CFMDF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "CIC Filter (Main Chain) Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFMC_A {
    #[doc = "0: CIC1"]
    VALUE1 = 0,
    #[doc = "1: CIC2"]
    VALUE2 = 1,
    #[doc = "2: CIC3"]
    VALUE3 = 2,
    #[doc = "3: CICF"]
    VALUE4 = 3,
}
impl From<CFMC_A> for u8 {
    #[inline(always)]
    fn from(variant: CFMC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CFMC_A {
    type Ux = u8;
}
impl crate::IsEnum for CFMC_A {}
#[doc = "Field `CFMC` reader - CIC Filter (Main Chain) Configuration"]
pub type CFMC_R = crate::FieldReader<CFMC_A>;
impl CFMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CFMC_A {
        match self.bits {
            0 => CFMC_A::VALUE1,
            1 => CFMC_A::VALUE2,
            2 => CFMC_A::VALUE3,
            3 => CFMC_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "CIC1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFMC_A::VALUE1
    }
    #[doc = "CIC2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFMC_A::VALUE2
    }
    #[doc = "CIC3"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CFMC_A::VALUE3
    }
    #[doc = "CICF"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CFMC_A::VALUE4
    }
}
#[doc = "Field `CFMC` writer - CIC Filter (Main Chain) Configuration"]
pub type CFMC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CFMC_A, crate::Safe>;
impl<'a, REG> CFMC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CIC1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CFMC_A::VALUE1)
    }
    #[doc = "CIC2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CFMC_A::VALUE2)
    }
    #[doc = "CIC3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CFMC_A::VALUE3)
    }
    #[doc = "CICF"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CFMC_A::VALUE4)
    }
}
#[doc = "CIC Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFEN_A {
    #[doc = "0: CIC filter disabled and bypassed"]
    VALUE1 = 0,
    #[doc = "1: Enable CIC filter"]
    VALUE2 = 1,
}
impl From<CFEN_A> for bool {
    #[inline(always)]
    fn from(variant: CFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFEN` reader - CIC Filter Enable"]
pub type CFEN_R = crate::BitReader<CFEN_A>;
impl CFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CFEN_A {
        match self.bits {
            false => CFEN_A::VALUE1,
            true => CFEN_A::VALUE2,
        }
    }
    #[doc = "CIC filter disabled and bypassed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFEN_A::VALUE1
    }
    #[doc = "Enable CIC filter"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFEN_A::VALUE2
    }
}
#[doc = "Field `CFEN` writer - CIC Filter Enable"]
pub type CFEN_W<'a, REG> = crate::BitWriter<'a, REG, CFEN_A>;
impl<'a, REG> CFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CIC filter disabled and bypassed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CFEN_A::VALUE1)
    }
    #[doc = "Enable CIC filter"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CFEN_A::VALUE2)
    }
}
#[doc = "Service Request Generation Main Chain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRGM_A {
    #[doc = "0: Never, service requests disabled"]
    VALUE1 = 0,
    #[doc = "3: Always, for each new result value"]
    VALUE4 = 3,
}
impl From<SRGM_A> for u8 {
    #[inline(always)]
    fn from(variant: SRGM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SRGM_A {
    type Ux = u8;
}
impl crate::IsEnum for SRGM_A {}
#[doc = "Field `SRGM` reader - Service Request Generation Main Chain"]
pub type SRGM_R = crate::FieldReader<SRGM_A>;
impl SRGM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SRGM_A> {
        match self.bits {
            0 => Some(SRGM_A::VALUE1),
            3 => Some(SRGM_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Never, service requests disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRGM_A::VALUE1
    }
    #[doc = "Always, for each new result value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SRGM_A::VALUE4
    }
}
#[doc = "Field `SRGM` writer - Service Request Generation Main Chain"]
pub type SRGM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SRGM_A>;
impl<'a, REG> SRGM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Never, service requests disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SRGM_A::VALUE1)
    }
    #[doc = "Always, for each new result value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(SRGM_A::VALUE4)
    }
}
#[doc = "Field `CFMSV` reader - CIC Filter (Main Chain) Start Value"]
pub type CFMSV_R = crate::FieldReader;
#[doc = "Field `CFMSV` writer - CIC Filter (Main Chain) Start Value"]
pub type CFMSV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CFMDCNT` reader - CIC Filter (Main Chain) Decimation Counter"]
pub type CFMDCNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CIC Filter (Main Chain) Decimation Factor"]
    #[inline(always)]
    pub fn cfmdf(&self) -> CFMDF_R {
        CFMDF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - CIC Filter (Main Chain) Configuration"]
    #[inline(always)]
    pub fn cfmc(&self) -> CFMC_R {
        CFMC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - CIC Filter Enable"]
    #[inline(always)]
    pub fn cfen(&self) -> CFEN_R {
        CFEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Service Request Generation Main Chain"]
    #[inline(always)]
    pub fn srgm(&self) -> SRGM_R {
        SRGM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - CIC Filter (Main Chain) Start Value"]
    #[inline(always)]
    pub fn cfmsv(&self) -> CFMSV_R {
        CFMSV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CIC Filter (Main Chain) Decimation Counter"]
    #[inline(always)]
    pub fn cfmdcnt(&self) -> CFMDCNT_R {
        CFMDCNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CIC Filter (Main Chain) Decimation Factor"]
    #[inline(always)]
    pub fn cfmdf(&mut self) -> CFMDF_W<FCFGC_SPEC> {
        CFMDF_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - CIC Filter (Main Chain) Configuration"]
    #[inline(always)]
    pub fn cfmc(&mut self) -> CFMC_W<FCFGC_SPEC> {
        CFMC_W::new(self, 8)
    }
    #[doc = "Bit 10 - CIC Filter Enable"]
    #[inline(always)]
    pub fn cfen(&mut self) -> CFEN_W<FCFGC_SPEC> {
        CFEN_W::new(self, 10)
    }
    #[doc = "Bits 14:15 - Service Request Generation Main Chain"]
    #[inline(always)]
    pub fn srgm(&mut self) -> SRGM_W<FCFGC_SPEC> {
        SRGM_W::new(self, 14)
    }
    #[doc = "Bits 16:23 - CIC Filter (Main Chain) Start Value"]
    #[inline(always)]
    pub fn cfmsv(&mut self) -> CFMSV_W<FCFGC_SPEC> {
        CFMSV_W::new(self, 16)
    }
}
#[doc = "Filter Configuration Register, Main CIC Filter\n\nYou can [`read`](crate::Reg::read) this register and get [`fcfgc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcfgc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCFGC_SPEC;
impl crate::RegisterSpec for FCFGC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcfgc::R`](R) reader structure"]
impl crate::Readable for FCFGC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fcfgc::W`](W) writer structure"]
impl crate::Writable for FCFGC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCFGC to value 0"]
impl crate::Resettable for FCFGC_SPEC {
    const RESET_VALUE: u32 = 0;
}
