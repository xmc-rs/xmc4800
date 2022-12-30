#[doc = "Register `FCFGC` reader"]
pub struct R(crate::R<FCFGC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCFGC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCFGC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCFGC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCFGC` writer"]
pub struct W(crate::W<FCFGC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCFGC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FCFGC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCFGC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFMDF` reader - CIC Filter (Main Chain) Decimation Factor"]
pub type CFMDF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFMDF` writer - CIC Filter (Main Chain) Decimation Factor"]
pub type CFMDF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCFGC_SPEC, u8, u8, 8, O>;
#[doc = "Field `CFMC` reader - CIC Filter (Main Chain) Configuration"]
pub type CFMC_R = crate::FieldReader<u8, CFMC_A>;
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
impl CFMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFMC_A {
        match self.bits {
            0 => CFMC_A::VALUE1,
            1 => CFMC_A::VALUE2,
            2 => CFMC_A::VALUE3,
            3 => CFMC_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFMC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFMC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CFMC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CFMC_A::VALUE4
    }
}
#[doc = "Field `CFMC` writer - CIC Filter (Main Chain) Configuration"]
pub type CFMC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FCFGC_SPEC, u8, CFMC_A, 2, O>;
impl<'a, const O: u8> CFMC_W<'a, O> {
    #[doc = "CIC1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFMC_A::VALUE1)
    }
    #[doc = "CIC2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFMC_A::VALUE2)
    }
    #[doc = "CIC3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CFMC_A::VALUE3)
    }
    #[doc = "CICF"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CFMC_A::VALUE4)
    }
}
#[doc = "Field `CFEN` reader - CIC Filter Enable"]
pub type CFEN_R = crate::BitReader<CFEN_A>;
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
impl CFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFEN_A {
        match self.bits {
            false => CFEN_A::VALUE1,
            true => CFEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFEN_A::VALUE2
    }
}
#[doc = "Field `CFEN` writer - CIC Filter Enable"]
pub type CFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCFGC_SPEC, CFEN_A, O>;
impl<'a, const O: u8> CFEN_W<'a, O> {
    #[doc = "CIC filter disabled and bypassed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFEN_A::VALUE1)
    }
    #[doc = "Enable CIC filter"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFEN_A::VALUE2)
    }
}
#[doc = "Field `SRGM` reader - Service Request Generation Main Chain"]
pub type SRGM_R = crate::FieldReader<u8, SRGM_A>;
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
impl SRGM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRGM_A> {
        match self.bits {
            0 => Some(SRGM_A::VALUE1),
            3 => Some(SRGM_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRGM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SRGM_A::VALUE4
    }
}
#[doc = "Field `SRGM` writer - Service Request Generation Main Chain"]
pub type SRGM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCFGC_SPEC, u8, SRGM_A, 2, O>;
impl<'a, const O: u8> SRGM_W<'a, O> {
    #[doc = "Never, service requests disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRGM_A::VALUE1)
    }
    #[doc = "Always, for each new result value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SRGM_A::VALUE4)
    }
}
#[doc = "Field `CFMSV` reader - CIC Filter (Main Chain) Start Value"]
pub type CFMSV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFMSV` writer - CIC Filter (Main Chain) Start Value"]
pub type CFMSV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCFGC_SPEC, u8, u8, 8, O>;
#[doc = "Field `CFMDCNT` reader - CIC Filter (Main Chain) Decimation Counter"]
pub type CFMDCNT_R = crate::FieldReader<u8, u8>;
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
    #[must_use]
    pub fn cfmdf(&mut self) -> CFMDF_W<0> {
        CFMDF_W::new(self)
    }
    #[doc = "Bits 8:9 - CIC Filter (Main Chain) Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn cfmc(&mut self) -> CFMC_W<8> {
        CFMC_W::new(self)
    }
    #[doc = "Bit 10 - CIC Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfen(&mut self) -> CFEN_W<10> {
        CFEN_W::new(self)
    }
    #[doc = "Bits 14:15 - Service Request Generation Main Chain"]
    #[inline(always)]
    #[must_use]
    pub fn srgm(&mut self) -> SRGM_W<14> {
        SRGM_W::new(self)
    }
    #[doc = "Bits 16:23 - CIC Filter (Main Chain) Start Value"]
    #[inline(always)]
    #[must_use]
    pub fn cfmsv(&mut self) -> CFMSV_W<16> {
        CFMSV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter Configuration Register, Main CIC Filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfgc](index.html) module"]
pub struct FCFGC_SPEC;
impl crate::RegisterSpec for FCFGC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcfgc::R](R) reader structure"]
impl crate::Readable for FCFGC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcfgc::W](W) writer structure"]
impl crate::Writable for FCFGC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCFGC to value 0"]
impl crate::Resettable for FCFGC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
