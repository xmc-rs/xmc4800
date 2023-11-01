#[doc = "Register `RSTSTAT` reader"]
pub type R = crate::R<RSTSTAT_SPEC>;
#[doc = "Field `RSTSTAT` reader - Reset Status Information"]
pub type RSTSTAT_R = crate::FieldReader<RSTSTAT_A>;
#[doc = "Reset Status Information\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSTSTAT_A {
    #[doc = "1: PORST reset"]
    VALUE1 = 1,
    #[doc = "2: SWD reset"]
    VALUE2 = 2,
    #[doc = "4: PV reset"]
    VALUE3 = 4,
    #[doc = "8: CPU system reset"]
    VALUE4 = 8,
    #[doc = "16: CPU lockup reset"]
    VALUE5 = 16,
    #[doc = "32: WDT reset"]
    VALUE6 = 32,
    #[doc = "128: Parity Error reset"]
    VALUE8 = 128,
}
impl From<RSTSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTSTAT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RSTSTAT_A {
    type Ux = u8;
}
impl RSTSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RSTSTAT_A> {
        match self.bits {
            1 => Some(RSTSTAT_A::VALUE1),
            2 => Some(RSTSTAT_A::VALUE2),
            4 => Some(RSTSTAT_A::VALUE3),
            8 => Some(RSTSTAT_A::VALUE4),
            16 => Some(RSTSTAT_A::VALUE5),
            32 => Some(RSTSTAT_A::VALUE6),
            128 => Some(RSTSTAT_A::VALUE8),
            _ => None,
        }
    }
    #[doc = "PORST reset"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RSTSTAT_A::VALUE1
    }
    #[doc = "SWD reset"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RSTSTAT_A::VALUE2
    }
    #[doc = "PV reset"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RSTSTAT_A::VALUE3
    }
    #[doc = "CPU system reset"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RSTSTAT_A::VALUE4
    }
    #[doc = "CPU lockup reset"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == RSTSTAT_A::VALUE5
    }
    #[doc = "WDT reset"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == RSTSTAT_A::VALUE6
    }
    #[doc = "Parity Error reset"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == RSTSTAT_A::VALUE8
    }
}
#[doc = "Field `HIBWK` reader - Hibernate Wake-up Status"]
pub type HIBWK_R = crate::BitReader<HIBWK_A>;
#[doc = "Hibernate Wake-up Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBWK_A {
    #[doc = "0: No Wake-up"]
    VALUE1 = 0,
    #[doc = "1: Wake-up event"]
    VALUE2 = 1,
}
impl From<HIBWK_A> for bool {
    #[inline(always)]
    fn from(variant: HIBWK_A) -> Self {
        variant as u8 != 0
    }
}
impl HIBWK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HIBWK_A {
        match self.bits {
            false => HIBWK_A::VALUE1,
            true => HIBWK_A::VALUE2,
        }
    }
    #[doc = "No Wake-up"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HIBWK_A::VALUE1
    }
    #[doc = "Wake-up event"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HIBWK_A::VALUE2
    }
}
#[doc = "Field `HIBRS` reader - Hibernate Reset Status"]
pub type HIBRS_R = crate::BitReader<HIBRS_A>;
#[doc = "Hibernate Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBRS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<HIBRS_A> for bool {
    #[inline(always)]
    fn from(variant: HIBRS_A) -> Self {
        variant as u8 != 0
    }
}
impl HIBRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HIBRS_A {
        match self.bits {
            false => HIBRS_A::VALUE1,
            true => HIBRS_A::VALUE2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HIBRS_A::VALUE1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HIBRS_A::VALUE2
    }
}
#[doc = "Field `LCKEN` reader - Enable Lockup Status"]
pub type LCKEN_R = crate::BitReader<LCKEN_A>;
#[doc = "Enable Lockup Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCKEN_A {
    #[doc = "0: Reset by Lockup disabled"]
    VALUE1 = 0,
    #[doc = "1: Reset by Lockup enabled"]
    VALUE2 = 1,
}
impl From<LCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: LCKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LCKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCKEN_A {
        match self.bits {
            false => LCKEN_A::VALUE1,
            true => LCKEN_A::VALUE2,
        }
    }
    #[doc = "Reset by Lockup disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LCKEN_A::VALUE1
    }
    #[doc = "Reset by Lockup enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LCKEN_A::VALUE2
    }
}
#[doc = "Field `ECAT0RS` reader - ECAT0 Reset Status Information"]
pub type ECAT0RS_R = crate::BitReader<ECAT0RS_A>;
#[doc = "ECAT0 Reset Status Information\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECAT0RS_A {
    #[doc = "0: Reset did not occur"]
    VALUE1 = 0,
    #[doc = "1: Reset occurred"]
    VALUE2 = 1,
}
impl From<ECAT0RS_A> for bool {
    #[inline(always)]
    fn from(variant: ECAT0RS_A) -> Self {
        variant as u8 != 0
    }
}
impl ECAT0RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECAT0RS_A {
        match self.bits {
            false => ECAT0RS_A::VALUE1,
            true => ECAT0RS_A::VALUE2,
        }
    }
    #[doc = "Reset did not occur"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ECAT0RS_A::VALUE1
    }
    #[doc = "Reset occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ECAT0RS_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:7 - Reset Status Information"]
    #[inline(always)]
    pub fn rststat(&self) -> RSTSTAT_R {
        RSTSTAT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Hibernate Wake-up Status"]
    #[inline(always)]
    pub fn hibwk(&self) -> HIBWK_R {
        HIBWK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Hibernate Reset Status"]
    #[inline(always)]
    pub fn hibrs(&self) -> HIBRS_R {
        HIBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Lockup Status"]
    #[inline(always)]
    pub fn lcken(&self) -> LCKEN_R {
        LCKEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - ECAT0 Reset Status Information"]
    #[inline(always)]
    pub fn ecat0rs(&self) -> ECAT0RS_R {
        ECAT0RS_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "RCU Reset Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rststat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSTSTAT_SPEC;
impl crate::RegisterSpec for RSTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rststat::R`](R) reader structure"]
impl crate::Readable for RSTSTAT_SPEC {}
#[doc = "`reset()` method sets RSTSTAT to value 0"]
impl crate::Resettable for RSTSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
