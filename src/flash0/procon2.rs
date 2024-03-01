#[doc = "Register `PROCON2` reader"]
pub type R = crate::R<Procon2Spec>;
#[doc = "Sector 0 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0rom {
    #[doc = "0: No ROM functionality configured for sector n."]
    Value1 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    Value2 = 1,
}
impl From<S0rom> for bool {
    #[inline(always)]
    fn from(variant: S0rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0ROM` reader - Sector 0 Locked Forever by User 2"]
pub type S0romR = crate::BitReader<S0rom>;
impl S0romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0rom {
        match self.bits {
            false => S0rom::Value1,
            true => S0rom::Value2,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0rom::Value1
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0rom::Value2
    }
}
#[doc = "Sector 1 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1rom {
    #[doc = "0: No ROM functionality configured for sector n."]
    Value1 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    Value2 = 1,
}
impl From<S1rom> for bool {
    #[inline(always)]
    fn from(variant: S1rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1ROM` reader - Sector 1 Locked Forever by User 2"]
pub type S1romR = crate::BitReader<S1rom>;
impl S1romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1rom {
        match self.bits {
            false => S1rom::Value1,
            true => S1rom::Value2,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1rom::Value1
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1rom::Value2
    }
}
#[doc = "Sector 2 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2rom {
    #[doc = "0: No ROM functionality configured for sector n."]
    Value1 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    Value2 = 1,
}
impl From<S2rom> for bool {
    #[inline(always)]
    fn from(variant: S2rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2ROM` reader - Sector 2 Locked Forever by User 2"]
pub type S2romR = crate::BitReader<S2rom>;
impl S2romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2rom {
        match self.bits {
            false => S2rom::Value1,
            true => S2rom::Value2,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S2rom::Value1
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S2rom::Value2
    }
}
#[doc = "Sector 3 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3rom {
    #[doc = "0: No ROM functionality configured for sector n."]
    Value1 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    Value2 = 1,
}
impl From<S3rom> for bool {
    #[inline(always)]
    fn from(variant: S3rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3ROM` reader - Sector 3 Locked Forever by User 2"]
pub type S3romR = crate::BitReader<S3rom>;
impl S3romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S3rom {
        match self.bits {
            false => S3rom::Value1,
            true => S3rom::Value2,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S3rom::Value1
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S3rom::Value2
    }
}
#[doc = "Sector 4 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S4rom {
    #[doc = "0: No ROM functionality configured for sector n."]
    Value1 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    Value2 = 1,
}
impl From<S4rom> for bool {
    #[inline(always)]
    fn from(variant: S4rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S4ROM` reader - Sector 4 Locked Forever by User 2"]
pub type S4romR = crate::BitReader<S4rom>;
impl S4romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S4rom {
        match self.bits {
            false => S4rom::Value1,
            true => S4rom::Value2,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S4rom::Value1
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S4rom::Value2
    }
}
#[doc = "Sector 5 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S5rom {
    #[doc = "0: No ROM functionality configured for sector n."]
    Value1 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    Value2 = 1,
}
impl From<S5rom> for bool {
    #[inline(always)]
    fn from(variant: S5rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S5ROM` reader - Sector 5 Locked Forever by User 2"]
pub type S5romR = crate::BitReader<S5rom>;
impl S5romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S5rom {
        match self.bits {
            false => S5rom::Value1,
            true => S5rom::Value2,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S5rom::Value1
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S5rom::Value2
    }
}
#[doc = "Sector 6 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S6rom {
    #[doc = "0: No ROM functionality configured for sector n."]
    Value1 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    Value2 = 1,
}
impl From<S6rom> for bool {
    #[inline(always)]
    fn from(variant: S6rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S6ROM` reader - Sector 6 Locked Forever by User 2"]
pub type S6romR = crate::BitReader<S6rom>;
impl S6romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S6rom {
        match self.bits {
            false => S6rom::Value1,
            true => S6rom::Value2,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S6rom::Value1
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S6rom::Value2
    }
}
#[doc = "Sector 7 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S7rom {
    #[doc = "0: No ROM functionality configured for sector n."]
    Value1 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    Value2 = 1,
}
impl From<S7rom> for bool {
    #[inline(always)]
    fn from(variant: S7rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S7ROM` reader - Sector 7 Locked Forever by User 2"]
pub type S7romR = crate::BitReader<S7rom>;
impl S7romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S7rom {
        match self.bits {
            false => S7rom::Value1,
            true => S7rom::Value2,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S7rom::Value1
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S7rom::Value2
    }
}
#[doc = "Sector 8 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S8rom {
    #[doc = "0: No ROM functionality configured for sector n."]
    Value1 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    Value2 = 1,
}
impl From<S8rom> for bool {
    #[inline(always)]
    fn from(variant: S8rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S8ROM` reader - Sector 8 Locked Forever by User 2"]
pub type S8romR = crate::BitReader<S8rom>;
impl S8romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S8rom {
        match self.bits {
            false => S8rom::Value1,
            true => S8rom::Value2,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S8rom::Value1
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S8rom::Value2
    }
}
#[doc = "Sector 9 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S9rom {
    #[doc = "0: No ROM functionality configured for sector n."]
    Value1 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    Value2 = 1,
}
impl From<S9rom> for bool {
    #[inline(always)]
    fn from(variant: S9rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S9ROM` reader - Sector 9 Locked Forever by User 2"]
pub type S9romR = crate::BitReader<S9rom>;
impl S9romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S9rom {
        match self.bits {
            false => S9rom::Value1,
            true => S9rom::Value2,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S9rom::Value1
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S9rom::Value2
    }
}
#[doc = "Sectors 10 and 11 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S10S11rom {
    #[doc = "0: No ROM functionality is configured for sectors 10+11."]
    Value1 = 0,
    #[doc = "1: ROM functionality is configured for sectors 10+11."]
    Value2 = 1,
}
impl From<S10S11rom> for bool {
    #[inline(always)]
    fn from(variant: S10S11rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S10_S11ROM` reader - Sectors 10 and 11 Locked Forever by User 2"]
pub type S10S11romR = crate::BitReader<S10S11rom>;
impl S10S11romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S10S11rom {
        match self.bits {
            false => S10S11rom::Value1,
            true => S10S11rom::Value2,
        }
    }
    #[doc = "No ROM functionality is configured for sectors 10+11."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S10S11rom::Value1
    }
    #[doc = "ROM functionality is configured for sectors 10+11."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S10S11rom::Value2
    }
}
#[doc = "Sectors 12 and 13 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S12S13rom {
    #[doc = "0: No ROM functionality is configured for sectors 12+13."]
    Value1 = 0,
    #[doc = "1: ROM functionality is configured for sectors 12+13."]
    Value2 = 1,
}
impl From<S12S13rom> for bool {
    #[inline(always)]
    fn from(variant: S12S13rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S12_S13ROM` reader - Sectors 12 and 13 Locked Forever by User 2"]
pub type S12S13romR = crate::BitReader<S12S13rom>;
impl S12S13romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S12S13rom {
        match self.bits {
            false => S12S13rom::Value1,
            true => S12S13rom::Value2,
        }
    }
    #[doc = "No ROM functionality is configured for sectors 12+13."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S12S13rom::Value1
    }
    #[doc = "ROM functionality is configured for sectors 12+13."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S12S13rom::Value2
    }
}
#[doc = "Sectors 14 and 15 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S14S15rom {
    #[doc = "0: No ROM functionality is configured for sectors 14+15."]
    Value1 = 0,
    #[doc = "1: ROM functionality is configured for sectors 14+15."]
    Value2 = 1,
}
impl From<S14S15rom> for bool {
    #[inline(always)]
    fn from(variant: S14S15rom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S14_S15ROM` reader - Sectors 14 and 15 Locked Forever by User 2"]
pub type S14S15romR = crate::BitReader<S14S15rom>;
impl S14S15romR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S14S15rom {
        match self.bits {
            false => S14S15rom::Value1,
            true => S14S15rom::Value2,
        }
    }
    #[doc = "No ROM functionality is configured for sectors 14+15."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S14S15rom::Value1
    }
    #[doc = "ROM functionality is configured for sectors 14+15."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S14S15rom::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Sector 0 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s0rom(&self) -> S0romR {
        S0romR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sector 1 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s1rom(&self) -> S1romR {
        S1romR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sector 2 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s2rom(&self) -> S2romR {
        S2romR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sector 3 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s3rom(&self) -> S3romR {
        S3romR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sector 4 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s4rom(&self) -> S4romR {
        S4romR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sector 5 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s5rom(&self) -> S5romR {
        S5romR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sector 6 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s6rom(&self) -> S6romR {
        S6romR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sector 7 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s7rom(&self) -> S7romR {
        S7romR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Sector 8 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s8rom(&self) -> S8romR {
        S8romR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Sector 9 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s9rom(&self) -> S9romR {
        S9romR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Sectors 10 and 11 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s10_s11rom(&self) -> S10S11romR {
        S10S11romR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Sectors 12 and 13 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s12_s13rom(&self) -> S12S13romR {
        S12S13romR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Sectors 14 and 15 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s14_s15rom(&self) -> S14S15romR {
        S14S15romR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Flash Protection Configuration Register User 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`procon2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Procon2Spec;
impl crate::RegisterSpec for Procon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`procon2::R`](R) reader structure"]
impl crate::Readable for Procon2Spec {}
#[doc = "`reset()` method sets PROCON2 to value 0"]
impl crate::Resettable for Procon2Spec {
    const RESET_VALUE: u32 = 0;
}
