#[doc = "Register `EVFR` reader"]
pub type R = crate::R<EVFR_SPEC>;
#[doc = "Register `EVFR` writer"]
pub type W = crate::W<EVFR_SPEC>;
#[doc = "Field `TSF` reader - Time Slice Interrupt Flag"]
pub type TSF_R = crate::BitReader;
#[doc = "Field `TFF` reader - (Extended) Time Frame Interrupt Flag"]
pub type TFF_R = crate::BitReader;
#[doc = "Field `TPF` reader - Autoscan Time Period Interrupt Flag"]
pub type TPF_R = crate::BitReader;
#[doc = "Field `TSCTROVF` reader - TS-Counter Overflow Indication"]
pub type TSCTROVF_R = crate::BitReader<TSCTROVF_A>;
#[doc = "TS-Counter Overflow Indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSCTROVF_A {
    #[doc = "0: No overflow has occurred."]
    VALUE1 = 0,
    #[doc = "1: The TS-counter has overflowed at least once."]
    VALUE2 = 1,
}
impl From<TSCTROVF_A> for bool {
    #[inline(always)]
    fn from(variant: TSCTROVF_A) -> Self {
        variant as u8 != 0
    }
}
impl TSCTROVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSCTROVF_A {
        match self.bits {
            false => TSCTROVF_A::VALUE1,
            true => TSCTROVF_A::VALUE2,
        }
    }
    #[doc = "No overflow has occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSCTROVF_A::VALUE1
    }
    #[doc = "The TS-counter has overflowed at least once."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSCTROVF_A::VALUE2
    }
}
#[doc = "Clear Time Slice Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSF_AW {
    #[doc = "0: No action."]
    VALUE1 = 0,
    #[doc = "1: Bit TSF is cleared."]
    VALUE2 = 1,
}
impl From<CTSF_AW> for bool {
    #[inline(always)]
    fn from(variant: CTSF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSF` writer - Clear Time Slice Interrupt Flag"]
pub type CTSF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CTSF_AW>;
impl<'a, REG, const O: u8> CTSF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSF_AW::VALUE1)
    }
    #[doc = "Bit TSF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CTSF_AW::VALUE2)
    }
}
#[doc = "Clear (Extended) Time Frame Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTFF_AW {
    #[doc = "0: No action."]
    VALUE1 = 0,
    #[doc = "1: Bit TFF is cleared."]
    VALUE2 = 1,
}
impl From<CTFF_AW> for bool {
    #[inline(always)]
    fn from(variant: CTFF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTFF` writer - Clear (Extended) Time Frame Interrupt Flag"]
pub type CTFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CTFF_AW>;
impl<'a, REG, const O: u8> CTFF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CTFF_AW::VALUE1)
    }
    #[doc = "Bit TFF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CTFF_AW::VALUE2)
    }
}
#[doc = "Clear Autoscan Time Period Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTPF_AW {
    #[doc = "0: No action."]
    VALUE1 = 0,
    #[doc = "1: Bit TPF is cleared."]
    VALUE2 = 1,
}
impl From<CTPF_AW> for bool {
    #[inline(always)]
    fn from(variant: CTPF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTPF` writer - Clear Autoscan Time Period Interrupt Flag"]
pub type CTPF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CTPF_AW>;
impl<'a, REG, const O: u8> CTPF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CTPF_AW::VALUE1)
    }
    #[doc = "Bit TPF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CTPF_AW::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Time Slice Interrupt Flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - (Extended) Time Frame Interrupt Flag"]
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Autoscan Time Period Interrupt Flag"]
    #[inline(always)]
    pub fn tpf(&self) -> TPF_R {
        TPF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TS-Counter Overflow Indication"]
    #[inline(always)]
    pub fn tsctrovf(&self) -> TSCTROVF_R {
        TSCTROVF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Clear Time Slice Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctsf(&mut self) -> CTSF_W<EVFR_SPEC, 16> {
        CTSF_W::new(self)
    }
    #[doc = "Bit 17 - Clear (Extended) Time Frame Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctff(&mut self) -> CTFF_W<EVFR_SPEC, 17> {
        CTFF_W::new(self)
    }
    #[doc = "Bit 18 - Clear Autoscan Time Period Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctpf(&mut self) -> CTPF_W<EVFR_SPEC, 18> {
        CTPF_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVFR_SPEC;
impl crate::RegisterSpec for EVFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evfr::R`](R) reader structure"]
impl crate::Readable for EVFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evfr::W`](W) writer structure"]
impl crate::Writable for EVFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVFR to value 0"]
impl crate::Resettable for EVFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
