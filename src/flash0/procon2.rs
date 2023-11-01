#[doc = "Register `PROCON2` reader"]
pub type R = crate::R<PROCON2_SPEC>;
#[doc = "Field `S0ROM` reader - Sector 0 Locked Forever by User 2"]
pub type S0ROM_R = crate::BitReader<S0ROM_A>;
#[doc = "Sector 0 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    VALUE2 = 1,
}
impl From<S0ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S0ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S0ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0ROM_A {
        match self.bits {
            false => S0ROM_A::VALUE1,
            true => S0ROM_A::VALUE2,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0ROM_A::VALUE1
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0ROM_A::VALUE2
    }
}
#[doc = "Field `S1ROM` reader - Sector 1 Locked Forever by User 2"]
pub type S1ROM_R = crate::BitReader<S1ROM_A>;
#[doc = "Sector 1 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    VALUE2 = 1,
}
impl From<S1ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S1ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S1ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1ROM_A {
        match self.bits {
            false => S1ROM_A::VALUE1,
            true => S1ROM_A::VALUE2,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1ROM_A::VALUE1
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1ROM_A::VALUE2
    }
}
#[doc = "Field `S2ROM` reader - Sector 2 Locked Forever by User 2"]
pub type S2ROM_R = crate::BitReader<S2ROM_A>;
#[doc = "Sector 2 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    VALUE2 = 1,
}
impl From<S2ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S2ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S2ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2ROM_A {
        match self.bits {
            false => S2ROM_A::VALUE1,
            true => S2ROM_A::VALUE2,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S2ROM_A::VALUE1
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S2ROM_A::VALUE2
    }
}
#[doc = "Field `S3ROM` reader - Sector 3 Locked Forever by User 2"]
pub type S3ROM_R = crate::BitReader<S3ROM_A>;
#[doc = "Sector 3 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    VALUE2 = 1,
}
impl From<S3ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S3ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S3ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S3ROM_A {
        match self.bits {
            false => S3ROM_A::VALUE1,
            true => S3ROM_A::VALUE2,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S3ROM_A::VALUE1
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S3ROM_A::VALUE2
    }
}
#[doc = "Field `S4ROM` reader - Sector 4 Locked Forever by User 2"]
pub type S4ROM_R = crate::BitReader<S4ROM_A>;
#[doc = "Sector 4 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S4ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    VALUE2 = 1,
}
impl From<S4ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S4ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S4ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S4ROM_A {
        match self.bits {
            false => S4ROM_A::VALUE1,
            true => S4ROM_A::VALUE2,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S4ROM_A::VALUE1
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S4ROM_A::VALUE2
    }
}
#[doc = "Field `S5ROM` reader - Sector 5 Locked Forever by User 2"]
pub type S5ROM_R = crate::BitReader<S5ROM_A>;
#[doc = "Sector 5 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S5ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    VALUE2 = 1,
}
impl From<S5ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S5ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S5ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S5ROM_A {
        match self.bits {
            false => S5ROM_A::VALUE1,
            true => S5ROM_A::VALUE2,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S5ROM_A::VALUE1
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S5ROM_A::VALUE2
    }
}
#[doc = "Field `S6ROM` reader - Sector 6 Locked Forever by User 2"]
pub type S6ROM_R = crate::BitReader<S6ROM_A>;
#[doc = "Sector 6 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S6ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    VALUE2 = 1,
}
impl From<S6ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S6ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S6ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S6ROM_A {
        match self.bits {
            false => S6ROM_A::VALUE1,
            true => S6ROM_A::VALUE2,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S6ROM_A::VALUE1
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S6ROM_A::VALUE2
    }
}
#[doc = "Field `S7ROM` reader - Sector 7 Locked Forever by User 2"]
pub type S7ROM_R = crate::BitReader<S7ROM_A>;
#[doc = "Sector 7 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S7ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    VALUE2 = 1,
}
impl From<S7ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S7ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S7ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S7ROM_A {
        match self.bits {
            false => S7ROM_A::VALUE1,
            true => S7ROM_A::VALUE2,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S7ROM_A::VALUE1
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S7ROM_A::VALUE2
    }
}
#[doc = "Field `S8ROM` reader - Sector 8 Locked Forever by User 2"]
pub type S8ROM_R = crate::BitReader<S8ROM_A>;
#[doc = "Sector 8 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S8ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    VALUE2 = 1,
}
impl From<S8ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S8ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S8ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S8ROM_A {
        match self.bits {
            false => S8ROM_A::VALUE1,
            true => S8ROM_A::VALUE2,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S8ROM_A::VALUE1
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S8ROM_A::VALUE2
    }
}
#[doc = "Field `S9ROM` reader - Sector 9 Locked Forever by User 2"]
pub type S9ROM_R = crate::BitReader<S9ROM_A>;
#[doc = "Sector 9 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S9ROM_A {
    #[doc = "0: No ROM functionality configured for sector n."]
    VALUE1 = 0,
    #[doc = "1: ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    VALUE2 = 1,
}
impl From<S9ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S9ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S9ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S9ROM_A {
        match self.bits {
            false => S9ROM_A::VALUE1,
            true => S9ROM_A::VALUE2,
        }
    }
    #[doc = "No ROM functionality configured for sector n."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S9ROM_A::VALUE1
    }
    #[doc = "ROM functionality is configured for sector n. Re-programming of this sector is no longer possible."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S9ROM_A::VALUE2
    }
}
#[doc = "Field `S10_S11ROM` reader - Sectors 10 and 11 Locked Forever by User 2"]
pub type S10_S11ROM_R = crate::BitReader<S10_S11ROM_A>;
#[doc = "Sectors 10 and 11 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S10_S11ROM_A {
    #[doc = "0: No ROM functionality is configured for sectors 10+11."]
    VALUE1 = 0,
    #[doc = "1: ROM functionality is configured for sectors 10+11."]
    VALUE2 = 1,
}
impl From<S10_S11ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S10_S11ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S10_S11ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S10_S11ROM_A {
        match self.bits {
            false => S10_S11ROM_A::VALUE1,
            true => S10_S11ROM_A::VALUE2,
        }
    }
    #[doc = "No ROM functionality is configured for sectors 10+11."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S10_S11ROM_A::VALUE1
    }
    #[doc = "ROM functionality is configured for sectors 10+11."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S10_S11ROM_A::VALUE2
    }
}
#[doc = "Field `S12_S13ROM` reader - Sectors 12 and 13 Locked Forever by User 2"]
pub type S12_S13ROM_R = crate::BitReader<S12_S13ROM_A>;
#[doc = "Sectors 12 and 13 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S12_S13ROM_A {
    #[doc = "0: No ROM functionality is configured for sectors 12+13."]
    VALUE1 = 0,
    #[doc = "1: ROM functionality is configured for sectors 12+13."]
    VALUE2 = 1,
}
impl From<S12_S13ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S12_S13ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S12_S13ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S12_S13ROM_A {
        match self.bits {
            false => S12_S13ROM_A::VALUE1,
            true => S12_S13ROM_A::VALUE2,
        }
    }
    #[doc = "No ROM functionality is configured for sectors 12+13."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S12_S13ROM_A::VALUE1
    }
    #[doc = "ROM functionality is configured for sectors 12+13."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S12_S13ROM_A::VALUE2
    }
}
#[doc = "Field `S14_S15ROM` reader - Sectors 14 and 15 Locked Forever by User 2"]
pub type S14_S15ROM_R = crate::BitReader<S14_S15ROM_A>;
#[doc = "Sectors 14 and 15 Locked Forever by User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S14_S15ROM_A {
    #[doc = "0: No ROM functionality is configured for sectors 14+15."]
    VALUE1 = 0,
    #[doc = "1: ROM functionality is configured for sectors 14+15."]
    VALUE2 = 1,
}
impl From<S14_S15ROM_A> for bool {
    #[inline(always)]
    fn from(variant: S14_S15ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl S14_S15ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S14_S15ROM_A {
        match self.bits {
            false => S14_S15ROM_A::VALUE1,
            true => S14_S15ROM_A::VALUE2,
        }
    }
    #[doc = "No ROM functionality is configured for sectors 14+15."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S14_S15ROM_A::VALUE1
    }
    #[doc = "ROM functionality is configured for sectors 14+15."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S14_S15ROM_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Sector 0 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s0rom(&self) -> S0ROM_R {
        S0ROM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sector 1 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s1rom(&self) -> S1ROM_R {
        S1ROM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sector 2 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s2rom(&self) -> S2ROM_R {
        S2ROM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sector 3 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s3rom(&self) -> S3ROM_R {
        S3ROM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sector 4 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s4rom(&self) -> S4ROM_R {
        S4ROM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sector 5 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s5rom(&self) -> S5ROM_R {
        S5ROM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sector 6 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s6rom(&self) -> S6ROM_R {
        S6ROM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sector 7 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s7rom(&self) -> S7ROM_R {
        S7ROM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Sector 8 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s8rom(&self) -> S8ROM_R {
        S8ROM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Sector 9 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s9rom(&self) -> S9ROM_R {
        S9ROM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Sectors 10 and 11 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s10_s11rom(&self) -> S10_S11ROM_R {
        S10_S11ROM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Sectors 12 and 13 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s12_s13rom(&self) -> S12_S13ROM_R {
        S12_S13ROM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Sectors 14 and 15 Locked Forever by User 2"]
    #[inline(always)]
    pub fn s14_s15rom(&self) -> S14_S15ROM_R {
        S14_S15ROM_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Flash Protection Configuration Register User 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`procon2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PROCON2_SPEC;
impl crate::RegisterSpec for PROCON2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`procon2::R`](R) reader structure"]
impl crate::Readable for PROCON2_SPEC {}
#[doc = "`reset()` method sets PROCON2 to value 0"]
impl crate::Resettable for PROCON2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
