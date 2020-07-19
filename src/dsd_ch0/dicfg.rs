#[doc = "Reader of register DICFG"]
pub type R = crate::R<u32, super::DICFG>;
#[doc = "Writer for register DICFG"]
pub type W = crate::W<u32, super::DICFG>;
#[doc = "Register DICFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DICFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Input Data Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `DSRC`"]
pub type DSRC_R = crate::R<u8, DSRC_A>;
impl DSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DSRC_A::VALUE1),
            2 => Val(DSRC_A::VALUE2),
            3 => Val(DSRC_A::VALUE3),
            4 => Val(DSRC_A::VALUE4),
            5 => Val(DSRC_A::VALUE5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DSRC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DSRC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DSRC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DSRC_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == DSRC_A::VALUE5
    }
}
#[doc = "Write proxy for field `DSRC`"]
pub struct DSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disconnected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DSRC_A::VALUE1)
    }
    #[doc = "External, from input A, direct"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DSRC_A::VALUE2)
    }
    #[doc = "External, from input A, inverted"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DSRC_A::VALUE3)
    }
    #[doc = "External, from input B, direct"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(DSRC_A::VALUE4)
    }
    #[doc = "External, from input B, inverted"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(DSRC_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Write Control for Data Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSWC_AW {
    #[doc = "0: No write access to data parameters"]
    VALUE1 = 0,
    #[doc = "1: Bitfield DSRC can be written"]
    VALUE2 = 1,
}
impl From<DSWC_AW> for bool {
    #[inline(always)]
    fn from(variant: DSWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DSWC`"]
pub struct DSWC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSWC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No write access to data parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DSWC_AW::VALUE1)
    }
    #[doc = "Bitfield DSRC can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DSWC_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Integrator Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `ITRMODE`"]
pub type ITRMODE_R = crate::R<u8, ITRMODE_A>;
impl ITRMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITRMODE_A {
        match self.bits {
            0 => ITRMODE_A::VALUE1,
            1 => ITRMODE_A::VALUE2,
            2 => ITRMODE_A::VALUE3,
            3 => ITRMODE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ITRMODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ITRMODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ITRMODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ITRMODE_A::VALUE4
    }
}
#[doc = "Write proxy for field `ITRMODE`"]
pub struct ITRMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITRMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITRMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No integration trigger, integrator bypassed, INTEN = 0 all the time"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ITRMODE_A::VALUE1)
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ITRMODE_A::VALUE2)
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ITRMODE_A::VALUE3)
    }
    #[doc = "No trigger, integrator active all the time, INTEN = 1 all the time"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ITRMODE_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Timestamp Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `TSTRMODE`"]
pub type TSTRMODE_R = crate::R<u8, TSTRMODE_A>;
impl TSTRMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTRMODE_A {
        match self.bits {
            0 => TSTRMODE_A::VALUE1,
            1 => TSTRMODE_A::VALUE2,
            2 => TSTRMODE_A::VALUE3,
            3 => TSTRMODE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSTRMODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSTRMODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TSTRMODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TSTRMODE_A::VALUE4
    }
}
#[doc = "Write proxy for field `TSTRMODE`"]
pub struct TSTRMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTRMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSTRMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No timestamp trigger"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSTRMODE_A::VALUE1)
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSTRMODE_A::VALUE2)
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TSTRMODE_A::VALUE3)
    }
    #[doc = "Trigger event upon each edge"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TSTRMODE_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `TRSEL`"]
pub type TRSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRSEL`"]
pub struct TRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Write Control for Trigger Parameters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRWC_AW {
    #[doc = "0: No write access to trigger parameters"]
    VALUE1 = 0,
    #[doc = "1: Bitfields TRSEL, TSTRMODE, ITRMODE can be written"]
    VALUE2 = 1,
}
impl From<TRWC_AW> for bool {
    #[inline(always)]
    fn from(variant: TRWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TRWC`"]
pub struct TRWC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRWC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No write access to trigger parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRWC_AW::VALUE1)
    }
    #[doc = "Bitfields TRSEL, TSTRMODE, ITRMODE can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRWC_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Sample Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `CSRC`"]
pub type CSRC_R = crate::R<u8, CSRC_A>;
impl CSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CSRC_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CSRC_A::VALUE2),
            2 => Val(CSRC_A::VALUE3),
            3 => Val(CSRC_A::VALUE4),
            4 => Val(CSRC_A::VALUE5),
            15 => Val(CSRC_A::VALUE6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CSRC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CSRC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CSRC_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == CSRC_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == CSRC_A::VALUE6
    }
}
#[doc = "Write proxy for field `CSRC`"]
pub struct CSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "External, from input A"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CSRC_A::VALUE2)
    }
    #[doc = "External, from input B"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CSRC_A::VALUE3)
    }
    #[doc = "External, from input C"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CSRC_A::VALUE4)
    }
    #[doc = "External, from input D"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(CSRC_A::VALUE5)
    }
    #[doc = "Internal clock"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(CSRC_A::VALUE6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Data Strobe Generatoion Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `STROBE`"]
pub type STROBE_R = crate::R<u8, STROBE_A>;
impl STROBE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STROBE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STROBE_A::VALUE1),
            1 => Val(STROBE_A::VALUE2),
            2 => Val(STROBE_A::VALUE3),
            3 => Val(STROBE_A::VALUE4),
            5 => Val(STROBE_A::VALUE6),
            6 => Val(STROBE_A::VALUE7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STROBE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STROBE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STROBE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STROBE_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == STROBE_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == STROBE_A::VALUE7
    }
}
#[doc = "Write proxy for field `STROBE`"]
pub struct STROBE_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STROBE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No data strobe"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STROBE_A::VALUE1)
    }
    #[doc = "Direct clock, a sample trigger is generated at each rising clock edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STROBE_A::VALUE2)
    }
    #[doc = "Direct clock, a sample trigger is generated at each falling clock edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(STROBE_A::VALUE3)
    }
    #[doc = "Double data, a sample trigger is generated at each rising and falling clock edge"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(STROBE_A::VALUE4)
    }
    #[doc = "Double clock, a sample trigger is generated at every 2nd rising clock edge"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(STROBE_A::VALUE6)
    }
    #[doc = "Double clock, a sample trigger is generated at every 2nd falling clock edge"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(STROBE_A::VALUE7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Write Control for Strobe/Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCWC_AW {
    #[doc = "0: No write access to strobe/clock parameters"]
    VALUE1 = 0,
    #[doc = "1: Bitfields STROBE, CSRC can be written"]
    VALUE2 = 1,
}
impl From<SCWC_AW> for bool {
    #[inline(always)]
    fn from(variant: SCWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SCWC`"]
pub struct SCWC_W<'a> {
    w: &'a mut W,
}
impl<'a> SCWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCWC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No write access to strobe/clock parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCWC_AW::VALUE1)
    }
    #[doc = "Bitfields STROBE, CSRC can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCWC_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
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
        ITRMODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Timestamp Trigger Mode"]
    #[inline(always)]
    pub fn tstrmode(&self) -> TSTRMODE_R {
        TSTRMODE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:14 - Trigger Select"]
    #[inline(always)]
    pub fn trsel(&self) -> TRSEL_R {
        TRSEL_R::new(((self.bits >> 12) & 0x07) as u8)
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
    pub fn dsrc(&mut self) -> DSRC_W {
        DSRC_W { w: self }
    }
    #[doc = "Bit 7 - Write Control for Data Selection"]
    #[inline(always)]
    pub fn dswc(&mut self) -> DSWC_W {
        DSWC_W { w: self }
    }
    #[doc = "Bits 8:9 - Integrator Trigger Mode"]
    #[inline(always)]
    pub fn itrmode(&mut self) -> ITRMODE_W {
        ITRMODE_W { w: self }
    }
    #[doc = "Bits 10:11 - Timestamp Trigger Mode"]
    #[inline(always)]
    pub fn tstrmode(&mut self) -> TSTRMODE_W {
        TSTRMODE_W { w: self }
    }
    #[doc = "Bits 12:14 - Trigger Select"]
    #[inline(always)]
    pub fn trsel(&mut self) -> TRSEL_W {
        TRSEL_W { w: self }
    }
    #[doc = "Bit 15 - Write Control for Trigger Parameters"]
    #[inline(always)]
    pub fn trwc(&mut self) -> TRWC_W {
        TRWC_W { w: self }
    }
    #[doc = "Bits 16:19 - Sample Clock Source Select"]
    #[inline(always)]
    pub fn csrc(&mut self) -> CSRC_W {
        CSRC_W { w: self }
    }
    #[doc = "Bits 20:23 - Data Strobe Generatoion Mode"]
    #[inline(always)]
    pub fn strobe(&mut self) -> STROBE_W {
        STROBE_W { w: self }
    }
    #[doc = "Bit 31 - Write Control for Strobe/Clock Selection"]
    #[inline(always)]
    pub fn scwc(&mut self) -> SCWC_W {
        SCWC_W { w: self }
    }
}
