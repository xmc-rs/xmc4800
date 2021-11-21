#[doc = "Register `BUSWAP3` reader"]
pub struct R(crate::R<BUSWAP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSWAP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSWAP3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSWAP3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUSWAP3` writer"]
pub struct W(crate::W<BUSWAP3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUSWAP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BUSWAP3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUSWAP3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Recovery Cycles between Different Regions\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `WRDTACS` reader - Recovery Cycles between Different Regions"]
pub struct WRDTACS_R(crate::FieldReader<u8, WRDTACS_A>);
impl WRDTACS_R {
    pub(crate) fn new(bits: u8) -> Self {
        WRDTACS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WRDTACS_A> {
        match self.bits {
            0 => Some(WRDTACS_A::VALUE1),
            1 => Some(WRDTACS_A::VALUE2),
            14 => Some(WRDTACS_A::VALUE3),
            15 => Some(WRDTACS_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == WRDTACS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WRDTACS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == WRDTACS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == WRDTACS_A::VALUE4
    }
}
impl core::ops::Deref for WRDTACS_R {
    type Target = crate::FieldReader<u8, WRDTACS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRDTACS` writer - Recovery Cycles between Different Regions"]
pub struct WRDTACS_W<'a> {
    w: &'a mut W,
}
impl<'a> WRDTACS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRDTACS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WRDTACS_A::VALUE1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WRDTACS_A::VALUE2)
    }
    #[doc = "14 clock cycles selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(WRDTACS_A::VALUE3)
    }
    #[doc = "15 clock cycles selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(WRDTACS_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Recovery Cycles after Write Accesses\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `WRRECOVC` reader - Recovery Cycles after Write Accesses"]
pub struct WRRECOVC_R(crate::FieldReader<u8, WRRECOVC_A>);
impl WRRECOVC_R {
    pub(crate) fn new(bits: u8) -> Self {
        WRRECOVC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WRRECOVC_A> {
        match self.bits {
            0 => Some(WRRECOVC_A::VALUE1),
            1 => Some(WRRECOVC_A::VALUE2),
            6 => Some(WRRECOVC_A::VALUE3),
            7 => Some(WRRECOVC_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == WRRECOVC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WRRECOVC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == WRRECOVC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == WRRECOVC_A::VALUE4
    }
}
impl core::ops::Deref for WRRECOVC_R {
    type Target = crate::FieldReader<u8, WRRECOVC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRRECOVC` writer - Recovery Cycles after Write Accesses"]
pub struct WRRECOVC_W<'a> {
    w: &'a mut W,
}
impl<'a> WRRECOVC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRRECOVC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WRRECOVC_A::VALUE1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WRRECOVC_A::VALUE2)
    }
    #[doc = "6 clock cycles selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(WRRECOVC_A::VALUE3)
    }
    #[doc = "7 clock cycles selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(WRRECOVC_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Programmed Wait States for write accesses\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `WAITWRC` reader - Programmed Wait States for write accesses"]
pub struct WAITWRC_R(crate::FieldReader<u8, WAITWRC_A>);
impl WAITWRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAITWRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WAITWRC_A> {
        match self.bits {
            0 => Some(WAITWRC_A::VALUE1),
            1 => Some(WAITWRC_A::VALUE2),
            2 => Some(WAITWRC_A::VALUE3),
            30 => Some(WAITWRC_A::VALUE4),
            31 => Some(WAITWRC_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == WAITWRC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WAITWRC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == WAITWRC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == WAITWRC_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == WAITWRC_A::VALUE5
    }
}
impl core::ops::Deref for WAITWRC_R {
    type Target = crate::FieldReader<u8, WAITWRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITWRC` writer - Programmed Wait States for write accesses"]
pub struct WAITWRC_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITWRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAITWRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 wait state."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAITWRC_A::VALUE1)
    }
    #[doc = "1 wait states."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAITWRC_A::VALUE2)
    }
    #[doc = "2 wait state."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(WAITWRC_A::VALUE3)
    }
    #[doc = "30 wait states."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(WAITWRC_A::VALUE4)
    }
    #[doc = "31 wait states."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(WAITWRC_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 7)) | ((value as u32 & 0x1f) << 7);
        self.w
    }
}
#[doc = "Data Hold Cycles for Write Accesses\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DATAC` reader - Data Hold Cycles for Write Accesses"]
pub struct DATAC_R(crate::FieldReader<u8, DATAC_A>);
impl DATAC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATAC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATAC_A> {
        match self.bits {
            0 => Some(DATAC_A::VALUE1),
            1 => Some(DATAC_A::VALUE2),
            14 => Some(DATAC_A::VALUE3),
            15 => Some(DATAC_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DATAC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DATAC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == DATAC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == DATAC_A::VALUE4
    }
}
impl core::ops::Deref for DATAC_R {
    type Target = crate::FieldReader<u8, DATAC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAC` writer - Data Hold Cycles for Write Accesses"]
pub struct DATAC_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATAC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATAC_A::VALUE1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATAC_A::VALUE2)
    }
    #[doc = "14 clock cycles selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DATAC_A::VALUE3)
    }
    #[doc = "15 clock cycles selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(DATAC_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Frequency of external clock at pin BFCLKO\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `EXTCLOCK` reader - Frequency of external clock at pin BFCLKO"]
pub struct EXTCLOCK_R(crate::FieldReader<u8, EXTCLOCK_A>);
impl EXTCLOCK_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTCLOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTCLOCK_A {
        match self.bits {
            0 => EXTCLOCK_A::VALUE1,
            1 => EXTCLOCK_A::VALUE2,
            2 => EXTCLOCK_A::VALUE3,
            3 => EXTCLOCK_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EXTCLOCK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EXTCLOCK_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == EXTCLOCK_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == EXTCLOCK_A::VALUE4
    }
}
impl core::ops::Deref for EXTCLOCK_R {
    type Target = crate::FieldReader<u8, EXTCLOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTCLOCK` writer - Frequency of external clock at pin BFCLKO"]
pub struct EXTCLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTCLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTCLOCK_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Equal to INT_CLK frequency."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXTCLOCK_A::VALUE1)
    }
    #[doc = "1/2 of INT_CLK frequency."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXTCLOCK_A::VALUE2)
    }
    #[doc = "1/3 of INT_CLK frequency."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EXTCLOCK_A::VALUE3)
    }
    #[doc = "1/4 of INT_CLK frequency (default after reset)."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EXTCLOCK_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Extended data\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `EXTDATA` reader - Extended data"]
