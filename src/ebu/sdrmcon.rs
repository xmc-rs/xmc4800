#[doc = "Register `SDRMCON` reader"]
pub struct R(crate::R<SDRMCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDRMCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDRMCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDRMCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDRMCON` writer"]
pub struct W(crate::W<SDRMCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDRMCON_SPEC>;
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
impl From<crate::W<SDRMCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDRMCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SDRAM clock mode select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDCMSEL_A {
    #[doc = "1: clock disabled between accesses"]
    VALUE1 = 1,
    #[doc = "0: clock continuously runs"]
    VALUE2 = 0,
}
impl From<SDCMSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SDCMSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDCMSEL` reader - SDRAM clock mode select"]
pub struct SDCMSEL_R(crate::FieldReader<bool, SDCMSEL_A>);
impl SDCMSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDCMSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDCMSEL_A {
        match self.bits {
            true => SDCMSEL_A::VALUE1,
            false => SDCMSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SDCMSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SDCMSEL_A::VALUE2
    }
}
impl core::ops::Deref for SDCMSEL_R {
    type Target = crate::FieldReader<bool, SDCMSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDCMSEL` writer - SDRAM clock mode select"]
pub struct SDCMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDCMSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDCMSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "clock disabled between accesses"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SDCMSEL_A::VALUE1)
    }
    #[doc = "clock continuously runs"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SDCMSEL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Power Save Mode used for gated clock mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWR_MODE_A {
    #[doc = "0: precharge before clock stop (default after reset)"]
    VALUE1 = 0,
    #[doc = "1: auto-precharge before clock stop"]
    VALUE2 = 1,
    #[doc = "2: active power down (stop clock without precharge)"]
    VALUE3 = 2,
    #[doc = "3: clock stop power down"]
    VALUE4 = 3,
}
impl From<PWR_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWR_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWR_MODE` reader - Power Save Mode used for gated clock mode"]
pub struct PWR_MODE_R(crate::FieldReader<u8, PWR_MODE_A>);
impl PWR_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWR_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWR_MODE_A {
        match self.bits {
            0 => PWR_MODE_A::VALUE1,
            1 => PWR_MODE_A::VALUE2,
            2 => PWR_MODE_A::VALUE3,
            3 => PWR_MODE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PWR_MODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PWR_MODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PWR_MODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PWR_MODE_A::VALUE4
    }
}
impl core::ops::Deref for PWR_MODE_R {
    type Target = crate::FieldReader<u8, PWR_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_MODE` writer - Power Save Mode used for gated clock mode"]
pub struct PWR_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWR_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "precharge before clock stop (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PWR_MODE_A::VALUE1)
    }
    #[doc = "auto-precharge before clock stop"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PWR_MODE_A::VALUE2)
    }
    #[doc = "active power down (stop clock without precharge)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PWR_MODE_A::VALUE3)
    }
    #[doc = "clock stop power down"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PWR_MODE_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
#[doc = "Disable SDRAM clock output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKDIS_A {
    #[doc = "0: clock enabled"]
    VALUE1 = 0,
    #[doc = "1: clock disabled"]
    VALUE2 = 1,
}
impl From<CLKDIS_A> for bool {
    #[inline(always)]
    fn from(variant: CLKDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKDIS` reader - Disable SDRAM clock output"]
pub struct CLKDIS_R(crate::FieldReader<bool, CLKDIS_A>);
impl CLKDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKDIS_A {
        match self.bits {
            false => CLKDIS_A::VALUE1,
            true => CLKDIS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CLKDIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CLKDIS_A::VALUE2
    }
}
impl core::ops::Deref for CLKDIS_R {
    type Target = crate::FieldReader<bool, CLKDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKDIS` writer - Disable SDRAM clock output"]
pub struct CLKDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "clock enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLKDIS_A::VALUE1)
    }
    #[doc = "clock disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CLKDIS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `CRCE` reader - Row cycle time counter extension"]
pub struct CRCE_R(crate::FieldReader<u8, u8>);
impl CRCE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCE` writer - Row cycle time counter extension"]
pub struct CRCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | ((value as u32 & 0x07) << 25);
        self.w
    }
}
#[doc = "Mask for bank tag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BANKM_A {
    #[doc = "1: Address bit 21 to 20"]
    VALUE2 = 1,
    #[doc = "2: Address bit 22 to 21"]
    VALUE3 = 2,
    #[doc = "3: Address bit 23 to 22"]
    VALUE4 = 3,
    #[doc = "4: Address bit 24 to 23"]
    VALUE5 = 4,
    #[doc = "5: Address bit 25 to 24"]
    VALUE6 = 5,
    #[doc = "6: Address bit 26 to 25"]
    VALUE7 = 6,
    #[doc = "7: Address bit 26"]
    VALUE8 = 7,
}
impl From<BANKM_A> for u8 {
    #[inline(always)]
    fn from(variant: BANKM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BANKM` reader - Mask for bank tag"]
pub struct BANKM_R(crate::FieldReader<u8, BANKM_A>);
impl BANKM_R {
    pub(crate) fn new(bits: u8) -> Self {
        BANKM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BANKM_A> {
        match self.bits {
            1 => Some(BANKM_A::VALUE2),
            2 => Some(BANKM_A::VALUE3),
            3 => Some(BANKM_A::VALUE4),
            4 => Some(BANKM_A::VALUE5),
            5 => Some(BANKM_A::VALUE6),
            6 => Some(BANKM_A::VALUE7),
            7 => Some(BANKM_A::VALUE8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BANKM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == BANKM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == BANKM_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == BANKM_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == BANKM_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        **self == BANKM_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        **self == BANKM_A::VALUE8
    }
}
impl core::ops::Deref for BANKM_R {
    type Target = crate::FieldReader<u8, BANKM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BANKM` writer - Mask for bank tag"]
pub struct BANKM_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BANKM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Address bit 21 to 20"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BANKM_A::VALUE2)
    }
    #[doc = "Address bit 22 to 21"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BANKM_A::VALUE3)
    }
    #[doc = "Address bit 23 to 22"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BANKM_A::VALUE4)
    }
    #[doc = "Address bit 24 to 23"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(BANKM_A::VALUE5)
    }
    #[doc = "Address bit 25 to 24"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(BANKM_A::VALUE6)
    }
    #[doc = "Address bit 26 to 25"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(BANKM_A::VALUE7)
    }
    #[doc = "Address bit 26"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(BANKM_A::VALUE8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | ((value as u32 & 0x07) << 22);
        self.w
    }
}
#[doc = "Mask for row tag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ROWM_A {
    #[doc = "1: Address bit 26 to 9"]
    VALUE2 = 1,
    #[doc = "2: Address bit 26 to 10"]
    VALUE3 = 2,
    #[doc = "3: Address bit 26 to 11"]
    VALUE4 = 3,
    #[doc = "4: Address bit 26 to 12"]
    VALUE5 = 4,
    #[doc = "5: Address bit 26 to 13"]
    VALUE6 = 5,
}
impl From<ROWM_A> for u8 {
    #[inline(always)]
    fn from(variant: ROWM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ROWM` reader - Mask for row tag"]
pub struct ROWM_R(crate::FieldReader<u8, ROWM_A>);
impl ROWM_R {
    pub(crate) fn new(bits: u8) -> Self {
        ROWM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ROWM_A> {
        match self.bits {
            1 => Some(ROWM_A::VALUE2),
            2 => Some(ROWM_A::VALUE3),
            3 => Some(ROWM_A::VALUE4),
            4 => Some(ROWM_A::VALUE5),
            5 => Some(ROWM_A::VALUE6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ROWM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == ROWM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == ROWM_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == ROWM_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == ROWM_A::VALUE6
    }
}
impl core::ops::Deref for ROWM_R {
    type Target = crate::FieldReader<u8, ROWM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROWM` writer - Mask for row tag"]
pub struct ROWM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROWM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROWM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Address bit 26 to 9"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ROWM_A::VALUE2)
    }
    #[doc = "Address bit 26 to 10"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ROWM_A::VALUE3)
    }
    #[doc = "Address bit 26 to 11"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ROWM_A::VALUE4)
    }
    #[doc = "Address bit 26 to 12"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(ROWM_A::VALUE5)
    }
    #[doc = "Address bit 26 to 13"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(ROWM_A::VALUE6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Field `CRC` reader - Row cycle time counter"]
