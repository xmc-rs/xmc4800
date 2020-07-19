#[doc = "Reader of register BUSRAP0"]
pub type R = crate::R<u32, super::BUSRAP0>;
#[doc = "Writer for register BUSRAP0"]
pub type W = crate::W<u32, super::BUSRAP0>;
#[doc = "Register BUSRAP0 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::BUSRAP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Recovery Cycles between Different Regions\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `RDDTACS`"]
pub type RDDTACS_R = crate::R<u8, RDDTACS_A>;
impl RDDTACS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RDDTACS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RDDTACS_A::VALUE1),
            1 => Val(RDDTACS_A::VALUE2),
            14 => Val(RDDTACS_A::VALUE3),
            15 => Val(RDDTACS_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RDDTACS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RDDTACS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RDDTACS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RDDTACS_A::VALUE4
    }
}
#[doc = "Write proxy for field `RDDTACS`"]
pub struct RDDTACS_W<'a> {
    w: &'a mut W,
}
impl<'a> RDDTACS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDDTACS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RDDTACS_A::VALUE1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RDDTACS_A::VALUE2)
    }
    #[doc = "14 clock cycles selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(RDDTACS_A::VALUE3)
    }
    #[doc = "15 clock cycles selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(RDDTACS_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Recovery Cycles after Read Accesses\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `RDRECOVC`"]
pub type RDRECOVC_R = crate::R<u8, RDRECOVC_A>;
impl RDRECOVC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RDRECOVC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RDRECOVC_A::VALUE1),
            1 => Val(RDRECOVC_A::VALUE2),
            6 => Val(RDRECOVC_A::VALUE3),
            7 => Val(RDRECOVC_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RDRECOVC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RDRECOVC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RDRECOVC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RDRECOVC_A::VALUE4
    }
}
#[doc = "Write proxy for field `RDRECOVC`"]
pub struct RDRECOVC_W<'a> {
    w: &'a mut W,
}
impl<'a> RDRECOVC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDRECOVC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RDRECOVC_A::VALUE1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RDRECOVC_A::VALUE2)
    }
    #[doc = "6 clock cycles selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(RDRECOVC_A::VALUE3)
    }
    #[doc = "7 clock cycles selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(RDRECOVC_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Programmed Wait States for read accesses\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `WAITRDC`"]
pub type WAITRDC_R = crate::R<u8, WAITRDC_A>;
impl WAITRDC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WAITRDC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WAITRDC_A::VALUE1),
            1 => Val(WAITRDC_A::VALUE2),
            2 => Val(WAITRDC_A::VALUE3),
            30 => Val(WAITRDC_A::VALUE4),
            31 => Val(WAITRDC_A::VALUE5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAITRDC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAITRDC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == WAITRDC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == WAITRDC_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == WAITRDC_A::VALUE5
    }
}
#[doc = "Write proxy for field `WAITRDC`"]
pub struct WAITRDC_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITRDC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAITRDC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 wait state."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAITRDC_A::VALUE1)
    }
    #[doc = "1 wait states."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAITRDC_A::VALUE2)
    }
    #[doc = "2 wait state."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(WAITRDC_A::VALUE3)
    }
    #[doc = "30 wait states."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(WAITRDC_A::VALUE4)
    }
    #[doc = "31 wait states."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(WAITRDC_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 7)) | (((value as u32) & 0x1f) << 7);
        self.w
    }
}
#[doc = "Reader of field `DATAC`"]
pub type DATAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATAC`"]
pub struct DATAC_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
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
#[doc = "Reader of field `EXTCLOCK`"]
pub type EXTCLOCK_R = crate::R<u8, EXTCLOCK_A>;
impl EXTCLOCK_R {
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
        *self == EXTCLOCK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EXTCLOCK_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EXTCLOCK_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EXTCLOCK_A::VALUE4
    }
}
#[doc = "Write proxy for field `EXTCLOCK`"]
pub struct EXTCLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTCLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTCLOCK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
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
#[doc = "Reader of field `EXTDATA`"]
pub type EXTDATA_R = crate::R<u8, EXTDATA_A>;
impl EXTDATA_R {
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
        *self == EXTDATA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EXTDATA_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EXTDATA_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EXTDATA_A::VALUE4
    }
}
#[doc = "Write proxy for field `EXTDATA`"]
pub struct EXTDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTDATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTDATA_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
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
#[doc = "Reader of field `CMDDELAY`"]
pub type CMDDELAY_R = crate::R<u8, CMDDELAY_A>;
impl CMDDELAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CMDDELAY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CMDDELAY_A::VALUE1),
            1 => Val(CMDDELAY_A::VALUE2),
            14 => Val(CMDDELAY_A::VALUE3),
            15 => Val(CMDDELAY_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMDDELAY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMDDELAY_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CMDDELAY_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CMDDELAY_A::VALUE4
    }
}
#[doc = "Write proxy for field `CMDDELAY`"]
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
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
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
#[doc = "Reader of field `AHOLDC`"]
pub type AHOLDC_R = crate::R<u8, AHOLDC_A>;
impl AHOLDC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AHOLDC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AHOLDC_A::VALUE1),
            1 => Val(AHOLDC_A::VALUE2),
            14 => Val(AHOLDC_A::VALUE3),
            15 => Val(AHOLDC_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AHOLDC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AHOLDC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == AHOLDC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == AHOLDC_A::VALUE4
    }
}
#[doc = "Write proxy for field `AHOLDC`"]
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
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
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
#[doc = "Reader of field `ADDRC`"]
pub type ADDRC_R = crate::R<u8, ADDRC_A>;
impl ADDRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADDRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADDRC_A::VALUE1),
            1 => Val(ADDRC_A::VALUE2),
            14 => Val(ADDRC_A::VALUE3),
            15 => Val(ADDRC_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ADDRC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ADDRC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ADDRC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ADDRC_A::VALUE4
    }
}
#[doc = "Write proxy for field `ADDRC`"]
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
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
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
        RDRECOVC_R::new(((self.bits >> 4) & 0x07) as u8)
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
    pub fn rddtacs(&mut self) -> RDDTACS_W {
        RDDTACS_W { w: self }
    }
    #[doc = "Bits 4:6 - Recovery Cycles after Read Accesses"]
    #[inline(always)]
    pub fn rdrecovc(&mut self) -> RDRECOVC_W {
        RDRECOVC_W { w: self }
    }
    #[doc = "Bits 7:11 - Programmed Wait States for read accesses"]
    #[inline(always)]
    pub fn waitrdc(&mut self) -> WAITRDC_W {
        WAITRDC_W { w: self }
    }
    #[doc = "Bits 12:15 - Data Hold Cycles for Read Accesses"]
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
}
