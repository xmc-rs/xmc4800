#[doc = "Register `IWCTR` reader"]
pub struct R(crate::R<IWCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IWCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IWCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IWCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IWCTR` writer"]
pub struct W(crate::W<IWCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IWCTR_SPEC>;
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
impl From<crate::W<IWCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IWCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NVALCNT` reader - Number of Values Counted"]
pub struct NVALCNT_R(crate::FieldReader<u8, u8>);
impl NVALCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        NVALCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVALCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Integration Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN_A {
    #[doc = "0: Integration stopped. INTEN is cleared at the end of the integration window, i.e. upon the inverse trigger event transition of the external trigger signal."]
    VALUE1 = 0,
    #[doc = "1: Integration enabled. INTEN is set upon the defined trigger event."]
    VALUE2 = 1,
}
impl From<INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN` reader - Integration Enable"]
pub struct INTEN_R(crate::FieldReader<bool, INTEN_A>);
impl INTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN_A {
        match self.bits {
            false => INTEN_A::VALUE1,
            true => INTEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == INTEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == INTEN_A::VALUE2
    }
}
impl core::ops::Deref for INTEN_R {
    type Target = crate::FieldReader<bool, INTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REPCNT` reader - Integration Cycle Counter"]
pub struct REPCNT_R(crate::FieldReader<u8, u8>);
impl REPCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        REPCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REPCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REPVAL` reader - Number of Integration Cycles"]
pub struct REPVAL_R(crate::FieldReader<u8, u8>);
impl REPVAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        REPVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REPVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REPVAL` writer - Number of Integration Cycles"]
pub struct REPVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> REPVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `NVALDIS` reader - Number of Values Discarded"]
pub struct NVALDIS_R(crate::FieldReader<u8, u8>);
impl NVALDIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        NVALDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVALDIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVALDIS` writer - Number of Values Discarded"]
pub struct NVALDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NVALDIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Integration Window SIze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IWS_A {
    #[doc = "0: Internal control: stop integrator after REPVAL+1 integration cycles"]
    VALUE1 = 0,
    #[doc = "1: External control: stop integrator when bit INTEN becomes 0"]
    VALUE2 = 1,
}
impl From<IWS_A> for bool {
    #[inline(always)]
    fn from(variant: IWS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWS` reader - Integration Window SIze"]
pub struct IWS_R(crate::FieldReader<bool, IWS_A>);
impl IWS_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWS_A {
        match self.bits {
            false => IWS_A::VALUE1,
            true => IWS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == IWS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == IWS_A::VALUE2
    }
}
impl core::ops::Deref for IWS_R {
    type Target = crate::FieldReader<bool, IWS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWS` writer - Integration Window SIze"]
pub struct IWS_W<'a> {
    w: &'a mut W,
}
impl<'a> IWS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IWS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal control: stop integrator after REPVAL+1 integration cycles"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IWS_A::VALUE1)
    }
    #[doc = "External control: stop integrator when bit INTEN becomes 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IWS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `NVALINT` reader - Number of Values Integrated"]
pub struct NVALINT_R(crate::FieldReader<u8, u8>);
impl NVALINT_R {
    pub(crate) fn new(bits: u8) -> Self {
        NVALINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVALINT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVALINT` writer - Number of Values Integrated"]
pub struct NVALINT_W<'a> {
    w: &'a mut W,
}
impl<'a> NVALINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Number of Values Counted"]
    #[inline(always)]
    pub fn nvalcnt(&self) -> NVALCNT_R {
        NVALCNT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Integration Enable"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Integration Cycle Counter"]
    #[inline(always)]
    pub fn repcnt(&self) -> REPCNT_R {
        REPCNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Number of Integration Cycles"]
    #[inline(always)]
    pub fn repval(&self) -> REPVAL_R {
        REPVAL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Number of Values Discarded"]
    #[inline(always)]
    pub fn nvaldis(&self) -> NVALDIS_R {
        NVALDIS_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Integration Window SIze"]
    #[inline(always)]
    pub fn iws(&self) -> IWS_R {
        IWS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:29 - Number of Values Integrated"]
    #[inline(always)]
    pub fn nvalint(&self) -> NVALINT_R {
        NVALINT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Number of Integration Cycles"]
    #[inline(always)]
    pub fn repval(&mut self) -> REPVAL_W {
        REPVAL_W { w: self }
    }
    #[doc = "Bits 16:21 - Number of Values Discarded"]
    #[inline(always)]
    pub fn nvaldis(&mut self) -> NVALDIS_W {
        NVALDIS_W { w: self }
    }
    #[doc = "Bit 23 - Integration Window SIze"]
    #[inline(always)]
    pub fn iws(&mut self) -> IWS_W {
        IWS_W { w: self }
    }
    #[doc = "Bits 24:29 - Number of Values Integrated"]
    #[inline(always)]
    pub fn nvalint(&mut self) -> NVALINT_W {
        NVALINT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Integration Window Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iwctr](index.html) module"]
pub struct IWCTR_SPEC;
impl crate::RegisterSpec for IWCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iwctr::R](R) reader structure"]
impl crate::Readable for IWCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iwctr::W](W) writer structure"]
impl crate::Writable for IWCTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IWCTR to value 0"]
impl crate::Resettable for IWCTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
