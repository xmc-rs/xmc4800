#[doc = "Register `FCFGA` reader"]
pub type R = crate::R<FCFGA_SPEC>;
#[doc = "Register `FCFGA` writer"]
pub type W = crate::W<FCFGA_SPEC>;
#[doc = "Field `CFADF` reader - CIC Filter (Auxiliary) Decimation Factor"]
pub type CFADF_R = crate::FieldReader;
#[doc = "Field `CFADF` writer - CIC Filter (Auxiliary) Decimation Factor"]
pub type CFADF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CFAC` reader - CIC Filter (Auxiliary) Configuration"]
pub type CFAC_R = crate::FieldReader<CFAC_A>;
#[doc = "CIC Filter (Auxiliary) Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for CFAC_A {
    type Ux = u8;
}
impl CFAC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CFAC_A {
        match self.bits {
            0 => CFAC_A::VALUE1,
            1 => CFAC_A::VALUE2,
            2 => CFAC_A::VALUE3,
            3 => CFAC_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "CIC1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFAC_A::VALUE1
    }
    #[doc = "CIC2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFAC_A::VALUE2
    }
    #[doc = "CIC3"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CFAC_A::VALUE3
    }
    #[doc = "CICF"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CFAC_A::VALUE4
    }
}
#[doc = "Field `CFAC` writer - CIC Filter (Auxiliary) Configuration"]
pub type CFAC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CFAC_A>;
impl<'a, REG> CFAC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CIC1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CFAC_A::VALUE1)
    }
    #[doc = "CIC2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CFAC_A::VALUE2)
    }
    #[doc = "CIC3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CFAC_A::VALUE3)
    }
    #[doc = "CICF"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CFAC_A::VALUE4)
    }
}
#[doc = "Field `SRGA` reader - Service Request Generation Auxiliary Filter"]
pub type SRGA_R = crate::FieldReader<SRGA_A>;
#[doc = "Service Request Generation Auxiliary Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for SRGA_A {
    type Ux = u8;
}
impl SRGA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SRGA_A> {
        match self.bits {
            0 => Some(SRGA_A::VALUE1),
            1 => Some(SRGA_A::VALUE2),
            2 => Some(SRGA_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Never, service requests disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRGA_A::VALUE1
    }
    #[doc = "Auxiliary filter: As selected by bitfields ESEL and EGT (if integrator enabled)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRGA_A::VALUE2
    }
    #[doc = "Alternate source: Capturing of a sign delay value to register CGSYNCx (x = 0 - 3)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SRGA_A::VALUE3
    }
}
#[doc = "Field `SRGA` writer - Service Request Generation Auxiliary Filter"]
pub type SRGA_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SRGA_A>;
impl<'a, REG> SRGA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Never, service requests disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SRGA_A::VALUE1)
    }
    #[doc = "Auxiliary filter: As selected by bitfields ESEL and EGT (if integrator enabled)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SRGA_A::VALUE2)
    }
    #[doc = "Alternate source: Capturing of a sign delay value to register CGSYNCx (x = 0 - 3)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(SRGA_A::VALUE3)
    }
}
#[doc = "Field `ESEL` reader - Event Select"]
pub type ESEL_R = crate::FieldReader<ESEL_A>;
#[doc = "Event Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for ESEL_A {
    type Ux = u8;
}
impl ESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ESEL_A> {
        match self.bits {
            0 => Some(ESEL_A::VALUE1),
            1 => Some(ESEL_A::VALUE2),
            2 => Some(ESEL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Always, for each new result value"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ESEL_A::VALUE1
    }
    #[doc = "If result is inside the boundary band"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ESEL_A::VALUE2
    }
    #[doc = "If result is outside the boundary band"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ESEL_A::VALUE3
    }
}
#[doc = "Field `ESEL` writer - Event Select"]
pub type ESEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ESEL_A>;
impl<'a, REG> ESEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Always, for each new result value"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ESEL_A::VALUE1)
    }
    #[doc = "If result is inside the boundary band"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ESEL_A::VALUE2)
    }
    #[doc = "If result is outside the boundary band"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(ESEL_A::VALUE3)
    }
}
#[doc = "Field `EGT` reader - Event Gating"]
pub type EGT_R = crate::BitReader<EGT_A>;
#[doc = "Event Gating\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl EGT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EGT_A {
        match self.bits {
            false => EGT_A::VALUE1,
            true => EGT_A::VALUE2,
        }
    }
    #[doc = "Separate: generate events according to ESEL"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EGT_A::VALUE1
    }
    #[doc = "Coupled: generate events only when the integrator is enabled and after the discard phase defined by bitfield NVALDISWhile the integrator is bypassed, event gating is disabled, i.e. service requests are generated according to bitfield ESEL. The event gating suppresses service requests, result values are still stored in register RESAx."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EGT_A::VALUE2
    }
}
#[doc = "Field `EGT` writer - Event Gating"]
pub type EGT_W<'a, REG> = crate::BitWriter<'a, REG, EGT_A>;
impl<'a, REG> EGT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Separate: generate events according to ESEL"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EGT_A::VALUE1)
    }
    #[doc = "Coupled: generate events only when the integrator is enabled and after the discard phase defined by bitfield NVALDISWhile the integrator is bypassed, event gating is disabled, i.e. service requests are generated according to bitfield ESEL. The event gating suppresses service requests, result values are still stored in register RESAx."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EGT_A::VALUE2)
    }
}
#[doc = "Field `CFADCNT` reader - CIC Filter (Auxiliary) Decimation Counter"]
pub type CFADCNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CIC Filter (Auxiliary) Decimation Factor"]
    #[inline(always)]
    pub fn cfadf(&self) -> CFADF_R {
        CFADF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - CIC Filter (Auxiliary) Configuration"]
    #[inline(always)]
    pub fn cfac(&self) -> CFAC_R {
        CFAC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Service Request Generation Auxiliary Filter"]
    #[inline(always)]
    pub fn srga(&self) -> SRGA_R {
        SRGA_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Event Select"]
    #[inline(always)]
    pub fn esel(&self) -> ESEL_R {
        ESEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Event Gating"]
    #[inline(always)]
    pub fn egt(&self) -> EGT_R {
        EGT_R::new(((self.bits >> 14) & 1) != 0)
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
    #[must_use]
    pub fn cfadf(&mut self) -> CFADF_W<FCFGA_SPEC> {
        CFADF_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - CIC Filter (Auxiliary) Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn cfac(&mut self) -> CFAC_W<FCFGA_SPEC> {
        CFAC_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Service Request Generation Auxiliary Filter"]
    #[inline(always)]
    #[must_use]
    pub fn srga(&mut self) -> SRGA_W<FCFGA_SPEC> {
        SRGA_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Event Select"]
    #[inline(always)]
    #[must_use]
    pub fn esel(&mut self) -> ESEL_W<FCFGA_SPEC> {
        ESEL_W::new(self, 12)
    }
    #[doc = "Bit 14 - Event Gating"]
    #[inline(always)]
    #[must_use]
    pub fn egt(&mut self) -> EGT_W<FCFGA_SPEC> {
        EGT_W::new(self, 14)
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
#[doc = "Filter Configuration Register, Auxiliary Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfga::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfga::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCFGA_SPEC;
impl crate::RegisterSpec for FCFGA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcfga::R`](R) reader structure"]
impl crate::Readable for FCFGA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fcfga::W`](W) writer structure"]
impl crate::Writable for FCFGA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCFGA to value 0"]
impl crate::Resettable for FCFGA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
