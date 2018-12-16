#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PRSTAT1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `CCU43RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU43RSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl CCU43RSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CCU43RSR::VALUE1 => false,
            CCU43RSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCU43RSR {
        match value {
            false => CCU43RSR::VALUE1,
            true => CCU43RSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CCU43RSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CCU43RSR::VALUE2
    }
}
#[doc = "Possible values of the field `LEDTSCU0RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEDTSCU0RSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl LEDTSCU0RSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LEDTSCU0RSR::VALUE1 => false,
            LEDTSCU0RSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LEDTSCU0RSR {
        match value {
            false => LEDTSCU0RSR::VALUE1,
            true => LEDTSCU0RSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LEDTSCU0RSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LEDTSCU0RSR::VALUE2
    }
}
#[doc = "Possible values of the field `MCAN0RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCAN0RSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl MCAN0RSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MCAN0RSR::VALUE1 => false,
            MCAN0RSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCAN0RSR {
        match value {
            false => MCAN0RSR::VALUE1,
            true => MCAN0RSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MCAN0RSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MCAN0RSR::VALUE2
    }
}
#[doc = "Possible values of the field `DACRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACRSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl DACRSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DACRSR::VALUE1 => false,
            DACRSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACRSR {
        match value {
            false => DACRSR::VALUE1,
            true => DACRSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DACRSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DACRSR::VALUE2
    }
}
#[doc = "Possible values of the field `MMCIRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMCIRSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl MMCIRSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MMCIRSR::VALUE1 => false,
            MMCIRSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MMCIRSR {
        match value {
            false => MMCIRSR::VALUE1,
            true => MMCIRSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MMCIRSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MMCIRSR::VALUE2
    }
}
#[doc = "Possible values of the field `USIC1RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC1RSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl USIC1RSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            USIC1RSR::VALUE1 => false,
            USIC1RSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USIC1RSR {
        match value {
            false => USIC1RSR::VALUE1,
            true => USIC1RSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == USIC1RSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == USIC1RSR::VALUE2
    }
}
#[doc = "Possible values of the field `USIC2RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC2RSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl USIC2RSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            USIC2RSR::VALUE1 => false,
            USIC2RSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USIC2RSR {
        match value {
            false => USIC2RSR::VALUE1,
            true => USIC2RSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == USIC2RSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == USIC2RSR::VALUE2
    }
}
#[doc = "Possible values of the field `PPORTSRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPORTSRSR {
    #[doc = "Reset de-asserted"]
    VALUE1,
    #[doc = "Reset asserted"]
    VALUE2,
}
impl PPORTSRSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PPORTSRSR::VALUE1 => false,
            PPORTSRSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPORTSRSR {
        match value {
            false => PPORTSRSR::VALUE1,
            true => PPORTSRSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PPORTSRSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PPORTSRSR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - CCU43 Reset Status"]
    #[inline]
    pub fn ccu43rs(&self) -> CCU43RSR {
        CCU43RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - LEDTS Reset Status"]
    #[inline]
    pub fn ledtscu0rs(&self) -> LEDTSCU0RSR {
        LEDTSCU0RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - MultiCAN Reset Status"]
    #[inline]
    pub fn mcan0rs(&self) -> MCAN0RSR {
        MCAN0RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - DAC Reset Status"]
    #[inline]
    pub fn dacrs(&self) -> DACRSR {
        DACRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - MMC Interface Reset Status"]
    #[inline]
    pub fn mmcirs(&self) -> MMCIRSR {
        MMCIRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - USIC1 Reset Status"]
    #[inline]
    pub fn usic1rs(&self) -> USIC1RSR {
        USIC1RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - USIC2 Reset Status"]
    #[inline]
    pub fn usic2rs(&self) -> USIC2RSR {
        USIC2RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - PORTS Reset Status"]
    #[inline]
    pub fn pportsrs(&self) -> PPORTSRSR {
        PPORTSRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
