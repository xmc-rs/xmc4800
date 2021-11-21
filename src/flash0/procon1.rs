#[doc = "Register `PROCON1` reader"]
pub struct R(crate::R<PROCON1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROCON1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROCON1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROCON1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Sector 0 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2 = 1,
}
impl From<S0L_A> for bool {
    #[inline(always)]
    fn from(variant: S0L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0L` reader - Sector 0 Locked for Write Protection by User 1"]
pub struct S0L_R(crate::FieldReader<bool, S0L_A>);
impl S0L_R {
    pub(crate) fn new(bits: bool) -> Self {
        S0L_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0L_A {
        match self.bits {
            false => S0L_A::VALUE1,
            true => S0L_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S0L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S0L_A::VALUE2
    }
}
impl core::ops::Deref for S0L_R {
    type Target = crate::FieldReader<bool, S0L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sector 1 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2 = 1,
}
impl From<S1L_A> for bool {
    #[inline(always)]
    fn from(variant: S1L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1L` reader - Sector 1 Locked for Write Protection by User 1"]
pub struct S1L_R(crate::FieldReader<bool, S1L_A>);
impl S1L_R {
    pub(crate) fn new(bits: bool) -> Self {
        S1L_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1L_A {
        match self.bits {
            false => S1L_A::VALUE1,
            true => S1L_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S1L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S1L_A::VALUE2
    }
}
impl core::ops::Deref for S1L_R {
    type Target = crate::FieldReader<bool, S1L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sector 2 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2 = 1,
}
impl From<S2L_A> for bool {
    #[inline(always)]
    fn from(variant: S2L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2L` reader - Sector 2 Locked for Write Protection by User 1"]
pub struct S2L_R(crate::FieldReader<bool, S2L_A>);
impl S2L_R {
    pub(crate) fn new(bits: bool) -> Self {
        S2L_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S2L_A {
        match self.bits {
            false => S2L_A::VALUE1,
            true => S2L_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S2L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S2L_A::VALUE2
    }
}
impl core::ops::Deref for S2L_R {
    type Target = crate::FieldReader<bool, S2L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sector 3 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2 = 1,
}
impl From<S3L_A> for bool {
    #[inline(always)]
    fn from(variant: S3L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3L` reader - Sector 3 Locked for Write Protection by User 1"]
pub struct S3L_R(crate::FieldReader<bool, S3L_A>);
impl S3L_R {
    pub(crate) fn new(bits: bool) -> Self {
        S3L_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S3L_A {
        match self.bits {
            false => S3L_A::VALUE1,
            true => S3L_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S3L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S3L_A::VALUE2
    }
}
impl core::ops::Deref for S3L_R {
    type Target = crate::FieldReader<bool, S3L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sector 4 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S4L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2 = 1,
}
impl From<S4L_A> for bool {
    #[inline(always)]
    fn from(variant: S4L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S4L` reader - Sector 4 Locked for Write Protection by User 1"]
pub struct S4L_R(crate::FieldReader<bool, S4L_A>);
impl S4L_R {
    pub(crate) fn new(bits: bool) -> Self {
        S4L_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S4L_A {
        match self.bits {
            false => S4L_A::VALUE1,
            true => S4L_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S4L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S4L_A::VALUE2
    }
}
impl core::ops::Deref for S4L_R {
    type Target = crate::FieldReader<bool, S4L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sector 5 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S5L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2 = 1,
}
impl From<S5L_A> for bool {
    #[inline(always)]
    fn from(variant: S5L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S5L` reader - Sector 5 Locked for Write Protection by User 1"]
pub struct S5L_R(crate::FieldReader<bool, S5L_A>);
impl S5L_R {
    pub(crate) fn new(bits: bool) -> Self {
        S5L_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S5L_A {
        match self.bits {
            false => S5L_A::VALUE1,
            true => S5L_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S5L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S5L_A::VALUE2
    }
}
impl core::ops::Deref for S5L_R {
    type Target = crate::FieldReader<bool, S5L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sector 6 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S6L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2 = 1,
}
impl From<S6L_A> for bool {
    #[inline(always)]
    fn from(variant: S6L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S6L` reader - Sector 6 Locked for Write Protection by User 1"]
pub struct S6L_R(crate::FieldReader<bool, S6L_A>);
impl S6L_R {
    pub(crate) fn new(bits: bool) -> Self {
        S6L_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S6L_A {
        match self.bits {
            false => S6L_A::VALUE1,
            true => S6L_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S6L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S6L_A::VALUE2
    }
}
impl core::ops::Deref for S6L_R {
    type Target = crate::FieldReader<bool, S6L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sector 7 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S7L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2 = 1,
}
impl From<S7L_A> for bool {
    #[inline(always)]
    fn from(variant: S7L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S7L` reader - Sector 7 Locked for Write Protection by User 1"]
pub struct S7L_R(crate::FieldReader<bool, S7L_A>);
impl S7L_R {
    pub(crate) fn new(bits: bool) -> Self {
        S7L_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S7L_A {
        match self.bits {
            false => S7L_A::VALUE1,
            true => S7L_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S7L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S7L_A::VALUE2
    }
}
impl core::ops::Deref for S7L_R {
    type Target = crate::FieldReader<bool, S7L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sector 8 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S8L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2 = 1,
}
impl From<S8L_A> for bool {
    #[inline(always)]
    fn from(variant: S8L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S8L` reader - Sector 8 Locked for Write Protection by User 1"]
pub struct S8L_R(crate::FieldReader<bool, S8L_A>);
impl S8L_R {
    pub(crate) fn new(bits: bool) -> Self {
        S8L_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S8L_A {
        match self.bits {
            false => S8L_A::VALUE1,
            true => S8L_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S8L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S8L_A::VALUE2
    }
}
impl core::ops::Deref for S8L_R {
    type Target = crate::FieldReader<bool, S8L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sector 9 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S9L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2 = 1,
}
impl From<S9L_A> for bool {
    #[inline(always)]
    fn from(variant: S9L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S9L` reader - Sector 9 Locked for Write Protection by User 1"]
pub struct S9L_R(crate::FieldReader<bool, S9L_A>);
impl S9L_R {
    pub(crate) fn new(bits: bool) -> Self {
        S9L_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S9L_A {
        match self.bits {
            false => S9L_A::VALUE1,
            true => S9L_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S9L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S9L_A::VALUE2
    }
}
impl core::ops::Deref for S9L_R {
    type Target = crate::FieldReader<bool, S9L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sectors 10 and 11 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S10_S11L_A {
    #[doc = "0: No write protection is configured for sectors 10+11."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sectors 10+11."]
    VALUE2 = 1,
}
impl From<S10_S11L_A> for bool {
    #[inline(always)]
    fn from(variant: S10_S11L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S10_S11L` reader - Sectors 10 and 11 Locked for Write Protection by User 1"]
pub struct S10_S11L_R(crate::FieldReader<bool, S10_S11L_A>);
impl S10_S11L_R {
    pub(crate) fn new(bits: bool) -> Self {
        S10_S11L_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S10_S11L_A {
        match self.bits {
            false => S10_S11L_A::VALUE1,
            true => S10_S11L_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S10_S11L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S10_S11L_A::VALUE2
    }
}
impl core::ops::Deref for S10_S11L_R {
    type Target = crate::FieldReader<bool, S10_S11L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sectors 12 and 13 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S12_S13L_A {
    #[doc = "0: No write protection is configured for sectors 12+13."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sectors 12+13."]
    VALUE2 = 1,
}
impl From<S12_S13L_A> for bool {
    #[inline(always)]
    fn from(variant: S12_S13L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S12_S13L` reader - Sectors 12 and 13 Locked for Write Protection by User 1"]
pub struct S12_S13L_R(crate::FieldReader<bool, S12_S13L_A>);
impl S12_S13L_R {
    pub(crate) fn new(bits: bool) -> Self {
        S12_S13L_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S12_S13L_A {
        match self.bits {
            false => S12_S13L_A::VALUE1,
            true => S12_S13L_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S12_S13L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S12_S13L_A::VALUE2
    }
}
impl core::ops::Deref for S12_S13L_R {
    type Target = crate::FieldReader<bool, S12_S13L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sectors 14 and 15 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S14_S15L_A {
    #[doc = "0: No write protection is configured for sectors 14+15."]
    VALUE1 = 0,
    #[doc = "1: Write protection is configured for sectors 14+15."]
    VALUE2 = 1,
}
impl From<S14_S15L_A> for bool {
    #[inline(always)]
    fn from(variant: S14_S15L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S14_S15L` reader - Sectors 14 and 15 Locked for Write Protection by User 1"]
pub struct S14_S15L_R(crate::FieldReader<bool, S14_S15L_A>);
impl S14_S15L_R {
    pub(crate) fn new(bits: bool) -> Self {
        S14_S15L_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S14_S15L_A {
        match self.bits {
            false => S14_S15L_A::VALUE1,
            true => S14_S15L_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S14_S15L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S14_S15L_A::VALUE2
    }
}
impl core::ops::Deref for S14_S15L_R {
    type Target = crate::FieldReader<bool, S14_S15L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Physical Sector Repair\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSR_A {
    #[doc = "0: Physical Sector Repair command disabled; Erase Physical Sector command sequence available."]
    VALUE1 = 0,
    #[doc = "1: Physical Sector Repair command sequence enabled; Erase Physical Sector command sequence disabled."]
    VALUE2 = 1,
}
impl From<PSR_A> for bool {
    #[inline(always)]
    fn from(variant: PSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSR` reader - Physical Sector Repair"]
pub struct PSR_R(crate::FieldReader<bool, PSR_A>);
impl PSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSR_A {
        match self.bits {
            false => PSR_A::VALUE1,
            true => PSR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PSR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PSR_A::VALUE2
    }
}
impl core::ops::Deref for PSR_R {
    type Target = crate::FieldReader<bool, PSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Sector 0 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s0l(&self) -> S0L_R {
        S0L_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sector 1 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s1l(&self) -> S1L_R {
        S1L_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sector 2 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s2l(&self) -> S2L_R {
        S2L_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sector 3 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s3l(&self) -> S3L_R {
        S3L_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sector 4 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s4l(&self) -> S4L_R {
        S4L_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sector 5 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s5l(&self) -> S5L_R {
        S5L_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Sector 6 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s6l(&self) -> S6L_R {
        S6L_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Sector 7 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s7l(&self) -> S7L_R {
        S7L_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Sector 8 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s8l(&self) -> S8L_R {
        S8L_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Sector 9 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s9l(&self) -> S9L_R {
        S9L_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Sectors 10 and 11 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s10_s11l(&self) -> S10_S11L_R {
        S10_S11L_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Sectors 12 and 13 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s12_s13l(&self) -> S12_S13L_R {
        S12_S13L_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Sectors 14 and 15 Locked for Write Protection by User 1"]
    #[inline(always)]
    pub fn s14_s15l(&self) -> S14_S15L_R {
        S14_S15L_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Physical Sector Repair"]
    #[inline(always)]
    pub fn psr(&self) -> PSR_R {
        PSR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "Flash Protection Configuration Register User 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [procon1](index.html) module"]
pub struct PROCON1_SPEC;
impl crate::RegisterSpec for PROCON1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [procon1::R](R) reader structure"]
impl crate::Readable for PROCON1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PROCON1 to value 0"]
impl crate::Resettable for PROCON1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
