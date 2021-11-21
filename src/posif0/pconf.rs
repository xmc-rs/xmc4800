#[doc = "Register `PCONF` reader"]
pub struct R(crate::R<PCONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCONF` writer"]
pub struct W(crate::W<PCONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCONF_SPEC>;
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
impl From<crate::W<PCONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Function Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FSEL_A {
    #[doc = "0: Hall Sensor Mode enabled"]
    VALUE1 = 0,
    #[doc = "1: Quadrature Decoder Mode enabled"]
    VALUE2 = 1,
    #[doc = "2: stand-alone Multi-Channel Mode enabled"]
    VALUE3 = 2,
    #[doc = "3: Quadrature Decoder and stand-alone Multi-Channel Mode enabled"]
    VALUE4 = 3,
}
impl From<FSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FSEL` reader - Function Selector"]
pub struct FSEL_R(crate::FieldReader<u8, FSEL_A>);
impl FSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL_A {
        match self.bits {
            0 => FSEL_A::VALUE1,
            1 => FSEL_A::VALUE2,
            2 => FSEL_A::VALUE3,
            3 => FSEL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FSEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == FSEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == FSEL_A::VALUE4
    }
}
impl core::ops::Deref for FSEL_R {
    type Target = crate::FieldReader<u8, FSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSEL` writer - Function Selector"]
pub struct FSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Hall Sensor Mode enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FSEL_A::VALUE1)
    }
    #[doc = "Quadrature Decoder Mode enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FSEL_A::VALUE2)
    }
    #[doc = "stand-alone Multi-Channel Mode enabled"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(FSEL_A::VALUE3)
    }
    #[doc = "Quadrature Decoder and stand-alone Multi-Channel Mode enabled"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(FSEL_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Position Decoder Mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QDCM_A {
    #[doc = "0: Position encoder is in Quadrature Mode"]
    VALUE1 = 0,
    #[doc = "1: Position encoder is in Direction Count Mode."]
    VALUE2 = 1,
}
impl From<QDCM_A> for bool {
    #[inline(always)]
    fn from(variant: QDCM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QDCM` reader - Position Decoder Mode selection"]
pub struct QDCM_R(crate::FieldReader<bool, QDCM_A>);
impl QDCM_R {
    pub(crate) fn new(bits: bool) -> Self {
        QDCM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QDCM_A {
        match self.bits {
            false => QDCM_A::VALUE1,
            true => QDCM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == QDCM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == QDCM_A::VALUE2
    }
}
impl core::ops::Deref for QDCM_R {
    type Target = crate::FieldReader<bool, QDCM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QDCM` writer - Position Decoder Mode selection"]
pub struct QDCM_W<'a> {
    w: &'a mut W,
}
impl<'a> QDCM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QDCM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Position encoder is in Quadrature Mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(QDCM_A::VALUE1)
    }
    #[doc = "Position encoder is in Direction Count Mode."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(QDCM_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `HIDG` reader - Idle generation enable"]
pub struct HIDG_R(crate::FieldReader<bool, bool>);
impl HIDG_R {
    pub(crate) fn new(bits: bool) -> Self {
        HIDG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIDG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIDG` writer - Idle generation enable"]
pub struct HIDG_W<'a> {
    w: &'a mut W,
}
impl<'a> HIDG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Multi-Channel Pattern SW update enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCUE_A {
    #[doc = "0: Multi-Channel pattern update is controlled via HW"]
    VALUE1 = 0,
    #[doc = "1: Multi-Channel pattern update is controlled via SW"]
    VALUE2 = 1,
}
impl From<MCUE_A> for bool {
    #[inline(always)]
    fn from(variant: MCUE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCUE` reader - Multi-Channel Pattern SW update enable"]
pub struct MCUE_R(crate::FieldReader<bool, MCUE_A>);
impl MCUE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCUE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCUE_A {
        match self.bits {
            false => MCUE_A::VALUE1,
            true => MCUE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MCUE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MCUE_A::VALUE2
    }
}
impl core::ops::Deref for MCUE_R {
    type Target = crate::FieldReader<bool, MCUE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCUE` writer - Multi-Channel Pattern SW update enable"]
pub struct MCUE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCUE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCUE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Multi-Channel pattern update is controlled via HW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCUE_A::VALUE1)
    }
    #[doc = "Multi-Channel pattern update is controlled via SW"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCUE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "PhaseA/Hal input 1 selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INSEL0_A {
    #[doc = "0: POSIFx.IN0A"]
    VALUE1 = 0,
    #[doc = "1: POSIFx.IN0B"]
    VALUE2 = 1,
    #[doc = "2: POSIFx.IN0C"]
    VALUE3 = 2,
    #[doc = "3: POSIFx.IN0D"]
    VALUE4 = 3,
}
impl From<INSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: INSEL0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INSEL0` reader - PhaseA/Hal input 1 selector"]
pub struct INSEL0_R(crate::FieldReader<u8, INSEL0_A>);
impl INSEL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        INSEL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INSEL0_A {
        match self.bits {
            0 => INSEL0_A::VALUE1,
            1 => INSEL0_A::VALUE2,
            2 => INSEL0_A::VALUE3,
            3 => INSEL0_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == INSEL0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == INSEL0_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == INSEL0_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == INSEL0_A::VALUE4
    }
}
impl core::ops::Deref for INSEL0_R {
    type Target = crate::FieldReader<u8, INSEL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSEL0` writer - PhaseA/Hal input 1 selector"]
pub struct INSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> INSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INSEL0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "POSIFx.IN0A"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(INSEL0_A::VALUE1)
    }
    #[doc = "POSIFx.IN0B"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(INSEL0_A::VALUE2)
    }
    #[doc = "POSIFx.IN0C"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(INSEL0_A::VALUE3)
    }
    #[doc = "POSIFx.IN0D"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(INSEL0_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "PhaseB/Hall input 2 selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INSEL1_A {
    #[doc = "0: POSIFx.IN1A"]
    VALUE1 = 0,
    #[doc = "1: POSIFx.IN1B"]
    VALUE2 = 1,
    #[doc = "2: POSIFx.IN1C"]
    VALUE3 = 2,
    #[doc = "3: POSIFx.IN1D"]
    VALUE4 = 3,
}
impl From<INSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: INSEL1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INSEL1` reader - PhaseB/Hall input 2 selector"]
pub struct INSEL1_R(crate::FieldReader<u8, INSEL1_A>);
impl INSEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        INSEL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INSEL1_A {
        match self.bits {
            0 => INSEL1_A::VALUE1,
            1 => INSEL1_A::VALUE2,
            2 => INSEL1_A::VALUE3,
            3 => INSEL1_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == INSEL1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == INSEL1_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == INSEL1_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == INSEL1_A::VALUE4
    }
}
impl core::ops::Deref for INSEL1_R {
    type Target = crate::FieldReader<u8, INSEL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSEL1` writer - PhaseB/Hall input 2 selector"]
