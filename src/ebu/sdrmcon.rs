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
#[doc = "Field `CRAS` reader - Row to precharge delay counter"]
pub type CRAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRAS` writer - Row to precharge delay counter"]
pub type CRAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRMCON_SPEC, u8, u8, 4, O>;
#[doc = "Field `CRFSH` reader - Initialization refresh commands counter"]
pub type CRFSH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRFSH` writer - Initialization refresh commands counter"]
pub type CRFSH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRMCON_SPEC, u8, u8, 4, O>;
#[doc = "Field `CRSC` reader - Mode register set-up time"]
pub type CRSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRSC` writer - Mode register set-up time"]
pub type CRSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRMCON_SPEC, u8, u8, 2, O>;
#[doc = "Field `CRP` reader - Row precharge time counter"]
pub type CRP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRP` writer - Row precharge time counter"]
pub type CRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRMCON_SPEC, u8, u8, 2, O>;
#[doc = "Field `AWIDTH` reader - Width of column address"]
pub type AWIDTH_R = crate::FieldReader<u8, AWIDTH_A>;
#[doc = "Width of column address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl AWIDTH_R {
    #[doc = "Get enumerated values variant"]
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
        *self == AWIDTH_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AWIDTH_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == AWIDTH_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == AWIDTH_A::VALUE4
    }
}
#[doc = "Field `AWIDTH` writer - Width of column address"]
pub type AWIDTH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SDRMCON_SPEC, u8, AWIDTH_A, 2, O>;
impl<'a, const O: u8> AWIDTH_W<'a, O> {
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
}
#[doc = "Field `CRCD` reader - Row to column delay counter"]
pub type CRCD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRCD` writer - Row to column delay counter"]
pub type CRCD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRMCON_SPEC, u8, u8, 2, O>;
#[doc = "Field `CRC` reader - Row cycle time counter"]
pub type CRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRC` writer - Row cycle time counter"]
pub type CRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRMCON_SPEC, u8, u8, 3, O>;
#[doc = "Field `ROWM` reader - Mask for row tag"]
pub type ROWM_R = crate::FieldReader<u8, ROWM_A>;
#[doc = "Mask for row tag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ROWM_R {
    #[doc = "Get enumerated values variant"]
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
        *self == ROWM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ROWM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ROWM_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == ROWM_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == ROWM_A::VALUE6
    }
}
#[doc = "Field `ROWM` writer - Mask for row tag"]
pub type ROWM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRMCON_SPEC, u8, ROWM_A, 3, O>;
impl<'a, const O: u8> ROWM_W<'a, O> {
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
}
#[doc = "Field `BANKM` reader - Mask for bank tag"]
pub type BANKM_R = crate::FieldReader<u8, BANKM_A>;
#[doc = "Mask for bank tag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl BANKM_R {
    #[doc = "Get enumerated values variant"]
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
        *self == BANKM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BANKM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BANKM_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == BANKM_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == BANKM_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == BANKM_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == BANKM_A::VALUE8
    }
}
#[doc = "Field `BANKM` writer - Mask for bank tag"]
pub type BANKM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRMCON_SPEC, u8, BANKM_A, 3, O>;
impl<'a, const O: u8> BANKM_W<'a, O> {
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
}
#[doc = "Field `CRCE` reader - Row cycle time counter extension"]
pub type CRCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRCE` writer - Row cycle time counter extension"]
pub type CRCE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRMCON_SPEC, u8, u8, 3, O>;
#[doc = "Field `CLKDIS` reader - Disable SDRAM clock output"]
pub type CLKDIS_R = crate::BitReader<CLKDIS_A>;
#[doc = "Disable SDRAM clock output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CLKDIS_R {
    #[doc = "Get enumerated values variant"]
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
        *self == CLKDIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CLKDIS_A::VALUE2
    }
}
#[doc = "Field `CLKDIS` writer - Disable SDRAM clock output"]
pub type CLKDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDRMCON_SPEC, CLKDIS_A, O>;
impl<'a, const O: u8> CLKDIS_W<'a, O> {
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
}
#[doc = "Field `PWR_MODE` reader - Power Save Mode used for gated clock mode"]
pub type PWR_MODE_R = crate::FieldReader<u8, PWR_MODE_A>;
#[doc = "Power Save Mode used for gated clock mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PWR_MODE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == PWR_MODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PWR_MODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PWR_MODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PWR_MODE_A::VALUE4
    }
}
#[doc = "Field `PWR_MODE` writer - Power Save Mode used for gated clock mode"]
pub type PWR_MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SDRMCON_SPEC, u8, PWR_MODE_A, 2, O>;
impl<'a, const O: u8> PWR_MODE_W<'a, O> {
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
}
#[doc = "Field `SDCMSEL` reader - SDRAM clock mode select"]
pub type SDCMSEL_R = crate::BitReader<SDCMSEL_A>;
#[doc = "SDRAM clock mode select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SDCMSEL_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SDCMSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SDCMSEL_A::VALUE2
    }
}
#[doc = "Field `SDCMSEL` writer - SDRAM clock mode select"]
pub type SDCMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDRMCON_SPEC, SDCMSEL_A, O>;
impl<'a, const O: u8> SDCMSEL_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:3 - Row to precharge delay counter"]
    #[inline(always)]
    pub fn cras(&self) -> CRAS_R {
        CRAS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Initialization refresh commands counter"]
    #[inline(always)]
    pub fn crfsh(&self) -> CRFSH_R {
        CRFSH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Mode register set-up time"]
    #[inline(always)]
    pub fn crsc(&self) -> CRSC_R {
        CRSC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Row precharge time counter"]
    #[inline(always)]
    pub fn crp(&self) -> CRP_R {
        CRP_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Width of column address"]
    #[inline(always)]
    pub fn awidth(&self) -> AWIDTH_R {
        AWIDTH_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Row to column delay counter"]
    #[inline(always)]
    pub fn crcd(&self) -> CRCD_R {
        CRCD_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Row cycle time counter"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Mask for row tag"]
    #[inline(always)]
    pub fn rowm(&self) -> ROWM_R {
        ROWM_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - Mask for bank tag"]
    #[inline(always)]
    pub fn bankm(&self) -> BANKM_R {
        BANKM_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - Row cycle time counter extension"]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - Disable SDRAM clock output"]
    #[inline(always)]
    pub fn clkdis(&self) -> CLKDIS_R {
        CLKDIS_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Power Save Mode used for gated clock mode"]
    #[inline(always)]
    pub fn pwr_mode(&self) -> PWR_MODE_R {
        PWR_MODE_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - SDRAM clock mode select"]
    #[inline(always)]
    pub fn sdcmsel(&self) -> SDCMSEL_R {
        SDCMSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Row to precharge delay counter"]
    #[inline(always)]
    #[must_use]
    pub fn cras(&mut self) -> CRAS_W<0> {
        CRAS_W::new(self)
    }
    #[doc = "Bits 4:7 - Initialization refresh commands counter"]
    #[inline(always)]
    #[must_use]
    pub fn crfsh(&mut self) -> CRFSH_W<4> {
        CRFSH_W::new(self)
    }
    #[doc = "Bits 8:9 - Mode register set-up time"]
    #[inline(always)]
    #[must_use]
    pub fn crsc(&mut self) -> CRSC_W<8> {
        CRSC_W::new(self)
    }
    #[doc = "Bits 10:11 - Row precharge time counter"]
    #[inline(always)]
    #[must_use]
    pub fn crp(&mut self) -> CRP_W<10> {
        CRP_W::new(self)
    }
    #[doc = "Bits 12:13 - Width of column address"]
    #[inline(always)]
    #[must_use]
    pub fn awidth(&mut self) -> AWIDTH_W<12> {
        AWIDTH_W::new(self)
    }
    #[doc = "Bits 14:15 - Row to column delay counter"]
    #[inline(always)]
    #[must_use]
    pub fn crcd(&mut self) -> CRCD_W<14> {
        CRCD_W::new(self)
    }
    #[doc = "Bits 16:18 - Row cycle time counter"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<16> {
        CRC_W::new(self)
    }
    #[doc = "Bits 19:21 - Mask for row tag"]
    #[inline(always)]
    #[must_use]
    pub fn rowm(&mut self) -> ROWM_W<19> {
        ROWM_W::new(self)
    }
    #[doc = "Bits 22:24 - Mask for bank tag"]
    #[inline(always)]
    #[must_use]
    pub fn bankm(&mut self) -> BANKM_W<22> {
        BANKM_W::new(self)
    }
    #[doc = "Bits 25:27 - Row cycle time counter extension"]
    #[inline(always)]
    #[must_use]
    pub fn crce(&mut self) -> CRCE_W<25> {
        CRCE_W::new(self)
    }
    #[doc = "Bit 28 - Disable SDRAM clock output"]
    #[inline(always)]
    #[must_use]
    pub fn clkdis(&mut self) -> CLKDIS_W<28> {
        CLKDIS_W::new(self)
    }
    #[doc = "Bits 29:30 - Power Save Mode used for gated clock mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_mode(&mut self) -> PWR_MODE_W<29> {
        PWR_MODE_W::new(self)
    }
    #[doc = "Bit 31 - SDRAM clock mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sdcmsel(&mut self) -> SDCMSEL_W<31> {
        SDCMSEL_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDRMCON to value 0x8000_0000"]
impl crate::Resettable for SDRMCON_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
