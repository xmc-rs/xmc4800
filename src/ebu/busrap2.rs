#[doc = "Register `BUSRAP2` reader"]
pub type R = crate::R<BUSRAP2_SPEC>;
#[doc = "Register `BUSRAP2` writer"]
pub type W = crate::W<BUSRAP2_SPEC>;
#[doc = "Recovery Cycles between Different Regions\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDDTACS_A {
    #[doc = "0: No Recovery Phase clock cycles available."]
    VALUE1 = 0,
    #[doc = "1: 1 clock cycle selected."]
    VALUE2 = 1,
    #[doc = "14: 14 clock cycles selected."]
    VALUE3 = 14,
    #[doc = "15: 15 clock cycles selected."]
    VALUE4 = 15,
}
impl From<RDDTACS_A> for u8 {
    #[inline(always)]
    fn from(variant: RDDTACS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RDDTACS_A {
    type Ux = u8;
}
impl crate::IsEnum for RDDTACS_A {}
#[doc = "Field `RDDTACS` reader - Recovery Cycles between Different Regions"]
pub type RDDTACS_R = crate::FieldReader<RDDTACS_A>;
impl RDDTACS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RDDTACS_A> {
        match self.bits {
            0 => Some(RDDTACS_A::VALUE1),
            1 => Some(RDDTACS_A::VALUE2),
            14 => Some(RDDTACS_A::VALUE3),
            15 => Some(RDDTACS_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RDDTACS_A::VALUE1
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RDDTACS_A::VALUE2
    }
    #[doc = "14 clock cycles selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RDDTACS_A::VALUE3
    }
    #[doc = "15 clock cycles selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RDDTACS_A::VALUE4
    }
}
#[doc = "Field `RDDTACS` writer - Recovery Cycles between Different Regions"]
pub type RDDTACS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, RDDTACS_A>;
impl<'a, REG> RDDTACS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RDDTACS_A::VALUE1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RDDTACS_A::VALUE2)
    }
    #[doc = "14 clock cycles selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(RDDTACS_A::VALUE3)
    }
    #[doc = "15 clock cycles selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(RDDTACS_A::VALUE4)
    }
}
#[doc = "Recovery Cycles after Read Accesses\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDRECOVC_A {
    #[doc = "0: No Recovery Phase clock cycles available."]
    VALUE1 = 0,
    #[doc = "1: 1 clock cycle selected."]
    VALUE2 = 1,
    #[doc = "6: 6 clock cycles selected."]
    VALUE3 = 6,
    #[doc = "7: 7 clock cycles selected."]
    VALUE4 = 7,
}
impl From<RDRECOVC_A> for u8 {
    #[inline(always)]
    fn from(variant: RDRECOVC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RDRECOVC_A {
    type Ux = u8;
}
impl crate::IsEnum for RDRECOVC_A {}
#[doc = "Field `RDRECOVC` reader - Recovery Cycles after Read Accesses"]
pub type RDRECOVC_R = crate::FieldReader<RDRECOVC_A>;
impl RDRECOVC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RDRECOVC_A> {
        match self.bits {
            0 => Some(RDRECOVC_A::VALUE1),
            1 => Some(RDRECOVC_A::VALUE2),
            6 => Some(RDRECOVC_A::VALUE3),
            7 => Some(RDRECOVC_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RDRECOVC_A::VALUE1
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RDRECOVC_A::VALUE2
    }
    #[doc = "6 clock cycles selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RDRECOVC_A::VALUE3
    }
    #[doc = "7 clock cycles selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RDRECOVC_A::VALUE4
    }
}
#[doc = "Field `RDRECOVC` writer - Recovery Cycles after Read Accesses"]
pub type RDRECOVC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, RDRECOVC_A>;
impl<'a, REG> RDRECOVC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RDRECOVC_A::VALUE1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RDRECOVC_A::VALUE2)
    }
    #[doc = "6 clock cycles selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(RDRECOVC_A::VALUE3)
    }
    #[doc = "7 clock cycles selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(RDRECOVC_A::VALUE4)
    }
}
#[doc = "Programmed Wait States for read accesses\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAITRDC_A {
    #[doc = "0: 1 wait state."]
    VALUE1 = 0,
    #[doc = "1: 1 wait states."]
    VALUE2 = 1,
    #[doc = "2: 2 wait state."]
    VALUE3 = 2,
    #[doc = "30: 30 wait states."]
    VALUE4 = 30,
    #[doc = "31: 31 wait states."]
    VALUE5 = 31,
}
impl From<WAITRDC_A> for u8 {
    #[inline(always)]
    fn from(variant: WAITRDC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WAITRDC_A {
    type Ux = u8;
}
impl crate::IsEnum for WAITRDC_A {}
#[doc = "Field `WAITRDC` reader - Programmed Wait States for read accesses"]
pub type WAITRDC_R = crate::FieldReader<WAITRDC_A>;
impl WAITRDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WAITRDC_A> {
        match self.bits {
            0 => Some(WAITRDC_A::VALUE1),
            1 => Some(WAITRDC_A::VALUE2),
            2 => Some(WAITRDC_A::VALUE3),
            30 => Some(WAITRDC_A::VALUE4),
            31 => Some(WAITRDC_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "1 wait state."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAITRDC_A::VALUE1
    }
    #[doc = "1 wait states."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAITRDC_A::VALUE2
    }
    #[doc = "2 wait state."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == WAITRDC_A::VALUE3
    }
    #[doc = "30 wait states."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == WAITRDC_A::VALUE4
    }
    #[doc = "31 wait states."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == WAITRDC_A::VALUE5
    }
}
#[doc = "Field `WAITRDC` writer - Programmed Wait States for read accesses"]
pub type WAITRDC_W<'a, REG> = crate::FieldWriter<'a, REG, 5, WAITRDC_A>;
impl<'a, REG> WAITRDC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 wait state."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WAITRDC_A::VALUE1)
    }
    #[doc = "1 wait states."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WAITRDC_A::VALUE2)
    }
    #[doc = "2 wait state."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(WAITRDC_A::VALUE3)
    }
    #[doc = "30 wait states."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(WAITRDC_A::VALUE4)
    }
    #[doc = "31 wait states."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(WAITRDC_A::VALUE5)
    }
}
#[doc = "Field `DATAC` reader - Data Hold Cycles for Read Accesses"]
pub type DATAC_R = crate::FieldReader;
#[doc = "Field `DATAC` writer - Data Hold Cycles for Read Accesses"]
pub type DATAC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Frequency of external clock at pin BFCLKO\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTCLOCK_A {
    #[doc = "0: Equal to INT_CLK frequency."]
    VALUE1 = 0,
    #[doc = "1: 1/2 of INT_CLK frequency."]
    VALUE2 = 1,
    #[doc = "2: 1/3 of INT_CLK frequency."]
    VALUE3 = 2,
    #[doc = "3: 1/4 of INT_CLK frequency (default after reset)."]
    VALUE4 = 3,
}
impl From<EXTCLOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTCLOCK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTCLOCK_A {
    type Ux = u8;
}
impl crate::IsEnum for EXTCLOCK_A {}
#[doc = "Field `EXTCLOCK` reader - Frequency of external clock at pin BFCLKO"]
pub type EXTCLOCK_R = crate::FieldReader<EXTCLOCK_A>;
impl EXTCLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTCLOCK_A {
        match self.bits {
            0 => EXTCLOCK_A::VALUE1,
            1 => EXTCLOCK_A::VALUE2,
            2 => EXTCLOCK_A::VALUE3,
            3 => EXTCLOCK_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Equal to INT_CLK frequency."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EXTCLOCK_A::VALUE1
    }
    #[doc = "1/2 of INT_CLK frequency."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EXTCLOCK_A::VALUE2
    }
    #[doc = "1/3 of INT_CLK frequency."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EXTCLOCK_A::VALUE3
    }
    #[doc = "1/4 of INT_CLK frequency (default after reset)."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EXTCLOCK_A::VALUE4
    }
}
#[doc = "Field `EXTCLOCK` writer - Frequency of external clock at pin BFCLKO"]
pub type EXTCLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTCLOCK_A, crate::Safe>;
impl<'a, REG> EXTCLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Equal to INT_CLK frequency."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTCLOCK_A::VALUE1)
    }
    #[doc = "1/2 of INT_CLK frequency."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTCLOCK_A::VALUE2)
    }
    #[doc = "1/3 of INT_CLK frequency."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTCLOCK_A::VALUE3)
    }
    #[doc = "1/4 of INT_CLK frequency (default after reset)."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTCLOCK_A::VALUE4)
    }
}
#[doc = "Extended data\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTDATA_A {
    #[doc = "0: external memory outputs data every BFCLK cycle"]
    VALUE1 = 0,
    #[doc = "1: external memory outputs data every two BFCLK cycles"]
    VALUE2 = 1,
    #[doc = "2: external memory outputs data every four BFCLK cycles"]
    VALUE3 = 2,
    #[doc = "3: external memory outputs data every eight BFCLK cycles"]
    VALUE4 = 3,
}
impl From<EXTDATA_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTDATA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTDATA_A {
    type Ux = u8;
}
impl crate::IsEnum for EXTDATA_A {}
#[doc = "Field `EXTDATA` reader - Extended data"]
pub type EXTDATA_R = crate::FieldReader<EXTDATA_A>;
impl EXTDATA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTDATA_A {
        match self.bits {
            0 => EXTDATA_A::VALUE1,
            1 => EXTDATA_A::VALUE2,
            2 => EXTDATA_A::VALUE3,
            3 => EXTDATA_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "external memory outputs data every BFCLK cycle"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EXTDATA_A::VALUE1
    }
    #[doc = "external memory outputs data every two BFCLK cycles"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EXTDATA_A::VALUE2
    }
    #[doc = "external memory outputs data every four BFCLK cycles"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EXTDATA_A::VALUE3
    }
    #[doc = "external memory outputs data every eight BFCLK cycles"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EXTDATA_A::VALUE4
    }
}
#[doc = "Field `EXTDATA` writer - Extended data"]
pub type EXTDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTDATA_A, crate::Safe>;
impl<'a, REG> EXTDATA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "external memory outputs data every BFCLK cycle"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTDATA_A::VALUE1)
    }
    #[doc = "external memory outputs data every two BFCLK cycles"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTDATA_A::VALUE2)
    }
    #[doc = "external memory outputs data every four BFCLK cycles"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTDATA_A::VALUE3)
    }
    #[doc = "external memory outputs data every eight BFCLK cycles"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTDATA_A::VALUE4)
    }
}
#[doc = "Command Delay Cycles\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDDELAY_A {
    #[doc = "0: 0 clock cycle selected."]
    VALUE1 = 0,
    #[doc = "1: 1 clock cycle selected."]
    VALUE2 = 1,
    #[doc = "14: 14 clock cycles selected."]
    VALUE3 = 14,
    #[doc = "15: 15 clock cycles selected."]
    VALUE4 = 15,
}
impl From<CMDDELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDDELAY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMDDELAY_A {
    type Ux = u8;
}
impl crate::IsEnum for CMDDELAY_A {}
#[doc = "Field `CMDDELAY` reader - Command Delay Cycles"]
pub type CMDDELAY_R = crate::FieldReader<CMDDELAY_A>;
impl CMDDELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMDDELAY_A> {
        match self.bits {
            0 => Some(CMDDELAY_A::VALUE1),
            1 => Some(CMDDELAY_A::VALUE2),
            14 => Some(CMDDELAY_A::VALUE3),
            15 => Some(CMDDELAY_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "0 clock cycle selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMDDELAY_A::VALUE1
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMDDELAY_A::VALUE2
    }
    #[doc = "14 clock cycles selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CMDDELAY_A::VALUE3
    }
    #[doc = "15 clock cycles selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CMDDELAY_A::VALUE4
    }
}
#[doc = "Field `CMDDELAY` writer - Command Delay Cycles"]
pub type CMDDELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CMDDELAY_A>;
impl<'a, REG> CMDDELAY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 clock cycle selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CMDDELAY_A::VALUE1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CMDDELAY_A::VALUE2)
    }
    #[doc = "14 clock cycles selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CMDDELAY_A::VALUE3)
    }
    #[doc = "15 clock cycles selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CMDDELAY_A::VALUE4)
    }
}
#[doc = "Address Hold Cycles\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AHOLDC_A {
    #[doc = "0: 0 clock cycle selected"]
    VALUE1 = 0,
    #[doc = "1: 1 clock cycle selected"]
    VALUE2 = 1,
    #[doc = "14: 14 clock cycles selected"]
    VALUE3 = 14,
    #[doc = "15: 15 clock cycles selected"]
    VALUE4 = 15,
}
impl From<AHOLDC_A> for u8 {
    #[inline(always)]
    fn from(variant: AHOLDC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AHOLDC_A {
    type Ux = u8;
}
impl crate::IsEnum for AHOLDC_A {}
#[doc = "Field `AHOLDC` reader - Address Hold Cycles"]
pub type AHOLDC_R = crate::FieldReader<AHOLDC_A>;
impl AHOLDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AHOLDC_A> {
        match self.bits {
            0 => Some(AHOLDC_A::VALUE1),
            1 => Some(AHOLDC_A::VALUE2),
            14 => Some(AHOLDC_A::VALUE3),
            15 => Some(AHOLDC_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "0 clock cycle selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AHOLDC_A::VALUE1
    }
    #[doc = "1 clock cycle selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AHOLDC_A::VALUE2
    }
    #[doc = "14 clock cycles selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == AHOLDC_A::VALUE3
    }
    #[doc = "15 clock cycles selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == AHOLDC_A::VALUE4
    }
}
#[doc = "Field `AHOLDC` writer - Address Hold Cycles"]
pub type AHOLDC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AHOLDC_A>;
impl<'a, REG> AHOLDC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 clock cycle selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AHOLDC_A::VALUE1)
    }
    #[doc = "1 clock cycle selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AHOLDC_A::VALUE2)
    }
    #[doc = "14 clock cycles selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(AHOLDC_A::VALUE3)
    }
    #[doc = "15 clock cycles selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(AHOLDC_A::VALUE4)
    }
}
#[doc = "Address Cycles\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADDRC_A {
    #[doc = "0: 1 clock cycle selected"]
    VALUE1 = 0,
    #[doc = "1: 1 clock cycle selected"]
    VALUE2 = 1,
    #[doc = "14: 14 clock cycles selected"]
    VALUE3 = 14,
    #[doc = "15: 15 clock cycles selected"]
    VALUE4 = 15,
}
impl From<ADDRC_A> for u8 {
    #[inline(always)]
    fn from(variant: ADDRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADDRC_A {
    type Ux = u8;
}
impl crate::IsEnum for ADDRC_A {}
#[doc = "Field `ADDRC` reader - Address Cycles"]
pub type ADDRC_R = crate::FieldReader<ADDRC_A>;
impl ADDRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADDRC_A> {
        match self.bits {
            0 => Some(ADDRC_A::VALUE1),
            1 => Some(ADDRC_A::VALUE2),
            14 => Some(ADDRC_A::VALUE3),
            15 => Some(ADDRC_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "1 clock cycle selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ADDRC_A::VALUE1
    }
    #[doc = "1 clock cycle selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ADDRC_A::VALUE2
    }
    #[doc = "14 clock cycles selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ADDRC_A::VALUE3
    }
    #[doc = "15 clock cycles selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ADDRC_A::VALUE4
    }
}
#[doc = "Field `ADDRC` writer - Address Cycles"]
pub type ADDRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ADDRC_A>;
impl<'a, REG> ADDRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 clock cycle selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRC_A::VALUE1)
    }
    #[doc = "1 clock cycle selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRC_A::VALUE2)
    }
    #[doc = "14 clock cycles selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRC_A::VALUE3)
    }
    #[doc = "15 clock cycles selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRC_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:3 - Recovery Cycles between Different Regions"]
    #[inline(always)]
    pub fn rddtacs(&self) -> RDDTACS_R {
        RDDTACS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Recovery Cycles after Read Accesses"]
    #[inline(always)]
    pub fn rdrecovc(&self) -> RDRECOVC_R {
        RDRECOVC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:11 - Programmed Wait States for read accesses"]
    #[inline(always)]
    pub fn waitrdc(&self) -> WAITRDC_R {
        WAITRDC_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:15 - Data Hold Cycles for Read Accesses"]
    #[inline(always)]
    pub fn datac(&self) -> DATAC_R {
        DATAC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Frequency of external clock at pin BFCLKO"]
    #[inline(always)]
    pub fn extclock(&self) -> EXTCLOCK_R {
        EXTCLOCK_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Extended data"]
    #[inline(always)]
    pub fn extdata(&self) -> EXTDATA_R {
        EXTDATA_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:23 - Command Delay Cycles"]
    #[inline(always)]
    pub fn cmddelay(&self) -> CMDDELAY_R {
        CMDDELAY_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Address Hold Cycles"]
    #[inline(always)]
    pub fn aholdc(&self) -> AHOLDC_R {
        AHOLDC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Address Cycles"]
    #[inline(always)]
    pub fn addrc(&self) -> ADDRC_R {
        ADDRC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Recovery Cycles between Different Regions"]
    #[inline(always)]
    #[must_use]
    pub fn rddtacs(&mut self) -> RDDTACS_W<BUSRAP2_SPEC> {
        RDDTACS_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Recovery Cycles after Read Accesses"]
    #[inline(always)]
    #[must_use]
    pub fn rdrecovc(&mut self) -> RDRECOVC_W<BUSRAP2_SPEC> {
        RDRECOVC_W::new(self, 4)
    }
    #[doc = "Bits 7:11 - Programmed Wait States for read accesses"]
    #[inline(always)]
    #[must_use]
    pub fn waitrdc(&mut self) -> WAITRDC_W<BUSRAP2_SPEC> {
        WAITRDC_W::new(self, 7)
    }
    #[doc = "Bits 12:15 - Data Hold Cycles for Read Accesses"]
    #[inline(always)]
    #[must_use]
    pub fn datac(&mut self) -> DATAC_W<BUSRAP2_SPEC> {
        DATAC_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - Frequency of external clock at pin BFCLKO"]
    #[inline(always)]
    #[must_use]
    pub fn extclock(&mut self) -> EXTCLOCK_W<BUSRAP2_SPEC> {
        EXTCLOCK_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Extended data"]
    #[inline(always)]
    #[must_use]
    pub fn extdata(&mut self) -> EXTDATA_W<BUSRAP2_SPEC> {
        EXTDATA_W::new(self, 18)
    }
    #[doc = "Bits 20:23 - Command Delay Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn cmddelay(&mut self) -> CMDDELAY_W<BUSRAP2_SPEC> {
        CMDDELAY_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Address Hold Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn aholdc(&mut self) -> AHOLDC_W<BUSRAP2_SPEC> {
        AHOLDC_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Address Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn addrc(&mut self) -> ADDRC_W<BUSRAP2_SPEC> {
        ADDRC_W::new(self, 28)
    }
}
#[doc = "EBU Bus Read Access Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busrap2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busrap2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUSRAP2_SPEC;
impl crate::RegisterSpec for BUSRAP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`busrap2::R`](R) reader structure"]
impl crate::Readable for BUSRAP2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`busrap2::W`](W) writer structure"]
impl crate::Writable for BUSRAP2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUSRAP2 to value 0xffff_ffff"]
impl crate::Resettable for BUSRAP2_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