pub struct EXTDATA_R(crate::FieldReader<u8, EXTDATA_A>);
impl EXTDATA_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTDATA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTDATA_A {
        match self.bits {
            0 => EXTDATA_A::VALUE1,
            1 => EXTDATA_A::VALUE2,
            2 => EXTDATA_A::VALUE3,
            3 => EXTDATA_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EXTDATA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EXTDATA_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == EXTDATA_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == EXTDATA_A::VALUE4
    }
}
impl core::ops::Deref for EXTDATA_R {
    type Target = crate::FieldReader<u8, EXTDATA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTDATA` writer - Extended data"]
pub struct EXTDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTDATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTDATA_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "external memory outputs data every BFCLK cycle"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXTDATA_A::VALUE1)
    }
    #[doc = "external memory outputs data every two BFCLK cycles"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXTDATA_A::VALUE2)
    }
    #[doc = "external memory outputs data every four BFCLK cycles"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EXTDATA_A::VALUE3)
    }
    #[doc = "external memory outputs data every eight BFCLK cycles"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EXTDATA_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Command Delay Cycles\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `CMDDELAY` reader - Command Delay Cycles"]
pub struct CMDDELAY_R(crate::FieldReader<u8, CMDDELAY_A>);
impl CMDDELAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMDDELAY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMDDELAY_A> {
        match self.bits {
            0 => Some(CMDDELAY_A::VALUE1),
            1 => Some(CMDDELAY_A::VALUE2),
            14 => Some(CMDDELAY_A::VALUE3),
            15 => Some(CMDDELAY_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CMDDELAY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CMDDELAY_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == CMDDELAY_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == CMDDELAY_A::VALUE4
    }
}
impl core::ops::Deref for CMDDELAY_R {
    type Target = crate::FieldReader<u8, CMDDELAY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDDELAY` writer - Command Delay Cycles"]
pub struct CMDDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDDELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDDELAY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0 clock cycle selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMDDELAY_A::VALUE1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMDDELAY_A::VALUE2)
    }
    #[doc = "14 clock cycles selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CMDDELAY_A::VALUE3)
    }
    #[doc = "15 clock cycles selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CMDDELAY_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Address Hold Cycles\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `AHOLDC` reader - Address Hold Cycles"]
