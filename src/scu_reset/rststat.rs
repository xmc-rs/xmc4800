#[doc = "Reader of register RSTSTAT"]
pub type R = crate::R<u32, super::RSTSTAT>;
#[doc = "Reset Status Information\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `RSTSTAT`"]
pub type RSTSTAT_R = crate::R<u8, RSTSTAT_A>;
impl RSTSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RSTSTAT_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(RSTSTAT_A::VALUE1),
            2 => Val(RSTSTAT_A::VALUE2),
            4 => Val(RSTSTAT_A::VALUE3),
            8 => Val(RSTSTAT_A::VALUE4),
            16 => Val(RSTSTAT_A::VALUE5),
            32 => Val(RSTSTAT_A::VALUE6),
            128 => Val(RSTSTAT_A::VALUE8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RSTSTAT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RSTSTAT_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RSTSTAT_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RSTSTAT_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == RSTSTAT_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == RSTSTAT_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == RSTSTAT_A::VALUE8
    }
}
#[doc = "Hibernate Wake-up Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `HIBWK`"]
pub type HIBWK_R = crate::R<bool, HIBWK_A>;
impl HIBWK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIBWK_A {
        match self.bits {
            false => HIBWK_A::VALUE1,
            true => HIBWK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HIBWK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HIBWK_A::VALUE2
    }
}
#[doc = "Hibernate Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `HIBRS`"]
pub type HIBRS_R = crate::R<bool, HIBRS_A>;
impl HIBRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIBRS_A {
        match self.bits {
            false => HIBRS_A::VALUE1,
            true => HIBRS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HIBRS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HIBRS_A::VALUE2
    }
}
#[doc = "Enable Lockup Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `LCKEN`"]
pub type LCKEN_R = crate::R<bool, LCKEN_A>;
impl LCKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCKEN_A {
        match self.bits {
            false => LCKEN_A::VALUE1,
            true => LCKEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LCKEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LCKEN_A::VALUE2
    }
}
#[doc = "ECAT0 Reset Status Information\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `ECAT0RS`"]
pub type ECAT0RS_R = crate::R<bool, ECAT0RS_A>;
impl ECAT0RS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECAT0RS_A {
        match self.bits {
            false => ECAT0RS_A::VALUE1,
            true => ECAT0RS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ECAT0RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
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
        HIBWK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Hibernate Reset Status"]
    #[inline(always)]
    pub fn hibrs(&self) -> HIBRS_R {
        HIBRS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable Lockup Status"]
    #[inline(always)]
    pub fn lcken(&self) -> LCKEN_R {
        LCKEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ECAT0 Reset Status Information"]
    #[inline(always)]
    pub fn ecat0rs(&self) -> ECAT0RS_R {
        ECAT0RS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
