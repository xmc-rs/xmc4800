#[doc = "Reader of register SDRSTAT"]
pub type R = crate::R<u32, super::SDRSTAT>;
#[doc = "SDRAM read error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDERR_A {
    #[doc = "0: Reads running successfully"]
    VALUE1 = 0,
    #[doc = "1: Read error condition has been detected"]
    VALUE2 = 1,
}
impl From<SDERR_A> for bool {
    #[inline(always)]
    fn from(variant: SDERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDERR`"]
pub type SDERR_R = crate::R<bool, SDERR_A>;
impl SDERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDERR_A {
        match self.bits {
            false => SDERR_A::VALUE1,
            true => SDERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SDERR_A::VALUE2
    }
}
#[doc = "SDRAM Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDRMBUSY_A {
    #[doc = "0: Power-up initialization sequence is not running"]
    VALUE1 = 0,
    #[doc = "1: Power-up initialization sequence is running"]
    VALUE2 = 1,
}
impl From<SDRMBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: SDRMBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDRMBUSY`"]
pub type SDRMBUSY_R = crate::R<bool, SDRMBUSY_A>;
impl SDRMBUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDRMBUSY_A {
        match self.bits {
            false => SDRMBUSY_A::VALUE1,
            true => SDRMBUSY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDRMBUSY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SDRMBUSY_A::VALUE2
    }
}
#[doc = "SDRAM Refresh Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFERR_A {
    #[doc = "0: No refresh error."]
    VALUE1 = 0,
    #[doc = "1: Refresh error occurred."]
    VALUE2 = 1,
}
impl From<REFERR_A> for bool {
    #[inline(always)]
    fn from(variant: REFERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFERR`"]
pub type REFERR_R = crate::R<bool, REFERR_A>;
impl REFERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFERR_A {
        match self.bits {
            false => REFERR_A::VALUE1,
            true => REFERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REFERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REFERR_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 2 - SDRAM read error"]
    #[inline(always)]
    pub fn sderr(&self) -> SDERR_R {
        SDERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SDRAM Busy"]
    #[inline(always)]
    pub fn sdrmbusy(&self) -> SDRMBUSY_R {
        SDRMBUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SDRAM Refresh Error"]
    #[inline(always)]
    pub fn referr(&self) -> REFERR_R {
        REFERR_R::new((self.bits & 0x01) != 0)
    }
}
