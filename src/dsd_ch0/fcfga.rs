#[doc = "Register `FCFGA` reader"]
pub struct R(crate::R<FCFGA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCFGA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCFGA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCFGA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCFGA` writer"]
pub struct W(crate::W<FCFGA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCFGA_SPEC>;
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
impl From<crate::W<FCFGA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCFGA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFADF` reader - CIC Filter (Auxiliary) Decimation Factor"]
pub struct CFADF_R(crate::FieldReader<u8, u8>);
impl CFADF_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFADF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFADF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFADF` writer - CIC Filter (Auxiliary) Decimation Factor"]
pub struct CFADF_W<'a> {
    w: &'a mut W,
}
impl<'a> CFADF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "CIC Filter (Auxiliary) Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFAC_A {
    #[doc = "0: CIC1"]
    VALUE1 = 0,
    #[doc = "1: CIC2"]
    VALUE2 = 1,
    #[doc = "2: CIC3"]
    VALUE3 = 2,
    #[doc = "3: CICF"]
    VALUE4 = 3,
}
impl From<CFAC_A> for u8 {
    #[inline(always)]
    fn from(variant: CFAC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFAC` reader - CIC Filter (Auxiliary) Configuration"]
pub struct CFAC_R(crate::FieldReader<u8, CFAC_A>);
impl CFAC_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFAC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFAC_A {
        match self.bits {
            0 => CFAC_A::VALUE1,
            1 => CFAC_A::VALUE2,
            2 => CFAC_A::VALUE3,
            3 => CFAC_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CFAC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CFAC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == CFAC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == CFAC_A::VALUE4
    }
}
impl core::ops::Deref for CFAC_R {
    type Target = crate::FieldReader<u8, CFAC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFAC` writer - CIC Filter (Auxiliary) Configuration"]
pub struct CFAC_W<'a> {
    w: &'a mut W,
}
impl<'a> CFAC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFAC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CIC1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFAC_A::VALUE1)
    }
    #[doc = "CIC2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFAC_A::VALUE2)
    }
    #[doc = "CIC3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CFAC_A::VALUE3)
    }
    #[doc = "CICF"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CFAC_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Service Request Generation Auxiliary Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRGA_A {
    #[doc = "0: Never, service requests disabled"]
    VALUE1 = 0,
    #[doc = "1: Auxiliary filter: As selected by bitfields ESEL and EGT (if integrator enabled)"]
    VALUE2 = 1,
    #[doc = "2: Alternate source: Capturing of a sign delay value to register CGSYNCx (x = 0 - 3)"]
    VALUE3 = 2,
}
impl From<SRGA_A> for u8 {
    #[inline(always)]
    fn from(variant: SRGA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRGA` reader - Service Request Generation Auxiliary Filter"]
pub struct SRGA_R(crate::FieldReader<u8, SRGA_A>);
impl SRGA_R {
    pub(crate) fn new(bits: u8) -> Self {
        SRGA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRGA_A> {
        match self.bits {
            0 => Some(SRGA_A::VALUE1),
            1 => Some(SRGA_A::VALUE2),
            2 => Some(SRGA_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SRGA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SRGA_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SRGA_A::VALUE3
    }
}
impl core::ops::Deref for SRGA_R {
    type Target = crate::FieldReader<u8, SRGA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRGA` writer - Service Request Generation Auxiliary Filter"]
pub struct SRGA_W<'a> {
    w: &'a mut W,
}
impl<'a> SRGA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRGA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Never, service requests disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRGA_A::VALUE1)
    }
    #[doc = "Auxiliary filter: As selected by bitfields ESEL and EGT (if integrator enabled)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRGA_A::VALUE2)
    }
    #[doc = "Alternate source: Capturing of a sign delay value to register CGSYNCx (x = 0 - 3)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SRGA_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Event Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ESEL_A {
    #[doc = "0: Always, for each new result value"]
    VALUE1 = 0,
    #[doc = "1: If result is inside the boundary band"]
    VALUE2 = 1,
    #[doc = "2: If result is outside the boundary band"]
    VALUE3 = 2,
}
impl From<ESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ESEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ESEL` reader - Event Select"]
pub struct ESEL_R(crate::FieldReader<u8, ESEL_A>);
impl ESEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ESEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ESEL_A> {
        match self.bits {
            0 => Some(ESEL_A::VALUE1),
            1 => Some(ESEL_A::VALUE2),
            2 => Some(ESEL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ESEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ESEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == ESEL_A::VALUE3
    }
}
impl core::ops::Deref for ESEL_R {
    type Target = crate::FieldReader<u8, ESEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESEL` writer - Event Select"]
pub struct ESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ESEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Always, for each new result value"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ESEL_A::VALUE1)
    }
    #[doc = "If result is inside the boundary band"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ESEL_A::VALUE2)
    }
    #[doc = "If result is outside the boundary band"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ESEL_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Event Gating\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EGT_A {
    #[doc = "0: Separate: generate events according to ESEL"]
    VALUE1 = 0,
    #[doc = "1: Coupled: generate events only when the integrator is enabled and after the discard phase defined by bitfield NVALDISWhile the integrator is bypassed, event gating is disabled, i.e. service requests are generated according to bitfield ESEL. The event gating suppresses service requests, result values are still stored in register RESAx."]
    VALUE2 = 1,
}
impl From<EGT_A> for bool {
    #[inline(always)]
    fn from(variant: EGT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EGT` reader - Event Gating"]
pub struct EGT_R(crate::FieldReader<bool, EGT_A>);
impl EGT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EGT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EGT_A {
        match self.bits {
            false => EGT_A::VALUE1,
            true => EGT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EGT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EGT_A::VALUE2
    }
}
impl core::ops::Deref for EGT_R {
    type Target = crate::FieldReader<bool, EGT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EGT` writer - Event Gating"]
pub struct EGT_W<'a> {
    w: &'a mut W,
}
impl<'a> EGT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EGT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Separate: generate events according to ESEL"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EGT_A::VALUE1)
    }
    #[doc = "Coupled: generate events only when the integrator is enabled and after the discard phase defined by bitfield NVALDISWhile the integrator is bypassed, event gating is disabled, i.e. service requests are generated according to bitfield ESEL. The event gating suppresses service requests, result values are still stored in register RESAx."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EGT_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `CFADCNT` reader - CIC Filter (Auxiliary) Decimation Counter"]
pub struct CFADCNT_R(crate::FieldReader<u8, u8>);
impl CFADCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFADCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFADCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - CIC Filter (Auxiliary) Decimation Factor"]
    #[inline(always)]
    pub fn cfadf(&self) -> CFADF_R {
        CFADF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - CIC Filter (Auxiliary) Configuration"]
    #[inline(always)]
    pub fn cfac(&self) -> CFAC_R {
        CFAC_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Service Request Generation Auxiliary Filter"]
    #[inline(always)]
    pub fn srga(&self) -> SRGA_R {
        SRGA_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Event Select"]
    #[inline(always)]
    pub fn esel(&self) -> ESEL_R {
        ESEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Event Gating"]
    #[inline(always)]
    pub fn egt(&self) -> EGT_R {
        EGT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - CIC Filter (Auxiliary) Decimation Counter"]
    #[inline(always)]
    pub fn cfadcnt(&self) -> CFADCNT_R {
        CFADCNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CIC Filter (Auxiliary) Decimation Factor"]
    #[inline(always)]
    pub fn cfadf(&mut self) -> CFADF_W {
        CFADF_W { w: self }
    }
    #[doc = "Bits 8:9 - CIC Filter (Auxiliary) Configuration"]
    #[inline(always)]
    pub fn cfac(&mut self) -> CFAC_W {
        CFAC_W { w: self }
    }
    #[doc = "Bits 10:11 - Service Request Generation Auxiliary Filter"]
    #[inline(always)]
    pub fn srga(&mut self) -> SRGA_W {
        SRGA_W { w: self }
    }
    #[doc = "Bits 12:13 - Event Select"]
    #[inline(always)]
    pub fn esel(&mut self) -> ESEL_W {
        ESEL_W { w: self }
    }
    #[doc = "Bit 14 - Event Gating"]
    #[inline(always)]
    pub fn egt(&mut self) -> EGT_W {
        EGT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter Configuration Register, Auxiliary Filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfga](index.html) module"]
pub struct FCFGA_SPEC;
impl crate::RegisterSpec for FCFGA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcfga::R](R) reader structure"]
impl crate::Readable for FCFGA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcfga::W](W) writer structure"]
impl crate::Writable for FCFGA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCFGA to value 0"]
impl crate::Resettable for FCFGA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
