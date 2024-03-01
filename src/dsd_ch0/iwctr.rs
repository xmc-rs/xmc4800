#[doc = "Register `IWCTR` reader"]
pub type R = crate::R<IwctrSpec>;
#[doc = "Register `IWCTR` writer"]
pub type W = crate::W<IwctrSpec>;
#[doc = "Field `NVALCNT` reader - Number of Values Counted"]
pub type NvalcntR = crate::FieldReader;
#[doc = "Integration Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inten {
    #[doc = "0: Integration stopped. INTEN is cleared at the end of the integration window, i.e. upon the inverse trigger event transition of the external trigger signal."]
    Value1 = 0,
    #[doc = "1: Integration enabled. INTEN is set upon the defined trigger event."]
    Value2 = 1,
}
impl From<Inten> for bool {
    #[inline(always)]
    fn from(variant: Inten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN` reader - Integration Enable"]
pub type IntenR = crate::BitReader<Inten>;
impl IntenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inten {
        match self.bits {
            false => Inten::Value1,
            true => Inten::Value2,
        }
    }
    #[doc = "Integration stopped. INTEN is cleared at the end of the integration window, i.e. upon the inverse trigger event transition of the external trigger signal."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Inten::Value1
    }
    #[doc = "Integration enabled. INTEN is set upon the defined trigger event."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Inten::Value2
    }
}
#[doc = "Field `REPCNT` reader - Integration Cycle Counter"]
pub type RepcntR = crate::FieldReader;
#[doc = "Field `REPVAL` reader - Number of Integration Cycles"]
pub type RepvalR = crate::FieldReader;
#[doc = "Field `REPVAL` writer - Number of Integration Cycles"]
pub type RepvalW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NVALDIS` reader - Number of Values Discarded"]
pub type NvaldisR = crate::FieldReader;
#[doc = "Field `NVALDIS` writer - Number of Values Discarded"]
pub type NvaldisW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Integration Window SIze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iws {
    #[doc = "0: Internal control: stop integrator after REPVAL+1 integration cycles"]
    Value1 = 0,
    #[doc = "1: External control: stop integrator when bit INTEN becomes 0"]
    Value2 = 1,
}
impl From<Iws> for bool {
    #[inline(always)]
    fn from(variant: Iws) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWS` reader - Integration Window SIze"]
pub type IwsR = crate::BitReader<Iws>;
impl IwsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iws {
        match self.bits {
            false => Iws::Value1,
            true => Iws::Value2,
        }
    }
    #[doc = "Internal control: stop integrator after REPVAL+1 integration cycles"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Iws::Value1
    }
    #[doc = "External control: stop integrator when bit INTEN becomes 0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Iws::Value2
    }
}
#[doc = "Field `IWS` writer - Integration Window SIze"]
pub type IwsW<'a, REG> = crate::BitWriter<'a, REG, Iws>;
impl<'a, REG> IwsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal control: stop integrator after REPVAL+1 integration cycles"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Iws::Value1)
    }
    #[doc = "External control: stop integrator when bit INTEN becomes 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Iws::Value2)
    }
}
#[doc = "Field `NVALINT` reader - Number of Values Integrated"]
pub type NvalintR = crate::FieldReader;
#[doc = "Field `NVALINT` writer - Number of Values Integrated"]
pub type NvalintW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Number of Values Counted"]
    #[inline(always)]
    pub fn nvalcnt(&self) -> NvalcntR {
        NvalcntR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Integration Enable"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Integration Cycle Counter"]
    #[inline(always)]
    pub fn repcnt(&self) -> RepcntR {
        RepcntR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Number of Integration Cycles"]
    #[inline(always)]
    pub fn repval(&self) -> RepvalR {
        RepvalR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Number of Values Discarded"]
    #[inline(always)]
    pub fn nvaldis(&self) -> NvaldisR {
        NvaldisR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Integration Window SIze"]
    #[inline(always)]
    pub fn iws(&self) -> IwsR {
        IwsR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Number of Values Integrated"]
    #[inline(always)]
    pub fn nvalint(&self) -> NvalintR {
        NvalintR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Number of Integration Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn repval(&mut self) -> RepvalW<IwctrSpec> {
        RepvalW::new(self, 12)
    }
    #[doc = "Bits 16:21 - Number of Values Discarded"]
    #[inline(always)]
    #[must_use]
    pub fn nvaldis(&mut self) -> NvaldisW<IwctrSpec> {
        NvaldisW::new(self, 16)
    }
    #[doc = "Bit 23 - Integration Window SIze"]
    #[inline(always)]
    #[must_use]
    pub fn iws(&mut self) -> IwsW<IwctrSpec> {
        IwsW::new(self, 23)
    }
    #[doc = "Bits 24:29 - Number of Values Integrated"]
    #[inline(always)]
    #[must_use]
    pub fn nvalint(&mut self) -> NvalintW<IwctrSpec> {
        NvalintW::new(self, 24)
    }
}
#[doc = "Integration Window Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwctrSpec;
impl crate::RegisterSpec for IwctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwctr::R`](R) reader structure"]
impl crate::Readable for IwctrSpec {}
#[doc = "`write(|w| ..)` method takes [`iwctr::W`](W) writer structure"]
impl crate::Writable for IwctrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IWCTR to value 0"]
impl crate::Resettable for IwctrSpec {
    const RESET_VALUE: u32 = 0;
}
