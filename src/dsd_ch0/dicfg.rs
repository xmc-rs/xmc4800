#[doc = "Register `DICFG` reader"]
pub type R = crate::R<DICFG_SPEC>;
#[doc = "Register `DICFG` writer"]
pub type W = crate::W<DICFG_SPEC>;
#[doc = "Input Data Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSRC_A {
    #[doc = "0: Disconnected"]
    VALUE1 = 0,
    #[doc = "2: External, from input A, direct"]
    VALUE2 = 2,
    #[doc = "3: External, from input A, inverted"]
    VALUE3 = 3,
    #[doc = "4: External, from input B, direct"]
    VALUE4 = 4,
    #[doc = "5: External, from input B, inverted"]
    VALUE5 = 5,
}
impl From<DSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: DSRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DSRC_A {
    type Ux = u8;
}
impl crate::IsEnum for DSRC_A {}
#[doc = "Field `DSRC` reader - Input Data Source Select"]
pub type DSRC_R = crate::FieldReader<DSRC_A>;
impl DSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DSRC_A> {
        match self.bits {
            0 => Some(DSRC_A::VALUE1),
            2 => Some(DSRC_A::VALUE2),
            3 => Some(DSRC_A::VALUE3),
            4 => Some(DSRC_A::VALUE4),
            5 => Some(DSRC_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Disconnected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DSRC_A::VALUE1
    }
    #[doc = "External, from input A, direct"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DSRC_A::VALUE2
    }
    #[doc = "External, from input A, inverted"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DSRC_A::VALUE3
    }
    #[doc = "External, from input B, direct"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DSRC_A::VALUE4
    }
    #[doc = "External, from input B, inverted"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == DSRC_A::VALUE5
    }
}
#[doc = "Field `DSRC` writer - Input Data Source Select"]
pub type DSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, DSRC_A>;
impl<'a, REG> DSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disconnected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DSRC_A::VALUE1)
    }
    #[doc = "External, from input A, direct"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DSRC_A::VALUE2)
    }
    #[doc = "External, from input A, inverted"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(DSRC_A::VALUE3)
    }
    #[doc = "External, from input B, direct"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(DSRC_A::VALUE4)
    }
    #[doc = "External, from input B, inverted"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(DSRC_A::VALUE5)
    }
}
#[doc = "Write Control for Data Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSWC_A {
    #[doc = "0: No write access to data parameters"]
    VALUE1 = 0,
    #[doc = "1: Bitfield DSRC can be written"]
    VALUE2 = 1,
}
impl From<DSWC_A> for bool {
    #[inline(always)]
    fn from(variant: DSWC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSWC` writer - Write Control for Data Selection"]
pub type DSWC_W<'a, REG> = crate::BitWriter<'a, REG, DSWC_A>;
impl<'a, REG> DSWC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write access to data parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DSWC_A::VALUE1)
    }
    #[doc = "Bitfield DSRC can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DSWC_A::VALUE2)
    }
}
#[doc = "Integrator Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ITRMODE_A {
    #[doc = "0: No integration trigger, integrator bypassed, INTEN = 0 all the time"]
    VALUE1 = 0,
    #[doc = "1: Trigger event upon a falling edge"]
    VALUE2 = 1,
    #[doc = "2: Trigger event upon a rising edge"]
    VALUE3 = 2,
    #[doc = "3: No trigger, integrator active all the time, INTEN = 1 all the time"]
    VALUE4 = 3,
}
impl From<ITRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ITRMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ITRMODE_A {
    type Ux = u8;
}
impl crate::IsEnum for ITRMODE_A {}
#[doc = "Field `ITRMODE` reader - Integrator Trigger Mode"]
pub type ITRMODE_R = crate::FieldReader<ITRMODE_A>;
impl ITRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITRMODE_A {
        match self.bits {
            0 => ITRMODE_A::VALUE1,
            1 => ITRMODE_A::VALUE2,
            2 => ITRMODE_A::VALUE3,
            3 => ITRMODE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "No integration trigger, integrator bypassed, INTEN = 0 all the time"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ITRMODE_A::VALUE1
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ITRMODE_A::VALUE2
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ITRMODE_A::VALUE3
    }
    #[doc = "No trigger, integrator active all the time, INTEN = 1 all the time"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ITRMODE_A::VALUE4
    }
}
#[doc = "Field `ITRMODE` writer - Integrator Trigger Mode"]
pub type ITRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ITRMODE_A, crate::Safe>;
impl<'a, REG> ITRMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No integration trigger, integrator bypassed, INTEN = 0 all the time"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ITRMODE_A::VALUE1)
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ITRMODE_A::VALUE2)
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(ITRMODE_A::VALUE3)
    }
    #[doc = "No trigger, integrator active all the time, INTEN = 1 all the time"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(ITRMODE_A::VALUE4)
    }
}
#[doc = "Timestamp Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTRMODE_A {
    #[doc = "0: No timestamp trigger"]
    VALUE1 = 0,
    #[doc = "1: Trigger event upon a falling edge"]
    VALUE2 = 1,
    #[doc = "2: Trigger event upon a rising edge"]
    VALUE3 = 2,
    #[doc = "3: Trigger event upon each edge"]
    VALUE4 = 3,
}
impl From<TSTRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTRMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSTRMODE_A {
    type Ux = u8;
}
impl crate::IsEnum for TSTRMODE_A {}
#[doc = "Field `TSTRMODE` reader - Timestamp Trigger Mode"]
pub type TSTRMODE_R = crate::FieldReader<TSTRMODE_A>;
impl TSTRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSTRMODE_A {
        match self.bits {
            0 => TSTRMODE_A::VALUE1,
            1 => TSTRMODE_A::VALUE2,
            2 => TSTRMODE_A::VALUE3,
            3 => TSTRMODE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "No timestamp trigger"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSTRMODE_A::VALUE1
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSTRMODE_A::VALUE2
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TSTRMODE_A::VALUE3
    }
    #[doc = "Trigger event upon each edge"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TSTRMODE_A::VALUE4
    }
}
#[doc = "Field `TSTRMODE` writer - Timestamp Trigger Mode"]
pub type TSTRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TSTRMODE_A, crate::Safe>;
impl<'a, REG> TSTRMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No timestamp trigger"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TSTRMODE_A::VALUE1)
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TSTRMODE_A::VALUE2)
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(TSTRMODE_A::VALUE3)
    }
    #[doc = "Trigger event upon each edge"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(TSTRMODE_A::VALUE4)
    }
}
#[doc = "Field `TRSEL` reader - Trigger Select"]
pub type TRSEL_R = crate::FieldReader;
#[doc = "Field `TRSEL` writer - Trigger Select"]
pub type TRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Write Control for Trigger Parameters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRWC_A {
    #[doc = "0: No write access to trigger parameters"]
    VALUE1 = 0,
    #[doc = "1: Bitfields TRSEL, TSTRMODE, ITRMODE can be written"]
    VALUE2 = 1,
}
impl From<TRWC_A> for bool {
    #[inline(always)]
    fn from(variant: TRWC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRWC` writer - Write Control for Trigger Parameters"]
pub type TRWC_W<'a, REG> = crate::BitWriter<'a, REG, TRWC_A>;
impl<'a, REG> TRWC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write access to trigger parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TRWC_A::VALUE1)
    }
    #[doc = "Bitfields TRSEL, TSTRMODE, ITRMODE can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TRWC_A::VALUE2)
    }
}
#[doc = "Sample Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSRC_A {
    #[doc = "1: External, from input A"]
    VALUE2 = 1,
    #[doc = "2: External, from input B"]
    VALUE3 = 2,
    #[doc = "3: External, from input C"]
    VALUE4 = 3,
    #[doc = "4: External, from input D"]
    VALUE5 = 4,
    #[doc = "15: Internal clock"]
    VALUE6 = 15,
}
impl From<CSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CSRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSRC_A {
    type Ux = u8;
}
impl crate::IsEnum for CSRC_A {}
#[doc = "Field `CSRC` reader - Sample Clock Source Select"]
pub type CSRC_R = crate::FieldReader<CSRC_A>;
impl CSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CSRC_A> {
        match self.bits {
            1 => Some(CSRC_A::VALUE2),
            2 => Some(CSRC_A::VALUE3),
            3 => Some(CSRC_A::VALUE4),
            4 => Some(CSRC_A::VALUE5),
            15 => Some(CSRC_A::VALUE6),
            _ => None,
        }
    }
    #[doc = "External, from input A"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CSRC_A::VALUE2
    }
    #[doc = "External, from input B"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CSRC_A::VALUE3
    }
    #[doc = "External, from input C"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CSRC_A::VALUE4
    }
    #[doc = "External, from input D"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == CSRC_A::VALUE5
    }
    #[doc = "Internal clock"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == CSRC_A::VALUE6
    }
}
#[doc = "Field `CSRC` writer - Sample Clock Source Select"]
pub type CSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CSRC_A>;
impl<'a, REG> CSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External, from input A"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CSRC_A::VALUE2)
    }
    #[doc = "External, from input B"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CSRC_A::VALUE3)
    }
    #[doc = "External, from input C"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CSRC_A::VALUE4)
    }
    #[doc = "External, from input D"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(CSRC_A::VALUE5)
    }
    #[doc = "Internal clock"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(CSRC_A::VALUE6)
    }
}
#[doc = "Data Strobe Generatoion Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STROBE_A {
    #[doc = "0: No data strobe"]
    VALUE1 = 0,
    #[doc = "1: Direct clock, a sample trigger is generated at each rising clock edge"]
    VALUE2 = 1,
    #[doc = "2: Direct clock, a sample trigger is generated at each falling clock edge"]
    VALUE3 = 2,
    #[doc = "3: Double data, a sample trigger is generated at each rising and falling clock edge"]
    VALUE4 = 3,
    #[doc = "5: Double clock, a sample trigger is generated at every 2nd rising clock edge"]
    VALUE6 = 5,
    #[doc = "6: Double clock, a sample trigger is generated at every 2nd falling clock edge"]
    VALUE7 = 6,
}
impl From<STROBE_A> for u8 {
    #[inline(always)]
    fn from(variant: STROBE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STROBE_A {
    type Ux = u8;
}
impl crate::IsEnum for STROBE_A {}
#[doc = "Field `STROBE` reader - Data Strobe Generatoion Mode"]
pub type STROBE_R = crate::FieldReader<STROBE_A>;
impl STROBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<STROBE_A> {
        match self.bits {
            0 => Some(STROBE_A::VALUE1),
            1 => Some(STROBE_A::VALUE2),
            2 => Some(STROBE_A::VALUE3),
            3 => Some(STROBE_A::VALUE4),
            5 => Some(STROBE_A::VALUE6),
            6 => Some(STROBE_A::VALUE7),
            _ => None,
        }
    }
    #[doc = "No data strobe"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STROBE_A::VALUE1
    }
    #[doc = "Direct clock, a sample trigger is generated at each rising clock edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STROBE_A::VALUE2
    }
    #[doc = "Direct clock, a sample trigger is generated at each falling clock edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STROBE_A::VALUE3
    }
    #[doc = "Double data, a sample trigger is generated at each rising and falling clock edge"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STROBE_A::VALUE4
    }
    #[doc = "Double clock, a sample trigger is generated at every 2nd rising clock edge"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == STROBE_A::VALUE6
    }
    #[doc = "Double clock, a sample trigger is generated at every 2nd falling clock edge"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == STROBE_A::VALUE7
    }
}
#[doc = "Field `STROBE` writer - Data Strobe Generatoion Mode"]
pub type STROBE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, STROBE_A>;
impl<'a, REG> STROBE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No data strobe"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(STROBE_A::VALUE1)
    }
    #[doc = "Direct clock, a sample trigger is generated at each rising clock edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(STROBE_A::VALUE2)
    }
    #[doc = "Direct clock, a sample trigger is generated at each falling clock edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(STROBE_A::VALUE3)
    }
    #[doc = "Double data, a sample trigger is generated at each rising and falling clock edge"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(STROBE_A::VALUE4)
    }
    #[doc = "Double clock, a sample trigger is generated at every 2nd rising clock edge"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(STROBE_A::VALUE6)
    }
    #[doc = "Double clock, a sample trigger is generated at every 2nd falling clock edge"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(STROBE_A::VALUE7)
    }
}
#[doc = "Write Control for Strobe/Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCWC_A {
    #[doc = "0: No write access to strobe/clock parameters"]
    VALUE1 = 0,
    #[doc = "1: Bitfields STROBE, CSRC can be written"]
    VALUE2 = 1,
}
impl From<SCWC_A> for bool {
    #[inline(always)]
    fn from(variant: SCWC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCWC` writer - Write Control for Strobe/Clock Selection"]
pub type SCWC_W<'a, REG> = crate::BitWriter<'a, REG, SCWC_A>;
impl<'a, REG> SCWC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write access to strobe/clock parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SCWC_A::VALUE1)
    }
    #[doc = "Bitfields STROBE, CSRC can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SCWC_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:3 - Input Data Source Select"]
    #[inline(always)]
    pub fn dsrc(&self) -> DSRC_R {
        DSRC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Integrator Trigger Mode"]
    #[inline(always)]
    pub fn itrmode(&self) -> ITRMODE_R {
        ITRMODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Timestamp Trigger Mode"]
    #[inline(always)]
    pub fn tstrmode(&self) -> TSTRMODE_R {
        TSTRMODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Trigger Select"]
    #[inline(always)]
    pub fn trsel(&self) -> TRSEL_R {
        TRSEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Sample Clock Source Select"]
    #[inline(always)]
    pub fn csrc(&self) -> CSRC_R {
        CSRC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Data Strobe Generatoion Mode"]
    #[inline(always)]
    pub fn strobe(&self) -> STROBE_R {
        STROBE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input Data Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn dsrc(&mut self) -> DSRC_W<DICFG_SPEC> {
        DSRC_W::new(self, 0)
    }
    #[doc = "Bit 7 - Write Control for Data Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dswc(&mut self) -> DSWC_W<DICFG_SPEC> {
        DSWC_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Integrator Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn itrmode(&mut self) -> ITRMODE_W<DICFG_SPEC> {
        ITRMODE_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Timestamp Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tstrmode(&mut self) -> TSTRMODE_W<DICFG_SPEC> {
        TSTRMODE_W::new(self, 10)
    }
    #[doc = "Bits 12:14 - Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn trsel(&mut self) -> TRSEL_W<DICFG_SPEC> {
        TRSEL_W::new(self, 12)
    }
    #[doc = "Bit 15 - Write Control for Trigger Parameters"]
    #[inline(always)]
    #[must_use]
    pub fn trwc(&mut self) -> TRWC_W<DICFG_SPEC> {
        TRWC_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Sample Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn csrc(&mut self) -> CSRC_W<DICFG_SPEC> {
        CSRC_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Data Strobe Generatoion Mode"]
    #[inline(always)]
    #[must_use]
    pub fn strobe(&mut self) -> STROBE_W<DICFG_SPEC> {
        STROBE_W::new(self, 20)
    }
    #[doc = "Bit 31 - Write Control for Strobe/Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn scwc(&mut self) -> SCWC_W<DICFG_SPEC> {
        SCWC_W::new(self, 31)
    }
}
#[doc = "Demodulator Input Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dicfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dicfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DICFG_SPEC;
impl crate::RegisterSpec for DICFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dicfg::R`](R) reader structure"]
impl crate::Readable for DICFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dicfg::W`](W) writer structure"]
impl crate::Writable for DICFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DICFG to value 0"]
impl crate::Resettable for DICFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
