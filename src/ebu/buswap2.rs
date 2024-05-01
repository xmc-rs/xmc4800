#[doc = "Register `BUSWAP2` reader"]
pub type R = crate::R<Buswap2Spec>;
#[doc = "Register `BUSWAP2` writer"]
pub type W = crate::W<Buswap2Spec>;
#[doc = "Recovery Cycles between Different Regions\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wrdtacs {
    #[doc = "0: No Recovery Phase clock cycles available."]
    Value1 = 0,
    #[doc = "1: 1 clock cycle selected."]
    Value2 = 1,
    #[doc = "14: 14 clock cycles selected."]
    Value3 = 14,
    #[doc = "15: 15 clock cycles selected."]
    Value4 = 15,
}
impl From<Wrdtacs> for u8 {
    #[inline(always)]
    fn from(variant: Wrdtacs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wrdtacs {
    type Ux = u8;
}
impl crate::IsEnum for Wrdtacs {}
#[doc = "Field `WRDTACS` reader - Recovery Cycles between Different Regions"]
pub type WrdtacsR = crate::FieldReader<Wrdtacs>;
impl WrdtacsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wrdtacs> {
        match self.bits {
            0 => Some(Wrdtacs::Value1),
            1 => Some(Wrdtacs::Value2),
            14 => Some(Wrdtacs::Value3),
            15 => Some(Wrdtacs::Value4),
            _ => None,
        }
    }
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wrdtacs::Value1
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wrdtacs::Value2
    }
    #[doc = "14 clock cycles selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Wrdtacs::Value3
    }
    #[doc = "15 clock cycles selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Wrdtacs::Value4
    }
}
#[doc = "Field `WRDTACS` writer - Recovery Cycles between Different Regions"]
pub type WrdtacsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Wrdtacs>;
impl<'a, REG> WrdtacsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wrdtacs::Value1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wrdtacs::Value2)
    }
    #[doc = "14 clock cycles selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Wrdtacs::Value3)
    }
    #[doc = "15 clock cycles selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Wrdtacs::Value4)
    }
}
#[doc = "Recovery Cycles after Write Accesses\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wrrecovc {
    #[doc = "0: No Recovery Phase clock cycles available."]
    Value1 = 0,
    #[doc = "1: 1 clock cycle selected."]
    Value2 = 1,
    #[doc = "6: 6 clock cycles selected."]
    Value3 = 6,
    #[doc = "7: 7 clock cycles selected."]
    Value4 = 7,
}
impl From<Wrrecovc> for u8 {
    #[inline(always)]
    fn from(variant: Wrrecovc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wrrecovc {
    type Ux = u8;
}
impl crate::IsEnum for Wrrecovc {}
#[doc = "Field `WRRECOVC` reader - Recovery Cycles after Write Accesses"]
pub type WrrecovcR = crate::FieldReader<Wrrecovc>;
impl WrrecovcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wrrecovc> {
        match self.bits {
            0 => Some(Wrrecovc::Value1),
            1 => Some(Wrrecovc::Value2),
            6 => Some(Wrrecovc::Value3),
            7 => Some(Wrrecovc::Value4),
            _ => None,
        }
    }
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wrrecovc::Value1
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wrrecovc::Value2
    }
    #[doc = "6 clock cycles selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Wrrecovc::Value3
    }
    #[doc = "7 clock cycles selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Wrrecovc::Value4
    }
}
#[doc = "Field `WRRECOVC` writer - Recovery Cycles after Write Accesses"]
pub type WrrecovcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Wrrecovc>;
impl<'a, REG> WrrecovcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wrrecovc::Value1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wrrecovc::Value2)
    }
    #[doc = "6 clock cycles selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Wrrecovc::Value3)
    }
    #[doc = "7 clock cycles selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Wrrecovc::Value4)
    }
}
#[doc = "Programmed Wait States for write accesses\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Waitwrc {
    #[doc = "0: 1 wait state."]
    Value1 = 0,
    #[doc = "1: 1 wait states."]
    Value2 = 1,
    #[doc = "2: 2 wait state."]
    Value3 = 2,
    #[doc = "30: 30 wait states."]
    Value4 = 30,
    #[doc = "31: 31 wait states."]
    Value5 = 31,
}
impl From<Waitwrc> for u8 {
    #[inline(always)]
    fn from(variant: Waitwrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Waitwrc {
    type Ux = u8;
}
impl crate::IsEnum for Waitwrc {}
#[doc = "Field `WAITWRC` reader - Programmed Wait States for write accesses"]
pub type WaitwrcR = crate::FieldReader<Waitwrc>;
impl WaitwrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Waitwrc> {
        match self.bits {
            0 => Some(Waitwrc::Value1),
            1 => Some(Waitwrc::Value2),
            2 => Some(Waitwrc::Value3),
            30 => Some(Waitwrc::Value4),
            31 => Some(Waitwrc::Value5),
            _ => None,
        }
    }
    #[doc = "1 wait state."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Waitwrc::Value1
    }
    #[doc = "1 wait states."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Waitwrc::Value2
    }
    #[doc = "2 wait state."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Waitwrc::Value3
    }
    #[doc = "30 wait states."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Waitwrc::Value4
    }
    #[doc = "31 wait states."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Waitwrc::Value5
    }
}
#[doc = "Field `WAITWRC` writer - Programmed Wait States for write accesses"]
pub type WaitwrcW<'a, REG> = crate::FieldWriter<'a, REG, 5, Waitwrc>;
impl<'a, REG> WaitwrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 wait state."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Waitwrc::Value1)
    }
    #[doc = "1 wait states."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Waitwrc::Value2)
    }
    #[doc = "2 wait state."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Waitwrc::Value3)
    }
    #[doc = "30 wait states."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Waitwrc::Value4)
    }
    #[doc = "31 wait states."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Waitwrc::Value5)
    }
}
#[doc = "Data Hold Cycles for Write Accesses\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datac {
    #[doc = "0: No Recovery Phase clock cycles available."]
    Value1 = 0,
    #[doc = "1: 1 clock cycle selected."]
    Value2 = 1,
    #[doc = "14: 14 clock cycles selected."]
    Value3 = 14,
    #[doc = "15: 15 clock cycles selected."]
    Value4 = 15,
}
impl From<Datac> for u8 {
    #[inline(always)]
    fn from(variant: Datac) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datac {
    type Ux = u8;
}
impl crate::IsEnum for Datac {}
#[doc = "Field `DATAC` reader - Data Hold Cycles for Write Accesses"]
pub type DatacR = crate::FieldReader<Datac>;
impl DatacR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Datac> {
        match self.bits {
            0 => Some(Datac::Value1),
            1 => Some(Datac::Value2),
            14 => Some(Datac::Value3),
            15 => Some(Datac::Value4),
            _ => None,
        }
    }
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Datac::Value1
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Datac::Value2
    }
    #[doc = "14 clock cycles selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Datac::Value3
    }
    #[doc = "15 clock cycles selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Datac::Value4
    }
}
#[doc = "Field `DATAC` writer - Data Hold Cycles for Write Accesses"]
pub type DatacW<'a, REG> = crate::FieldWriter<'a, REG, 4, Datac>;
impl<'a, REG> DatacW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Datac::Value1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Datac::Value2)
    }
    #[doc = "14 clock cycles selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Datac::Value3)
    }
    #[doc = "15 clock cycles selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Datac::Value4)
    }
}
#[doc = "Frequency of external clock at pin BFCLKO\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extclock {
    #[doc = "0: Equal to INT_CLK frequency."]
    Value1 = 0,
    #[doc = "1: 1/2 of INT_CLK frequency."]
    Value2 = 1,
    #[doc = "2: 1/3 of INT_CLK frequency."]
    Value3 = 2,
    #[doc = "3: 1/4 of INT_CLK frequency (default after reset)."]
    Value4 = 3,
}
impl From<Extclock> for u8 {
    #[inline(always)]
    fn from(variant: Extclock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extclock {
    type Ux = u8;
}
impl crate::IsEnum for Extclock {}
#[doc = "Field `EXTCLOCK` reader - Frequency of external clock at pin BFCLKO"]
pub type ExtclockR = crate::FieldReader<Extclock>;
impl ExtclockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extclock {
        match self.bits {
            0 => Extclock::Value1,
            1 => Extclock::Value2,
            2 => Extclock::Value3,
            3 => Extclock::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Equal to INT_CLK frequency."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Extclock::Value1
    }
    #[doc = "1/2 of INT_CLK frequency."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Extclock::Value2
    }
    #[doc = "1/3 of INT_CLK frequency."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Extclock::Value3
    }
    #[doc = "1/4 of INT_CLK frequency (default after reset)."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Extclock::Value4
    }
}
#[doc = "Field `EXTCLOCK` writer - Frequency of external clock at pin BFCLKO"]
pub type ExtclockW<'a, REG> = crate::FieldWriter<'a, REG, 2, Extclock, crate::Safe>;
impl<'a, REG> ExtclockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Equal to INT_CLK frequency."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Extclock::Value1)
    }
    #[doc = "1/2 of INT_CLK frequency."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Extclock::Value2)
    }
    #[doc = "1/3 of INT_CLK frequency."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Extclock::Value3)
    }
    #[doc = "1/4 of INT_CLK frequency (default after reset)."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Extclock::Value4)
    }
}
#[doc = "Extended data\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extdata {
    #[doc = "0: external memory outputs data every BFCLK cycle"]
    Value1 = 0,
    #[doc = "1: external memory outputs data every two BFCLK cycles"]
    Value2 = 1,
    #[doc = "2: external memory outputs data every four BFCLK cycles"]
    Value3 = 2,
    #[doc = "3: external memory outputs data every eight BFCLK cycles"]
    Value4 = 3,
}
impl From<Extdata> for u8 {
    #[inline(always)]
    fn from(variant: Extdata) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extdata {
    type Ux = u8;
}
impl crate::IsEnum for Extdata {}
#[doc = "Field `EXTDATA` reader - Extended data"]
pub type ExtdataR = crate::FieldReader<Extdata>;
impl ExtdataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extdata {
        match self.bits {
            0 => Extdata::Value1,
            1 => Extdata::Value2,
            2 => Extdata::Value3,
            3 => Extdata::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "external memory outputs data every BFCLK cycle"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Extdata::Value1
    }
    #[doc = "external memory outputs data every two BFCLK cycles"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Extdata::Value2
    }
    #[doc = "external memory outputs data every four BFCLK cycles"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Extdata::Value3
    }
    #[doc = "external memory outputs data every eight BFCLK cycles"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Extdata::Value4
    }
}
#[doc = "Field `EXTDATA` writer - Extended data"]
pub type ExtdataW<'a, REG> = crate::FieldWriter<'a, REG, 2, Extdata, crate::Safe>;
impl<'a, REG> ExtdataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "external memory outputs data every BFCLK cycle"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Extdata::Value1)
    }
    #[doc = "external memory outputs data every two BFCLK cycles"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Extdata::Value2)
    }
    #[doc = "external memory outputs data every four BFCLK cycles"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Extdata::Value3)
    }
    #[doc = "external memory outputs data every eight BFCLK cycles"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Extdata::Value4)
    }
}
#[doc = "Command Delay Cycles\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmddelay {
    #[doc = "0: 0 clock cycle selected."]
    Value1 = 0,
    #[doc = "1: 1 clock cycle selected."]
    Value2 = 1,
    #[doc = "14: 14 clock cycles selected."]
    Value3 = 14,
    #[doc = "15: 15 clock cycles selected."]
    Value4 = 15,
}
impl From<Cmddelay> for u8 {
    #[inline(always)]
    fn from(variant: Cmddelay) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmddelay {
    type Ux = u8;
}
impl crate::IsEnum for Cmddelay {}
#[doc = "Field `CMDDELAY` reader - Command Delay Cycles"]
pub type CmddelayR = crate::FieldReader<Cmddelay>;
impl CmddelayR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmddelay> {
        match self.bits {
            0 => Some(Cmddelay::Value1),
            1 => Some(Cmddelay::Value2),
            14 => Some(Cmddelay::Value3),
            15 => Some(Cmddelay::Value4),
            _ => None,
        }
    }
    #[doc = "0 clock cycle selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cmddelay::Value1
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cmddelay::Value2
    }
    #[doc = "14 clock cycles selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cmddelay::Value3
    }
    #[doc = "15 clock cycles selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cmddelay::Value4
    }
}
#[doc = "Field `CMDDELAY` writer - Command Delay Cycles"]
pub type CmddelayW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cmddelay>;
impl<'a, REG> CmddelayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 clock cycle selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmddelay::Value1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cmddelay::Value2)
    }
    #[doc = "14 clock cycles selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cmddelay::Value3)
    }
    #[doc = "15 clock cycles selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cmddelay::Value4)
    }
}
#[doc = "Address Hold Cycles\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aholdc {
    #[doc = "0: 0 clock cycle selected"]
    Value1 = 0,
    #[doc = "1: 1 clock cycle selected"]
    Value2 = 1,
    #[doc = "14: 14 clock cycles selected"]
    Value3 = 14,
    #[doc = "15: 15 clock cycles selected"]
    Value4 = 15,
}
impl From<Aholdc> for u8 {
    #[inline(always)]
    fn from(variant: Aholdc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aholdc {
    type Ux = u8;
}
impl crate::IsEnum for Aholdc {}
#[doc = "Field `AHOLDC` reader - Address Hold Cycles"]
pub type AholdcR = crate::FieldReader<Aholdc>;
impl AholdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Aholdc> {
        match self.bits {
            0 => Some(Aholdc::Value1),
            1 => Some(Aholdc::Value2),
            14 => Some(Aholdc::Value3),
            15 => Some(Aholdc::Value4),
            _ => None,
        }
    }
    #[doc = "0 clock cycle selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Aholdc::Value1
    }
    #[doc = "1 clock cycle selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Aholdc::Value2
    }
    #[doc = "14 clock cycles selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Aholdc::Value3
    }
    #[doc = "15 clock cycles selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Aholdc::Value4
    }
}
#[doc = "Field `AHOLDC` writer - Address Hold Cycles"]
pub type AholdcW<'a, REG> = crate::FieldWriter<'a, REG, 4, Aholdc>;
impl<'a, REG> AholdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 clock cycle selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Aholdc::Value1)
    }
    #[doc = "1 clock cycle selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Aholdc::Value2)
    }
    #[doc = "14 clock cycles selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Aholdc::Value3)
    }
    #[doc = "15 clock cycles selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Aholdc::Value4)
    }
}
#[doc = "Address Cycles\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Addrc {
    #[doc = "0: 1 clock cycle selected"]
    Value1 = 0,
    #[doc = "1: 1 clock cycle selected"]
    Value2 = 1,
    #[doc = "14: 14 clock cycles selected"]
    Value3 = 14,
    #[doc = "15: 15 clock cycles selected"]
    Value4 = 15,
}
impl From<Addrc> for u8 {
    #[inline(always)]
    fn from(variant: Addrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Addrc {
    type Ux = u8;
}
impl crate::IsEnum for Addrc {}
#[doc = "Field `ADDRC` reader - Address Cycles"]
pub type AddrcR = crate::FieldReader<Addrc>;
impl AddrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Addrc> {
        match self.bits {
            0 => Some(Addrc::Value1),
            1 => Some(Addrc::Value2),
            14 => Some(Addrc::Value3),
            15 => Some(Addrc::Value4),
            _ => None,
        }
    }
    #[doc = "1 clock cycle selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Addrc::Value1
    }
    #[doc = "1 clock cycle selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Addrc::Value2
    }
    #[doc = "14 clock cycles selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Addrc::Value3
    }
    #[doc = "15 clock cycles selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Addrc::Value4
    }
}
#[doc = "Field `ADDRC` writer - Address Cycles"]
pub type AddrcW<'a, REG> = crate::FieldWriter<'a, REG, 4, Addrc>;
impl<'a, REG> AddrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 clock cycle selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Addrc::Value1)
    }
    #[doc = "1 clock cycle selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Addrc::Value2)
    }
    #[doc = "14 clock cycles selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Addrc::Value3)
    }
    #[doc = "15 clock cycles selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Addrc::Value4)
    }
}
impl R {
    #[doc = "Bits 0:3 - Recovery Cycles between Different Regions"]
    #[inline(always)]
    pub fn wrdtacs(&self) -> WrdtacsR {
        WrdtacsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Recovery Cycles after Write Accesses"]
    #[inline(always)]
    pub fn wrrecovc(&self) -> WrrecovcR {
        WrrecovcR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:11 - Programmed Wait States for write accesses"]
    #[inline(always)]
    pub fn waitwrc(&self) -> WaitwrcR {
        WaitwrcR::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:15 - Data Hold Cycles for Write Accesses"]
    #[inline(always)]
    pub fn datac(&self) -> DatacR {
        DatacR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Frequency of external clock at pin BFCLKO"]
    #[inline(always)]
    pub fn extclock(&self) -> ExtclockR {
        ExtclockR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Extended data"]
    #[inline(always)]
    pub fn extdata(&self) -> ExtdataR {
        ExtdataR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:23 - Command Delay Cycles"]
    #[inline(always)]
    pub fn cmddelay(&self) -> CmddelayR {
        CmddelayR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Address Hold Cycles"]
    #[inline(always)]
    pub fn aholdc(&self) -> AholdcR {
        AholdcR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Address Cycles"]
    #[inline(always)]
    pub fn addrc(&self) -> AddrcR {
        AddrcR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Recovery Cycles between Different Regions"]
    #[inline(always)]
    #[must_use]
    pub fn wrdtacs(&mut self) -> WrdtacsW<Buswap2Spec> {
        WrdtacsW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Recovery Cycles after Write Accesses"]
    #[inline(always)]
    #[must_use]
    pub fn wrrecovc(&mut self) -> WrrecovcW<Buswap2Spec> {
        WrrecovcW::new(self, 4)
    }
    #[doc = "Bits 7:11 - Programmed Wait States for write accesses"]
    #[inline(always)]
    #[must_use]
    pub fn waitwrc(&mut self) -> WaitwrcW<Buswap2Spec> {
        WaitwrcW::new(self, 7)
    }
    #[doc = "Bits 12:15 - Data Hold Cycles for Write Accesses"]
    #[inline(always)]
    #[must_use]
    pub fn datac(&mut self) -> DatacW<Buswap2Spec> {
        DatacW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Frequency of external clock at pin BFCLKO"]
    #[inline(always)]
    #[must_use]
    pub fn extclock(&mut self) -> ExtclockW<Buswap2Spec> {
        ExtclockW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Extended data"]
    #[inline(always)]
    #[must_use]
    pub fn extdata(&mut self) -> ExtdataW<Buswap2Spec> {
        ExtdataW::new(self, 18)
    }
    #[doc = "Bits 20:23 - Command Delay Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn cmddelay(&mut self) -> CmddelayW<Buswap2Spec> {
        CmddelayW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Address Hold Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn aholdc(&mut self) -> AholdcW<Buswap2Spec> {
        AholdcW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Address Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn addrc(&mut self) -> AddrcW<Buswap2Spec> {
        AddrcW::new(self, 28)
    }
}
#[doc = "EBU Bus Write Access Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buswap2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buswap2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buswap2Spec;
impl crate::RegisterSpec for Buswap2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buswap2::R`](R) reader structure"]
impl crate::Readable for Buswap2Spec {}
#[doc = "`write(|w| ..)` method takes [`buswap2::W`](W) writer structure"]
impl crate::Writable for Buswap2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUSWAP2 to value 0xffff_ffff"]
impl crate::Resettable for Buswap2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
