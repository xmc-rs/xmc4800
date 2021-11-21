#[doc = "Register `DSLEEPCR` reader"]
pub struct R(crate::R<DSLEEPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSLEEPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSLEEPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSLEEPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSLEEPCR` writer"]
pub struct W(crate::W<DSLEEPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSLEEPCR_SPEC>;
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
impl From<crate::W<DSLEEPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSLEEPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "System Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSSEL_A {
    #[doc = "0: fOFI clock"]
    VALUE1 = 0,
    #[doc = "1: fPLL clock"]
    VALUE2 = 1,
}
impl From<SYSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSSEL` reader - System Clock Selection Value"]
pub struct SYSSEL_R(crate::FieldReader<u8, SYSSEL_A>);
impl SYSSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYSSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSSEL_A> {
        match self.bits {
            0 => Some(SYSSEL_A::VALUE1),
            1 => Some(SYSSEL_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SYSSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SYSSEL_A::VALUE2
    }
}
impl core::ops::Deref for SYSSEL_R {
    type Target = crate::FieldReader<u8, SYSSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSSEL` writer - System Clock Selection Value"]
pub struct SYSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SYSSEL_A::VALUE1)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SYSSEL_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Flash Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPDN_A {
    #[doc = "1: Flash power down module"]
    VALUE1 = 1,
    #[doc = "0: No effect"]
    VALUE2 = 0,
}
impl From<FPDN_A> for bool {
    #[inline(always)]
    fn from(variant: FPDN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPDN` reader - Flash Power Down"]
pub struct FPDN_R(crate::FieldReader<bool, FPDN_A>);
impl FPDN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPDN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPDN_A {
        match self.bits {
            true => FPDN_A::VALUE1,
            false => FPDN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FPDN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FPDN_A::VALUE2
    }
}
impl core::ops::Deref for FPDN_R {
    type Target = crate::FieldReader<bool, FPDN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPDN` writer - Flash Power Down"]
pub struct FPDN_W<'a> {
    w: &'a mut W,
}
impl<'a> FPDN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPDN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flash power down module"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FPDN_A::VALUE1)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FPDN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "PLL Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLPDN_A {
    #[doc = "1: Switch off main PLL"]
    VALUE1 = 1,
    #[doc = "0: No effect"]
    VALUE2 = 0,
}
impl From<PLLPDN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLPDN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLPDN` reader - PLL Power Down"]
pub struct PLLPDN_R(crate::FieldReader<bool, PLLPDN_A>);
impl PLLPDN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLPDN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLPDN_A {
        match self.bits {
            true => PLLPDN_A::VALUE1,
            false => PLLPDN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PLLPDN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PLLPDN_A::VALUE2
    }
}
impl core::ops::Deref for PLLPDN_R {
    type Target = crate::FieldReader<bool, PLLPDN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLPDN` writer - PLL Power Down"]
pub struct PLLPDN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLPDN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLPDN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Switch off main PLL"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PLLPDN_A::VALUE1)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PLLPDN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "VCO Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOPDN_A {
    #[doc = "1: Switch off VCO of main PLL"]
    VALUE1 = 1,
    #[doc = "0: No effect"]
    VALUE2 = 0,
}
impl From<VCOPDN_A> for bool {
    #[inline(always)]
    fn from(variant: VCOPDN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOPDN` reader - VCO Power Down"]
pub struct VCOPDN_R(crate::FieldReader<bool, VCOPDN_A>);
impl VCOPDN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VCOPDN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOPDN_A {
        match self.bits {
            true => VCOPDN_A::VALUE1,
            false => VCOPDN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VCOPDN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == VCOPDN_A::VALUE2
    }
}
impl core::ops::Deref for VCOPDN_R {
    type Target = crate::FieldReader<bool, VCOPDN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCOPDN` writer - VCO Power Down"]
pub struct VCOPDN_W<'a> {
    w: &'a mut W,
}
impl<'a> VCOPDN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCOPDN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Switch off VCO of main PLL"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VCOPDN_A::VALUE1)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VCOPDN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "USB Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBCR_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<USBCR_A> for bool {
    #[inline(always)]
    fn from(variant: USBCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCR` reader - USB Clock Control"]
pub struct USBCR_R(crate::FieldReader<bool, USBCR_A>);
impl USBCR_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBCR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBCR_A {
        match self.bits {
            false => USBCR_A::VALUE1,
            true => USBCR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == USBCR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == USBCR_A::VALUE2
    }
}
impl core::ops::Deref for USBCR_R {
    type Target = crate::FieldReader<bool, USBCR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBCR` writer - USB Clock Control"]
pub struct USBCR_W<'a> {
    w: &'a mut W,
}
impl<'a> USBCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBCR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USBCR_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USBCR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "MMC Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMCCR_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<MMCCR_A> for bool {
    #[inline(always)]
    fn from(variant: MMCCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCCR` reader - MMC Clock Control"]
pub struct MMCCR_R(crate::FieldReader<bool, MMCCR_A>);
impl MMCCR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMCCR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMCCR_A {
        match self.bits {
            false => MMCCR_A::VALUE1,
            true => MMCCR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MMCCR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MMCCR_A::VALUE2
    }
}
impl core::ops::Deref for MMCCR_R {
    type Target = crate::FieldReader<bool, MMCCR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMCCR` writer - MMC Clock Control"]
pub struct MMCCR_W<'a> {
    w: &'a mut W,
}
impl<'a> MMCCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMCCR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MMCCR_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MMCCR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Ethernet Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETH0CR_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<ETH0CR_A> for bool {
    #[inline(always)]
    fn from(variant: ETH0CR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0CR` reader - Ethernet Clock Control"]
pub struct ETH0CR_R(crate::FieldReader<bool, ETH0CR_A>);
impl ETH0CR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETH0CR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETH0CR_A {
        match self.bits {
            false => ETH0CR_A::VALUE1,
            true => ETH0CR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ETH0CR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ETH0CR_A::VALUE2
    }
}
impl core::ops::Deref for ETH0CR_R {
    type Target = crate::FieldReader<bool, ETH0CR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETH0CR` writer - Ethernet Clock Control"]
pub struct ETH0CR_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH0CR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETH0CR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ETH0CR_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ETH0CR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "EBU Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBUCR_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<EBUCR_A> for bool {
    #[inline(always)]
    fn from(variant: EBUCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBUCR` reader - EBU Clock Control"]
pub struct EBUCR_R(crate::FieldReader<bool, EBUCR_A>);
impl EBUCR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EBUCR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EBUCR_A {
        match self.bits {
            false => EBUCR_A::VALUE1,
            true => EBUCR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EBUCR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EBUCR_A::VALUE2
    }
}
impl core::ops::Deref for EBUCR_R {
    type Target = crate::FieldReader<bool, EBUCR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EBUCR` writer - EBU Clock Control"]
pub struct EBUCR_W<'a> {
    w: &'a mut W,
}
impl<'a> EBUCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EBUCR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EBUCR_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EBUCR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "CCU Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCUCR_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<CCUCR_A> for bool {
    #[inline(always)]
    fn from(variant: CCUCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCR` reader - CCU Clock Control"]
pub struct CCUCR_R(crate::FieldReader<bool, CCUCR_A>);
impl CCUCR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCUCR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCUCR_A {
        match self.bits {
            false => CCUCR_A::VALUE1,
            true => CCUCR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CCUCR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CCUCR_A::VALUE2
    }
}
impl core::ops::Deref for CCUCR_R {
    type Target = crate::FieldReader<bool, CCUCR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCUCR` writer - CCU Clock Control"]
pub struct CCUCR_W<'a> {
    w: &'a mut W,
}
impl<'a> CCUCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCUCR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCUCR_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCUCR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "WDT Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTCR_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<WDTCR_A> for bool {
    #[inline(always)]
    fn from(variant: WDTCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCR` reader - WDT Clock Control"]
pub struct WDTCR_R(crate::FieldReader<bool, WDTCR_A>);
impl WDTCR_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTCR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTCR_A {
        match self.bits {
            false => WDTCR_A::VALUE1,
            true => WDTCR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == WDTCR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WDTCR_A::VALUE2
    }
}
impl core::ops::Deref for WDTCR_R {
    type Target = crate::FieldReader<bool, WDTCR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTCR` writer - WDT Clock Control"]
pub struct WDTCR_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTCR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WDTCR_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WDTCR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&self) -> SYSSEL_R {
        SYSSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 11 - Flash Power Down"]
    #[inline(always)]
    pub fn fpdn(&self) -> FPDN_R {
        FPDN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PLL Power Down"]
    #[inline(always)]
    pub fn pllpdn(&self) -> PLLPDN_R {
        PLLPDN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - VCO Power Down"]
    #[inline(always)]
    pub fn vcopdn(&self) -> VCOPDN_R {
        VCOPDN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USB Clock Control"]
    #[inline(always)]
    pub fn usbcr(&self) -> USBCR_R {
        USBCR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MMC Clock Control"]
    #[inline(always)]
    pub fn mmccr(&self) -> MMCCR_R {
        MMCCR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Ethernet Clock Control"]
    #[inline(always)]
    pub fn eth0cr(&self) -> ETH0CR_R {
        ETH0CR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - EBU Clock Control"]
    #[inline(always)]
    pub fn ebucr(&self) -> EBUCR_R {
        EBUCR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CCU Clock Control"]
    #[inline(always)]
    pub fn ccucr(&self) -> CCUCR_R {
        CCUCR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - WDT Clock Control"]
    #[inline(always)]
    pub fn wdtcr(&self) -> WDTCR_R {
        WDTCR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&mut self) -> SYSSEL_W {
        SYSSEL_W { w: self }
    }
    #[doc = "Bit 11 - Flash Power Down"]
    #[inline(always)]
    pub fn fpdn(&mut self) -> FPDN_W {
        FPDN_W { w: self }
    }
    #[doc = "Bit 12 - PLL Power Down"]
    #[inline(always)]
    pub fn pllpdn(&mut self) -> PLLPDN_W {
        PLLPDN_W { w: self }
    }
    #[doc = "Bit 13 - VCO Power Down"]
    #[inline(always)]
    pub fn vcopdn(&mut self) -> VCOPDN_W {
        VCOPDN_W { w: self }
    }
    #[doc = "Bit 16 - USB Clock Control"]
    #[inline(always)]
    pub fn usbcr(&mut self) -> USBCR_W {
        USBCR_W { w: self }
    }
    #[doc = "Bit 17 - MMC Clock Control"]
    #[inline(always)]
    pub fn mmccr(&mut self) -> MMCCR_W {
        MMCCR_W { w: self }
    }
    #[doc = "Bit 18 - Ethernet Clock Control"]
    #[inline(always)]
    pub fn eth0cr(&mut self) -> ETH0CR_W {
        ETH0CR_W { w: self }
    }
    #[doc = "Bit 19 - EBU Clock Control"]
    #[inline(always)]
    pub fn ebucr(&mut self) -> EBUCR_W {
        EBUCR_W { w: self }
    }
    #[doc = "Bit 20 - CCU Clock Control"]
    #[inline(always)]
    pub fn ccucr(&mut self) -> CCUCR_W {
        CCUCR_W { w: self }
    }
    #[doc = "Bit 21 - WDT Clock Control"]
    #[inline(always)]
    pub fn wdtcr(&mut self) -> WDTCR_W {
        WDTCR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Sleep Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsleepcr](index.html) module"]
pub struct DSLEEPCR_SPEC;
impl crate::RegisterSpec for DSLEEPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsleepcr::R](R) reader structure"]
impl crate::Readable for DSLEEPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsleepcr::W](W) writer structure"]
impl crate::Writable for DSLEEPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSLEEPCR to value 0"]
impl crate::Resettable for DSLEEPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
