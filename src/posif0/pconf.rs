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
#[doc = "Field `FSEL` reader - Function Selector"]
pub type FSEL_R = crate::FieldReader<u8, FSEL_A>;
#[doc = "Function Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl FSEL_R {
    #[doc = "Get enumerated values variant"]
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
        *self == FSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FSEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == FSEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == FSEL_A::VALUE4
    }
}
#[doc = "Field `FSEL` writer - Function Selector"]
pub type FSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PCONF_SPEC, u8, FSEL_A, 2, O>;
impl<'a, const O: u8> FSEL_W<'a, O> {
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
}
#[doc = "Field `QDCM` reader - Position Decoder Mode selection"]
pub type QDCM_R = crate::BitReader<QDCM_A>;
#[doc = "Position Decoder Mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl QDCM_R {
    #[doc = "Get enumerated values variant"]
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
        *self == QDCM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == QDCM_A::VALUE2
    }
}
#[doc = "Field `QDCM` writer - Position Decoder Mode selection"]
pub type QDCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONF_SPEC, QDCM_A, O>;
impl<'a, const O: u8> QDCM_W<'a, O> {
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
}
#[doc = "Field `HIDG` reader - Idle generation enable"]
pub type HIDG_R = crate::BitReader<bool>;
#[doc = "Field `HIDG` writer - Idle generation enable"]
pub type HIDG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONF_SPEC, bool, O>;
#[doc = "Field `MCUE` reader - Multi-Channel Pattern SW update enable"]
pub type MCUE_R = crate::BitReader<MCUE_A>;
#[doc = "Multi-Channel Pattern SW update enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MCUE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == MCUE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCUE_A::VALUE2
    }
}
#[doc = "Field `MCUE` writer - Multi-Channel Pattern SW update enable"]
pub type MCUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONF_SPEC, MCUE_A, O>;
impl<'a, const O: u8> MCUE_W<'a, O> {
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
}
#[doc = "Field `INSEL0` reader - PhaseA/Hal input 1 selector"]
pub type INSEL0_R = crate::FieldReader<u8, INSEL0_A>;
#[doc = "PhaseA/Hal input 1 selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl INSEL0_R {
    #[doc = "Get enumerated values variant"]
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
        *self == INSEL0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INSEL0_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == INSEL0_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == INSEL0_A::VALUE4
    }
}
#[doc = "Field `INSEL0` writer - PhaseA/Hal input 1 selector"]
pub type INSEL0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PCONF_SPEC, u8, INSEL0_A, 2, O>;
impl<'a, const O: u8> INSEL0_W<'a, O> {
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
}
#[doc = "Field `INSEL1` reader - PhaseB/Hall input 2 selector"]
pub type INSEL1_R = crate::FieldReader<u8, INSEL1_A>;
#[doc = "PhaseB/Hall input 2 selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl INSEL1_R {
    #[doc = "Get enumerated values variant"]
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
        *self == INSEL1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INSEL1_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == INSEL1_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == INSEL1_A::VALUE4
    }
}
#[doc = "Field `INSEL1` writer - PhaseB/Hall input 2 selector"]
pub type INSEL1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PCONF_SPEC, u8, INSEL1_A, 2, O>;
impl<'a, const O: u8> INSEL1_W<'a, O> {
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
}
#[doc = "Field `INSEL2` reader - Index/Hall input 3 selector"]
pub type INSEL2_R = crate::FieldReader<u8, INSEL2_A>;
#[doc = "Index/Hall input 3 selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl INSEL2_R {
    #[doc = "Get enumerated values variant"]
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
        *self == INSEL2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INSEL2_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == INSEL2_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == INSEL2_A::VALUE4
    }
}
#[doc = "Field `INSEL2` writer - Index/Hall input 3 selector"]
pub type INSEL2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PCONF_SPEC, u8, INSEL2_A, 2, O>;
impl<'a, const O: u8> INSEL2_W<'a, O> {
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
}
#[doc = "Field `DSEL` reader - Delay Pin selector"]
pub type DSEL_R = crate::BitReader<DSEL_A>;
#[doc = "Delay Pin selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DSEL_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DSEL_A::VALUE2
    }
}
#[doc = "Field `DSEL` writer - Delay Pin selector"]
pub type DSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONF_SPEC, DSEL_A, O>;
impl<'a, const O: u8> DSEL_W<'a, O> {
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
}
#[doc = "Field `SPES` reader - Edge selector for the sampling trigger"]
pub type SPES_R = crate::BitReader<SPES_A>;
#[doc = "Edge selector for the sampling trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SPES_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SPES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SPES_A::VALUE2
    }
}
#[doc = "Field `SPES` writer - Edge selector for the sampling trigger"]
pub type SPES_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONF_SPEC, SPES_A, O>;
impl<'a, const O: u8> SPES_W<'a, O> {
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
}
#[doc = "Field `MSETS` reader - Pattern update signal select"]
pub type MSETS_R = crate::FieldReader<u8, MSETS_A>;
#[doc = "Pattern update signal select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MSETS_R {
    #[doc = "Get enumerated values variant"]
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
        *self == MSETS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSETS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MSETS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MSETS_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == MSETS_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == MSETS_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == MSETS_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == MSETS_A::VALUE8
    }
}
#[doc = "Field `MSETS` writer - Pattern update signal select"]
pub type MSETS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PCONF_SPEC, u8, MSETS_A, 3, O>;
impl<'a, const O: u8> MSETS_W<'a, O> {
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
}
#[doc = "Field `MSES` reader - Multi-Channel pattern update trigger edge"]
pub type MSES_R = crate::BitReader<MSES_A>;
#[doc = "Multi-Channel pattern update trigger edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MSES_R {
    #[doc = "Get enumerated values variant"]
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
        *self == MSES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSES_A::VALUE2
    }
}
#[doc = "Field `MSES` writer - Multi-Channel pattern update trigger edge"]
pub type MSES_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONF_SPEC, MSES_A, O>;
impl<'a, const O: u8> MSES_W<'a, O> {
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
}
#[doc = "Field `MSYNS` reader - PWM synchronization signal selector"]
pub type MSYNS_R = crate::FieldReader<u8, MSYNS_A>;
#[doc = "PWM synchronization signal selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MSYNS_R {
    #[doc = "Get enumerated values variant"]
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
        *self == MSYNS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSYNS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MSYNS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MSYNS_A::VALUE4
    }
}
#[doc = "Field `MSYNS` writer - PWM synchronization signal selector"]
pub type MSYNS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PCONF_SPEC, u8, MSYNS_A, 2, O>;
impl<'a, const O: u8> MSYNS_W<'a, O> {
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
}
#[doc = "Field `EWIS` reader - Wrong Hall Event selection"]
pub type EWIS_R = crate::FieldReader<u8, EWIS_A>;
#[doc = "Wrong Hall Event selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl EWIS_R {
    #[doc = "Get enumerated values variant"]
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
        *self == EWIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EWIS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EWIS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EWIS_A::VALUE4
    }
}
#[doc = "Field `EWIS` writer - Wrong Hall Event selection"]
pub type EWIS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PCONF_SPEC, u8, EWIS_A, 2, O>;
impl<'a, const O: u8> EWIS_W<'a, O> {
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
}
#[doc = "Field `EWIE` reader - External Wrong Hall Event enable"]
pub type EWIE_R = crate::BitReader<EWIE_A>;
#[doc = "External Wrong Hall Event enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl EWIE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == EWIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EWIE_A::VALUE2
    }
}
#[doc = "Field `EWIE` writer - External Wrong Hall Event enable"]
pub type EWIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONF_SPEC, EWIE_A, O>;
impl<'a, const O: u8> EWIE_W<'a, O> {
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
}
#[doc = "Field `EWIL` reader - External Wrong Hall Event active level"]
pub type EWIL_R = crate::BitReader<EWIL_A>;
#[doc = "External Wrong Hall Event active level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl EWIL_R {
    #[doc = "Get enumerated values variant"]
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
        *self == EWIL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EWIL_A::VALUE2
    }
}
#[doc = "Field `EWIL` writer - External Wrong Hall Event active level"]
pub type EWIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONF_SPEC, EWIL_A, O>;
impl<'a, const O: u8> EWIL_W<'a, O> {
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
}
#[doc = "Field `LPC` reader - Low Pass Filters Configuration"]
pub type LPC_R = crate::FieldReader<u8, LPC_A>;
#[doc = "Low Pass Filters Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LPC_R {
    #[doc = "Get enumerated values variant"]
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
        *self == LPC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LPC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LPC_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == LPC_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == LPC_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == LPC_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == LPC_A::VALUE8
    }
}
#[doc = "Field `LPC` writer - Low Pass Filters Configuration"]
pub type LPC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PCONF_SPEC, u8, LPC_A, 3, O>;
impl<'a, const O: u8> LPC_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:1 - Function Selector"]
    #[inline(always)]
    pub fn fsel(&self) -> FSEL_R {
        FSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Position Decoder Mode selection"]
    #[inline(always)]
    pub fn qdcm(&self) -> QDCM_R {
        QDCM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Idle generation enable"]
    #[inline(always)]
    pub fn hidg(&self) -> HIDG_R {
        HIDG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi-Channel Pattern SW update enable"]
    #[inline(always)]
    pub fn mcue(&self) -> MCUE_R {
        MCUE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9 - PhaseA/Hal input 1 selector"]
    #[inline(always)]
    pub fn insel0(&self) -> INSEL0_R {
        INSEL0_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PhaseB/Hall input 2 selector"]
    #[inline(always)]
    pub fn insel1(&self) -> INSEL1_R {
        INSEL1_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Index/Hall input 3 selector"]
    #[inline(always)]
    pub fn insel2(&self) -> INSEL2_R {
        INSEL2_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - Delay Pin selector"]
    #[inline(always)]
    pub fn dsel(&self) -> DSEL_R {
        DSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Edge selector for the sampling trigger"]
    #[inline(always)]
    pub fn spes(&self) -> SPES_R {
        SPES_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Pattern update signal select"]
    #[inline(always)]
    pub fn msets(&self) -> MSETS_R {
        MSETS_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - Multi-Channel pattern update trigger edge"]
    #[inline(always)]
    pub fn mses(&self) -> MSES_R {
        MSES_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - PWM synchronization signal selector"]
    #[inline(always)]
    pub fn msyns(&self) -> MSYNS_R {
        MSYNS_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Wrong Hall Event selection"]
    #[inline(always)]
    pub fn ewis(&self) -> EWIS_R {
        EWIS_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - External Wrong Hall Event enable"]
    #[inline(always)]
    pub fn ewie(&self) -> EWIE_R {
        EWIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - External Wrong Hall Event active level"]
    #[inline(always)]
    pub fn ewil(&self) -> EWIL_R {
        EWIL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Low Pass Filters Configuration"]
    #[inline(always)]
    pub fn lpc(&self) -> LPC_R {
        LPC_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Function Selector"]
    #[inline(always)]
    #[must_use]
    pub fn fsel(&mut self) -> FSEL_W<0> {
        FSEL_W::new(self)
    }
    #[doc = "Bit 2 - Position Decoder Mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn qdcm(&mut self) -> QDCM_W<2> {
        QDCM_W::new(self)
    }
    #[doc = "Bit 4 - Idle generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn hidg(&mut self) -> HIDG_W<4> {
        HIDG_W::new(self)
    }
    #[doc = "Bit 5 - Multi-Channel Pattern SW update enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcue(&mut self) -> MCUE_W<5> {
        MCUE_W::new(self)
    }
    #[doc = "Bits 8:9 - PhaseA/Hal input 1 selector"]
    #[inline(always)]
    #[must_use]
    pub fn insel0(&mut self) -> INSEL0_W<8> {
        INSEL0_W::new(self)
    }
    #[doc = "Bits 10:11 - PhaseB/Hall input 2 selector"]
    #[inline(always)]
    #[must_use]
    pub fn insel1(&mut self) -> INSEL1_W<10> {
        INSEL1_W::new(self)
    }
    #[doc = "Bits 12:13 - Index/Hall input 3 selector"]
    #[inline(always)]
    #[must_use]
    pub fn insel2(&mut self) -> INSEL2_W<12> {
        INSEL2_W::new(self)
    }
    #[doc = "Bit 16 - Delay Pin selector"]
    #[inline(always)]
    #[must_use]
    pub fn dsel(&mut self) -> DSEL_W<16> {
        DSEL_W::new(self)
    }
    #[doc = "Bit 17 - Edge selector for the sampling trigger"]
    #[inline(always)]
    #[must_use]
    pub fn spes(&mut self) -> SPES_W<17> {
        SPES_W::new(self)
    }
    #[doc = "Bits 18:20 - Pattern update signal select"]
    #[inline(always)]
    #[must_use]
    pub fn msets(&mut self) -> MSETS_W<18> {
        MSETS_W::new(self)
    }
    #[doc = "Bit 21 - Multi-Channel pattern update trigger edge"]
    #[inline(always)]
    #[must_use]
    pub fn mses(&mut self) -> MSES_W<21> {
        MSES_W::new(self)
    }
    #[doc = "Bits 22:23 - PWM synchronization signal selector"]
    #[inline(always)]
    #[must_use]
    pub fn msyns(&mut self) -> MSYNS_W<22> {
        MSYNS_W::new(self)
    }
    #[doc = "Bits 24:25 - Wrong Hall Event selection"]
    #[inline(always)]
    #[must_use]
    pub fn ewis(&mut self) -> EWIS_W<24> {
        EWIS_W::new(self)
    }
    #[doc = "Bit 26 - External Wrong Hall Event enable"]
    #[inline(always)]
    #[must_use]
    pub fn ewie(&mut self) -> EWIE_W<26> {
        EWIE_W::new(self)
    }
    #[doc = "Bit 27 - External Wrong Hall Event active level"]
    #[inline(always)]
    #[must_use]
    pub fn ewil(&mut self) -> EWIL_W<27> {
        EWIL_W::new(self)
    }
    #[doc = "Bits 28:30 - Low Pass Filters Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn lpc(&mut self) -> LPC_W<28> {
        LPC_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCONF to value 0"]
impl crate::Resettable for PCONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