pub struct CRC_R(crate::FieldReader<u8, u8>);
impl CRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC` writer - Row cycle time counter"]
pub struct CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `CRCD` reader - Row to column delay counter"]
pub struct CRCD_R(crate::FieldReader<u8, u8>);
impl CRCD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCD` writer - Row to column delay counter"]
pub struct CRCD_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Width of column address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AWIDTH_A {
    #[doc = "0: do not use"]
    VALUE1 = 0,
    #[doc = "1: Address(8:0)"]
    VALUE2 = 1,
    #[doc = "2: Address(9:0)"]
    VALUE3 = 2,
    #[doc = "3: Address(10:0)"]
    VALUE4 = 3,
}
impl From<AWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: AWIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AWIDTH` reader - Width of column address"]
pub struct AWIDTH_R(crate::FieldReader<u8, AWIDTH_A>);
impl AWIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        AWIDTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWIDTH_A {
        match self.bits {
            0 => AWIDTH_A::VALUE1,
            1 => AWIDTH_A::VALUE2,
            2 => AWIDTH_A::VALUE3,
            3 => AWIDTH_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == AWIDTH_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == AWIDTH_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == AWIDTH_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == AWIDTH_A::VALUE4
    }
}
impl core::ops::Deref for AWIDTH_R {
    type Target = crate::FieldReader<u8, AWIDTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWIDTH` writer - Width of column address"]
