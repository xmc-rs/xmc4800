#[doc = "Reader of register MODCON"]
pub type R = crate::R<u32, super::MODCON>;
#[doc = "Writer for register MODCON"]
pub type W = crate::W<u32, super::MODCON>;
#[doc = "Register MODCON `reset()`'s with value 0x20"]
impl crate::ResetValue for super::MODCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Reader of field `STS`"]
pub type STS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LCKABRT`"]
pub type LCKABRT_R = crate::R<bool, bool>;
#[doc = "SDRAM Tristate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDTRI_A {
    #[doc = "0: SDRAM control signals are driven by the EBU when the EBU does not own the external bus. SDRAM cannot be shared."]
    VALUE1,
    #[doc = "1: SDRAM control signals are tri-stated by the EBU when the EBU does not own the external bus. The SDRAM can be shared."]
    VALUE2,
}
impl From<SDTRI_A> for bool {
    #[inline(always)]
    fn from(variant: SDTRI_A) -> Self {
        match variant {
            SDTRI_A::VALUE1 => false,
            SDTRI_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SDTRI`"]
pub type SDTRI_R = crate::R<bool, SDTRI_A>;
impl SDTRI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDTRI_A {
        match self.bits {
            false => SDTRI_A::VALUE1,
            true => SDTRI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDTRI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SDTRI_A::VALUE2
    }
}
#[doc = "Write proxy for field `SDTRI`"]
pub struct SDTRI_W<'a> {
    w: &'a mut W,
}
impl<'a> SDTRI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDTRI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SDRAM control signals are driven by the EBU when the EBU does not own the external bus. SDRAM cannot be shared."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SDTRI_A::VALUE1)
    }
    #[doc = "SDRAM control signals are tri-stated by the EBU when the EBU does not own the external bus. The SDRAM can be shared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SDTRI_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "External Bus Lock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTLOCK_A {
    #[doc = "0: External bus is not locked after the EBU gains ownership"]
    VALUE1,
    #[doc = "1: External bus is locked after the EBU gains ownership"]
    VALUE2,
}
impl From<EXTLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: EXTLOCK_A) -> Self {
        match variant {
            EXTLOCK_A::VALUE1 => false,
            EXTLOCK_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `EXTLOCK`"]
pub type EXTLOCK_R = crate::R<bool, EXTLOCK_A>;
impl EXTLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTLOCK_A {
        match self.bits {
            false => EXTLOCK_A::VALUE1,
            true => EXTLOCK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EXTLOCK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EXTLOCK_A::VALUE2
    }
}
#[doc = "Write proxy for field `EXTLOCK`"]
pub struct EXTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTLOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External bus is not locked after the EBU gains ownership"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXTLOCK_A::VALUE1)
    }
    #[doc = "External bus is locked after the EBU gains ownership"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXTLOCK_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Arbitration Signal Synchronization Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBSYNC_A {
    #[doc = "0: Arbitration inputs are synchronous"]
    VALUE1,
    #[doc = "1: Arbitration inputs are asynchronous"]
    VALUE2,
}
impl From<ARBSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: ARBSYNC_A) -> Self {
        match variant {
            ARBSYNC_A::VALUE1 => false,
            ARBSYNC_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ARBSYNC`"]
pub type ARBSYNC_R = crate::R<bool, ARBSYNC_A>;
impl ARBSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBSYNC_A {
        match self.bits {
            false => ARBSYNC_A::VALUE1,
            true => ARBSYNC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ARBSYNC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ARBSYNC_A::VALUE2
    }
}
#[doc = "Write proxy for field `ARBSYNC`"]
pub struct ARBSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARBSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Arbitration inputs are synchronous"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARBSYNC_A::VALUE1)
    }
    #[doc = "Arbitration inputs are asynchronous"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARBSYNC_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Arbitration Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBMODE_A {
    #[doc = "0: No Bus arbitration mode selected"]
    VALUE1,
    #[doc = "1: Arbiter Mode arbitration mode selected"]
    VALUE2,
    #[doc = "2: Participant arbitration mode selected"]
    VALUE3,
    #[doc = "3: Sole Master arbitration mode selected"]
    VALUE4,
}
impl From<ARBMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ARBMODE_A) -> Self {
        match variant {
            ARBMODE_A::VALUE1 => 0,
            ARBMODE_A::VALUE2 => 1,
            ARBMODE_A::VALUE3 => 2,
            ARBMODE_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `ARBMODE`"]
pub type ARBMODE_R = crate::R<u8, ARBMODE_A>;
impl ARBMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBMODE_A {
        match self.bits {
            0 => ARBMODE_A::VALUE1,
            1 => ARBMODE_A::VALUE2,
            2 => ARBMODE_A::VALUE3,
            3 => ARBMODE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ARBMODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ARBMODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ARBMODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ARBMODE_A::VALUE4
    }
}
#[doc = "Write proxy for field `ARBMODE`"]
pub struct ARBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARBMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No Bus arbitration mode selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARBMODE_A::VALUE1)
    }
    #[doc = "Arbiter Mode arbitration mode selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARBMODE_A::VALUE2)
    }
    #[doc = "Participant arbitration mode selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ARBMODE_A::VALUE3)
    }
    #[doc = "Sole Master arbitration mode selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ARBMODE_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Bus Time-out Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUTC_A {
    #[doc = "0: Time-out is disabled."]
    VALUE1,
    #[doc = "1: Time-out is generated after 1 8 clock cycles."]
    VALUE2,
    #[doc = "255: Time-out is generated after 255 8 clock cycles."]
    VALUE3,
}
impl From<TIMEOUTC_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUTC_A) -> Self {
        match variant {
            TIMEOUTC_A::VALUE1 => 0,
            TIMEOUTC_A::VALUE2 => 1,
            TIMEOUTC_A::VALUE3 => 255,
        }
    }
}
#[doc = "Reader of field `TIMEOUTC`"]
pub type TIMEOUTC_R = crate::R<u8, TIMEOUTC_A>;
impl TIMEOUTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMEOUTC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIMEOUTC_A::VALUE1),
            1 => Val(TIMEOUTC_A::VALUE2),
            255 => Val(TIMEOUTC_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TIMEOUTC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TIMEOUTC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TIMEOUTC_A::VALUE3
    }
}
#[doc = "Write proxy for field `TIMEOUTC`"]
pub struct TIMEOUTC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEOUTC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Time-out is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TIMEOUTC_A::VALUE1)
    }
    #[doc = "Time-out is generated after 1 8 clock cycles."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TIMEOUTC_A::VALUE2)
    }
    #[doc = "Time-out is generated after 255 8 clock cycles."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TIMEOUTC_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `LOCKTIMEOUT`"]
pub type LOCKTIMEOUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LOCKTIMEOUT`"]
pub struct LOCKTIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKTIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `GLOBALCS`"]
pub type GLOBALCS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GLOBALCS`"]
pub struct GLOBALCS_W<'a> {
    w: &'a mut W,
}
impl<'a> GLOBALCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `ACCSINH`"]
pub type ACCSINH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACCSINH`"]
pub struct ACCSINH_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCSINH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `ACCSINHACK`"]
pub type ACCSINHACK_R = crate::R<bool, bool>;
#[doc = "ALE Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALE_A {
    #[doc = "0: Output is ADV"]
    VALUE1,
    #[doc = "1: Output is ALE"]
    VALUE2,
}
impl From<ALE_A> for bool {
    #[inline(always)]
    fn from(variant: ALE_A) -> Self {
        match variant {
            ALE_A::VALUE1 => false,
            ALE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ALE`"]
pub type ALE_R = crate::R<bool, ALE_A>;
impl ALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALE_A {
        match self.bits {
            false => ALE_A::VALUE1,
            true => ALE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALE_A::VALUE2
    }
}
#[doc = "Write proxy for field `ALE`"]
pub struct ALE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output is ADV"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALE_A::VALUE1)
    }
    #[doc = "Output is ALE"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALE_A::VALUE2)
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
    #[doc = "Bit 0 - Memory Status Bit"]
    #[inline(always)]
    pub fn sts(&self) -> STS_R {
        STS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Lock Abort"]
    #[inline(always)]
    pub fn lckabrt(&self) -> LCKABRT_R {
        LCKABRT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SDRAM Tristate"]
    #[inline(always)]
    pub fn sdtri(&self) -> SDTRI_R {
        SDTRI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External Bus Lock Control"]
    #[inline(always)]
    pub fn extlock(&self) -> EXTLOCK_R {
        EXTLOCK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Arbitration Signal Synchronization Control"]
    #[inline(always)]
    pub fn arbsync(&self) -> ARBSYNC_R {
        ARBSYNC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Arbitration Mode Selection"]
    #[inline(always)]
    pub fn arbmode(&self) -> ARBMODE_R {
        ARBMODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - Bus Time-out Control"]
    #[inline(always)]
    pub fn timeoutc(&self) -> TIMEOUTC_R {
        TIMEOUTC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Lock Timeout Counter Preload"]
    #[inline(always)]
    pub fn locktimeout(&self) -> LOCKTIMEOUT_R {
        LOCKTIMEOUT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Global Chip Select Enable"]
    #[inline(always)]
    pub fn globalcs(&self) -> GLOBALCS_R {
        GLOBALCS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Access Inhibit request"]
    #[inline(always)]
    pub fn accsinh(&self) -> ACCSINH_R {
        ACCSINH_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Access inhibit acknowledge"]
    #[inline(always)]
    pub fn accsinhack(&self) -> ACCSINHACK_R {
        ACCSINHACK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ALE Mode"]
    #[inline(always)]
    pub fn ale(&self) -> ALE_R {
        ALE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SDRAM Tristate"]
    #[inline(always)]
    pub fn sdtri(&mut self) -> SDTRI_W {
        SDTRI_W { w: self }
    }
    #[doc = "Bit 4 - External Bus Lock Control"]
    #[inline(always)]
    pub fn extlock(&mut self) -> EXTLOCK_W {
        EXTLOCK_W { w: self }
    }
    #[doc = "Bit 5 - Arbitration Signal Synchronization Control"]
    #[inline(always)]
    pub fn arbsync(&mut self) -> ARBSYNC_W {
        ARBSYNC_W { w: self }
    }
    #[doc = "Bits 6:7 - Arbitration Mode Selection"]
    #[inline(always)]
    pub fn arbmode(&mut self) -> ARBMODE_W {
        ARBMODE_W { w: self }
    }
    #[doc = "Bits 8:15 - Bus Time-out Control"]
    #[inline(always)]
    pub fn timeoutc(&mut self) -> TIMEOUTC_W {
        TIMEOUTC_W { w: self }
    }
    #[doc = "Bits 16:23 - Lock Timeout Counter Preload"]
    #[inline(always)]
    pub fn locktimeout(&mut self) -> LOCKTIMEOUT_W {
        LOCKTIMEOUT_W { w: self }
    }
    #[doc = "Bits 24:27 - Global Chip Select Enable"]
    #[inline(always)]
    pub fn globalcs(&mut self) -> GLOBALCS_W {
        GLOBALCS_W { w: self }
    }
    #[doc = "Bit 28 - Access Inhibit request"]
    #[inline(always)]
    pub fn accsinh(&mut self) -> ACCSINH_W {
        ACCSINH_W { w: self }
    }
    #[doc = "Bit 31 - ALE Mode"]
    #[inline(always)]
    pub fn ale(&mut self) -> ALE_W {
        ALE_W { w: self }
    }
}
