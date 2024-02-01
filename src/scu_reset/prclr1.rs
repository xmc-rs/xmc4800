#[doc = "Register `PRCLR1` writer"]
pub type W = crate::W<PRCLR1_SPEC>;
#[doc = "CCU43 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU43RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<CCU43RS_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU43RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU43RS` writer - CCU43 Reset Clear"]
pub type CCU43RS_W<'a, REG> = crate::BitWriter<'a, REG, CCU43RS_AW>;
impl<'a, REG> CCU43RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCU43RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCU43RS_AW::VALUE2)
    }
}
#[doc = "LEDTS Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEDTSCU0RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<LEDTSCU0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: LEDTSCU0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDTSCU0RS` writer - LEDTS Reset Clear"]
pub type LEDTSCU0RS_W<'a, REG> = crate::BitWriter<'a, REG, LEDTSCU0RS_AW>;
impl<'a, REG> LEDTSCU0RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LEDTSCU0RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LEDTSCU0RS_AW::VALUE2)
    }
}
#[doc = "MultiCAN Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCAN0RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<MCAN0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: MCAN0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCAN0RS` writer - MultiCAN Reset Clear"]
pub type MCAN0RS_W<'a, REG> = crate::BitWriter<'a, REG, MCAN0RS_AW>;
impl<'a, REG> MCAN0RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MCAN0RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MCAN0RS_AW::VALUE2)
    }
}
#[doc = "DAC Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACRS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<DACRS_AW> for bool {
    #[inline(always)]
    fn from(variant: DACRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACRS` writer - DAC Reset Clear"]
pub type DACRS_W<'a, REG> = crate::BitWriter<'a, REG, DACRS_AW>;
impl<'a, REG> DACRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DACRS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DACRS_AW::VALUE2)
    }
}
#[doc = "MMC Interface Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMCIRS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<MMCIRS_AW> for bool {
    #[inline(always)]
    fn from(variant: MMCIRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCIRS` writer - MMC Interface Reset Clear"]
pub type MMCIRS_W<'a, REG> = crate::BitWriter<'a, REG, MMCIRS_AW>;
impl<'a, REG> MMCIRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MMCIRS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MMCIRS_AW::VALUE2)
    }
}
#[doc = "USIC1 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC1RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<USIC1RS_AW> for bool {
    #[inline(always)]
    fn from(variant: USIC1RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1RS` writer - USIC1 Reset Clear"]
pub type USIC1RS_W<'a, REG> = crate::BitWriter<'a, REG, USIC1RS_AW>;
impl<'a, REG> USIC1RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USIC1RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USIC1RS_AW::VALUE2)
    }
}
#[doc = "USIC2 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC2RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<USIC2RS_AW> for bool {
    #[inline(always)]
    fn from(variant: USIC2RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC2RS` writer - USIC2 Reset Clear"]
pub type USIC2RS_W<'a, REG> = crate::BitWriter<'a, REG, USIC2RS_AW>;
impl<'a, REG> USIC2RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USIC2RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USIC2RS_AW::VALUE2)
    }
}
#[doc = "PORTS Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPORTSRS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<PPORTSRS_AW> for bool {
    #[inline(always)]
    fn from(variant: PPORTSRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPORTSRS` writer - PORTS Reset Clear"]
pub type PPORTSRS_W<'a, REG> = crate::BitWriter<'a, REG, PPORTSRS_AW>;
impl<'a, REG> PPORTSRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PPORTSRS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PPORTSRS_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - CCU43 Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccu43rs(&mut self) -> CCU43RS_W<PRCLR1_SPEC> {
        CCU43RS_W::new(self, 0)
    }
    #[doc = "Bit 3 - LEDTS Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ledtscu0rs(&mut self) -> LEDTSCU0RS_W<PRCLR1_SPEC> {
        LEDTSCU0RS_W::new(self, 3)
    }
    #[doc = "Bit 4 - MultiCAN Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn mcan0rs(&mut self) -> MCAN0RS_W<PRCLR1_SPEC> {
        MCAN0RS_W::new(self, 4)
    }
    #[doc = "Bit 5 - DAC Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dacrs(&mut self) -> DACRS_W<PRCLR1_SPEC> {
        DACRS_W::new(self, 5)
    }
    #[doc = "Bit 6 - MMC Interface Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn mmcirs(&mut self) -> MMCIRS_W<PRCLR1_SPEC> {
        MMCIRS_W::new(self, 6)
    }
    #[doc = "Bit 7 - USIC1 Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usic1rs(&mut self) -> USIC1RS_W<PRCLR1_SPEC> {
        USIC1RS_W::new(self, 7)
    }
    #[doc = "Bit 8 - USIC2 Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usic2rs(&mut self) -> USIC2RS_W<PRCLR1_SPEC> {
        USIC2RS_W::new(self, 8)
    }
    #[doc = "Bit 9 - PORTS Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pportsrs(&mut self) -> PPORTSRS_W<PRCLR1_SPEC> {
        PPORTSRS_W::new(self, 9)
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
#[doc = "RCU Peripheral 1 Reset Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prclr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRCLR1_SPEC;
impl crate::RegisterSpec for PRCLR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prclr1::W`](W) writer structure"]
impl crate::Writable for PRCLR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRCLR1 to value 0"]
impl crate::Resettable for PRCLR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
