#[doc = "Reader of register BUSWCON0"]
pub type R = crate::R<u32, super::BUSWCON0>;
#[doc = "Writer for register BUSWCON0"]
pub type W = crate::W<u32, super::BUSWCON0>;
#[doc = "Register BUSWCON0 `reset()`'s with value 0x00d3_0000"]
impl crate::ResetValue for super::BUSWCON0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00d3_0000
    }
}
#[doc = "Burst Length for Synchronous Burst\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FETBLEN_A {
    #[doc = "0: 1 data access (default after reset)."]
    VALUE1,
    #[doc = "1: 2 data accesses."]
    VALUE2,
    #[doc = "2: 4 data accesses."]
    VALUE3,
    #[doc = "3: 8 data accesses."]
    VALUE4,
}
impl From<FETBLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: FETBLEN_A) -> Self {
        match variant {
            FETBLEN_A::VALUE1 => 0,
            FETBLEN_A::VALUE2 => 1,
            FETBLEN_A::VALUE3 => 2,
            FETBLEN_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `FETBLEN`"]
pub type FETBLEN_R = crate::R<u8, FETBLEN_A>;
impl FETBLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FETBLEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FETBLEN_A::VALUE1),
            1 => Val(FETBLEN_A::VALUE2),
            2 => Val(FETBLEN_A::VALUE3),
            3 => Val(FETBLEN_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FETBLEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FETBLEN_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == FETBLEN_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == FETBLEN_A::VALUE4
    }
}
#[doc = "Write proxy for field `FETBLEN`"]
pub struct FETBLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FETBLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FETBLEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 data access (default after reset)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FETBLEN_A::VALUE1)
    }
    #[doc = "2 data accesses."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FETBLEN_A::VALUE2)
    }
    #[doc = "4 data accesses."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(FETBLEN_A::VALUE3)
    }
    #[doc = "8 data accesses."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(FETBLEN_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Synchronous burst buffer mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FBBMSEL_A {
    #[doc = "0: Burst buffer length defined by value in FETBLEN (default after reset)."]
    VALUE1,
    #[doc = "1: Continuous mode. All data required for transaction transferred in single burst"]
    VALUE2,
}
impl From<FBBMSEL_A> for bool {
    #[inline(always)]
    fn from(variant: FBBMSEL_A) -> Self {
        match variant {
            FBBMSEL_A::VALUE1 => false,
            FBBMSEL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `FBBMSEL`"]
pub type FBBMSEL_R = crate::R<bool, FBBMSEL_A>;
impl FBBMSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FBBMSEL_A {
        match self.bits {
            false => FBBMSEL_A::VALUE1,
            true => FBBMSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FBBMSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FBBMSEL_A::VALUE2
    }
}
#[doc = "Write proxy for field `FBBMSEL`"]
pub struct FBBMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FBBMSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FBBMSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Burst buffer length defined by value in FETBLEN (default after reset)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FBBMSEL_A::VALUE1)
    }
    #[doc = "Continuous mode. All data required for transaction transferred in single burst"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FBBMSEL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `NAA`"]
pub type NAA_R = crate::R<bool, bool>;
#[doc = "Early Chip Select for Synchronous Burst\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECSE_A {
    #[doc = "0: CS is delayed."]
    VALUE1,
    #[doc = "1: CS is not delayed."]
    VALUE2,
}
impl From<ECSE_A> for bool {
    #[inline(always)]
    fn from(variant: ECSE_A) -> Self {
        match variant {
            ECSE_A::VALUE1 => false,
            ECSE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ECSE`"]
pub type ECSE_R = crate::R<bool, ECSE_A>;
impl ECSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECSE_A {
        match self.bits {
            false => ECSE_A::VALUE1,
            true => ECSE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ECSE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ECSE_A::VALUE2
    }
}
#[doc = "Write proxy for field `ECSE`"]
pub struct ECSE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CS is delayed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECSE_A::VALUE1)
    }
    #[doc = "CS is not delayed."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECSE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Early Burst Signal Enable for Synchronous Burst\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBSE_A {
    #[doc = "0: ADV is delayed."]
    VALUE1,
    #[doc = "1: ADV is not delayed."]
    VALUE2,
}
impl From<EBSE_A> for bool {
    #[inline(always)]
    fn from(variant: EBSE_A) -> Self {
        match variant {
            EBSE_A::VALUE1 => false,
            EBSE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `EBSE`"]
pub type EBSE_R = crate::R<bool, EBSE_A>;
impl EBSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EBSE_A {
        match self.bits {
            false => EBSE_A::VALUE1,
            true => EBSE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EBSE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EBSE_A::VALUE2
    }
}
#[doc = "Write proxy for field `EBSE`"]
pub struct EBSE_W<'a> {
    w: &'a mut W,
}
impl<'a> EBSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EBSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADV is delayed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EBSE_A::VALUE1)
    }
    #[doc = "ADV is not delayed."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EBSE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reversed polarity at WAIT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITINV_A {
    #[doc = "0: input at WAIT pin is active low (default after reset)."]
    VALUE1,
    #[doc = "1: input at WAIT pin is active high."]
    VALUE2,
}
impl From<WAITINV_A> for bool {
    #[inline(always)]
    fn from(variant: WAITINV_A) -> Self {
        match variant {
            WAITINV_A::VALUE1 => false,
            WAITINV_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `WAITINV`"]
pub type WAITINV_R = crate::R<bool, WAITINV_A>;
impl WAITINV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITINV_A {
        match self.bits {
            false => WAITINV_A::VALUE1,
            true => WAITINV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAITINV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAITINV_A::VALUE2
    }
}
#[doc = "Write proxy for field `WAITINV`"]
pub struct WAITINV_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAITINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "input at WAIT pin is active low (default after reset)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAITINV_A::VALUE1)
    }
    #[doc = "input at WAIT pin is active high."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAITINV_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Byte Control Signal Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCGEN_A {
    #[doc = "0: Byte control signals follow chip select timing."]
    VALUE1,
    #[doc = "1: Byte control signals follow control signal timing (RD, RD/WR) (default after reset)."]
    VALUE2,
    #[doc = "2: Byte control signals follow write enable signal timing (RD/WR only)."]
    VALUE3,
}
impl From<BCGEN_A> for u8 {
    #[inline(always)]
    fn from(variant: BCGEN_A) -> Self {
        match variant {
            BCGEN_A::VALUE1 => 0,
            BCGEN_A::VALUE2 => 1,
            BCGEN_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `BCGEN`"]
pub type BCGEN_R = crate::R<u8, BCGEN_A>;
impl BCGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BCGEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BCGEN_A::VALUE1),
            1 => Val(BCGEN_A::VALUE2),
            2 => Val(BCGEN_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BCGEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BCGEN_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BCGEN_A::VALUE3
    }
}
#[doc = "Write proxy for field `BCGEN`"]
pub struct BCGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BCGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCGEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Byte control signals follow chip select timing."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BCGEN_A::VALUE1)
    }
    #[doc = "Byte control signals follow control signal timing (RD, RD/WR) (default after reset)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BCGEN_A::VALUE2)
    }
    #[doc = "Byte control signals follow write enable signal timing (RD/WR only)."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BCGEN_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `PORTW`"]
pub type PORTW_R = crate::R<u8, u8>;
#[doc = "Reader of field `WAIT`"]
pub type WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAIT`"]
pub struct WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Asynchronous Address phase:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AAP_A {
    #[doc = "0: Clock is enabled at beginning of access."]
    VALUE1,
    #[doc = "1: Clock is enabled at after address phase."]
    VALUE2,
}
impl From<AAP_A> for bool {
    #[inline(always)]
    fn from(variant: AAP_A) -> Self {
        match variant {
            AAP_A::VALUE1 => false,
            AAP_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `AAP`"]
pub type AAP_R = crate::R<bool, AAP_A>;
impl AAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AAP_A {
        match self.bits {
            false => AAP_A::VALUE1,
            true => AAP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AAP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AAP_A::VALUE2
    }
}
#[doc = "Write proxy for field `AAP`"]
pub struct AAP_W<'a> {
    w: &'a mut W,
}
impl<'a> AAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AAP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock is enabled at beginning of access."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AAP_A::VALUE1)
    }
    #[doc = "Clock is enabled at after address phase."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AAP_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Lock Chip Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKCS_A {
    #[doc = "0: Chip Select cannot be locked (default after reset)."]
    VALUE1,
    #[doc = "1: Chip Select will be automatically locked when written to from the processor data port."]
    VALUE2,
}
impl From<LOCKCS_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKCS_A) -> Self {
        match variant {
            LOCKCS_A::VALUE1 => false,
            LOCKCS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `LOCKCS`"]
pub type LOCKCS_R = crate::R<bool, LOCKCS_A>;
impl LOCKCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKCS_A {
        match self.bits {
            false => LOCKCS_A::VALUE1,
            true => LOCKCS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LOCKCS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LOCKCS_A::VALUE2
    }
}
#[doc = "Write proxy for field `LOCKCS`"]
pub struct LOCKCS_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKCS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Chip Select cannot be locked (default after reset)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOCKCS_A::VALUE1)
    }
    #[doc = "Chip Select will be automatically locked when written to from the processor data port."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LOCKCS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `AGEN`"]
pub type AGEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AGEN`"]
pub struct AGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AGEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Burst Length for Synchronous Burst"]
    #[inline(always)]
    pub fn fetblen(&self) -> FETBLEN_R {
        FETBLEN_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Synchronous burst buffer mode select"]
    #[inline(always)]
    pub fn fbbmsel(&self) -> FBBMSEL_R {
        FBBMSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable flash non-array access workaround"]
    #[inline(always)]
    pub fn naa(&self) -> NAA_R {
        NAA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Early Chip Select for Synchronous Burst"]
    #[inline(always)]
    pub fn ecse(&self) -> ECSE_R {
        ECSE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Early Burst Signal Enable for Synchronous Burst"]
    #[inline(always)]
    pub fn ebse(&self) -> EBSE_R {
        EBSE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Reversed polarity at WAIT"]
    #[inline(always)]
    pub fn waitinv(&self) -> WAITINV_R {
        WAITINV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Byte Control Signal Control"]
    #[inline(always)]
    pub fn bcgen(&self) -> BCGEN_R {
        BCGEN_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Device Addressing Mode"]
    #[inline(always)]
    pub fn portw(&self) -> PORTW_R {
        PORTW_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - External Wait Control: 0=OFF (default after reset)., 1=Asynchronous input at WAIT., 2=Synchronous input at WAIT., 3=reserved., 0=OFF (default after reset)., 1=Wait for page load (Early WAIT)., 2=Wait for page load (WAIT with data)., 3=Abort and retry access.,"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Asynchronous Address phase:"]
    #[inline(always)]
    pub fn aap(&self) -> AAP_R {
        AAP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Lock Chip Select"]
    #[inline(always)]
    pub fn lockcs(&self) -> LOCKCS_R {
        LOCKCS_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - Device Type for Region"]
    #[inline(always)]
    pub fn agen(&self) -> AGEN_R {
        AGEN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Burst Length for Synchronous Burst"]
    #[inline(always)]
    pub fn fetblen(&mut self) -> FETBLEN_W {
        FETBLEN_W { w: self }
    }
    #[doc = "Bit 3 - Synchronous burst buffer mode select"]
    #[inline(always)]
    pub fn fbbmsel(&mut self) -> FBBMSEL_W {
        FBBMSEL_W { w: self }
    }
    #[doc = "Bit 16 - Early Chip Select for Synchronous Burst"]
    #[inline(always)]
    pub fn ecse(&mut self) -> ECSE_W {
        ECSE_W { w: self }
    }
    #[doc = "Bit 17 - Early Burst Signal Enable for Synchronous Burst"]
    #[inline(always)]
    pub fn ebse(&mut self) -> EBSE_W {
        EBSE_W { w: self }
    }
    #[doc = "Bit 19 - Reversed polarity at WAIT"]
    #[inline(always)]
    pub fn waitinv(&mut self) -> WAITINV_W {
        WAITINV_W { w: self }
    }
    #[doc = "Bits 20:21 - Byte Control Signal Control"]
    #[inline(always)]
    pub fn bcgen(&mut self) -> BCGEN_W {
        BCGEN_W { w: self }
    }
    #[doc = "Bits 24:25 - External Wait Control: 0=OFF (default after reset)., 1=Asynchronous input at WAIT., 2=Synchronous input at WAIT., 3=reserved., 0=OFF (default after reset)., 1=Wait for page load (Early WAIT)., 2=Wait for page load (WAIT with data)., 3=Abort and retry access.,"]
    #[inline(always)]
    pub fn wait(&mut self) -> WAIT_W {
        WAIT_W { w: self }
    }
    #[doc = "Bit 26 - Asynchronous Address phase:"]
    #[inline(always)]
    pub fn aap(&mut self) -> AAP_W {
        AAP_W { w: self }
    }
    #[doc = "Bit 27 - Lock Chip Select"]
    #[inline(always)]
    pub fn lockcs(&mut self) -> LOCKCS_W {
        LOCKCS_W { w: self }
    }
    #[doc = "Bits 28:31 - Device Type for Region"]
    #[inline(always)]
    pub fn agen(&mut self) -> AGEN_W {
        AGEN_W { w: self }
    }
}
