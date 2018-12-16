#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PLLSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `VCOBYST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOBYSTR {
    #[doc = "Free-running / Normal Mode is entered"]
    VALUE1,
    #[doc = "Prescaler Mode is entered"]
    VALUE2,
}
impl VCOBYSTR {
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
            VCOBYSTR::VALUE1 => false,
            VCOBYSTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VCOBYSTR {
        match value {
            false => VCOBYSTR::VALUE1,
            true => VCOBYSTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VCOBYSTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VCOBYSTR::VALUE2
    }
}
#[doc = "Possible values of the field `PWDSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWDSTATR {
    #[doc = "PLL Power-saving Mode was not entered"]
    VALUE1,
    #[doc = "PLL Power-saving Mode was entered"]
    VALUE2,
}
impl PWDSTATR {
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
            PWDSTATR::VALUE1 => false,
            PWDSTATR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWDSTATR {
        match value {
            false => PWDSTATR::VALUE1,
            true => PWDSTATR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PWDSTATR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PWDSTATR::VALUE2
    }
}
#[doc = "Possible values of the field `VCOLOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOLOCKR {
    #[doc = "PLL not locked"]
    VALUE1,
    #[doc = "PLL locked"]
    VALUE2,
}
impl VCOLOCKR {
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
            VCOLOCKR::VALUE1 => false,
            VCOLOCKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VCOLOCKR {
        match value {
            false => VCOLOCKR::VALUE1,
            true => VCOLOCKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VCOLOCKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VCOLOCKR::VALUE2
    }
}
#[doc = "Possible values of the field `K1RDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum K1RDYR {
    #[doc = "K1-Divider does not operate with the new value"]
    VALUE1,
    #[doc = "K1-Divider operate with the new value"]
    VALUE2,
}
impl K1RDYR {
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
            K1RDYR::VALUE1 => false,
            K1RDYR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> K1RDYR {
        match value {
            false => K1RDYR::VALUE1,
            true => K1RDYR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == K1RDYR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == K1RDYR::VALUE2
    }
}
#[doc = "Possible values of the field `K2RDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum K2RDYR {
    #[doc = "K2-Divider does not operate with the new value"]
    VALUE1,
    #[doc = "K2-Divider operate with the new value"]
    VALUE2,
}
impl K2RDYR {
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
            K2RDYR::VALUE1 => false,
            K2RDYR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> K2RDYR {
        match value {
            false => K2RDYR::VALUE1,
            true => K2RDYR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == K2RDYR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == K2RDYR::VALUE2
    }
}
#[doc = "Possible values of the field `BY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYR {
    #[doc = "Bypass Mode is not entered"]
    VALUE1,
    #[doc = "Bypass Mode is entered. Input fOSC is selected as output fPLL."]
    VALUE2,
}
impl BYR {
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
            BYR::VALUE1 => false,
            BYR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BYR {
        match value {
            false => BYR::VALUE1,
            true => BYR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BYR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BYR::VALUE2
    }
}
#[doc = "Possible values of the field `PLLLV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLLVR {
    #[doc = "The OSC frequency is not usable. Frequency fREF is too low."]
    VALUE1,
    #[doc = "The OSC frequency is usable"]
    VALUE2,
}
impl PLLLVR {
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
            PLLLVR::VALUE1 => false,
            PLLLVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLLVR {
        match value {
            false => PLLLVR::VALUE1,
            true => PLLLVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PLLLVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PLLLVR::VALUE2
    }
}
#[doc = "Possible values of the field `PLLHV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLHVR {
    #[doc = "The OSC frequency is not usable. Frequency fOSC is too high."]
    VALUE1,
    #[doc = "The OSC frequency is usable"]
    VALUE2,
}
impl PLLHVR {
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
            PLLHVR::VALUE1 => false,
            PLLHVR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLHVR {
        match value {
            false => PLLHVR::VALUE1,
            true => PLLHVR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PLLHVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PLLHVR::VALUE2
    }
}
#[doc = "Possible values of the field `PLLSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSPR {
    #[doc = "The OSC frequency is not usable. Spikes are detected that disturb a locked operation"]
    VALUE1,
    #[doc = "The OSC frequency is usable"]
    VALUE2,
}
impl PLLSPR {
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
            PLLSPR::VALUE1 => false,
            PLLSPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLSPR {
        match value {
            false => PLLSPR::VALUE1,
            true => PLLSPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PLLSPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PLLSPR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - VCO Bypass Status"]
    #[inline]
    pub fn vcobyst(&self) -> VCOBYSTR {
        VCOBYSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - PLL Power-saving Mode Status"]
    #[inline]
    pub fn pwdstat(&self) -> PWDSTATR {
        PWDSTATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - PLL LOCK Status"]
    #[inline]
    pub fn vcolock(&self) -> VCOLOCKR {
        VCOLOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - K1 Divider Ready Status"]
    #[inline]
    pub fn k1rdy(&self) -> K1RDYR {
        K1RDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - K2 Divider Ready Status"]
    #[inline]
    pub fn k2rdy(&self) -> K2RDYR {
        K2RDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Bypass Mode Status"]
    #[inline]
    pub fn by(&self) -> BYR {
        BYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Oscillator for PLL Valid Low Status Bit"]
    #[inline]
    pub fn plllv(&self) -> PLLLVR {
        PLLLVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Oscillator for PLL Valid High Status Bit"]
    #[inline]
    pub fn pllhv(&self) -> PLLHVR {
        PLLHVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Oscillator for PLL Valid Spike Status Bit"]
    #[inline]
    pub fn pllsp(&self) -> PLLSPR {
        PLLSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