pub struct INSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> INSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INSEL1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "POSIFx.IN1A"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(INSEL1_A::VALUE1)
    }
    #[doc = "POSIFx.IN1B"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(INSEL1_A::VALUE2)
    }
    #[doc = "POSIFx.IN1C"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(INSEL1_A::VALUE3)
    }
    #[doc = "POSIFx.IN1D"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(INSEL1_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Index/Hall input 3 selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INSEL2_A {
    #[doc = "0: POSIFx.IN2A"]
    VALUE1 = 0,
    #[doc = "1: POSIFx.IN2B"]
    VALUE2 = 1,
    #[doc = "2: POSIFx.IN2C"]
    VALUE3 = 2,
    #[doc = "3: POSIFx.IN2D"]
    VALUE4 = 3,
}
impl From<INSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: INSEL2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INSEL2` reader - Index/Hall input 3 selector"]
pub struct INSEL2_R(crate::FieldReader<u8, INSEL2_A>);
impl INSEL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        INSEL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INSEL2_A {
        match self.bits {
            0 => INSEL2_A::VALUE1,
            1 => INSEL2_A::VALUE2,
            2 => INSEL2_A::VALUE3,
            3 => INSEL2_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == INSEL2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == INSEL2_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == INSEL2_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == INSEL2_A::VALUE4
    }
}
impl core::ops::Deref for INSEL2_R {
    type Target = crate::FieldReader<u8, INSEL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSEL2` writer - Index/Hall input 3 selector"]
