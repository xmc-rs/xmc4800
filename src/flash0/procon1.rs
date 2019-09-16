#[doc = "Reader of register PROCON1"]
pub type R = crate::R<u32, super::PROCON1>;
#[doc = "Sector 0 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2,
}
impl From<S0L_A> for bool {
    #[inline(always)]
    fn from(variant: S0L_A) -> Self {
        match variant {
            S0L_A::VALUE1 => false,
            S0L_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `S0L`"]
pub type S0L_R = crate::R<bool, S0L_A>;
impl S0L_R {
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
        *self == S0L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0L_A::VALUE2
    }
}
#[doc = "Sector 1 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2,
}
impl From<S1L_A> for bool {
    #[inline(always)]
    fn from(variant: S1L_A) -> Self {
        match variant {
            S1L_A::VALUE1 => false,
            S1L_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `S1L`"]
pub type S1L_R = crate::R<bool, S1L_A>;
impl S1L_R {
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
        *self == S1L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1L_A::VALUE2
    }
}
#[doc = "Sector 2 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2,
}
impl From<S2L_A> for bool {
    #[inline(always)]
    fn from(variant: S2L_A) -> Self {
        match variant {
            S2L_A::VALUE1 => false,
            S2L_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `S2L`"]
pub type S2L_R = crate::R<bool, S2L_A>;
impl S2L_R {
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
        *self == S2L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S2L_A::VALUE2
    }
}
#[doc = "Sector 3 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2,
}
impl From<S3L_A> for bool {
    #[inline(always)]
    fn from(variant: S3L_A) -> Self {
        match variant {
            S3L_A::VALUE1 => false,
            S3L_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `S3L`"]
pub type S3L_R = crate::R<bool, S3L_A>;
impl S3L_R {
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
        *self == S3L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S3L_A::VALUE2
    }
}
#[doc = "Sector 4 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S4L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2,
}
impl From<S4L_A> for bool {
    #[inline(always)]
    fn from(variant: S4L_A) -> Self {
        match variant {
            S4L_A::VALUE1 => false,
            S4L_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `S4L`"]
pub type S4L_R = crate::R<bool, S4L_A>;
impl S4L_R {
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
        *self == S4L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S4L_A::VALUE2
    }
}
#[doc = "Sector 5 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S5L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2,
}
impl From<S5L_A> for bool {
    #[inline(always)]
    fn from(variant: S5L_A) -> Self {
        match variant {
            S5L_A::VALUE1 => false,
            S5L_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `S5L`"]
pub type S5L_R = crate::R<bool, S5L_A>;
impl S5L_R {
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
        *self == S5L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S5L_A::VALUE2
    }
}
#[doc = "Sector 6 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S6L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2,
}
impl From<S6L_A> for bool {
    #[inline(always)]
    fn from(variant: S6L_A) -> Self {
        match variant {
            S6L_A::VALUE1 => false,
            S6L_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `S6L`"]
pub type S6L_R = crate::R<bool, S6L_A>;
impl S6L_R {
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
        *self == S6L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S6L_A::VALUE2
    }
}
#[doc = "Sector 7 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S7L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2,
}
impl From<S7L_A> for bool {
    #[inline(always)]
    fn from(variant: S7L_A) -> Self {
        match variant {
            S7L_A::VALUE1 => false,
            S7L_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `S7L`"]
pub type S7L_R = crate::R<bool, S7L_A>;
impl S7L_R {
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
        *self == S7L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S7L_A::VALUE2
    }
}
#[doc = "Sector 8 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S8L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2,
}
impl From<S8L_A> for bool {
    #[inline(always)]
    fn from(variant: S8L_A) -> Self {
        match variant {
            S8L_A::VALUE1 => false,
            S8L_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `S8L`"]
pub type S8L_R = crate::R<bool, S8L_A>;
impl S8L_R {
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
        *self == S8L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S8L_A::VALUE2
    }
}
#[doc = "Sector 9 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S9L_A {
    #[doc = "0: No write protection is configured for sector n."]
    VALUE1,
    #[doc = "1: Write protection is configured for sector n."]
    VALUE2,
}
impl From<S9L_A> for bool {
    #[inline(always)]
    fn from(variant: S9L_A) -> Self {
        match variant {
            S9L_A::VALUE1 => false,
            S9L_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `S9L`"]
pub type S9L_R = crate::R<bool, S9L_A>;
impl S9L_R {
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
        *self == S9L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S9L_A::VALUE2
    }
}
#[doc = "Sectors 10 and 11 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S10_S11L_A {
    #[doc = "0: No write protection is configured for sectors 10+11."]
    VALUE1,
    #[doc = "1: Write protection is configured for sectors 10+11."]
    VALUE2,
}
impl From<S10_S11L_A> for bool {
    #[inline(always)]
    fn from(variant: S10_S11L_A) -> Self {
        match variant {
            S10_S11L_A::VALUE1 => false,
            S10_S11L_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `S10_S11L`"]
pub type S10_S11L_R = crate::R<bool, S10_S11L_A>;
impl S10_S11L_R {
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
        *self == S10_S11L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S10_S11L_A::VALUE2
    }
}
#[doc = "Sectors 12 and 13 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S12_S13L_A {
    #[doc = "0: No write protection is configured for sectors 12+13."]
    VALUE1,
    #[doc = "1: Write protection is configured for sectors 12+13."]
    VALUE2,
}
impl From<S12_S13L_A> for bool {
    #[inline(always)]
    fn from(variant: S12_S13L_A) -> Self {
        match variant {
            S12_S13L_A::VALUE1 => false,
            S12_S13L_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `S12_S13L`"]
pub type S12_S13L_R = crate::R<bool, S12_S13L_A>;
impl S12_S13L_R {
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
        *self == S12_S13L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S12_S13L_A::VALUE2
    }
}
#[doc = "Sectors 14 and 15 Locked for Write Protection by User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S14_S15L_A {
    #[doc = "0: No write protection is configured for sectors 14+15."]
    VALUE1,
    #[doc = "1: Write protection is configured for sectors 14+15."]
    VALUE2,
}
impl From<S14_S15L_A> for bool {
    #[inline(always)]
    fn from(variant: S14_S15L_A) -> Self {
        match variant {
            S14_S15L_A::VALUE1 => false,
            S14_S15L_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `S14_S15L`"]
pub type S14_S15L_R = crate::R<bool, S14_S15L_A>;
impl S14_S15L_R {
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
        *self == S14_S15L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S14_S15L_A::VALUE2
    }
}
#[doc = "Physical Sector Repair\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSR_A {
    #[doc = "0: Physical Sector Repair command disabled; Erase Physical Sector command sequence available."]
    VALUE1,
    #[doc = "1: Physical Sector Repair command sequence enabled; Erase Physical Sector command sequence disabled."]
    VALUE2,
}
impl From<PSR_A> for bool {
    #[inline(always)]
    fn from(variant: PSR_A) -> Self {
        match variant {
            PSR_A::VALUE1 => false,
            PSR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PSR`"]
pub type PSR_R = crate::R<bool, PSR_A>;
impl PSR_R {
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
        *self == PSR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSR_A::VALUE2
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
