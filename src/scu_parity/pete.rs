#[doc = "Reader of register PETE"]
pub type R = crate::R<u32, super::PETE>;
#[doc = "Writer for register PETE"]
pub type W = crate::W<u32, super::PETE>;
#[doc = "Register PETE `reset()`'s with value 0"]
impl crate::ResetValue for super::PETE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Parity Error Trap Enable for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEPS_A {
    #[doc = "0: Disabled"]
    VALUE1,
    #[doc = "1: Enabled"]
    VALUE2,
}
impl From<PETEPS_A> for bool {
    #[inline(always)]
    fn from(variant: PETEPS_A) -> Self {
        match variant {
            PETEPS_A::VALUE1 => false,
            PETEPS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PETEPS`"]
pub type PETEPS_R = crate::R<bool, PETEPS_A>;
impl PETEPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEPS_A {
        match self.bits {
            false => PETEPS_A::VALUE1,
            true => PETEPS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEPS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEPS_A::VALUE2
    }
}
#[doc = "Write proxy for field `PETEPS`"]
pub struct PETEPS_W<'a> {
    w: &'a mut W,
}
impl<'a> PETEPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETEPS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEPS_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEPS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Parity Error Trap Enable for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEDS1_A {
    #[doc = "0: Disabled"]
    VALUE1,
    #[doc = "1: Enabled"]
    VALUE2,
}
impl From<PETEDS1_A> for bool {
    #[inline(always)]
    fn from(variant: PETEDS1_A) -> Self {
        match variant {
            PETEDS1_A::VALUE1 => false,
            PETEDS1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PETEDS1`"]
pub type PETEDS1_R = crate::R<bool, PETEDS1_A>;
impl PETEDS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEDS1_A {
        match self.bits {
            false => PETEDS1_A::VALUE1,
            true => PETEDS1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEDS1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEDS1_A::VALUE2
    }
}
#[doc = "Write proxy for field `PETEDS1`"]
pub struct PETEDS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PETEDS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETEDS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEDS1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEDS1_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Parity Error Trap Enable for DSRAM2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEDS2_A {
    #[doc = "0: Disabled"]
    VALUE1,
    #[doc = "1: Enabled"]
    VALUE2,
}
impl From<PETEDS2_A> for bool {
    #[inline(always)]
    fn from(variant: PETEDS2_A) -> Self {
        match variant {
            PETEDS2_A::VALUE1 => false,
            PETEDS2_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PETEDS2`"]
pub type PETEDS2_R = crate::R<bool, PETEDS2_A>;
impl PETEDS2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEDS2_A {
        match self.bits {
            false => PETEDS2_A::VALUE1,
            true => PETEDS2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEDS2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEDS2_A::VALUE2
    }
}
#[doc = "Write proxy for field `PETEDS2`"]
pub struct PETEDS2_W<'a> {
    w: &'a mut W,
}
impl<'a> PETEDS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETEDS2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEDS2_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEDS2_A::VALUE2)
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
#[doc = "Parity Error Trap Enable for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEU0_A {
    #[doc = "0: Disabled"]
    VALUE1,
    #[doc = "1: Enabled"]
    VALUE2,
}
impl From<PETEU0_A> for bool {
    #[inline(always)]
    fn from(variant: PETEU0_A) -> Self {
        match variant {
            PETEU0_A::VALUE1 => false,
            PETEU0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PETEU0`"]
pub type PETEU0_R = crate::R<bool, PETEU0_A>;
impl PETEU0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEU0_A {
        match self.bits {
            false => PETEU0_A::VALUE1,
            true => PETEU0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEU0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEU0_A::VALUE2
    }
}
#[doc = "Write proxy for field `PETEU0`"]
pub struct PETEU0_W<'a> {
    w: &'a mut W,
}
impl<'a> PETEU0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETEU0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEU0_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEU0_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Parity Error Trap Enable for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEU1_A {
    #[doc = "0: Disabled"]
    VALUE1,
    #[doc = "1: Enabled"]
    VALUE2,
}
impl From<PETEU1_A> for bool {
    #[inline(always)]
    fn from(variant: PETEU1_A) -> Self {
        match variant {
            PETEU1_A::VALUE1 => false,
            PETEU1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PETEU1`"]
pub type PETEU1_R = crate::R<bool, PETEU1_A>;
impl PETEU1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEU1_A {
        match self.bits {
            false => PETEU1_A::VALUE1,
            true => PETEU1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEU1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEU1_A::VALUE2
    }
}
#[doc = "Write proxy for field `PETEU1`"]
pub struct PETEU1_W<'a> {
    w: &'a mut W,
}
impl<'a> PETEU1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETEU1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEU1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEU1_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Parity Error Trap Enable for USIC2 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEU2_A {
    #[doc = "0: Disabled"]
    VALUE1,
    #[doc = "1: Enabled"]
    VALUE2,
}
impl From<PETEU2_A> for bool {
    #[inline(always)]
    fn from(variant: PETEU2_A) -> Self {
        match variant {
            PETEU2_A::VALUE1 => false,
            PETEU2_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PETEU2`"]
pub type PETEU2_R = crate::R<bool, PETEU2_A>;
impl PETEU2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEU2_A {
        match self.bits {
            false => PETEU2_A::VALUE1,
            true => PETEU2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEU2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEU2_A::VALUE2
    }
}
#[doc = "Write proxy for field `PETEU2`"]
pub struct PETEU2_W<'a> {
    w: &'a mut W,
}
impl<'a> PETEU2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETEU2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEU2_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEU2_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Parity Error Trap Enable for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEMC_A {
    #[doc = "0: Disabled"]
    VALUE1,
    #[doc = "1: Enabled"]
    VALUE2,
}
impl From<PETEMC_A> for bool {
    #[inline(always)]
    fn from(variant: PETEMC_A) -> Self {
        match variant {
            PETEMC_A::VALUE1 => false,
            PETEMC_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PETEMC`"]
pub type PETEMC_R = crate::R<bool, PETEMC_A>;
impl PETEMC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEMC_A {
        match self.bits {
            false => PETEMC_A::VALUE1,
            true => PETEMC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEMC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEMC_A::VALUE2
    }
}
#[doc = "Write proxy for field `PETEMC`"]
pub struct PETEMC_W<'a> {
    w: &'a mut W,
}
impl<'a> PETEMC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETEMC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEMC_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEMC_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Parity Error Trap Enable for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEPPRF_A {
    #[doc = "0: Disabled"]
    VALUE1,
    #[doc = "1: Enabled"]
    VALUE2,
}
impl From<PETEPPRF_A> for bool {
    #[inline(always)]
    fn from(variant: PETEPPRF_A) -> Self {
        match variant {
            PETEPPRF_A::VALUE1 => false,
            PETEPPRF_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PETEPPRF`"]
pub type PETEPPRF_R = crate::R<bool, PETEPPRF_A>;
impl PETEPPRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEPPRF_A {
        match self.bits {
            false => PETEPPRF_A::VALUE1,
            true => PETEPPRF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEPPRF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEPPRF_A::VALUE2
    }
}
#[doc = "Write proxy for field `PETEPPRF`"]
pub struct PETEPPRF_W<'a> {
    w: &'a mut W,
}
impl<'a> PETEPPRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETEPPRF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEPPRF_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEPPRF_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Parity Error Trap Enable for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEUSB_A {
    #[doc = "0: Disabled"]
    VALUE1,
    #[doc = "1: Enabled"]
    VALUE2,
}
impl From<PETEUSB_A> for bool {
    #[inline(always)]
    fn from(variant: PETEUSB_A) -> Self {
        match variant {
            PETEUSB_A::VALUE1 => false,
            PETEUSB_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PETEUSB`"]
pub type PETEUSB_R = crate::R<bool, PETEUSB_A>;
impl PETEUSB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEUSB_A {
        match self.bits {
            false => PETEUSB_A::VALUE1,
            true => PETEUSB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEUSB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEUSB_A::VALUE2
    }
}
#[doc = "Write proxy for field `PETEUSB`"]
pub struct PETEUSB_W<'a> {
    w: &'a mut W,
}
impl<'a> PETEUSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETEUSB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEUSB_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEUSB_A::VALUE2)
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
#[doc = "Parity Error Trap Enable for ETH 0TX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEETH0TX_A {
    #[doc = "0: Disabled"]
    VALUE1,
    #[doc = "1: Enabled"]
    VALUE2,
}
impl From<PETEETH0TX_A> for bool {
    #[inline(always)]
    fn from(variant: PETEETH0TX_A) -> Self {
        match variant {
            PETEETH0TX_A::VALUE1 => false,
            PETEETH0TX_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PETEETH0TX`"]
pub type PETEETH0TX_R = crate::R<bool, PETEETH0TX_A>;
impl PETEETH0TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEETH0TX_A {
        match self.bits {
            false => PETEETH0TX_A::VALUE1,
            true => PETEETH0TX_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEETH0TX_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEETH0TX_A::VALUE2
    }
}
#[doc = "Write proxy for field `PETEETH0TX`"]
pub struct PETEETH0TX_W<'a> {
    w: &'a mut W,
}
impl<'a> PETEETH0TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETEETH0TX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEETH0TX_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEETH0TX_A::VALUE2)
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
#[doc = "Parity Error Trap Enable for ETH0 RX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEETH0RX_A {
    #[doc = "0: Disabled"]
    VALUE1,
    #[doc = "1: Enabled"]
    VALUE2,
}
impl From<PETEETH0RX_A> for bool {
    #[inline(always)]
    fn from(variant: PETEETH0RX_A) -> Self {
        match variant {
            PETEETH0RX_A::VALUE1 => false,
            PETEETH0RX_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PETEETH0RX`"]
pub type PETEETH0RX_R = crate::R<bool, PETEETH0RX_A>;
impl PETEETH0RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEETH0RX_A {
        match self.bits {
            false => PETEETH0RX_A::VALUE1,
            true => PETEETH0RX_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEETH0RX_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEETH0RX_A::VALUE2
    }
}
#[doc = "Write proxy for field `PETEETH0RX`"]
pub struct PETEETH0RX_W<'a> {
    w: &'a mut W,
}
impl<'a> PETEETH0RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETEETH0RX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEETH0RX_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEETH0RX_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Parity Error Trap Enable for SDMMC SRAM 0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETESD0_A {
    #[doc = "0: Disabled"]
    VALUE1,
    #[doc = "1: Enabled"]
    VALUE2,
}
impl From<PETESD0_A> for bool {
    #[inline(always)]
    fn from(variant: PETESD0_A) -> Self {
        match variant {
            PETESD0_A::VALUE1 => false,
            PETESD0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PETESD0`"]
pub type PETESD0_R = crate::R<bool, PETESD0_A>;
impl PETESD0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETESD0_A {
        match self.bits {
            false => PETESD0_A::VALUE1,
            true => PETESD0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETESD0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETESD0_A::VALUE2
    }
}
#[doc = "Write proxy for field `PETESD0`"]
pub struct PETESD0_W<'a> {
    w: &'a mut W,
}
impl<'a> PETESD0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETESD0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETESD0_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETESD0_A::VALUE2)
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
#[doc = "Parity Error Trap Enable for SDMMC SRAM 1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETESD1_A {
    #[doc = "0: Disabled"]
    VALUE1,
    #[doc = "1: Enabled"]
    VALUE2,
}
impl From<PETESD1_A> for bool {
    #[inline(always)]
    fn from(variant: PETESD1_A) -> Self {
        match variant {
            PETESD1_A::VALUE1 => false,
            PETESD1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PETESD1`"]
pub type PETESD1_R = crate::R<bool, PETESD1_A>;
impl PETESD1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETESD1_A {
        match self.bits {
            false => PETESD1_A::VALUE1,
            true => PETESD1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETESD1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETESD1_A::VALUE2
    }
}
#[doc = "Write proxy for field `PETESD1`"]
pub struct PETESD1_W<'a> {
    w: &'a mut W,
}
impl<'a> PETESD1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETESD1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETESD1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETESD1_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Parity Error Trap Enable for ECAT0 SRAM Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEECAT0_A {
    #[doc = "0: Disabled"]
    VALUE1,
    #[doc = "1: Enabled"]
    VALUE2,
}
impl From<PETEECAT0_A> for bool {
    #[inline(always)]
    fn from(variant: PETEECAT0_A) -> Self {
        match variant {
            PETEECAT0_A::VALUE1 => false,
            PETEECAT0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PETEECAT0`"]
pub type PETEECAT0_R = crate::R<bool, PETEECAT0_A>;
impl PETEECAT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEECAT0_A {
        match self.bits {
            false => PETEECAT0_A::VALUE1,
            true => PETEECAT0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEECAT0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEECAT0_A::VALUE2
    }
}
#[doc = "Write proxy for field `PETEECAT0`"]
pub struct PETEECAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PETEECAT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETEECAT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEECAT0_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEECAT0_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Parity Error Trap Enable for PSRAM"]
    #[inline(always)]
    pub fn peteps(&self) -> PETEPS_R {
        PETEPS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Parity Error Trap Enable for DSRAM1"]
    #[inline(always)]
    pub fn peteds1(&self) -> PETEDS1_R {
        PETEDS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Parity Error Trap Enable for DSRAM2"]
    #[inline(always)]
    pub fn peteds2(&self) -> PETEDS2_R {
        PETEDS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Parity Error Trap Enable for USIC0 Memory"]
    #[inline(always)]
    pub fn peteu0(&self) -> PETEU0_R {
        PETEU0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Parity Error Trap Enable for USIC1 Memory"]
    #[inline(always)]
    pub fn peteu1(&self) -> PETEU1_R {
        PETEU1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Parity Error Trap Enable for USIC2 Memory"]
    #[inline(always)]
    pub fn peteu2(&self) -> PETEU2_R {
        PETEU2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Parity Error Trap Enable for MultiCAN Memory"]
    #[inline(always)]
    pub fn petemc(&self) -> PETEMC_R {
        PETEMC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Parity Error Trap Enable for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn petepprf(&self) -> PETEPPRF_R {
        PETEPPRF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Parity Error Trap Enable for USB Memory"]
    #[inline(always)]
    pub fn peteusb(&self) -> PETEUSB_R {
        PETEUSB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Parity Error Trap Enable for ETH 0TX Memory"]
    #[inline(always)]
    pub fn peteeth0tx(&self) -> PETEETH0TX_R {
        PETEETH0TX_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Parity Error Trap Enable for ETH0 RX Memory"]
    #[inline(always)]
    pub fn peteeth0rx(&self) -> PETEETH0RX_R {
        PETEETH0RX_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Parity Error Trap Enable for SDMMC SRAM 0 Memory"]
    #[inline(always)]
    pub fn petesd0(&self) -> PETESD0_R {
        PETESD0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Parity Error Trap Enable for SDMMC SRAM 1 Memory"]
    #[inline(always)]
    pub fn petesd1(&self) -> PETESD1_R {
        PETESD1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Parity Error Trap Enable for ECAT0 SRAM Memory"]
    #[inline(always)]
    pub fn peteecat0(&self) -> PETEECAT0_R {
        PETEECAT0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Trap Enable for PSRAM"]
    #[inline(always)]
    pub fn peteps(&mut self) -> PETEPS_W {
        PETEPS_W { w: self }
    }
    #[doc = "Bit 1 - Parity Error Trap Enable for DSRAM1"]
    #[inline(always)]
    pub fn peteds1(&mut self) -> PETEDS1_W {
        PETEDS1_W { w: self }
    }
    #[doc = "Bit 2 - Parity Error Trap Enable for DSRAM2"]
    #[inline(always)]
    pub fn peteds2(&mut self) -> PETEDS2_W {
        PETEDS2_W { w: self }
    }
    #[doc = "Bit 8 - Parity Error Trap Enable for USIC0 Memory"]
    #[inline(always)]
    pub fn peteu0(&mut self) -> PETEU0_W {
        PETEU0_W { w: self }
    }
    #[doc = "Bit 9 - Parity Error Trap Enable for USIC1 Memory"]
    #[inline(always)]
    pub fn peteu1(&mut self) -> PETEU1_W {
        PETEU1_W { w: self }
    }
    #[doc = "Bit 10 - Parity Error Trap Enable for USIC2 Memory"]
    #[inline(always)]
    pub fn peteu2(&mut self) -> PETEU2_W {
        PETEU2_W { w: self }
    }
    #[doc = "Bit 12 - Parity Error Trap Enable for MultiCAN Memory"]
    #[inline(always)]
    pub fn petemc(&mut self) -> PETEMC_W {
        PETEMC_W { w: self }
    }
    #[doc = "Bit 13 - Parity Error Trap Enable for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn petepprf(&mut self) -> PETEPPRF_W {
        PETEPPRF_W { w: self }
    }
    #[doc = "Bit 16 - Parity Error Trap Enable for USB Memory"]
    #[inline(always)]
    pub fn peteusb(&mut self) -> PETEUSB_W {
        PETEUSB_W { w: self }
    }
    #[doc = "Bit 17 - Parity Error Trap Enable for ETH 0TX Memory"]
    #[inline(always)]
    pub fn peteeth0tx(&mut self) -> PETEETH0TX_W {
        PETEETH0TX_W { w: self }
    }
    #[doc = "Bit 18 - Parity Error Trap Enable for ETH0 RX Memory"]
    #[inline(always)]
    pub fn peteeth0rx(&mut self) -> PETEETH0RX_W {
        PETEETH0RX_W { w: self }
    }
    #[doc = "Bit 19 - Parity Error Trap Enable for SDMMC SRAM 0 Memory"]
    #[inline(always)]
    pub fn petesd0(&mut self) -> PETESD0_W {
        PETESD0_W { w: self }
    }
    #[doc = "Bit 20 - Parity Error Trap Enable for SDMMC SRAM 1 Memory"]
    #[inline(always)]
    pub fn petesd1(&mut self) -> PETESD1_W {
        PETESD1_W { w: self }
    }
    #[doc = "Bit 24 - Parity Error Trap Enable for ECAT0 SRAM Memory"]
    #[inline(always)]
    pub fn peteecat0(&mut self) -> PETEECAT0_W {
        PETEECAT0_W { w: self }
    }
}
