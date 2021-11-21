#[doc = "Register `PFLG` reader"]
pub struct R(crate::R<PFLG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFLG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFLG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFLG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Correct Hall Event Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHES_A {
    #[doc = "0: Correct Hall Event not detected"]
    VALUE1 = 0,
    #[doc = "1: Correct Hall Event detected"]
    VALUE2 = 1,
}
impl From<CHES_A> for bool {
    #[inline(always)]
    fn from(variant: CHES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHES` reader - Correct Hall Event Status"]
pub struct CHES_R(crate::FieldReader<bool, CHES_A>);
impl CHES_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHES_A {
        match self.bits {
            false => CHES_A::VALUE1,
            true => CHES_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CHES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CHES_A::VALUE2
    }
}
impl core::ops::Deref for CHES_R {
    type Target = crate::FieldReader<bool, CHES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Wrong Hall Event Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WHES_A {
    #[doc = "0: Wrong Hall Event not detected"]
    VALUE1 = 0,
    #[doc = "1: Wrong Hall Event detected"]
    VALUE2 = 1,
}
impl From<WHES_A> for bool {
    #[inline(always)]
    fn from(variant: WHES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WHES` reader - Wrong Hall Event Status"]
pub struct WHES_R(crate::FieldReader<bool, WHES_A>);
impl WHES_R {
    pub(crate) fn new(bits: bool) -> Self {
        WHES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WHES_A {
        match self.bits {
            false => WHES_A::VALUE1,
            true => WHES_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == WHES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WHES_A::VALUE2
    }
}
impl core::ops::Deref for WHES_R {
    type Target = crate::FieldReader<bool, WHES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Hall Inputs Update Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIES_A {
    #[doc = "0: Transition on the Hall Inputs not detected"]
    VALUE1 = 0,
    #[doc = "1: Transition on the Hall Inputs detected"]
    VALUE2 = 1,
}
impl From<HIES_A> for bool {
    #[inline(always)]
    fn from(variant: HIES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIES` reader - Hall Inputs Update Status"]
pub struct HIES_R(crate::FieldReader<bool, HIES_A>);
impl HIES_R {
    pub(crate) fn new(bits: bool) -> Self {
        HIES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIES_A {
        match self.bits {
            false => HIES_A::VALUE1,
            true => HIES_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == HIES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HIES_A::VALUE2
    }
}
impl core::ops::Deref for HIES_R {
    type Target = crate::FieldReader<bool, HIES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Multi-Channel pattern shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTS_A {
    #[doc = "0: Shadow transfer not done"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer done"]
    VALUE2 = 1,
}
impl From<MSTS_A> for bool {
    #[inline(always)]
    fn from(variant: MSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTS` reader - Multi-Channel pattern shadow transfer status"]
pub struct MSTS_R(crate::FieldReader<bool, MSTS_A>);
impl MSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTS_A {
        match self.bits {
            false => MSTS_A::VALUE1,
            true => MSTS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MSTS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MSTS_A::VALUE2
    }
}
impl core::ops::Deref for MSTS_R {
    type Target = crate::FieldReader<bool, MSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Quadrature Index Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INDXS_A {
    #[doc = "0: Index event not detected"]
    VALUE1 = 0,
    #[doc = "1: Index event detected"]
    VALUE2 = 1,
}
impl From<INDXS_A> for bool {
    #[inline(always)]
    fn from(variant: INDXS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INDXS` reader - Quadrature Index Status"]
pub struct INDXS_R(crate::FieldReader<bool, INDXS_A>);
impl INDXS_R {
    pub(crate) fn new(bits: bool) -> Self {
        INDXS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INDXS_A {
        match self.bits {
            false => INDXS_A::VALUE1,
            true => INDXS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == INDXS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == INDXS_A::VALUE2
    }
}
impl core::ops::Deref for INDXS_R {
    type Target = crate::FieldReader<bool, INDXS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Quadrature Phase Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRS_A {
    #[doc = "0: Phase Error event not detected"]
    VALUE1 = 0,
    #[doc = "1: Phase Error event detected"]
    VALUE2 = 1,
}
impl From<ERRS_A> for bool {
    #[inline(always)]
    fn from(variant: ERRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRS` reader - Quadrature Phase Error Status"]
pub struct ERRS_R(crate::FieldReader<bool, ERRS_A>);
impl ERRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRS_A {
        match self.bits {
            false => ERRS_A::VALUE1,
            true => ERRS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ERRS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ERRS_A::VALUE2
    }
}
impl core::ops::Deref for ERRS_R {
    type Target = crate::FieldReader<bool, ERRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Quadrature CLK Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTS_A {
    #[doc = "0: Quadrature clock not generated"]
    VALUE1 = 0,
    #[doc = "1: Quadrature clock generated"]
    VALUE2 = 1,
}
impl From<CNTS_A> for bool {
    #[inline(always)]
    fn from(variant: CNTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTS` reader - Quadrature CLK Status"]
pub struct CNTS_R(crate::FieldReader<bool, CNTS_A>);
impl CNTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTS_A {
        match self.bits {
            false => CNTS_A::VALUE1,
            true => CNTS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CNTS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CNTS_A::VALUE2
    }
}
impl core::ops::Deref for CNTS_R {
    type Target = crate::FieldReader<bool, CNTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Quadrature Direction Change\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRS_A {
    #[doc = "0: Change on direction not detected"]
    VALUE1 = 0,
    #[doc = "1: Change on direction detected"]
    VALUE2 = 1,
}
impl From<DIRS_A> for bool {
    #[inline(always)]
    fn from(variant: DIRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRS` reader - Quadrature Direction Change"]
pub struct DIRS_R(crate::FieldReader<bool, DIRS_A>);
impl DIRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRS_A {
        match self.bits {
            false => DIRS_A::VALUE1,
            true => DIRS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DIRS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DIRS_A::VALUE2
    }
}
impl core::ops::Deref for DIRS_R {
    type Target = crate::FieldReader<bool, DIRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Quadrature Period Clk Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLKS_A {
    #[doc = "0: Period clock not generated"]
    VALUE1 = 0,
    #[doc = "1: Period clock generated"]
    VALUE2 = 1,
}
impl From<PCLKS_A> for bool {
    #[inline(always)]
    fn from(variant: PCLKS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCLKS` reader - Quadrature Period Clk Status"]
pub struct PCLKS_R(crate::FieldReader<bool, PCLKS_A>);
impl PCLKS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLKS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLKS_A {
        match self.bits {
            false => PCLKS_A::VALUE1,
            true => PCLKS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PCLKS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PCLKS_A::VALUE2
    }
}
impl core::ops::Deref for PCLKS_R {
    type Target = crate::FieldReader<bool, PCLKS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Correct Hall Event Status"]
    #[inline(always)]
    pub fn ches(&self) -> CHES_R {
        CHES_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wrong Hall Event Status"]
    #[inline(always)]
    pub fn whes(&self) -> WHES_R {
        WHES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Hall Inputs Update Status"]
    #[inline(always)]
    pub fn hies(&self) -> HIES_R {
        HIES_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Multi-Channel pattern shadow transfer status"]
    #[inline(always)]
    pub fn msts(&self) -> MSTS_R {
        MSTS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Quadrature Index Status"]
    #[inline(always)]
    pub fn indxs(&self) -> INDXS_R {
        INDXS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Quadrature Phase Error Status"]
    #[inline(always)]
    pub fn errs(&self) -> ERRS_R {
        ERRS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Quadrature CLK Status"]
    #[inline(always)]
    pub fn cnts(&self) -> CNTS_R {
        CNTS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Quadrature Direction Change"]
    #[inline(always)]
    pub fn dirs(&self) -> DIRS_R {
        DIRS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Quadrature Period Clk Status"]
    #[inline(always)]
    pub fn pclks(&self) -> PCLKS_R {
        PCLKS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
#[doc = "POSIF Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pflg](index.html) module"]
pub struct PFLG_SPEC;
impl crate::RegisterSpec for PFLG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pflg::R](R) reader structure"]
impl crate::Readable for PFLG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PFLG to value 0"]
impl crate::Resettable for PFLG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
