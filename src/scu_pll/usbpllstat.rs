#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USBPLLSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `VCOBYST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOBYSTR {
    #[doc = "Normal Mode is entered"]
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
    #[doc = "The frequency difference of fREF and fDIV is greater than allowed. The VCO part of the PLL can not lock on a target frequency."]
    VALUE1,
    #[doc = "The frequency difference of fREF and fDIV is small enough to enable a stable VCO operation"]
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
#[doc = "Possible values of the field `VCOLOCKED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOLOCKEDR {
    #[doc = "PLL not locked"]
    VALUE1,
    #[doc = "PLL locked"]
    VALUE2,
}
impl VCOLOCKEDR {
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
            VCOLOCKEDR::VALUE1 => false,
            VCOLOCKEDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VCOLOCKEDR {
        match value {
            false => VCOLOCKEDR::VALUE1,
            true => VCOLOCKEDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VCOLOCKEDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VCOLOCKEDR::VALUE2
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
    #[doc = "Bit 2 - PLL VCO Lock Status"]
    #[inline]
    pub fn vcolock(&self) -> VCOLOCKR {
        VCOLOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 7 - PLL LOCK Status"]
    #[inline]
    pub fn vcolocked(&self) -> VCOLOCKEDR {
        VCOLOCKEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
