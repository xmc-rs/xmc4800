#[doc = "Register `BUSRAP0` reader"]
pub type R = crate::R<Busrap0Spec>;
#[doc = "Register `BUSRAP0` writer"]
pub type W = crate::W<Busrap0Spec>;
#[doc = "Recovery Cycles between Different Regions\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rddtacs {
    #[doc = "0: No Recovery Phase clock cycles available."]
    Value1 = 0,
    #[doc = "1: 1 clock cycle selected."]
    Value2 = 1,
    #[doc = "14: 14 clock cycles selected."]
    Value3 = 14,
    #[doc = "15: 15 clock cycles selected."]
    Value4 = 15,
}
impl From<Rddtacs> for u8 {
    #[inline(always)]
    fn from(variant: Rddtacs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rddtacs {
    type Ux = u8;
}
#[doc = "Field `RDDTACS` reader - Recovery Cycles between Different Regions"]
pub type RddtacsR = crate::FieldReader<Rddtacs>;
impl RddtacsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rddtacs> {
        match self.bits {
            0 => Some(Rddtacs::Value1),
            1 => Some(Rddtacs::Value2),
            14 => Some(Rddtacs::Value3),
            15 => Some(Rddtacs::Value4),
            _ => None,
        }
    }
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rddtacs::Value1
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rddtacs::Value2
    }
    #[doc = "14 clock cycles selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rddtacs::Value3
    }
    #[doc = "15 clock cycles selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rddtacs::Value4
    }
}
#[doc = "Field `RDDTACS` writer - Recovery Cycles between Different Regions"]
pub type RddtacsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rddtacs>;
impl<'a, REG> RddtacsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rddtacs::Value1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rddtacs::Value2)
    }
    #[doc = "14 clock cycles selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rddtacs::Value3)
    }
    #[doc = "15 clock cycles selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rddtacs::Value4)
    }
}
#[doc = "Recovery Cycles after Read Accesses\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rdrecovc {
    #[doc = "0: No Recovery Phase clock cycles available."]
    Value1 = 0,
    #[doc = "1: 1 clock cycle selected."]
    Value2 = 1,
    #[doc = "6: 6 clock cycles selected."]
    Value3 = 6,
    #[doc = "7: 7 clock cycles selected."]
    Value4 = 7,
}
impl From<Rdrecovc> for u8 {
    #[inline(always)]
    fn from(variant: Rdrecovc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rdrecovc {
    type Ux = u8;
}
#[doc = "Field `RDRECOVC` reader - Recovery Cycles after Read Accesses"]
pub type RdrecovcR = crate::FieldReader<Rdrecovc>;
impl RdrecovcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rdrecovc> {
        match self.bits {
            0 => Some(Rdrecovc::Value1),
            1 => Some(Rdrecovc::Value2),
            6 => Some(Rdrecovc::Value3),
            7 => Some(Rdrecovc::Value4),
            _ => None,
        }
    }
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rdrecovc::Value1
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rdrecovc::Value2
    }
    #[doc = "6 clock cycles selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rdrecovc::Value3
    }
    #[doc = "7 clock cycles selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rdrecovc::Value4
    }
}
#[doc = "Field `RDRECOVC` writer - Recovery Cycles after Read Accesses"]
pub type RdrecovcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rdrecovc>;
impl<'a, REG> RdrecovcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdrecovc::Value1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rdrecovc::Value2)
    }
    #[doc = "6 clock cycles selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rdrecovc::Value3)
    }
    #[doc = "7 clock cycles selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rdrecovc::Value4)
    }
}
#[doc = "Programmed Wait States for read accesses\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Waitrdc {
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
impl From<Waitrdc> for u8 {
    #[inline(always)]
    fn from(variant: Waitrdc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Waitrdc {
    type Ux = u8;
}
#[doc = "Field `WAITRDC` reader - Programmed Wait States for read accesses"]
pub type WaitrdcR = crate::FieldReader<Waitrdc>;
impl WaitrdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Waitrdc> {
        match self.bits {
            0 => Some(Waitrdc::Value1),
            1 => Some(Waitrdc::Value2),
            2 => Some(Waitrdc::Value3),
            30 => Some(Waitrdc::Value4),
            31 => Some(Waitrdc::Value5),
            _ => None,
        }
    }
    #[doc = "1 wait state."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Waitrdc::Value1
    }
    #[doc = "1 wait states."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Waitrdc::Value2
    }
    #[doc = "2 wait state."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Waitrdc::Value3
    }
    #[doc = "30 wait states."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Waitrdc::Value4
    }
    #[doc = "31 wait states."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Waitrdc::Value5
    }
}
#[doc = "Field `WAITRDC` writer - Programmed Wait States for read accesses"]
pub type WaitrdcW<'a, REG> = crate::FieldWriter<'a, REG, 5, Waitrdc>;
impl<'a, REG> WaitrdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 wait state."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Waitrdc::Value1)
    }
    #[doc = "1 wait states."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Waitrdc::Value2)
    }
    #[doc = "2 wait state."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Waitrdc::Value3)
    }
    #[doc = "30 wait states."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Waitrdc::Value4)
    }
    #[doc = "31 wait states."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Waitrdc::Value5)
    }
}
#[doc = "Field `DATAC` reader - Data Hold Cycles for Read Accesses"]
pub type DatacR = crate::FieldReader;
#[doc = "Field `DATAC` writer - Data Hold Cycles for Read Accesses"]
pub type DatacW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
pub type ExtclockW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Extclock>;
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
pub type ExtdataW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Extdata>;
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
    pub fn rddtacs(&self) -> RddtacsR {
        RddtacsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Recovery Cycles after Read Accesses"]
    #[inline(always)]
    pub fn rdrecovc(&self) -> RdrecovcR {
        RdrecovcR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:11 - Programmed Wait States for read accesses"]
    #[inline(always)]
    pub fn waitrdc(&self) -> WaitrdcR {
        WaitrdcR::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:15 - Data Hold Cycles for Read Accesses"]
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
    pub fn rddtacs(&mut self) -> RddtacsW<Busrap0Spec> {
        RddtacsW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Recovery Cycles after Read Accesses"]
    #[inline(always)]
    #[must_use]
    pub fn rdrecovc(&mut self) -> RdrecovcW<Busrap0Spec> {
        RdrecovcW::new(self, 4)
    }
    #[doc = "Bits 7:11 - Programmed Wait States for read accesses"]
    #[inline(always)]
    #[must_use]
    pub fn waitrdc(&mut self) -> WaitrdcW<Busrap0Spec> {
        WaitrdcW::new(self, 7)
    }
    #[doc = "Bits 12:15 - Data Hold Cycles for Read Accesses"]
    #[inline(always)]
    #[must_use]
    pub fn datac(&mut self) -> DatacW<Busrap0Spec> {
        DatacW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Frequency of external clock at pin BFCLKO"]
    #[inline(always)]
    #[must_use]
    pub fn extclock(&mut self) -> ExtclockW<Busrap0Spec> {
        ExtclockW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Extended data"]
    #[inline(always)]
    #[must_use]
    pub fn extdata(&mut self) -> ExtdataW<Busrap0Spec> {
        ExtdataW::new(self, 18)
    }
    #[doc = "Bits 20:23 - Command Delay Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn cmddelay(&mut self) -> CmddelayW<Busrap0Spec> {
        CmddelayW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Address Hold Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn aholdc(&mut self) -> AholdcW<Busrap0Spec> {
        AholdcW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Address Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn addrc(&mut self) -> AddrcW<Busrap0Spec> {
        AddrcW::new(self, 28)
    }
}
#[doc = "EBU Bus Read Access Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busrap0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busrap0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Busrap0Spec;
impl crate::RegisterSpec for Busrap0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`busrap0::R`](R) reader structure"]
impl crate::Readable for Busrap0Spec {}
#[doc = "`write(|w| ..)` method takes [`busrap0::W`](W) writer structure"]
impl crate::Writable for Busrap0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUSRAP0 to value 0xffff_ffff"]
impl crate::Resettable for Busrap0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
