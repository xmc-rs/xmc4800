#[doc = "Reader of register CGATSTAT1"]
pub type R = crate::R<u32, super::CGATSTAT1>;
#[doc = "CCU43 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU43_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1,
    #[doc = "1: Gating asserted"]
    VALUE2,
}
impl From<CCU43_A> for bool {
    #[inline(always)]
    fn from(variant: CCU43_A) -> Self {
        match variant {
            CCU43_A::VALUE1 => false,
            CCU43_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CCU43`"]
pub type CCU43_R = crate::R<bool, CCU43_A>;
impl CCU43_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU43_A {
        match self.bits {
            false => CCU43_A::VALUE1,
            true => CCU43_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCU43_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCU43_A::VALUE2
    }
}
#[doc = "LEDTS Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEDTSCU0_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1,
    #[doc = "1: Gating asserted"]
    VALUE2,
}
impl From<LEDTSCU0_A> for bool {
    #[inline(always)]
    fn from(variant: LEDTSCU0_A) -> Self {
        match variant {
            LEDTSCU0_A::VALUE1 => false,
            LEDTSCU0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `LEDTSCU0`"]
pub type LEDTSCU0_R = crate::R<bool, LEDTSCU0_A>;
impl LEDTSCU0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEDTSCU0_A {
        match self.bits {
            false => LEDTSCU0_A::VALUE1,
            true => LEDTSCU0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LEDTSCU0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LEDTSCU0_A::VALUE2
    }
}
#[doc = "MultiCAN Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCAN0_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1,
    #[doc = "1: Gating asserted"]
    VALUE2,
}
impl From<MCAN0_A> for bool {
    #[inline(always)]
    fn from(variant: MCAN0_A) -> Self {
        match variant {
            MCAN0_A::VALUE1 => false,
            MCAN0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `MCAN0`"]
pub type MCAN0_R = crate::R<bool, MCAN0_A>;
impl MCAN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCAN0_A {
        match self.bits {
            false => MCAN0_A::VALUE1,
            true => MCAN0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MCAN0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCAN0_A::VALUE2
    }
}
#[doc = "DAC Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1,
    #[doc = "1: Gating asserted"]
    VALUE2,
}
impl From<DAC_A> for bool {
    #[inline(always)]
    fn from(variant: DAC_A) -> Self {
        match variant {
            DAC_A::VALUE1 => false,
            DAC_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DAC`"]
pub type DAC_R = crate::R<bool, DAC_A>;
impl DAC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC_A {
        match self.bits {
            false => DAC_A::VALUE1,
            true => DAC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DAC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DAC_A::VALUE2
    }
}
#[doc = "MMC Interface Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMCI_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1,
    #[doc = "1: Gating asserted"]
    VALUE2,
}
impl From<MMCI_A> for bool {
    #[inline(always)]
    fn from(variant: MMCI_A) -> Self {
        match variant {
            MMCI_A::VALUE1 => false,
            MMCI_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `MMCI`"]
pub type MMCI_R = crate::R<bool, MMCI_A>;
impl MMCI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMCI_A {
        match self.bits {
            false => MMCI_A::VALUE1,
            true => MMCI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MMCI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MMCI_A::VALUE2
    }
}
#[doc = "USIC1 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC1_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1,
    #[doc = "1: Gating asserted"]
    VALUE2,
}
impl From<USIC1_A> for bool {
    #[inline(always)]
    fn from(variant: USIC1_A) -> Self {
        match variant {
            USIC1_A::VALUE1 => false,
            USIC1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `USIC1`"]
pub type USIC1_R = crate::R<bool, USIC1_A>;
impl USIC1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USIC1_A {
        match self.bits {
            false => USIC1_A::VALUE1,
            true => USIC1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USIC1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USIC1_A::VALUE2
    }
}
#[doc = "USIC2 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC2_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1,
    #[doc = "1: Gating asserted"]
    VALUE2,
}
impl From<USIC2_A> for bool {
    #[inline(always)]
    fn from(variant: USIC2_A) -> Self {
        match variant {
            USIC2_A::VALUE1 => false,
            USIC2_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `USIC2`"]
pub type USIC2_R = crate::R<bool, USIC2_A>;
impl USIC2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USIC2_A {
        match self.bits {
            false => USIC2_A::VALUE1,
            true => USIC2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USIC2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USIC2_A::VALUE2
    }
}
#[doc = "PORTS Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPORTS_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1,
    #[doc = "1: Gating asserted"]
    VALUE2,
}
impl From<PPORTS_A> for bool {
    #[inline(always)]
    fn from(variant: PPORTS_A) -> Self {
        match variant {
            PPORTS_A::VALUE1 => false,
            PPORTS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PPORTS`"]
pub type PPORTS_R = crate::R<bool, PPORTS_A>;
impl PPORTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPORTS_A {
        match self.bits {
            false => PPORTS_A::VALUE1,
            true => PPORTS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PPORTS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PPORTS_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - CCU43 Gating Status"]
    #[inline(always)]
    pub fn ccu43(&self) -> CCU43_R {
        CCU43_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - LEDTS Gating Status"]
    #[inline(always)]
    pub fn ledtscu0(&self) -> LEDTSCU0_R {
        LEDTSCU0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MultiCAN Gating Status"]
    #[inline(always)]
    pub fn mcan0(&self) -> MCAN0_R {
        MCAN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DAC Gating Status"]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MMC Interface Gating Status"]
    #[inline(always)]
    pub fn mmci(&self) -> MMCI_R {
        MMCI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USIC1 Gating Status"]
    #[inline(always)]
    pub fn usic1(&self) -> USIC1_R {
        USIC1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USIC2 Gating Status"]
    #[inline(always)]
    pub fn usic2(&self) -> USIC2_R {
        USIC2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PORTS Gating Status"]
    #[inline(always)]
    pub fn pports(&self) -> PPORTS_R {
        PPORTS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
