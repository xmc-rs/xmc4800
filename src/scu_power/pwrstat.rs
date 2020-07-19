#[doc = "Reader of register PWRSTAT"]
pub type R = crate::R<u32, super::PWRSTAT>;
#[doc = "Hibernate Domain Enable Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBEN_A {
    #[doc = "0: Inactive"]
    VALUE1 = 0,
    #[doc = "1: Active"]
    VALUE2 = 1,
}
impl From<HIBEN_A> for bool {
    #[inline(always)]
    fn from(variant: HIBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HIBEN`"]
pub type HIBEN_R = crate::R<bool, HIBEN_A>;
impl HIBEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIBEN_A {
        match self.bits {
            false => HIBEN_A::VALUE1,
            true => HIBEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HIBEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HIBEN_A::VALUE2
    }
}
#[doc = "USB PHY Transceiver State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPHYPDQ_A {
    #[doc = "0: Power-down"]
    VALUE1 = 0,
    #[doc = "1: Active"]
    VALUE2 = 1,
}
impl From<USBPHYPDQ_A> for bool {
    #[inline(always)]
    fn from(variant: USBPHYPDQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBPHYPDQ`"]
pub type USBPHYPDQ_R = crate::R<bool, USBPHYPDQ_A>;
impl USBPHYPDQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBPHYPDQ_A {
        match self.bits {
            false => USBPHYPDQ_A::VALUE1,
            true => USBPHYPDQ_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USBPHYPDQ_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USBPHYPDQ_A::VALUE2
    }
}
#[doc = "USB On-The-Go Comparators State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBOTGEN_A {
    #[doc = "0: Power-down"]
    VALUE1 = 0,
    #[doc = "1: Active"]
    VALUE2 = 1,
}
impl From<USBOTGEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBOTGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBOTGEN`"]
pub type USBOTGEN_R = crate::R<bool, USBOTGEN_A>;
impl USBOTGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBOTGEN_A {
        match self.bits {
            false => USBOTGEN_A::VALUE1,
            true => USBOTGEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USBOTGEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USBOTGEN_A::VALUE2
    }
}
#[doc = "USB Weak Pull-Up at PADN State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPUWQ_A {
    #[doc = "0: Pull-up active"]
    VALUE1 = 0,
    #[doc = "1: Pull-up not active"]
    VALUE2 = 1,
}
impl From<USBPUWQ_A> for bool {
    #[inline(always)]
    fn from(variant: USBPUWQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBPUWQ`"]
pub type USBPUWQ_R = crate::R<bool, USBPUWQ_A>;
impl USBPUWQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBPUWQ_A {
        match self.bits {
            false => USBPUWQ_A::VALUE1,
            true => USBPUWQ_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USBPUWQ_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USBPUWQ_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Hibernate Domain Enable Status"]
    #[inline(always)]
    pub fn hiben(&self) -> HIBEN_R {
        HIBEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - USB PHY Transceiver State"]
    #[inline(always)]
    pub fn usbphypdq(&self) -> USBPHYPDQ_R {
        USBPHYPDQ_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - USB On-The-Go Comparators State"]
    #[inline(always)]
    pub fn usbotgen(&self) -> USBOTGEN_R {
        USBOTGEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - USB Weak Pull-Up at PADN State"]
    #[inline(always)]
    pub fn usbpuwq(&self) -> USBPUWQ_R {
        USBPUWQ_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