pub struct INSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> INSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INSEL2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "POSIFx.IN2A"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(INSEL2_A::VALUE1)
    }
    #[doc = "POSIFx.IN2B"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(INSEL2_A::VALUE2)
    }
    #[doc = "POSIFx.IN2C"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(INSEL2_A::VALUE3)
    }
    #[doc = "POSIFx.IN2D"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(INSEL2_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Delay Pin selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSEL_A {
    #[doc = "0: POSIFx.HSDA"]
    VALUE1 = 0,
    #[doc = "1: POSIFx.HSDB"]
    VALUE2 = 1,
}
impl From<DSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSEL` reader - Delay Pin selector"]
pub struct DSEL_R(crate::FieldReader<bool, DSEL_A>);
impl DSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSEL_A {
        match self.bits {
            false => DSEL_A::VALUE1,
            true => DSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DSEL_A::VALUE2
    }
}
impl core::ops::Deref for DSEL_R {
    type Target = crate::FieldReader<bool, DSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSEL` writer - Delay Pin selector"]
pub struct DSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "POSIFx.HSDA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DSEL_A::VALUE1)
    }
    #[doc = "POSIFx.HSDB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DSEL_A::VALUE2)
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
#[doc = "Edge selector for the sampling trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPES_A {
    #[doc = "0: Rising edge"]
    VALUE1 = 0,
    #[doc = "1: Falling edge"]
    VALUE2 = 1,
}
impl From<SPES_A> for bool {
    #[inline(always)]
    fn from(variant: SPES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPES` reader - Edge selector for the sampling trigger"]
pub struct SPES_R(crate::FieldReader<bool, SPES_A>);
impl SPES_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPES_A {
        match self.bits {
            false => SPES_A::VALUE1,
            true => SPES_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SPES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SPES_A::VALUE2
    }
}
impl core::ops::Deref for SPES_R {
    type Target = crate::FieldReader<bool, SPES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPES` writer - Edge selector for the sampling trigger"]
pub struct SPES_W<'a> {
    w: &'a mut W,
}
impl<'a> SPES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPES_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SPES_A::VALUE1)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SPES_A::VALUE2)
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
#[doc = "Pattern update signal select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSETS_A {
    #[doc = "0: POSIFx.MSETA"]
    VALUE1 = 0,
    #[doc = "1: POSIFx.MSETB"]
    VALUE2 = 1,
    #[doc = "2: POSIFx.MSETC"]
    VALUE3 = 2,
    #[doc = "3: POSIFx.MSETD"]
    VALUE4 = 3,
    #[doc = "4: POSIFx.MSETE"]
    VALUE5 = 4,
    #[doc = "5: POSIFx.MSETF"]
    VALUE6 = 5,
    #[doc = "6: POSIFx.MSETG"]
    VALUE7 = 6,
    #[doc = "7: POSIFx.MSETH"]
    VALUE8 = 7,
}
impl From<MSETS_A> for u8 {
    #[inline(always)]
    fn from(variant: MSETS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MSETS` reader - Pattern update signal select"]
pub struct MSETS_R(crate::FieldReader<u8, MSETS_A>);
impl MSETS_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSETS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSETS_A {
        match self.bits {
            0 => MSETS_A::VALUE1,
            1 => MSETS_A::VALUE2,
            2 => MSETS_A::VALUE3,
            3 => MSETS_A::VALUE4,
            4 => MSETS_A::VALUE5,
            5 => MSETS_A::VALUE6,
            6 => MSETS_A::VALUE7,
            7 => MSETS_A::VALUE8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MSETS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MSETS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == MSETS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == MSETS_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == MSETS_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == MSETS_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        **self == MSETS_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        **self == MSETS_A::VALUE8
    }
}
impl core::ops::Deref for MSETS_R {
    type Target = crate::FieldReader<u8, MSETS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSETS` writer - Pattern update signal select"]
pub struct MSETS_W<'a> {
    w: &'a mut W,
}
impl<'a> MSETS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSETS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "POSIFx.MSETA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSETS_A::VALUE1)
    }
    #[doc = "POSIFx.MSETB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSETS_A::VALUE2)
    }
    #[doc = "POSIFx.MSETC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(MSETS_A::VALUE3)
    }
    #[doc = "POSIFx.MSETD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(MSETS_A::VALUE4)
    }
    #[doc = "POSIFx.MSETE"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(MSETS_A::VALUE5)
    }
    #[doc = "POSIFx.MSETF"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(MSETS_A::VALUE6)
    }
    #[doc = "POSIFx.MSETG"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(MSETS_A::VALUE7)
    }
    #[doc = "POSIFx.MSETH"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(MSETS_A::VALUE8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Multi-Channel pattern update trigger edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSES_A {
    #[doc = "0: The signal used to enable a pattern update is active on the rising edge"]
    VALUE1 = 0,
    #[doc = "1: The signal used to enable a pattern update is active on the falling edge"]
    VALUE2 = 1,
}
impl From<MSES_A> for bool {
    #[inline(always)]
    fn from(variant: MSES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSES` reader - Multi-Channel pattern update trigger edge"]
pub struct MSES_R(crate::FieldReader<bool, MSES_A>);
impl MSES_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSES_A {
        match self.bits {
            false => MSES_A::VALUE1,
            true => MSES_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MSES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MSES_A::VALUE2
    }
}
impl core::ops::Deref for MSES_R {
    type Target = crate::FieldReader<bool, MSES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSES` writer - Multi-Channel pattern update trigger edge"]
pub struct MSES_W<'a> {
    w: &'a mut W,
}
impl<'a> MSES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSES_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The signal used to enable a pattern update is active on the rising edge"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSES_A::VALUE1)
    }
    #[doc = "The signal used to enable a pattern update is active on the falling edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSES_A::VALUE2)
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
#[doc = "PWM synchronization signal selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSYNS_A {
    #[doc = "0: POSIFx.MSYNCA"]
    VALUE1 = 0,
    #[doc = "1: POSIFx.MSYNCB"]
    VALUE2 = 1,
    #[doc = "2: POSIFx.MSYNCC"]
    VALUE3 = 2,
    #[doc = "3: POSIFx.MSYNCD"]
    VALUE4 = 3,
}
impl From<MSYNS_A> for u8 {
    #[inline(always)]
    fn from(variant: MSYNS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MSYNS` reader - PWM synchronization signal selector"]
pub struct MSYNS_R(crate::FieldReader<u8, MSYNS_A>);
impl MSYNS_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSYNS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSYNS_A {
        match self.bits {
            0 => MSYNS_A::VALUE1,
            1 => MSYNS_A::VALUE2,
            2 => MSYNS_A::VALUE3,
            3 => MSYNS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MSYNS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MSYNS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == MSYNS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == MSYNS_A::VALUE4
    }
}
impl core::ops::Deref for MSYNS_R {
    type Target = crate::FieldReader<u8, MSYNS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSYNS` writer - PWM synchronization signal selector"]
pub struct MSYNS_W<'a> {
    w: &'a mut W,
}
impl<'a> MSYNS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSYNS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "POSIFx.MSYNCA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSYNS_A::VALUE1)
    }
    #[doc = "POSIFx.MSYNCB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSYNS_A::VALUE2)
    }
    #[doc = "POSIFx.MSYNCC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(MSYNS_A::VALUE3)
    }
    #[doc = "POSIFx.MSYNCD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(MSYNS_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Wrong Hall Event selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EWIS_A {
    #[doc = "0: POSIFx.EWHEA"]
    VALUE1 = 0,
    #[doc = "1: POSIFx.EWHEB"]
    VALUE2 = 1,
    #[doc = "2: POSIFx.EWHEC"]
    VALUE3 = 2,
    #[doc = "3: POSIFx.EWHED"]
    VALUE4 = 3,
}
impl From<EWIS_A> for u8 {
    #[inline(always)]
    fn from(variant: EWIS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EWIS` reader - Wrong Hall Event selection"]
pub struct EWIS_R(crate::FieldReader<u8, EWIS_A>);
impl EWIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EWIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWIS_A {
        match self.bits {
            0 => EWIS_A::VALUE1,
            1 => EWIS_A::VALUE2,
            2 => EWIS_A::VALUE3,
            3 => EWIS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EWIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EWIS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == EWIS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == EWIS_A::VALUE4
    }
}
impl core::ops::Deref for EWIS_R {
    type Target = crate::FieldReader<u8, EWIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWIS` writer - Wrong Hall Event selection"]
pub struct EWIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EWIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EWIS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "POSIFx.EWHEA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EWIS_A::VALUE1)
    }
    #[doc = "POSIFx.EWHEB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EWIS_A::VALUE2)
    }
    #[doc = "POSIFx.EWHEC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EWIS_A::VALUE3)
    }
    #[doc = "POSIFx.EWHED"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EWIS_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "External Wrong Hall Event enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWIE_A {
    #[doc = "0: External wrong hall event emulation signal, POSIFx.EWHE\\[D...A\\], is disabled"]
    VALUE1 = 0,
    #[doc = "1: External wrong hall event emulation signal, POSIFx.EWHE\\[D...A\\], is enabled."]
    VALUE2 = 1,
}
impl From<EWIE_A> for bool {
    #[inline(always)]
    fn from(variant: EWIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWIE` reader - External Wrong Hall Event enable"]
pub struct EWIE_R(crate::FieldReader<bool, EWIE_A>);
impl EWIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWIE_A {
        match self.bits {
            false => EWIE_A::VALUE1,
            true => EWIE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EWIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EWIE_A::VALUE2
    }
}
impl core::ops::Deref for EWIE_R {
    type Target = crate::FieldReader<bool, EWIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWIE` writer - External Wrong Hall Event enable"]
pub struct EWIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EWIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EWIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External wrong hall event emulation signal, POSIFx.EWHE\\[D...A\\], is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EWIE_A::VALUE1)
    }
    #[doc = "External wrong hall event emulation signal, POSIFx.EWHE\\[D...A\\], is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EWIE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "External Wrong Hall Event active level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWIL_A {
    #[doc = "0: POSIFx.EWHE\\[D...A\\]
