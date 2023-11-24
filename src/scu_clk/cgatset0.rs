#[doc = "Register `CGATSET0` writer"]
pub type W = crate::W<CGATSET0_SPEC>;
#[doc = "VADC Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VADC_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<VADC_AW> for bool {
    #[inline(always)]
    fn from(variant: VADC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VADC` writer - VADC Gating Set"]
pub type VADC_W<'a, REG> = crate::BitWriter<'a, REG, VADC_AW>;
impl<'a, REG> VADC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VADC_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VADC_AW::VALUE2)
    }
}
#[doc = "DSD Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSD_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<DSD_AW> for bool {
    #[inline(always)]
    fn from(variant: DSD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSD` writer - DSD Gating Set"]
pub type DSD_W<'a, REG> = crate::BitWriter<'a, REG, DSD_AW>;
impl<'a, REG> DSD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DSD_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DSD_AW::VALUE2)
    }
}
#[doc = "CCU40 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU40_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<CCU40_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU40_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU40` writer - CCU40 Gating Set"]
pub type CCU40_W<'a, REG> = crate::BitWriter<'a, REG, CCU40_AW>;
impl<'a, REG> CCU40_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCU40_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCU40_AW::VALUE2)
    }
}
#[doc = "CCU41 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU41_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<CCU41_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU41_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU41` writer - CCU41 Gating Set"]
pub type CCU41_W<'a, REG> = crate::BitWriter<'a, REG, CCU41_AW>;
impl<'a, REG> CCU41_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCU41_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCU41_AW::VALUE2)
    }
}
#[doc = "CCU42 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU42_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<CCU42_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU42_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU42` writer - CCU42 Gating Set"]
pub type CCU42_W<'a, REG> = crate::BitWriter<'a, REG, CCU42_AW>;
impl<'a, REG> CCU42_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCU42_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCU42_AW::VALUE2)
    }
}
#[doc = "CCU80 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU80_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<CCU80_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU80_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU80` writer - CCU80 Gating Set"]
pub type CCU80_W<'a, REG> = crate::BitWriter<'a, REG, CCU80_AW>;
impl<'a, REG> CCU80_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCU80_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCU80_AW::VALUE2)
    }
}
#[doc = "CCU81 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU81_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<CCU81_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU81_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU81` writer - CCU81 Gating Set"]
pub type CCU81_W<'a, REG> = crate::BitWriter<'a, REG, CCU81_AW>;
impl<'a, REG> CCU81_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCU81_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCU81_AW::VALUE2)
    }
}
#[doc = "POSIF0 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POSIF0_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<POSIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: POSIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSIF0` writer - POSIF0 Gating Set"]
pub type POSIF0_W<'a, REG> = crate::BitWriter<'a, REG, POSIF0_AW>;
impl<'a, REG> POSIF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(POSIF0_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(POSIF0_AW::VALUE2)
    }
}
#[doc = "POSIF1 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POSIF1_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<POSIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: POSIF1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSIF1` writer - POSIF1 Gating Set"]
pub type POSIF1_W<'a, REG> = crate::BitWriter<'a, REG, POSIF1_AW>;
impl<'a, REG> POSIF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(POSIF1_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(POSIF1_AW::VALUE2)
    }
}
#[doc = "USIC0 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC0_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<USIC0_AW> for bool {
    #[inline(always)]
    fn from(variant: USIC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC0` writer - USIC0 Gating Set"]
pub type USIC0_W<'a, REG> = crate::BitWriter<'a, REG, USIC0_AW>;
impl<'a, REG> USIC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USIC0_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USIC0_AW::VALUE2)
    }
}
#[doc = "ERU1 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERU1_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<ERU1_AW> for bool {
    #[inline(always)]
    fn from(variant: ERU1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU1` writer - ERU1 Gating Set"]
pub type ERU1_W<'a, REG> = crate::BitWriter<'a, REG, ERU1_AW>;
impl<'a, REG> ERU1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ERU1_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ERU1_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - VADC Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn vadc(&mut self) -> VADC_W<CGATSET0_SPEC> {
        VADC_W::new(self, 0)
    }
    #[doc = "Bit 1 - DSD Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn dsd(&mut self) -> DSD_W<CGATSET0_SPEC> {
        DSD_W::new(self, 1)
    }
    #[doc = "Bit 2 - CCU40 Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn ccu40(&mut self) -> CCU40_W<CGATSET0_SPEC> {
        CCU40_W::new(self, 2)
    }
    #[doc = "Bit 3 - CCU41 Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn ccu41(&mut self) -> CCU41_W<CGATSET0_SPEC> {
        CCU41_W::new(self, 3)
    }
    #[doc = "Bit 4 - CCU42 Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn ccu42(&mut self) -> CCU42_W<CGATSET0_SPEC> {
        CCU42_W::new(self, 4)
    }
    #[doc = "Bit 7 - CCU80 Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn ccu80(&mut self) -> CCU80_W<CGATSET0_SPEC> {
        CCU80_W::new(self, 7)
    }
    #[doc = "Bit 8 - CCU81 Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn ccu81(&mut self) -> CCU81_W<CGATSET0_SPEC> {
        CCU81_W::new(self, 8)
    }
    #[doc = "Bit 9 - POSIF0 Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn posif0(&mut self) -> POSIF0_W<CGATSET0_SPEC> {
        POSIF0_W::new(self, 9)
    }
    #[doc = "Bit 10 - POSIF1 Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn posif1(&mut self) -> POSIF1_W<CGATSET0_SPEC> {
        POSIF1_W::new(self, 10)
    }
    #[doc = "Bit 11 - USIC0 Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn usic0(&mut self) -> USIC0_W<CGATSET0_SPEC> {
        USIC0_W::new(self, 11)
    }
    #[doc = "Bit 16 - ERU1 Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn eru1(&mut self) -> ERU1_W<CGATSET0_SPEC> {
        ERU1_W::new(self, 16)
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
#[doc = "Peripheral 0 Clock Gating Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatset0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CGATSET0_SPEC;
impl crate::RegisterSpec for CGATSET0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cgatset0::W`](W) writer structure"]
impl crate::Writable for CGATSET0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CGATSET0 to value 0"]
impl crate::Resettable for CGATSET0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
