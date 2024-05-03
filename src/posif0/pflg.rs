#[doc = "Register `PFLG` reader"]
pub type R = crate::R<PFLG_SPEC>;
#[doc = "Correct Hall Event Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type CHES_R = crate::BitReader<CHES_A>;
impl CHES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHES_A {
        match self.bits {
            false => CHES_A::VALUE1,
            true => CHES_A::VALUE2,
        }
    }
    #[doc = "Correct Hall Event not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHES_A::VALUE1
    }
    #[doc = "Correct Hall Event detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHES_A::VALUE2
    }
}
#[doc = "Wrong Hall Event Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type WHES_R = crate::BitReader<WHES_A>;
impl WHES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WHES_A {
        match self.bits {
            false => WHES_A::VALUE1,
            true => WHES_A::VALUE2,
        }
    }
    #[doc = "Wrong Hall Event not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WHES_A::VALUE1
    }
    #[doc = "Wrong Hall Event detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WHES_A::VALUE2
    }
}
#[doc = "Hall Inputs Update Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type HIES_R = crate::BitReader<HIES_A>;
impl HIES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HIES_A {
        match self.bits {
            false => HIES_A::VALUE1,
            true => HIES_A::VALUE2,
        }
    }
    #[doc = "Transition on the Hall Inputs not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HIES_A::VALUE1
    }
    #[doc = "Transition on the Hall Inputs detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HIES_A::VALUE2
    }
}
#[doc = "Multi-Channel pattern shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type MSTS_R = crate::BitReader<MSTS_A>;
impl MSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTS_A {
        match self.bits {
            false => MSTS_A::VALUE1,
            true => MSTS_A::VALUE2,
        }
    }
    #[doc = "Shadow transfer not done"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MSTS_A::VALUE1
    }
    #[doc = "Shadow transfer done"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSTS_A::VALUE2
    }
}
#[doc = "Quadrature Index Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type INDXS_R = crate::BitReader<INDXS_A>;
impl INDXS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INDXS_A {
        match self.bits {
            false => INDXS_A::VALUE1,
            true => INDXS_A::VALUE2,
        }
    }
    #[doc = "Index event not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INDXS_A::VALUE1
    }
    #[doc = "Index event detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INDXS_A::VALUE2
    }
}
#[doc = "Quadrature Phase Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type ERRS_R = crate::BitReader<ERRS_A>;
impl ERRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRS_A {
        match self.bits {
            false => ERRS_A::VALUE1,
            true => ERRS_A::VALUE2,
        }
    }
    #[doc = "Phase Error event not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERRS_A::VALUE1
    }
    #[doc = "Phase Error event detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERRS_A::VALUE2
    }
}
#[doc = "Quadrature CLK Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type CNTS_R = crate::BitReader<CNTS_A>;
impl CNTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CNTS_A {
        match self.bits {
            false => CNTS_A::VALUE1,
            true => CNTS_A::VALUE2,
        }
    }
    #[doc = "Quadrature clock not generated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CNTS_A::VALUE1
    }
    #[doc = "Quadrature clock generated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CNTS_A::VALUE2
    }
}
#[doc = "Quadrature Direction Change\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type DIRS_R = crate::BitReader<DIRS_A>;
impl DIRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIRS_A {
        match self.bits {
            false => DIRS_A::VALUE1,
            true => DIRS_A::VALUE2,
        }
    }
    #[doc = "Change on direction not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIRS_A::VALUE1
    }
    #[doc = "Change on direction detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIRS_A::VALUE2
    }
}
#[doc = "Quadrature Period Clk Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type PCLKS_R = crate::BitReader<PCLKS_A>;
impl PCLKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLKS_A {
        match self.bits {
            false => PCLKS_A::VALUE1,
            true => PCLKS_A::VALUE2,
        }
    }
    #[doc = "Period clock not generated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PCLKS_A::VALUE1
    }
    #[doc = "Period clock generated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PCLKS_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Correct Hall Event Status"]
    #[inline(always)]
    pub fn ches(&self) -> CHES_R {
        CHES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wrong Hall Event Status"]
    #[inline(always)]
    pub fn whes(&self) -> WHES_R {
        WHES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hall Inputs Update Status"]
    #[inline(always)]
    pub fn hies(&self) -> HIES_R {
        HIES_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Multi-Channel pattern shadow transfer status"]
    #[inline(always)]
    pub fn msts(&self) -> MSTS_R {
        MSTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Quadrature Index Status"]
    #[inline(always)]
    pub fn indxs(&self) -> INDXS_R {
        INDXS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Quadrature Phase Error Status"]
    #[inline(always)]
    pub fn errs(&self) -> ERRS_R {
        ERRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Quadrature CLK Status"]
    #[inline(always)]
    pub fn cnts(&self) -> CNTS_R {
        CNTS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Quadrature Direction Change"]
    #[inline(always)]
    pub fn dirs(&self) -> DIRS_R {
        DIRS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Quadrature Period Clk Status"]
    #[inline(always)]
    pub fn pclks(&self) -> PCLKS_R {
        PCLKS_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "POSIF Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pflg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFLG_SPEC;
impl crate::RegisterSpec for PFLG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pflg::R`](R) reader structure"]
impl crate::Readable for PFLG_SPEC {}
#[doc = "`reset()` method sets PFLG to value 0"]
impl crate::Resettable for PFLG_SPEC {
    const RESET_VALUE: u32 = 0;
}