pub struct AHOLDC_R(crate::FieldReader<u8, AHOLDC_A>);
impl AHOLDC_R {
    pub(crate) fn new(bits: u8) -> Self {
        AHOLDC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AHOLDC_A> {
        match self.bits {
            0 => Some(AHOLDC_A::VALUE1),
            1 => Some(AHOLDC_A::VALUE2),
            14 => Some(AHOLDC_A::VALUE3),
            15 => Some(AHOLDC_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == AHOLDC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == AHOLDC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == AHOLDC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == AHOLDC_A::VALUE4
    }
}
impl core::ops::Deref for AHOLDC_R {
    type Target = crate::FieldReader<u8, AHOLDC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHOLDC` writer - Address Hold Cycles"]
pub struct AHOLDC_W<'a> {
    w: &'a mut W,
}
impl<'a> AHOLDC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHOLDC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0 clock cycle selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHOLDC_A::VALUE1)
    }
    #[doc = "1 clock cycle selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AHOLDC_A::VALUE2)
    }
    #[doc = "14 clock cycles selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(AHOLDC_A::VALUE3)
    }
    #[doc = "15 clock cycles selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(AHOLDC_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Address Cycles\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ADDRC` reader - Address Cycles"]
pub struct ADDRC_R(crate::FieldReader<u8, ADDRC_A>);
impl ADDRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADDRC_A> {
        match self.bits {
            0 => Some(ADDRC_A::VALUE1),
            1 => Some(ADDRC_A::VALUE2),
            14 => Some(ADDRC_A::VALUE3),
            15 => Some(ADDRC_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ADDRC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ADDRC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == ADDRC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == ADDRC_A::VALUE4
    }
}
impl core::ops::Deref for ADDRC_R {
    type Target = crate::FieldReader<u8, ADDRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRC` writer - Address Cycles"]
pub struct ADDRC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 clock cycle selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ADDRC_A::VALUE1)
    }
    #[doc = "1 clock cycle selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ADDRC_A::VALUE2)
    }
    #[doc = "14 clock cycles selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ADDRC_A::VALUE3)
    }
    #[doc = "15 clock cycles selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ADDRC_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
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
        WRRECOVC_R::new(((self.bits >> 4) & 0x07) as u8)
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
        EXTCLOCK_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Extended data"]
    #[inline(always)]
    pub fn extdata(&self) -> EXTDATA_R {
        EXTDATA_R::new(((self.bits >> 18) & 0x03) as u8)
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
    pub fn wrdtacs(&mut self) -> WRDTACS_W {
        WRDTACS_W { w: self }
    }
    #[doc = "Bits 4:6 - Recovery Cycles after Write Accesses"]
    #[inline(always)]
    pub fn wrrecovc(&mut self) -> WRRECOVC_W {
        WRRECOVC_W { w: self }
    }
    #[doc = "Bits 7:11 - Programmed Wait States for write accesses"]
    #[inline(always)]
    pub fn waitwrc(&mut self) -> WAITWRC_W {
        WAITWRC_W { w: self }
    }
    #[doc = "Bits 12:15 - Data Hold Cycles for Write Accesses"]
    #[inline(always)]
    pub fn datac(&mut self) -> DATAC_W {
        DATAC_W { w: self }
    }
    #[doc = "Bits 16:17 - Frequency of external clock at pin BFCLKO"]
    #[inline(always)]
    pub fn extclock(&mut self) -> EXTCLOCK_W {
        EXTCLOCK_W { w: self }
    }
    #[doc = "Bits 18:19 - Extended data"]
    #[inline(always)]
    pub fn extdata(&mut self) -> EXTDATA_W {
        EXTDATA_W { w: self }
    }
    #[doc = "Bits 20:23 - Command Delay Cycles"]
    #[inline(always)]
    pub fn cmddelay(&mut self) -> CMDDELAY_W {
        CMDDELAY_W { w: self }
    }
    #[doc = "Bits 24:27 - Address Hold Cycles"]
    #[inline(always)]
    pub fn aholdc(&mut self) -> AHOLDC_W {
        AHOLDC_W { w: self }
    }
    #[doc = "Bits 28:31 - Address Cycles"]
    #[inline(always)]
    pub fn addrc(&mut self) -> ADDRC_W {
        ADDRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBU Bus Write Access Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buswap3](index.html) module"]
pub struct BUSWAP3_SPEC;
impl crate::RegisterSpec for BUSWAP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buswap3::R](R) reader structure"]
impl crate::Readable for BUSWAP3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buswap3::W](W) writer structure"]
impl crate::Writable for BUSWAP3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUSWAP3 to value 0xffff_ffff"]
impl crate::Resettable for BUSWAP3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
