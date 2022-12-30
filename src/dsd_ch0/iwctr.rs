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
pub type NVALCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTEN` reader - Integration Enable"]
pub type INTEN_R = crate::BitReader<INTEN_A>;
#[doc = "Integration Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl INTEN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == INTEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INTEN_A::VALUE2
    }
}
#[doc = "Field `REPCNT` reader - Integration Cycle Counter"]
pub type REPCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REPVAL` reader - Number of Integration Cycles"]
pub type REPVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REPVAL` writer - Number of Integration Cycles"]
pub type REPVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IWCTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `NVALDIS` reader - Number of Values Discarded"]
pub type NVALDIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NVALDIS` writer - Number of Values Discarded"]
pub type NVALDIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IWCTR_SPEC, u8, u8, 6, O>;
#[doc = "Field `IWS` reader - Integration Window SIze"]
pub type IWS_R = crate::BitReader<IWS_A>;
#[doc = "Integration Window SIze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl IWS_R {
    #[doc = "Get enumerated values variant"]
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
        *self == IWS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IWS_A::VALUE2
    }
}
#[doc = "Field `IWS` writer - Integration Window SIze"]
pub type IWS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IWCTR_SPEC, IWS_A, O>;
impl<'a, const O: u8> IWS_W<'a, O> {
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
}
#[doc = "Field `NVALINT` reader - Number of Values Integrated"]
pub type NVALINT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NVALINT` writer - Number of Values Integrated"]
pub type NVALINT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IWCTR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Number of Values Counted"]
    #[inline(always)]
    pub fn nvalcnt(&self) -> NVALCNT_R {
        NVALCNT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Integration Enable"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 7) & 1) != 0)
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
        IWS_R::new(((self.bits >> 23) & 1) != 0)
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
    #[must_use]
    pub fn repval(&mut self) -> REPVAL_W<12> {
        REPVAL_W::new(self)
    }
    #[doc = "Bits 16:21 - Number of Values Discarded"]
    #[inline(always)]
    #[must_use]
    pub fn nvaldis(&mut self) -> NVALDIS_W<16> {
        NVALDIS_W::new(self)
    }
    #[doc = "Bit 23 - Integration Window SIze"]
    #[inline(always)]
    #[must_use]
    pub fn iws(&mut self) -> IWS_W<23> {
        IWS_W::new(self)
    }
    #[doc = "Bits 24:29 - Number of Values Integrated"]
    #[inline(always)]
    #[must_use]
    pub fn nvalint(&mut self) -> NVALINT_W<24> {
        NVALINT_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IWCTR to value 0"]
impl crate::Resettable for IWCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
