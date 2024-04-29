#[doc = "Register `IWCTR` reader"]
pub type R = crate::R<IWCTR_SPEC>;
#[doc = "Register `IWCTR` writer"]
pub type W = crate::W<IWCTR_SPEC>;
#[doc = "Field `NVALCNT` reader - Number of Values Counted"]
pub type NVALCNT_R = crate::FieldReader;
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
#[doc = "Field `INTEN` reader - Integration Enable"]
pub type INTEN_R = crate::BitReader<INTEN_A>;
impl INTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTEN_A {
        match self.bits {
            false => INTEN_A::VALUE1,
            true => INTEN_A::VALUE2,
        }
    }
    #[doc = "Integration stopped. INTEN is cleared at the end of the integration window, i.e. upon the inverse trigger event transition of the external trigger signal."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INTEN_A::VALUE1
    }
    #[doc = "Integration enabled. INTEN is set upon the defined trigger event."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INTEN_A::VALUE2
    }
}
#[doc = "Field `REPCNT` reader - Integration Cycle Counter"]
pub type REPCNT_R = crate::FieldReader;
#[doc = "Field `REPVAL` reader - Number of Integration Cycles"]
pub type REPVAL_R = crate::FieldReader;
#[doc = "Field `REPVAL` writer - Number of Integration Cycles"]
pub type REPVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NVALDIS` reader - Number of Values Discarded"]
pub type NVALDIS_R = crate::FieldReader;
#[doc = "Field `NVALDIS` writer - Number of Values Discarded"]
pub type NVALDIS_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
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
#[doc = "Field `IWS` reader - Integration Window SIze"]
pub type IWS_R = crate::BitReader<IWS_A>;
impl IWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IWS_A {
        match self.bits {
            false => IWS_A::VALUE1,
            true => IWS_A::VALUE2,
        }
    }
    #[doc = "Internal control: stop integrator after REPVAL+1 integration cycles"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IWS_A::VALUE1
    }
    #[doc = "External control: stop integrator when bit INTEN becomes 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IWS_A::VALUE2
    }
}
#[doc = "Field `IWS` writer - Integration Window SIze"]
pub type IWS_W<'a, REG> = crate::BitWriter<'a, REG, IWS_A>;
impl<'a, REG> IWS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal control: stop integrator after REPVAL+1 integration cycles"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(IWS_A::VALUE1)
    }
    #[doc = "External control: stop integrator when bit INTEN becomes 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(IWS_A::VALUE2)
    }
}
#[doc = "Field `NVALINT` reader - Number of Values Integrated"]
pub type NVALINT_R = crate::FieldReader;
#[doc = "Field `NVALINT` writer - Number of Values Integrated"]
pub type NVALINT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
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
    pub fn repval(&mut self) -> REPVAL_W<IWCTR_SPEC> {
        REPVAL_W::new(self, 12)
    }
    #[doc = "Bits 16:21 - Number of Values Discarded"]
    #[inline(always)]
    #[must_use]
    pub fn nvaldis(&mut self) -> NVALDIS_W<IWCTR_SPEC> {
        NVALDIS_W::new(self, 16)
    }
    #[doc = "Bit 23 - Integration Window SIze"]
    #[inline(always)]
    #[must_use]
    pub fn iws(&mut self) -> IWS_W<IWCTR_SPEC> {
        IWS_W::new(self, 23)
    }
    #[doc = "Bits 24:29 - Number of Values Integrated"]
    #[inline(always)]
    #[must_use]
    pub fn nvalint(&mut self) -> NVALINT_W<IWCTR_SPEC> {
        NVALINT_W::new(self, 24)
    }
}
#[doc = "Integration Window Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IWCTR_SPEC;
impl crate::RegisterSpec for IWCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwctr::R`](R) reader structure"]
impl crate::Readable for IWCTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iwctr::W`](W) writer structure"]
impl crate::Writable for IWCTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IWCTR to value 0"]
impl crate::Resettable for IWCTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