signal is active HIGH"]
    VALUE1 = 0,
    #[doc = "1: POSIFx.EWHE\\[D...A\\]
signal is active LOW"]
    VALUE2 = 1,
}
impl From<EWIL_A> for bool {
    #[inline(always)]
    fn from(variant: EWIL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWIL` reader - External Wrong Hall Event active level"]
pub struct EWIL_R(crate::FieldReader<bool, EWIL_A>);
impl EWIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWIL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWIL_A {
        match self.bits {
            false => EWIL_A::VALUE1,
            true => EWIL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EWIL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EWIL_A::VALUE2
    }
}
impl core::ops::Deref for EWIL_R {
    type Target = crate::FieldReader<bool, EWIL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWIL` writer - External Wrong Hall Event active level"]
pub struct EWIL_W<'a> {
    w: &'a mut W,
}
impl<'a> EWIL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EWIL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "POSIFx.EWHE\\[D...A\\]
signal is active HIGH"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EWIL_A::VALUE1)
    }
    #[doc = "POSIFx.EWHE\\[D...A\\]
signal is active LOW"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EWIL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Low Pass Filters Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPC_A {
    #[doc = "0: Low pass filter disabled"]
    VALUE1 = 0,
    #[doc = "1: Low pass of 1 clock cycle"]
    VALUE2 = 1,
    #[doc = "2: Low pass of 2 clock cycles"]
    VALUE3 = 2,
    #[doc = "3: Low pass of 4 clock cycles"]
    VALUE4 = 3,
    #[doc = "4: Low pass of 8 clock cycles"]
    VALUE5 = 4,
    #[doc = "5: Low pass of 16 clock cycles"]
    VALUE6 = 5,
    #[doc = "6: Low pass of 32 clock cycles"]
    VALUE7 = 6,
    #[doc = "7: Low pass of 64 clock cycles"]
    VALUE8 = 7,
}
impl From<LPC_A> for u8 {
    #[inline(always)]
    fn from(variant: LPC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPC` reader - Low Pass Filters Configuration"]
pub struct LPC_R(crate::FieldReader<u8, LPC_A>);
impl LPC_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPC_A {
        match self.bits {
            0 => LPC_A::VALUE1,
            1 => LPC_A::VALUE2,
            2 => LPC_A::VALUE3,
            3 => LPC_A::VALUE4,
            4 => LPC_A::VALUE5,
            5 => LPC_A::VALUE6,
            6 => LPC_A::VALUE7,
            7 => LPC_A::VALUE8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == LPC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LPC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == LPC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == LPC_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == LPC_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == LPC_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        **self == LPC_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        **self == LPC_A::VALUE8
    }
}
impl core::ops::Deref for LPC_R {
    type Target = crate::FieldReader<u8, LPC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPC` writer - Low Pass Filters Configuration"]
pub struct LPC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low pass filter disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPC_A::VALUE1)
    }
    #[doc = "Low pass of 1 clock cycle"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPC_A::VALUE2)
    }
    #[doc = "Low pass of 2 clock cycles"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LPC_A::VALUE3)
    }
    #[doc = "Low pass of 4 clock cycles"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(LPC_A::VALUE4)
    }
    #[doc = "Low pass of 8 clock cycles"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(LPC_A::VALUE5)
    }
    #[doc = "Low pass of 16 clock cycles"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(LPC_A::VALUE6)
    }
    #[doc = "Low pass of 32 clock cycles"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(LPC_A::VALUE7)
    }
    #[doc = "Low pass of 64 clock cycles"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(LPC_A::VALUE8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Function Selector"]
    #[inline(always)]
    pub fn fsel(&self) -> FSEL_R {
        FSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Position Decoder Mode selection"]
    #[inline(always)]
    pub fn qdcm(&self) -> QDCM_R {
        QDCM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Idle generation enable"]
    #[inline(always)]
    pub fn hidg(&self) -> HIDG_R {
        HIDG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Multi-Channel Pattern SW update enable"]
    #[inline(always)]
    pub fn mcue(&self) -> MCUE_R {
        MCUE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - PhaseA/Hal input 1 selector"]
    #[inline(always)]
    pub fn insel0(&self) -> INSEL0_R {
        INSEL0_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - PhaseB/Hall input 2 selector"]
    #[inline(always)]
    pub fn insel1(&self) -> INSEL1_R {
        INSEL1_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Index/Hall input 3 selector"]
    #[inline(always)]
    pub fn insel2(&self) -> INSEL2_R {
        INSEL2_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Delay Pin selector"]
    #[inline(always)]
    pub fn dsel(&self) -> DSEL_R {
        DSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Edge selector for the sampling trigger"]
    #[inline(always)]
    pub fn spes(&self) -> SPES_R {
        SPES_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:20 - Pattern update signal select"]
    #[inline(always)]
    pub fn msets(&self) -> MSETS_R {
        MSETS_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 21 - Multi-Channel pattern update trigger edge"]
    #[inline(always)]
    pub fn mses(&self) -> MSES_R {
        MSES_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - PWM synchronization signal selector"]
    #[inline(always)]
    pub fn msyns(&self) -> MSYNS_R {
        MSYNS_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Wrong Hall Event selection"]
    #[inline(always)]
    pub fn ewis(&self) -> EWIS_R {
        EWIS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - External Wrong Hall Event enable"]
    #[inline(always)]
    pub fn ewie(&self) -> EWIE_R {
        EWIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - External Wrong Hall Event active level"]
    #[inline(always)]
    pub fn ewil(&self) -> EWIL_R {
        EWIL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - Low Pass Filters Configuration"]
    #[inline(always)]
    pub fn lpc(&self) -> LPC_R {
        LPC_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Function Selector"]
    #[inline(always)]
    pub fn fsel(&mut self) -> FSEL_W {
        FSEL_W { w: self }
    }
    #[doc = "Bit 2 - Position Decoder Mode selection"]
    #[inline(always)]
    pub fn qdcm(&mut self) -> QDCM_W {
        QDCM_W { w: self }
    }
    #[doc = "Bit 4 - Idle generation enable"]
    #[inline(always)]
    pub fn hidg(&mut self) -> HIDG_W {
        HIDG_W { w: self }
    }
    #[doc = "Bit 5 - Multi-Channel Pattern SW update enable"]
    #[inline(always)]
    pub fn mcue(&mut self) -> MCUE_W {
        MCUE_W { w: self }
    }
    #[doc = "Bits 8:9 - PhaseA/Hal input 1 selector"]
    #[inline(always)]
    pub fn insel0(&mut self) -> INSEL0_W {
        INSEL0_W { w: self }
    }
    #[doc = "Bits 10:11 - PhaseB/Hall input 2 selector"]
    #[inline(always)]
    pub fn insel1(&mut self) -> INSEL1_W {
        INSEL1_W { w: self }
    }
    #[doc = "Bits 12:13 - Index/Hall input 3 selector"]
    #[inline(always)]
    pub fn insel2(&mut self) -> INSEL2_W {
        INSEL2_W { w: self }
    }
    #[doc = "Bit 16 - Delay Pin selector"]
    #[inline(always)]
    pub fn dsel(&mut self) -> DSEL_W {
        DSEL_W { w: self }
    }
    #[doc = "Bit 17 - Edge selector for the sampling trigger"]
    #[inline(always)]
    pub fn spes(&mut self) -> SPES_W {
        SPES_W { w: self }
    }
    #[doc = "Bits 18:20 - Pattern update signal select"]
    #[inline(always)]
    pub fn msets(&mut self) -> MSETS_W {
        MSETS_W { w: self }
    }
    #[doc = "Bit 21 - Multi-Channel pattern update trigger edge"]
    #[inline(always)]
    pub fn mses(&mut self) -> MSES_W {
        MSES_W { w: self }
    }
    #[doc = "Bits 22:23 - PWM synchronization signal selector"]
    #[inline(always)]
    pub fn msyns(&mut self) -> MSYNS_W {
        MSYNS_W { w: self }
    }
    #[doc = "Bits 24:25 - Wrong Hall Event selection"]
    #[inline(always)]
    pub fn ewis(&mut self) -> EWIS_W {
        EWIS_W { w: self }
    }
    #[doc = "Bit 26 - External Wrong Hall Event enable"]
    #[inline(always)]
    pub fn ewie(&mut self) -> EWIE_W {
        EWIE_W { w: self }
    }
    #[doc = "Bit 27 - External Wrong Hall Event active level"]
    #[inline(always)]
    pub fn ewil(&mut self) -> EWIL_W {
        EWIL_W { w: self }
    }
    #[doc = "Bits 28:30 - Low Pass Filters Configuration"]
    #[inline(always)]
    pub fn lpc(&mut self) -> LPC_W {
        LPC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "POSIF configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pconf](index.html) module"]
pub struct PCONF_SPEC;
impl crate::RegisterSpec for PCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pconf::R](R) reader structure"]
impl crate::Readable for PCONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pconf::W](W) writer structure"]
impl crate::Writable for PCONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCONF to value 0"]
impl crate::Resettable for PCONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
