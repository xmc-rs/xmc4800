#[doc = "Reader of register TRAPRAW"]
pub type R = crate::R<u32, super::TRAPRAW>;
#[doc = "OSC_HP Oscillator Watchdog Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCWDGT_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<SOSCWDGT_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCWDGT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SOSCWDGT`"]
pub type SOSCWDGT_R = crate::R<bool, SOSCWDGT_A>;
impl SOSCWDGT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCWDGT_A {
        match self.bits {
            false => SOSCWDGT_A::VALUE1,
            true => SOSCWDGT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SOSCWDGT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SOSCWDGT_A::VALUE2
    }
}
#[doc = "System VCO Lock Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCOLCKT_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<SVCOLCKT_A> for bool {
    #[inline(always)]
    fn from(variant: SVCOLCKT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SVCOLCKT`"]
pub type SVCOLCKT_R = crate::R<bool, SVCOLCKT_A>;
impl SVCOLCKT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVCOLCKT_A {
        match self.bits {
            false => SVCOLCKT_A::VALUE1,
            true => SVCOLCKT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SVCOLCKT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SVCOLCKT_A::VALUE2
    }
}
#[doc = "USB VCO Lock Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UVCOLCKT_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<UVCOLCKT_A> for bool {
    #[inline(always)]
    fn from(variant: UVCOLCKT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UVCOLCKT`"]
pub type UVCOLCKT_R = crate::R<bool, UVCOLCKT_A>;
impl UVCOLCKT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UVCOLCKT_A {
        match self.bits {
            false => UVCOLCKT_A::VALUE1,
            true => UVCOLCKT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == UVCOLCKT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == UVCOLCKT_A::VALUE2
    }
}
#[doc = "Parity Error Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PET_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<PET_A> for bool {
    #[inline(always)]
    fn from(variant: PET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PET`"]
pub type PET_R = crate::R<bool, PET_A>;
impl PET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PET_A {
        match self.bits {
            false => PET_A::VALUE1,
            true => PET_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PET_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PET_A::VALUE2
    }
}
#[doc = "Brown Out Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRWNT_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<BRWNT_A> for bool {
    #[inline(always)]
    fn from(variant: BRWNT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRWNT`"]
pub type BRWNT_R = crate::R<bool, BRWNT_A>;
impl BRWNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRWNT_A {
        match self.bits {
            false => BRWNT_A::VALUE1,
            true => BRWNT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BRWNT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BRWNT_A::VALUE2
    }
}
#[doc = "OSC_ULP Oscillator Watchdog Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULPWDGT_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<ULPWDGT_A> for bool {
    #[inline(always)]
    fn from(variant: ULPWDGT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ULPWDGT`"]
pub type ULPWDGT_R = crate::R<bool, ULPWDGT_A>;
impl ULPWDGT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ULPWDGT_A {
        match self.bits {
            false => ULPWDGT_A::VALUE1,
            true => ULPWDGT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ULPWDGT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ULPWDGT_A::VALUE2
    }
}
#[doc = "Peripheral Bridge 0 Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWERR0T_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<BWERR0T_A> for bool {
    #[inline(always)]
    fn from(variant: BWERR0T_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BWERR0T`"]
pub type BWERR0T_R = crate::R<bool, BWERR0T_A>;
impl BWERR0T_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWERR0T_A {
        match self.bits {
            false => BWERR0T_A::VALUE1,
            true => BWERR0T_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BWERR0T_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BWERR0T_A::VALUE2
    }
}
#[doc = "Peripheral Bridge 1 Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWERR1T_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<BWERR1T_A> for bool {
    #[inline(always)]
    fn from(variant: BWERR1T_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BWERR1T`"]
pub type BWERR1T_R = crate::R<bool, BWERR1T_A>;
impl BWERR1T_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWERR1T_A {
        match self.bits {
            false => BWERR1T_A::VALUE1,
            true => BWERR1T_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BWERR1T_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BWERR1T_A::VALUE2
    }
}
#[doc = "EtherCat Reset 0 Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECAT0RST_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<ECAT0RST_A> for bool {
    #[inline(always)]
    fn from(variant: ECAT0RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ECAT0RST`"]
pub type ECAT0RST_R = crate::R<bool, ECAT0RST_A>;
impl ECAT0RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECAT0RST_A {
        match self.bits {
            false => ECAT0RST_A::VALUE1,
            true => ECAT0RST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ECAT0RST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ECAT0RST_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Raw Status"]
    #[inline(always)]
    pub fn soscwdgt(&self) -> SOSCWDGT_R {
        SOSCWDGT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - System VCO Lock Trap Raw Status"]
    #[inline(always)]
    pub fn svcolckt(&self) -> SVCOLCKT_R {
        SVCOLCKT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Raw Status"]
    #[inline(always)]
    pub fn uvcolckt(&self) -> UVCOLCKT_R {
        UVCOLCKT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Parity Error Trap Raw Status"]
    #[inline(always)]
    pub fn pet(&self) -> PET_R {
        PET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Brown Out Trap Raw Status"]
    #[inline(always)]
    pub fn brwnt(&self) -> BRWNT_R {
        BRWNT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Raw Status"]
    #[inline(always)]
    pub fn ulpwdgt(&self) -> ULPWDGT_R {
        ULPWDGT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Raw Status"]
    #[inline(always)]
    pub fn bwerr0t(&self) -> BWERR0T_R {
        BWERR0T_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Raw Status"]
    #[inline(always)]
    pub fn bwerr1t(&self) -> BWERR1T_R {
        BWERR1T_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - EtherCat Reset 0 Trap Raw Status"]
    #[inline(always)]
    pub fn ecat0rst(&self) -> ECAT0RST_R {
        ECAT0RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
