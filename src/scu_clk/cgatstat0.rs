#[doc = "Reader of register CGATSTAT0"]
pub type R = crate::R<u32, super::CGATSTAT0>;
#[doc = "VADC Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VADC_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1,
    #[doc = "1: Gating asserted"]
    VALUE2,
}
impl From<VADC_A> for bool {
    #[inline(always)]
    fn from(variant: VADC_A) -> Self {
        match variant {
            VADC_A::VALUE1 => false,
            VADC_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `VADC`"]
pub type VADC_R = crate::R<bool, VADC_A>;
impl VADC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VADC_A {
        match self.bits {
            false => VADC_A::VALUE1,
            true => VADC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VADC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VADC_A::VALUE2
    }
}
#[doc = "DSD Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSD_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1,
    #[doc = "1: Gating asserted"]
    VALUE2,
}
impl From<DSD_A> for bool {
    #[inline(always)]
    fn from(variant: DSD_A) -> Self {
        match variant {
            DSD_A::VALUE1 => false,
            DSD_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DSD`"]
pub type DSD_R = crate::R<bool, DSD_A>;
impl DSD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSD_A {
        match self.bits {
            false => DSD_A::VALUE1,
            true => DSD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DSD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DSD_A::VALUE2
    }
}
#[doc = "CCU40 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU40_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1,
    #[doc = "1: Gating asserted"]
    VALUE2,
}
impl From<CCU40_A> for bool {
    #[inline(always)]
    fn from(variant: CCU40_A) -> Self {
        match variant {
            CCU40_A::VALUE1 => false,
            CCU40_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CCU40`"]
pub type CCU40_R = crate::R<bool, CCU40_A>;
impl CCU40_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU40_A {
        match self.bits {
            false => CCU40_A::VALUE1,
            true => CCU40_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCU40_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCU40_A::VALUE2
    }
}
#[doc = "CCU41 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU41_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1,
    #[doc = "1: Gating asserted"]
    VALUE2,
}
impl From<CCU41_A> for bool {
    #[inline(always)]
    fn from(variant: CCU41_A) -> Self {
        match variant {
            CCU41_A::VALUE1 => false,
            CCU41_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CCU41`"]
pub type CCU41_R = crate::R<bool, CCU41_A>;
impl CCU41_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU41_A {
        match self.bits {
            false => CCU41_A::VALUE1,
            true => CCU41_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCU41_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCU41_A::VALUE2
    }
}
#[doc = "CCU42 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU42_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1,
    #[doc = "1: Gating asserted"]
    VALUE2,
}
impl From<CCU42_A> for bool {
    #[inline(always)]
    fn from(variant: CCU42_A) -> Self {
        match variant {
            CCU42_A::VALUE1 => false,
            CCU42_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CCU42`"]
pub type CCU42_R = crate::R<bool, CCU42_A>;
impl CCU42_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU42_A {
        match self.bits {
            false => CCU42_A::VALUE1,
            true => CCU42_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCU42_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCU42_A::VALUE2
    }
}
#[doc = "CCU80 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU80_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1,
    #[doc = "1: Gating asserted"]
    VALUE2,
}
impl From<CCU80_A> for bool {
    #[inline(always)]
    fn from(variant: CCU80_A) -> Self {
        match variant {
            CCU80_A::VALUE1 => false,
            CCU80_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CCU80`"]
pub type CCU80_R = crate::R<bool, CCU80_A>;
impl CCU80_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU80_A {
        match self.bits {
            false => CCU80_A::VALUE1,
            true => CCU80_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCU80_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCU80_A::VALUE2
    }
}
#[doc = "CCU81 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU81_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1,
    #[doc = "1: Gating asserted"]
    VALUE2,
}
impl From<CCU81_A> for bool {
    #[inline(always)]
    fn from(variant: CCU81_A) -> Self {
        match variant {
            CCU81_A::VALUE1 => false,
            CCU81_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CCU81`"]
pub type CCU81_R = crate::R<bool, CCU81_A>;
impl CCU81_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU81_A {
        match self.bits {
            false => CCU81_A::VALUE1,
            true => CCU81_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCU81_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCU81_A::VALUE2
    }
}
#[doc = "POSIF0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSIF0_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1,
    #[doc = "1: Gating asserted"]
    VALUE2,
}
impl From<POSIF0_A> for bool {
    #[inline(always)]
    fn from(variant: POSIF0_A) -> Self {
        match variant {
            POSIF0_A::VALUE1 => false,
            POSIF0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `POSIF0`"]
pub type POSIF0_R = crate::R<bool, POSIF0_A>;
impl POSIF0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POSIF0_A {
        match self.bits {
            false => POSIF0_A::VALUE1,
            true => POSIF0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == POSIF0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == POSIF0_A::VALUE2
    }
}
#[doc = "POSIF1 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSIF1_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1,
    #[doc = "1: Gating asserted"]
    VALUE2,
}
impl From<POSIF1_A> for bool {
    #[inline(always)]
    fn from(variant: POSIF1_A) -> Self {
        match variant {
            POSIF1_A::VALUE1 => false,
            POSIF1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `POSIF1`"]
pub type POSIF1_R = crate::R<bool, POSIF1_A>;
impl POSIF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POSIF1_A {
        match self.bits {
            false => POSIF1_A::VALUE1,
            true => POSIF1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == POSIF1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == POSIF1_A::VALUE2
    }
}
#[doc = "USIC0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC0_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1,
    #[doc = "1: Gating asserted"]
    VALUE2,
}
impl From<USIC0_A> for bool {
    #[inline(always)]
    fn from(variant: USIC0_A) -> Self {
        match variant {
            USIC0_A::VALUE1 => false,
            USIC0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `USIC0`"]
pub type USIC0_R = crate::R<bool, USIC0_A>;
impl USIC0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USIC0_A {
        match self.bits {
            false => USIC0_A::VALUE1,
            true => USIC0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USIC0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USIC0_A::VALUE2
    }
}
#[doc = "ERU1 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERU1_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1,
    #[doc = "1: Gating asserted"]
    VALUE2,
}
impl From<ERU1_A> for bool {
    #[inline(always)]
    fn from(variant: ERU1_A) -> Self {
        match variant {
            ERU1_A::VALUE1 => false,
            ERU1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ERU1`"]
pub type ERU1_R = crate::R<bool, ERU1_A>;
impl ERU1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERU1_A {
        match self.bits {
            false => ERU1_A::VALUE1,
            true => ERU1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERU1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERU1_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - VADC Gating Status"]
    #[inline(always)]
    pub fn vadc(&self) -> VADC_R {
        VADC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DSD Gating Status"]
    #[inline(always)]
    pub fn dsd(&self) -> DSD_R {
        DSD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CCU40 Gating Status"]
    #[inline(always)]
    pub fn ccu40(&self) -> CCU40_R {
        CCU40_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CCU41 Gating Status"]
    #[inline(always)]
    pub fn ccu41(&self) -> CCU41_R {
        CCU41_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CCU42 Gating Status"]
    #[inline(always)]
    pub fn ccu42(&self) -> CCU42_R {
        CCU42_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CCU80 Gating Status"]
    #[inline(always)]
    pub fn ccu80(&self) -> CCU80_R {
        CCU80_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CCU81 Gating Status"]
    #[inline(always)]
    pub fn ccu81(&self) -> CCU81_R {
        CCU81_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - POSIF0 Gating Status"]
    #[inline(always)]
    pub fn posif0(&self) -> POSIF0_R {
        POSIF0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - POSIF1 Gating Status"]
    #[inline(always)]
    pub fn posif1(&self) -> POSIF1_R {
        POSIF1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USIC0 Gating Status"]
    #[inline(always)]
    pub fn usic0(&self) -> USIC0_R {
        USIC0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ERU1 Gating Status"]
    #[inline(always)]
    pub fn eru1(&self) -> ERU1_R {
        ERU1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
