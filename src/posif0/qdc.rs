#[doc = "Register `QDC` reader"]
pub struct R(crate::R<QDC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QDC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QDC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QDC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QDC` writer"]
pub struct W(crate::W<QDC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QDC_SPEC>;
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
impl From<crate::W<QDC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QDC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PALS` reader - Phase A Level selector"]
pub type PALS_R = crate::BitReader<PALS_A>;
#[doc = "Phase A Level selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PALS_A {
    #[doc = "0: Phase A is active HIGH"]
    VALUE1 = 0,
    #[doc = "1: Phase A is active LOW"]
    VALUE2 = 1,
}
impl From<PALS_A> for bool {
    #[inline(always)]
    fn from(variant: PALS_A) -> Self {
        variant as u8 != 0
    }
}
impl PALS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PALS_A {
        match self.bits {
            false => PALS_A::VALUE1,
            true => PALS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PALS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PALS_A::VALUE2
    }
}
#[doc = "Field `PALS` writer - Phase A Level selector"]
pub type PALS_W<'a, const O: u8> = crate::BitWriter<'a, u32, QDC_SPEC, PALS_A, O>;
impl<'a, const O: u8> PALS_W<'a, O> {
    #[doc = "Phase A is active HIGH"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PALS_A::VALUE1)
    }
    #[doc = "Phase A is active LOW"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PALS_A::VALUE2)
    }
}
#[doc = "Field `PBLS` reader - Phase B Level selector"]
pub type PBLS_R = crate::BitReader<PBLS_A>;
#[doc = "Phase B Level selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PBLS_A {
    #[doc = "0: Phase B is active HIGH"]
    VALUE1 = 0,
    #[doc = "1: Phase B is active LOW"]
    VALUE2 = 1,
}
impl From<PBLS_A> for bool {
    #[inline(always)]
    fn from(variant: PBLS_A) -> Self {
        variant as u8 != 0
    }
}
impl PBLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBLS_A {
        match self.bits {
            false => PBLS_A::VALUE1,
            true => PBLS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PBLS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PBLS_A::VALUE2
    }
}
#[doc = "Field `PBLS` writer - Phase B Level selector"]
pub type PBLS_W<'a, const O: u8> = crate::BitWriter<'a, u32, QDC_SPEC, PBLS_A, O>;
impl<'a, const O: u8> PBLS_W<'a, O> {
    #[doc = "Phase B is active HIGH"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PBLS_A::VALUE1)
    }
    #[doc = "Phase B is active LOW"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PBLS_A::VALUE2)
    }
}
#[doc = "Field `PHS` reader - Phase signals swap"]
pub type PHS_R = crate::BitReader<PHS_A>;
#[doc = "Phase signals swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHS_A {
    #[doc = "0: Phase A is the leading signal for clockwise rotation"]
    VALUE1 = 0,
    #[doc = "1: Phase B is the leading signal for clockwise rotation"]
    VALUE2 = 1,
}
impl From<PHS_A> for bool {
    #[inline(always)]
    fn from(variant: PHS_A) -> Self {
        variant as u8 != 0
    }
}
impl PHS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHS_A {
        match self.bits {
            false => PHS_A::VALUE1,
            true => PHS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PHS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PHS_A::VALUE2
    }
}
#[doc = "Field `PHS` writer - Phase signals swap"]
pub type PHS_W<'a, const O: u8> = crate::BitWriter<'a, u32, QDC_SPEC, PHS_A, O>;
impl<'a, const O: u8> PHS_W<'a, O> {
    #[doc = "Phase A is the leading signal for clockwise rotation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PHS_A::VALUE1)
    }
    #[doc = "Phase B is the leading signal for clockwise rotation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PHS_A::VALUE2)
    }
}
#[doc = "Field `ICM` reader - Index Marker generations control"]
pub type ICM_R = crate::FieldReader<u8, ICM_A>;
#[doc = "Index Marker generations control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICM_A {
    #[doc = "0: No index marker generation on POSIFx.OUT3"]
    VALUE1 = 0,
    #[doc = "1: Only first index occurrence generated on POSIFx.OUT3"]
    VALUE2 = 1,
    #[doc = "2: All index occurrences generated on POSIFx.OUT3"]
    VALUE3 = 2,
}
impl From<ICM_A> for u8 {
    #[inline(always)]
    fn from(variant: ICM_A) -> Self {
        variant as _
    }
}
impl ICM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ICM_A> {
        match self.bits {
            0 => Some(ICM_A::VALUE1),
            1 => Some(ICM_A::VALUE2),
            2 => Some(ICM_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ICM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ICM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ICM_A::VALUE3
    }
}
#[doc = "Field `ICM` writer - Index Marker generations control"]
pub type ICM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QDC_SPEC, u8, ICM_A, 2, O>;
impl<'a, const O: u8> ICM_W<'a, O> {
    #[doc = "No index marker generation on POSIFx.OUT3"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ICM_A::VALUE1)
    }
    #[doc = "Only first index occurrence generated on POSIFx.OUT3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ICM_A::VALUE2)
    }
    #[doc = "All index occurrences generated on POSIFx.OUT3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ICM_A::VALUE3)
    }
}
#[doc = "Field `DVAL` reader - Current rotation direction"]
pub type DVAL_R = crate::BitReader<DVAL_A>;
#[doc = "Current rotation direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVAL_A {
    #[doc = "0: Counterclockwise rotation"]
    VALUE1 = 0,
    #[doc = "1: Clockwise rotation"]
    VALUE2 = 1,
}
impl From<DVAL_A> for bool {
    #[inline(always)]
    fn from(variant: DVAL_A) -> Self {
        variant as u8 != 0
    }
}
impl DVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVAL_A {
        match self.bits {
            false => DVAL_A::VALUE1,
            true => DVAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DVAL_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Phase A Level selector"]
    #[inline(always)]
    pub fn pals(&self) -> PALS_R {
        PALS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Phase B Level selector"]
    #[inline(always)]
    pub fn pbls(&self) -> PBLS_R {
        PBLS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Phase signals swap"]
    #[inline(always)]
    pub fn phs(&self) -> PHS_R {
        PHS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Index Marker generations control"]
    #[inline(always)]
    pub fn icm(&self) -> ICM_R {
        ICM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Current rotation direction"]
    #[inline(always)]
    pub fn dval(&self) -> DVAL_R {
        DVAL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Phase A Level selector"]
    #[inline(always)]
    #[must_use]
    pub fn pals(&mut self) -> PALS_W<0> {
        PALS_W::new(self)
    }
    #[doc = "Bit 1 - Phase B Level selector"]
    #[inline(always)]
    #[must_use]
    pub fn pbls(&mut self) -> PBLS_W<1> {
        PBLS_W::new(self)
    }
    #[doc = "Bit 2 - Phase signals swap"]
    #[inline(always)]
    #[must_use]
    pub fn phs(&mut self) -> PHS_W<2> {
        PHS_W::new(self)
    }
    #[doc = "Bits 4:5 - Index Marker generations control"]
    #[inline(always)]
    #[must_use]
    pub fn icm(&mut self) -> ICM_W<4> {
        ICM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Quadrature Decoder Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qdc](index.html) module"]
pub struct QDC_SPEC;
impl crate::RegisterSpec for QDC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qdc::R](R) reader structure"]
impl crate::Readable for QDC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qdc::W](W) writer structure"]
impl crate::Writable for QDC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QDC to value 0"]
impl crate::Resettable for QDC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
