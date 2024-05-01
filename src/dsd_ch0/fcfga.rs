#[doc = "Register `FCFGA` reader"]
pub type R = crate::R<FcfgaSpec>;
#[doc = "Register `FCFGA` writer"]
pub type W = crate::W<FcfgaSpec>;
#[doc = "Field `CFADF` reader - CIC Filter (Auxiliary) Decimation Factor"]
pub type CfadfR = crate::FieldReader;
#[doc = "Field `CFADF` writer - CIC Filter (Auxiliary) Decimation Factor"]
pub type CfadfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "CIC Filter (Auxiliary) Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfac {
    #[doc = "0: CIC1"]
    Value1 = 0,
    #[doc = "1: CIC2"]
    Value2 = 1,
    #[doc = "2: CIC3"]
    Value3 = 2,
    #[doc = "3: CICF"]
    Value4 = 3,
}
impl From<Cfac> for u8 {
    #[inline(always)]
    fn from(variant: Cfac) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfac {
    type Ux = u8;
}
impl crate::IsEnum for Cfac {}
#[doc = "Field `CFAC` reader - CIC Filter (Auxiliary) Configuration"]
pub type CfacR = crate::FieldReader<Cfac>;
impl CfacR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfac {
        match self.bits {
            0 => Cfac::Value1,
            1 => Cfac::Value2,
            2 => Cfac::Value3,
            3 => Cfac::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "CIC1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cfac::Value1
    }
    #[doc = "CIC2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cfac::Value2
    }
    #[doc = "CIC3"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cfac::Value3
    }
    #[doc = "CICF"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cfac::Value4
    }
}
#[doc = "Field `CFAC` writer - CIC Filter (Auxiliary) Configuration"]
pub type CfacW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cfac, crate::Safe>;
impl<'a, REG> CfacW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CIC1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfac::Value1)
    }
    #[doc = "CIC2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cfac::Value2)
    }
    #[doc = "CIC3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cfac::Value3)
    }
    #[doc = "CICF"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cfac::Value4)
    }
}
#[doc = "Service Request Generation Auxiliary Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Srga {
    #[doc = "0: Never, service requests disabled"]
    Value1 = 0,
    #[doc = "1: Auxiliary filter: As selected by bitfields ESEL and EGT (if integrator enabled)"]
    Value2 = 1,
    #[doc = "2: Alternate source: Capturing of a sign delay value to register CGSYNCx (x = 0 - 3)"]
    Value3 = 2,
}
impl From<Srga> for u8 {
    #[inline(always)]
    fn from(variant: Srga) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Srga {
    type Ux = u8;
}
impl crate::IsEnum for Srga {}
#[doc = "Field `SRGA` reader - Service Request Generation Auxiliary Filter"]
pub type SrgaR = crate::FieldReader<Srga>;
impl SrgaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Srga> {
        match self.bits {
            0 => Some(Srga::Value1),
            1 => Some(Srga::Value2),
            2 => Some(Srga::Value3),
            _ => None,
        }
    }
    #[doc = "Never, service requests disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Srga::Value1
    }
    #[doc = "Auxiliary filter: As selected by bitfields ESEL and EGT (if integrator enabled)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Srga::Value2
    }
    #[doc = "Alternate source: Capturing of a sign delay value to register CGSYNCx (x = 0 - 3)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Srga::Value3
    }
}
#[doc = "Field `SRGA` writer - Service Request Generation Auxiliary Filter"]
pub type SrgaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Srga>;
impl<'a, REG> SrgaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Never, service requests disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Srga::Value1)
    }
    #[doc = "Auxiliary filter: As selected by bitfields ESEL and EGT (if integrator enabled)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Srga::Value2)
    }
    #[doc = "Alternate source: Capturing of a sign delay value to register CGSYNCx (x = 0 - 3)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Srga::Value3)
    }
}
#[doc = "Event Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Esel {
    #[doc = "0: Always, for each new result value"]
    Value1 = 0,
    #[doc = "1: If result is inside the boundary band"]
    Value2 = 1,
    #[doc = "2: If result is outside the boundary band"]
    Value3 = 2,
}
impl From<Esel> for u8 {
    #[inline(always)]
    fn from(variant: Esel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Esel {
    type Ux = u8;
}
impl crate::IsEnum for Esel {}
#[doc = "Field `ESEL` reader - Event Select"]
pub type EselR = crate::FieldReader<Esel>;
impl EselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Esel> {
        match self.bits {
            0 => Some(Esel::Value1),
            1 => Some(Esel::Value2),
            2 => Some(Esel::Value3),
            _ => None,
        }
    }
    #[doc = "Always, for each new result value"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Esel::Value1
    }
    #[doc = "If result is inside the boundary band"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Esel::Value2
    }
    #[doc = "If result is outside the boundary band"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Esel::Value3
    }
}
#[doc = "Field `ESEL` writer - Event Select"]
pub type EselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Esel>;
impl<'a, REG> EselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Always, for each new result value"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Esel::Value1)
    }
    #[doc = "If result is inside the boundary band"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Esel::Value2)
    }
    #[doc = "If result is outside the boundary band"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Esel::Value3)
    }
}
#[doc = "Event Gating\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Egt {
    #[doc = "0: Separate: generate events according to ESEL"]
    Value1 = 0,
    #[doc = "1: Coupled: generate events only when the integrator is enabled and after the discard phase defined by bitfield NVALDISWhile the integrator is bypassed, event gating is disabled, i.e. service requests are generated according to bitfield ESEL. The event gating suppresses service requests, result values are still stored in register RESAx."]
    Value2 = 1,
}
impl From<Egt> for bool {
    #[inline(always)]
    fn from(variant: Egt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EGT` reader - Event Gating"]
pub type EgtR = crate::BitReader<Egt>;
impl EgtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Egt {
        match self.bits {
            false => Egt::Value1,
            true => Egt::Value2,
        }
    }
    #[doc = "Separate: generate events according to ESEL"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Egt::Value1
    }
    #[doc = "Coupled: generate events only when the integrator is enabled and after the discard phase defined by bitfield NVALDISWhile the integrator is bypassed, event gating is disabled, i.e. service requests are generated according to bitfield ESEL. The event gating suppresses service requests, result values are still stored in register RESAx."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Egt::Value2
    }
}
#[doc = "Field `EGT` writer - Event Gating"]
pub type EgtW<'a, REG> = crate::BitWriter<'a, REG, Egt>;
impl<'a, REG> EgtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Separate: generate events according to ESEL"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Egt::Value1)
    }
    #[doc = "Coupled: generate events only when the integrator is enabled and after the discard phase defined by bitfield NVALDISWhile the integrator is bypassed, event gating is disabled, i.e. service requests are generated according to bitfield ESEL. The event gating suppresses service requests, result values are still stored in register RESAx."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Egt::Value2)
    }
}
#[doc = "Field `CFADCNT` reader - CIC Filter (Auxiliary) Decimation Counter"]
pub type CfadcntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CIC Filter (Auxiliary) Decimation Factor"]
    #[inline(always)]
    pub fn cfadf(&self) -> CfadfR {
        CfadfR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - CIC Filter (Auxiliary) Configuration"]
    #[inline(always)]
    pub fn cfac(&self) -> CfacR {
        CfacR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Service Request Generation Auxiliary Filter"]
    #[inline(always)]
    pub fn srga(&self) -> SrgaR {
        SrgaR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Event Select"]
    #[inline(always)]
    pub fn esel(&self) -> EselR {
        EselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Event Gating"]
    #[inline(always)]
    pub fn egt(&self) -> EgtR {
        EgtR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 24:31 - CIC Filter (Auxiliary) Decimation Counter"]
    #[inline(always)]
    pub fn cfadcnt(&self) -> CfadcntR {
        CfadcntR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CIC Filter (Auxiliary) Decimation Factor"]
    #[inline(always)]
    #[must_use]
    pub fn cfadf(&mut self) -> CfadfW<FcfgaSpec> {
        CfadfW::new(self, 0)
    }
    #[doc = "Bits 8:9 - CIC Filter (Auxiliary) Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn cfac(&mut self) -> CfacW<FcfgaSpec> {
        CfacW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Service Request Generation Auxiliary Filter"]
    #[inline(always)]
    #[must_use]
    pub fn srga(&mut self) -> SrgaW<FcfgaSpec> {
        SrgaW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Event Select"]
    #[inline(always)]
    #[must_use]
    pub fn esel(&mut self) -> EselW<FcfgaSpec> {
        EselW::new(self, 12)
    }
    #[doc = "Bit 14 - Event Gating"]
    #[inline(always)]
    #[must_use]
    pub fn egt(&mut self) -> EgtW<FcfgaSpec> {
        EgtW::new(self, 14)
    }
}
#[doc = "Filter Configuration Register, Auxiliary Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfga::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfga::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcfgaSpec;
impl crate::RegisterSpec for FcfgaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcfga::R`](R) reader structure"]
impl crate::Readable for FcfgaSpec {}
#[doc = "`write(|w| ..)` method takes [`fcfga::W`](W) writer structure"]
impl crate::Writable for FcfgaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCFGA to value 0"]
impl crate::Resettable for FcfgaSpec {
    const RESET_VALUE: u32 = 0;
}