pub struct AWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> AWIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWIDTH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "do not use"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AWIDTH_A::VALUE1)
    }
    #[doc = "Address(8:0)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AWIDTH_A::VALUE2)
    }
    #[doc = "Address(9:0)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(AWIDTH_A::VALUE3)
    }
    #[doc = "Address(10:0)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(AWIDTH_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `CRP` reader - Row precharge time counter"]
pub struct CRP_R(crate::FieldReader<u8, u8>);
impl CRP_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRP` writer - Row precharge time counter"]
pub struct CRP_W<'a> {
    w: &'a mut W,
}
impl<'a> CRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `CRSC` reader - Mode register set-up time"]
pub struct CRSC_R(crate::FieldReader<u8, u8>);
impl CRSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRSC` writer - Mode register set-up time"]
pub struct CRSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `CRFSH` reader - Initialization refresh commands counter"]
pub struct CRFSH_R(crate::FieldReader<u8, u8>);
impl CRFSH_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRFSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRFSH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRFSH` writer - Initialization refresh commands counter"]
pub struct CRFSH_W<'a> {
    w: &'a mut W,
}
impl<'a> CRFSH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `CRAS` reader - Row to precharge delay counter"]
pub struct CRAS_R(crate::FieldReader<u8, u8>);
impl CRAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRAS` writer - Row to precharge delay counter"]
pub struct CRAS_W<'a> {
    w: &'a mut W,
}
impl<'a> CRAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - SDRAM clock mode select"]
    #[inline(always)]
    pub fn sdcmsel(&self) -> SDCMSEL_R {
        SDCMSEL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - Power Save Mode used for gated clock mode"]
    #[inline(always)]
    pub fn pwr_mode(&self) -> PWR_MODE_R {
        PWR_MODE_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - Disable SDRAM clock output"]
    #[inline(always)]
    pub fn clkdis(&self) -> CLKDIS_R {
        CLKDIS_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 25:27 - Row cycle time counter extension"]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 22:24 - Mask for bank tag"]
    #[inline(always)]
    pub fn bankm(&self) -> BANKM_R {
        BANKM_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 19:21 - Mask for row tag"]
    #[inline(always)]
    pub fn rowm(&self) -> ROWM_R {
        ROWM_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Row cycle time counter"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 14:15 - Row to column delay counter"]
    #[inline(always)]
    pub fn crcd(&self) -> CRCD_R {
        CRCD_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Width of column address"]
    #[inline(always)]
    pub fn awidth(&self) -> AWIDTH_R {
        AWIDTH_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Row precharge time counter"]
    #[inline(always)]
    pub fn crp(&self) -> CRP_R {
        CRP_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Mode register set-up time"]
    #[inline(always)]
    pub fn crsc(&self) -> CRSC_R {
        CRSC_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - Initialization refresh commands counter"]
    #[inline(always)]
    pub fn crfsh(&self) -> CRFSH_R {
        CRFSH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Row to precharge delay counter"]
    #[inline(always)]
    pub fn cras(&self) -> CRAS_R {
        CRAS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - SDRAM clock mode select"]
    #[inline(always)]
    pub fn sdcmsel(&mut self) -> SDCMSEL_W {
        SDCMSEL_W { w: self }
    }
    #[doc = "Bits 29:30 - Power Save Mode used for gated clock mode"]
    #[inline(always)]
    pub fn pwr_mode(&mut self) -> PWR_MODE_W {
        PWR_MODE_W { w: self }
    }
    #[doc = "Bit 28 - Disable SDRAM clock output"]
    #[inline(always)]
    pub fn clkdis(&mut self) -> CLKDIS_W {
        CLKDIS_W { w: self }
    }
    #[doc = "Bits 25:27 - Row cycle time counter extension"]
    #[inline(always)]
    pub fn crce(&mut self) -> CRCE_W {
        CRCE_W { w: self }
    }
    #[doc = "Bits 22:24 - Mask for bank tag"]
    #[inline(always)]
    pub fn bankm(&mut self) -> BANKM_W {
        BANKM_W { w: self }
    }
    #[doc = "Bits 19:21 - Mask for row tag"]
    #[inline(always)]
    pub fn rowm(&mut self) -> ROWM_W {
        ROWM_W { w: self }
    }
    #[doc = "Bits 16:18 - Row cycle time counter"]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W {
        CRC_W { w: self }
    }
    #[doc = "Bits 14:15 - Row to column delay counter"]
    #[inline(always)]
    pub fn crcd(&mut self) -> CRCD_W {
        CRCD_W { w: self }
    }
    #[doc = "Bits 12:13 - Width of column address"]
    #[inline(always)]
    pub fn awidth(&mut self) -> AWIDTH_W {
        AWIDTH_W { w: self }
    }
    #[doc = "Bits 10:11 - Row precharge time counter"]
    #[inline(always)]
    pub fn crp(&mut self) -> CRP_W {
        CRP_W { w: self }
    }
    #[doc = "Bits 8:9 - Mode register set-up time"]
    #[inline(always)]
    pub fn crsc(&mut self) -> CRSC_W {
        CRSC_W { w: self }
    }
    #[doc = "Bits 4:7 - Initialization refresh commands counter"]
    #[inline(always)]
    pub fn crfsh(&mut self) -> CRFSH_W {
        CRFSH_W { w: self }
    }
    #[doc = "Bits 0:3 - Row to precharge delay counter"]
    #[inline(always)]
    pub fn cras(&mut self) -> CRAS_W {
        CRAS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBU SDRAM Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdrmcon](index.html) module"]
pub struct SDRMCON_SPEC;
impl crate::RegisterSpec for SDRMCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdrmcon::R](R) reader structure"]
impl crate::Readable for SDRMCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdrmcon::W](W) writer structure"]
impl crate::Writable for SDRMCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDRMCON to value 0x8000_0000"]
impl crate::Resettable for SDRMCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
