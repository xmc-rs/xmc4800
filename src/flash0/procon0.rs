#[doc = "Register `PROCON0` reader"]
pub type R = crate::R<Procon0Spec>;
#[doc = "Sector 0 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0l {
    #[doc = "0: No write protection is configured for sector n."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    Value2 = 1,
}
impl From<S0l> for bool {
    #[inline(always)]
    fn from(variant: S0l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0L` reader - Sector 0 Locked for Write Protection by User 0"]
pub type S0lR = crate::BitReader<S0l>;
impl S0lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0l {
        match self.bits {
            false => S0l::Value1,
            true => S0l::Value2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0l::Value1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0l::Value2
    }
}
#[doc = "Sector 1 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1l {
    #[doc = "0: No write protection is configured for sector n."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    Value2 = 1,
}
impl From<S1l> for bool {
    #[inline(always)]
    fn from(variant: S1l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1L` reader - Sector 1 Locked for Write Protection by User 0"]
pub type S1lR = crate::BitReader<S1l>;
impl S1lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1l {
        match self.bits {
            false => S1l::Value1,
            true => S1l::Value2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1l::Value1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1l::Value2
    }
}
#[doc = "Sector 2 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2l {
    #[doc = "0: No write protection is configured for sector n."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    Value2 = 1,
}
impl From<S2l> for bool {
    #[inline(always)]
    fn from(variant: S2l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2L` reader - Sector 2 Locked for Write Protection by User 0"]
pub type S2lR = crate::BitReader<S2l>;
impl S2lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2l {
        match self.bits {
            false => S2l::Value1,
            true => S2l::Value2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S2l::Value1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S2l::Value2
    }
}
#[doc = "Sector 3 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3l {
    #[doc = "0: No write protection is configured for sector n."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    Value2 = 1,
}
impl From<S3l> for bool {
    #[inline(always)]
    fn from(variant: S3l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3L` reader - Sector 3 Locked for Write Protection by User 0"]
pub type S3lR = crate::BitReader<S3l>;
impl S3lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S3l {
        match self.bits {
            false => S3l::Value1,
            true => S3l::Value2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S3l::Value1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S3l::Value2
    }
}
#[doc = "Sector 4 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S4l {
    #[doc = "0: No write protection is configured for sector n."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    Value2 = 1,
}
impl From<S4l> for bool {
    #[inline(always)]
    fn from(variant: S4l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S4L` reader - Sector 4 Locked for Write Protection by User 0"]
pub type S4lR = crate::BitReader<S4l>;
impl S4lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S4l {
        match self.bits {
            false => S4l::Value1,
            true => S4l::Value2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S4l::Value1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S4l::Value2
    }
}
#[doc = "Sector 5 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S5l {
    #[doc = "0: No write protection is configured for sector n."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    Value2 = 1,
}
impl From<S5l> for bool {
    #[inline(always)]
    fn from(variant: S5l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S5L` reader - Sector 5 Locked for Write Protection by User 0"]
pub type S5lR = crate::BitReader<S5l>;
impl S5lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S5l {
        match self.bits {
            false => S5l::Value1,
            true => S5l::Value2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S5l::Value1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S5l::Value2
    }
}
#[doc = "Sector 6 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S6l {
    #[doc = "0: No write protection is configured for sector n."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    Value2 = 1,
}
impl From<S6l> for bool {
    #[inline(always)]
    fn from(variant: S6l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S6L` reader - Sector 6 Locked for Write Protection by User 0"]
pub type S6lR = crate::BitReader<S6l>;
impl S6lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S6l {
        match self.bits {
            false => S6l::Value1,
            true => S6l::Value2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S6l::Value1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S6l::Value2
    }
}
#[doc = "Sector 7 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S7l {
    #[doc = "0: No write protection is configured for sector n."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    Value2 = 1,
}
impl From<S7l> for bool {
    #[inline(always)]
    fn from(variant: S7l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S7L` reader - Sector 7 Locked for Write Protection by User 0"]
pub type S7lR = crate::BitReader<S7l>;
impl S7lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S7l {
        match self.bits {
            false => S7l::Value1,
            true => S7l::Value2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S7l::Value1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S7l::Value2
    }
}
#[doc = "Sector 8 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S8l {
    #[doc = "0: No write protection is configured for sector n."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    Value2 = 1,
}
impl From<S8l> for bool {
    #[inline(always)]
    fn from(variant: S8l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S8L` reader - Sector 8 Locked for Write Protection by User 0"]
pub type S8lR = crate::BitReader<S8l>;
impl S8lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S8l {
        match self.bits {
            false => S8l::Value1,
            true => S8l::Value2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S8l::Value1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S8l::Value2
    }
}
#[doc = "Sector 9 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S9l {
    #[doc = "0: No write protection is configured for sector n."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sector n."]
    Value2 = 1,
}
impl From<S9l> for bool {
    #[inline(always)]
    fn from(variant: S9l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S9L` reader - Sector 9 Locked for Write Protection by User 0"]
pub type S9lR = crate::BitReader<S9l>;
impl S9lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S9l {
        match self.bits {
            false => S9l::Value1,
            true => S9l::Value2,
        }
    }
    #[doc = "No write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S9l::Value1
    }
    #[doc = "Write protection is configured for sector n."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S9l::Value2
    }
}
#[doc = "Sectors 10 and 11 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S10S11l {
    #[doc = "0: No write protection is configured for sectors 10+11."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sectors 10+11."]
    Value2 = 1,
}
impl From<S10S11l> for bool {
    #[inline(always)]
    fn from(variant: S10S11l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S10_S11L` reader - Sectors 10 and 11 Locked for Write Protection by User 0"]
pub type S10S11lR = crate::BitReader<S10S11l>;
impl S10S11lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S10S11l {
        match self.bits {
            false => S10S11l::Value1,
            true => S10S11l::Value2,
        }
    }
    #[doc = "No write protection is configured for sectors 10+11."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S10S11l::Value1
    }
    #[doc = "Write protection is configured for sectors 10+11."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S10S11l::Value2
    }
}
#[doc = "Sectors 12 and 13 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S12S13l {
    #[doc = "0: No write protection is configured for sectors 12+13."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sectors 12+13."]
    Value2 = 1,
}
impl From<S12S13l> for bool {
    #[inline(always)]
    fn from(variant: S12S13l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S12_S13L` reader - Sectors 12 and 13 Locked for Write Protection by User 0"]
pub type S12S13lR = crate::BitReader<S12S13l>;
impl S12S13lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S12S13l {
        match self.bits {
            false => S12S13l::Value1,
            true => S12S13l::Value2,
        }
    }
    #[doc = "No write protection is configured for sectors 12+13."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S12S13l::Value1
    }
    #[doc = "Write protection is configured for sectors 12+13."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S12S13l::Value2
    }
}
#[doc = "Sectors 14 and 15 Locked for Write Protection by User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S14S15l {
    #[doc = "0: No write protection is configured for sectors 14+15."]
    Value1 = 0,
    #[doc = "1: Write protection is configured for sectors 14+15."]
    Value2 = 1,
}
impl From<S14S15l> for bool {
    #[inline(always)]
    fn from(variant: S14S15l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S14_S15L` reader - Sectors 14 and 15 Locked for Write Protection by User 0"]
pub type S14S15lR = crate::BitReader<S14S15l>;
impl S14S15lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S14S15l {
        match self.bits {
            false => S14S15l::Value1,
            true => S14S15l::Value2,
        }
    }
    #[doc = "No write protection is configured for sectors 14+15."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S14S15l::Value1
    }
    #[doc = "Write protection is configured for sectors 14+15."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S14S15l::Value2
    }
}
#[doc = "Read Protection Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpro {
    #[doc = "0: No read protection configured"]
    Value1 = 0,
    #[doc = "1: Read protection and global write protection is configured by user 0 (master user)"]
    Value2 = 1,
}
impl From<Rpro> for bool {
    #[inline(always)]
    fn from(variant: Rpro) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPRO` reader - Read Protection Configuration"]
pub type RproR = crate::BitReader<Rpro>;
impl RproR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpro {
        match self.bits {
            false => Rpro::Value1,
            true => Rpro::Value2,
        }
    }
    #[doc = "No read protection configured"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rpro::Value1
    }
    #[doc = "Read protection and global write protection is configured by user 0 (master user)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rpro::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Sector 0 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s0l(&self) -> S0lR {
        S0lR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sector 1 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s1l(&self) -> S1lR {
        S1lR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sector 2 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s2l(&self) -> S2lR {
        S2lR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sector 3 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s3l(&self) -> S3lR {
        S3lR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sector 4 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s4l(&self) -> S4lR {
        S4lR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sector 5 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s5l(&self) -> S5lR {
        S5lR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sector 6 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s6l(&self) -> S6lR {
        S6lR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sector 7 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s7l(&self) -> S7lR {
        S7lR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Sector 8 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s8l(&self) -> S8lR {
        S8lR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Sector 9 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s9l(&self) -> S9lR {
        S9lR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Sectors 10 and 11 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s10_s11l(&self) -> S10S11lR {
        S10S11lR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Sectors 12 and 13 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s12_s13l(&self) -> S12S13lR {
        S12S13lR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Sectors 14 and 15 Locked for Write Protection by User 0"]
    #[inline(always)]
    pub fn s14_s15l(&self) -> S14S15lR {
        S14S15lR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Read Protection Configuration"]
    #[inline(always)]
    pub fn rpro(&self) -> RproR {
        RproR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Flash Protection Configuration Register User 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`procon0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Procon0Spec;
impl crate::RegisterSpec for Procon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`procon0::R`](R) reader structure"]
impl crate::Readable for Procon0Spec {}
#[doc = "`reset()` method sets PROCON0 to value 0"]
impl crate::Resettable for Procon0Spec {
    const RESET_VALUE: u32 = 0;
}
