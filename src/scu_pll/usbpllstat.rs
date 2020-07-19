#[doc = "Reader of register USBPLLSTAT"]
pub type R = crate::R<u32, super::USBPLLSTAT>;
#[doc = "VCO Bypass Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOBYST_A {
    #[doc = "0: Normal Mode is entered"]
    VALUE1 = 0,
    #[doc = "1: Prescaler Mode is entered"]
    VALUE2 = 1,
}
impl From<VCOBYST_A> for bool {
    #[inline(always)]
    fn from(variant: VCOBYST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VCOBYST`"]
pub type VCOBYST_R = crate::R<bool, VCOBYST_A>;
impl VCOBYST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOBYST_A {
        match self.bits {
            false => VCOBYST_A::VALUE1,
            true => VCOBYST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VCOBYST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VCOBYST_A::VALUE2
    }
}
#[doc = "PLL Power-saving Mode Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWDSTAT_A {
    #[doc = "0: PLL Power-saving Mode was not entered"]
    VALUE1 = 0,
    #[doc = "1: PLL Power-saving Mode was entered"]
    VALUE2 = 1,
}
impl From<PWDSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: PWDSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWDSTAT`"]
pub type PWDSTAT_R = crate::R<bool, PWDSTAT_A>;
impl PWDSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWDSTAT_A {
        match self.bits {
            false => PWDSTAT_A::VALUE1,
            true => PWDSTAT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PWDSTAT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PWDSTAT_A::VALUE2
    }
}
#[doc = "PLL VCO Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOLOCK_A {
    #[doc = "0: The frequency difference of fREF and fDIV is greater than allowed. The VCO part of the PLL can not lock on a target frequency."]
    VALUE1 = 0,
    #[doc = "1: The frequency difference of fREF and fDIV is small enough to enable a stable VCO operation"]
    VALUE2 = 1,
}
impl From<VCOLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: VCOLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VCOLOCK`"]
pub type VCOLOCK_R = crate::R<bool, VCOLOCK_A>;
impl VCOLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOLOCK_A {
        match self.bits {
            false => VCOLOCK_A::VALUE1,
            true => VCOLOCK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VCOLOCK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VCOLOCK_A::VALUE2
    }
}
#[doc = "Bypass Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BY_A {
    #[doc = "0: Bypass Mode is not entered"]
    VALUE1 = 0,
    #[doc = "1: Bypass Mode is entered. Input fOSC is selected as output fPLL."]
    VALUE2 = 1,
}
impl From<BY_A> for bool {
    #[inline(always)]
    fn from(variant: BY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BY`"]
pub type BY_R = crate::R<bool, BY_A>;
impl BY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BY_A {
        match self.bits {
            false => BY_A::VALUE1,
            true => BY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BY_A::VALUE2
    }
}
#[doc = "PLL LOCK Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOLOCKED_A {
    #[doc = "0: PLL not locked"]
    VALUE1 = 0,
    #[doc = "1: PLL locked"]
    VALUE2 = 1,
}
impl From<VCOLOCKED_A> for bool {
    #[inline(always)]
    fn from(variant: VCOLOCKED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VCOLOCKED`"]
pub type VCOLOCKED_R = crate::R<bool, VCOLOCKED_A>;
impl VCOLOCKED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOLOCKED_A {
        match self.bits {
            false => VCOLOCKED_A::VALUE1,
            true => VCOLOCKED_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VCOLOCKED_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VCOLOCKED_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - VCO Bypass Status"]
    #[inline(always)]
    pub fn vcobyst(&self) -> VCOBYST_R {
        VCOBYST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PLL Power-saving Mode Status"]
    #[inline(always)]
    pub fn pwdstat(&self) -> PWDSTAT_R {
        PWDSTAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PLL VCO Lock Status"]
    #[inline(always)]
    pub fn vcolock(&self) -> VCOLOCK_R {
        VCOLOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Bypass Mode Status"]
    #[inline(always)]
    pub fn by(&self) -> BY_R {
        BY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PLL LOCK Status"]
    #[inline(always)]
    pub fn vcolocked(&self) -> VCOLOCKED_R {
        VCOLOCKED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
