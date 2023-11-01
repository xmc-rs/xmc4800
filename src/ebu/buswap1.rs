#[doc = "Register `BUSWAP1` reader"]
pub type R = crate::R<BUSWAP1_SPEC>;
#[doc = "Register `BUSWAP1` writer"]
pub type W = crate::W<BUSWAP1_SPEC>;
#[doc = "Field `WRDTACS` reader - Recovery Cycles between Different Regions"]
pub type WRDTACS_R = crate::FieldReader<WRDTACS_A>;
#[doc = "Recovery Cycles between Different Regions\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRDTACS_A {
    #[doc = "0: No Recovery Phase clock cycles available."]
    VALUE1 = 0,
    #[doc = "1: 1 clock cycle selected."]
    VALUE2 = 1,
    #[doc = "14: 14 clock cycles selected."]
    VALUE3 = 14,
    #[doc = "15: 15 clock cycles selected."]
    VALUE4 = 15,
}
impl From<WRDTACS_A> for u8 {
    #[inline(always)]
    fn from(variant: WRDTACS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WRDTACS_A {
    type Ux = u8;
}
impl WRDTACS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WRDTACS_A> {
        match self.bits {
            0 => Some(WRDTACS_A::VALUE1),
            1 => Some(WRDTACS_A::VALUE2),
            14 => Some(WRDTACS_A::VALUE3),
            15 => Some(WRDTACS_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WRDTACS_A::VALUE1
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WRDTACS_A::VALUE2
    }
    #[doc = "14 clock cycles selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == WRDTACS_A::VALUE3
    }
    #[doc = "15 clock cycles selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == WRDTACS_A::VALUE4
    }
}
#[doc = "Field `WRDTACS` writer - Recovery Cycles between Different Regions"]
pub type WRDTACS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, WRDTACS_A>;
impl<'a, REG, const O: u8> WRDTACS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WRDTACS_A::VALUE1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WRDTACS_A::VALUE2)
    }
    #[doc = "14 clock cycles selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(WRDTACS_A::VALUE3)
    }
    #[doc = "15 clock cycles selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(WRDTACS_A::VALUE4)
    }
}
#[doc = "Field `WRRECOVC` reader - Recovery Cycles after Write Accesses"]
pub type WRRECOVC_R = crate::FieldReader<WRRECOVC_A>;
#[doc = "Recovery Cycles after Write Accesses\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRRECOVC_A {
    #[doc = "0: No Recovery Phase clock cycles available."]
    VALUE1 = 0,
    #[doc = "1: 1 clock cycle selected."]
    VALUE2 = 1,
    #[doc = "6: 6 clock cycles selected."]
    VALUE3 = 6,
    #[doc = "7: 7 clock cycles selected."]
    VALUE4 = 7,
}
impl From<WRRECOVC_A> for u8 {
    #[inline(always)]
    fn from(variant: WRRECOVC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WRRECOVC_A {
    type Ux = u8;
}
impl WRRECOVC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WRRECOVC_A> {
        match self.bits {
            0 => Some(WRRECOVC_A::VALUE1),
            1 => Some(WRRECOVC_A::VALUE2),
            6 => Some(WRRECOVC_A::VALUE3),
            7 => Some(WRRECOVC_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WRRECOVC_A::VALUE1
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WRRECOVC_A::VALUE2
    }
    #[doc = "6 clock cycles selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == WRRECOVC_A::VALUE3
    }
    #[doc = "7 clock cycles selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == WRRECOVC_A::VALUE4
    }
}
#[doc = "Field `WRRECOVC` writer - Recovery Cycles after Write Accesses"]
pub type WRRECOVC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, WRRECOVC_A>;
impl<'a, REG, const O: u8> WRRECOVC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WRRECOVC_A::VALUE1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WRRECOVC_A::VALUE2)
    }
    #[doc = "6 clock cycles selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(WRRECOVC_A::VALUE3)
    }
    #[doc = "7 clock cycles selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(WRRECOVC_A::VALUE4)
    }
}
#[doc = "Field `WAITWRC` reader - Programmed Wait States for write accesses"]
pub type WAITWRC_R = crate::FieldReader<WAITWRC_A>;
#[doc = "Programmed Wait States for write accesses\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAITWRC_A {
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
impl From<WAITWRC_A> for u8 {
    #[inline(always)]
    fn from(variant: WAITWRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WAITWRC_A {
    type Ux = u8;
}
impl WAITWRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WAITWRC_A> {
        match self.bits {
            0 => Some(WAITWRC_A::VALUE1),
            1 => Some(WAITWRC_A::VALUE2),
            2 => Some(WAITWRC_A::VALUE3),
            30 => Some(WAITWRC_A::VALUE4),
            31 => Some(WAITWRC_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "1 wait state."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAITWRC_A::VALUE1
    }
    #[doc = "1 wait states."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAITWRC_A::VALUE2
    }
    #[doc = "2 wait state."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == WAITWRC_A::VALUE3
    }
    #[doc = "30 wait states."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == WAITWRC_A::VALUE4
    }
    #[doc = "31 wait states."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == WAITWRC_A::VALUE5
    }
}
#[doc = "Field `WAITWRC` writer - Programmed Wait States for write accesses"]
pub type WAITWRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O, WAITWRC_A>;
impl<'a, REG, const O: u8> WAITWRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 wait state."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WAITWRC_A::VALUE1)
    }
    #[doc = "1 wait states."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WAITWRC_A::VALUE2)
    }
    #[doc = "2 wait state."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(WAITWRC_A::VALUE3)
    }
    #[doc = "30 wait states."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(WAITWRC_A::VALUE4)
    }
    #[doc = "31 wait states."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(WAITWRC_A::VALUE5)
    }
}
#[doc = "Field `DATAC` reader - Data Hold Cycles for Write Accesses"]
pub type DATAC_R = crate::FieldReader<DATAC_A>;
#[doc = "Data Hold Cycles for Write Accesses\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATAC_A {
    #[doc = "0: No Recovery Phase clock cycles available."]
    VALUE1 = 0,
    #[doc = "1: 1 clock cycle selected."]
    VALUE2 = 1,
    #[doc = "14: 14 clock cycles selected."]
    VALUE3 = 14,
    #[doc = "15: 15 clock cycles selected."]
    VALUE4 = 15,
}
impl From<DATAC_A> for u8 {
    #[inline(always)]
    fn from(variant: DATAC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATAC_A {
    type Ux = u8;
}
impl DATAC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DATAC_A> {
        match self.bits {
            0 => Some(DATAC_A::VALUE1),
            1 => Some(DATAC_A::VALUE2),
            14 => Some(DATAC_A::VALUE3),
            15 => Some(DATAC_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DATAC_A::VALUE1
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DATAC_A::VALUE2
    }
    #[doc = "14 clock cycles selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DATAC_A::VALUE3
    }
    #[doc = "15 clock cycles selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DATAC_A::VALUE4
    }
}
#[doc = "Field `DATAC` writer - Data Hold Cycles for Write Accesses"]
pub type DATAC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, DATAC_A>;
impl<'a, REG, const O: u8> DATAC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DATAC_A::VALUE1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DATAC_A::VALUE2)
    }
    #[doc = "14 clock cycles selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(DATAC_A::VALUE3)
    }
    #[doc = "15 clock cycles selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(DATAC_A::VALUE4)
    }
}
#[doc = "Field `EXTCLOCK` reader - Frequency of external clock at pin BFCLKO"]
pub type EXTCLOCK_R = crate::FieldReader<EXTCLOCK_A>;
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
pub type EXTCLOCK_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, EXTCLOCK_A>;
impl<'a, REG, const O: u8> EXTCLOCK_W<'a, REG, O>
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
#[doc = "Field `EXTDATA` reader - Extended data"]
pub type EXTDATA_R = crate::FieldReader<EXTDATA_A>;
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
pub type EXTDATA_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, EXTDATA_A>;
impl<'a, REG, const O: u8> EXTDATA_W<'a, REG, O>
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
#[doc = "Field `CMDDELAY` reader - Command Delay Cycles"]
pub type CMDDELAY_R = crate::FieldReader<CMDDELAY_A>;
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
pub type CMDDELAY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, CMDDELAY_A>;
impl<'a, REG, const O: u8> CMDDELAY_W<'a, REG, O>
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
#[doc = "Field `AHOLDC` reader - Address Hold Cycles"]
pub type AHOLDC_R = crate::FieldReader<AHOLDC_A>;
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
pub type AHOLDC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, AHOLDC_A>;
impl<'a, REG, const O: u8> AHOLDC_W<'a, REG, O>
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
#[doc = "Field `ADDRC` reader - Address Cycles"]
pub type ADDRC_R = crate::FieldReader<ADDRC_A>;
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
pub type ADDRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, ADDRC_A>;
impl<'a, REG, const O: u8> ADDRC_W<'a, REG, O>
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
    pub fn wrdtacs(&self) -> WRDTACS_R {
        WRDTACS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Recovery Cycles after Write Accesses"]
    #[inline(always)]
    pub fn wrrecovc(&self) -> WRRECOVC_R {
        WRRECOVC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:11 - Programmed Wait States for write accesses"]
    #[inline(always)]
    pub fn waitwrc(&self) -> WAITWRC_R {
        WAITWRC_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:15 - Data Hold Cycles for Write Accesses"]
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
    pub fn wrdtacs(&mut self) -> WRDTACS_W<BUSWAP1_SPEC, 0> {
        WRDTACS_W::new(self)
    }
    #[doc = "Bits 4:6 - Recovery Cycles after Write Accesses"]
    #[inline(always)]
    #[must_use]
    pub fn wrrecovc(&mut self) -> WRRECOVC_W<BUSWAP1_SPEC, 4> {
        WRRECOVC_W::new(self)
    }
    #[doc = "Bits 7:11 - Programmed Wait States for write accesses"]
    #[inline(always)]
    #[must_use]
    pub fn waitwrc(&mut self) -> WAITWRC_W<BUSWAP1_SPEC, 7> {
        WAITWRC_W::new(self)
    }
    #[doc = "Bits 12:15 - Data Hold Cycles for Write Accesses"]
    #[inline(always)]
    #[must_use]
    pub fn datac(&mut self) -> DATAC_W<BUSWAP1_SPEC, 12> {
        DATAC_W::new(self)
    }
    #[doc = "Bits 16:17 - Frequency of external clock at pin BFCLKO"]
    #[inline(always)]
    #[must_use]
    pub fn extclock(&mut self) -> EXTCLOCK_W<BUSWAP1_SPEC, 16> {
        EXTCLOCK_W::new(self)
    }
    #[doc = "Bits 18:19 - Extended data"]
    #[inline(always)]
    #[must_use]
    pub fn extdata(&mut self) -> EXTDATA_W<BUSWAP1_SPEC, 18> {
        EXTDATA_W::new(self)
    }
    #[doc = "Bits 20:23 - Command Delay Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn cmddelay(&mut self) -> CMDDELAY_W<BUSWAP1_SPEC, 20> {
        CMDDELAY_W::new(self)
    }
    #[doc = "Bits 24:27 - Address Hold Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn aholdc(&mut self) -> AHOLDC_W<BUSWAP1_SPEC, 24> {
        AHOLDC_W::new(self)
    }
    #[doc = "Bits 28:31 - Address Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn addrc(&mut self) -> ADDRC_W<BUSWAP1_SPEC, 28> {
        ADDRC_W::new(self)
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
#[doc = "EBU Bus Write Access Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buswap1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buswap1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUSWAP1_SPEC;
impl crate::RegisterSpec for BUSWAP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buswap1::R`](R) reader structure"]
impl crate::Readable for BUSWAP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buswap1::W`](W) writer structure"]
impl crate::Writable for BUSWAP1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUSWAP1 to value 0xffff_ffff"]
impl crate::Resettable for BUSWAP1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
