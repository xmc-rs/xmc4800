#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SRRAW {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `PRWARN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRWARNR {
    #[doc = "Inactive"]
    VALUE1,
    #[doc = "Active"]
    VALUE2,
}
impl PRWARNR {
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
            PRWARNR::VALUE1 => false,
            PRWARNR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRWARNR {
        match value {
            false => PRWARNR::VALUE1,
            true => PRWARNR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PRWARNR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PRWARNR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct PIR {
    bits: bool,
}
impl PIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AIR {
    bits: bool,
}
impl AIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct DLROVRR {
    bits: bool,
}
impl DLROVRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `HDCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDCLRR {
    #[doc = "Not updated"]
    VALUE1,
    #[doc = "Update completed"]
    VALUE2,
}
impl HDCLRR {
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
            HDCLRR::VALUE1 => false,
            HDCLRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HDCLRR {
        match value {
            false => HDCLRR::VALUE1,
            true => HDCLRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HDCLRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HDCLRR::VALUE2
    }
}
#[doc = "Possible values of the field `HDSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDSETR {
    #[doc = "Not updated"]
    VALUE1,
    #[doc = "Update completed"]
    VALUE2,
}
impl HDSETR {
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
            HDSETR::VALUE1 => false,
            HDSETR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HDSETR {
        match value {
            false => HDSETR::VALUE1,
            true => HDSETR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HDSETR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HDSETR::VALUE2
    }
}
#[doc = "Possible values of the field `HDCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDCRR {
    #[doc = "Not updated"]
    VALUE1,
    #[doc = "Update completed"]
    VALUE2,
}
impl HDCRR {
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
            HDCRR::VALUE1 => false,
            HDCRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HDCRR {
        match value {
            false => HDCRR::VALUE1,
            true => HDCRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HDCRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HDCRR::VALUE2
    }
}
#[doc = "Possible values of the field `OSCSICTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSICTRLR {
    #[doc = "Not updated"]
    VALUE1,
    #[doc = "Update completed"]
    VALUE2,
}
impl OSCSICTRLR {
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
            OSCSICTRLR::VALUE1 => false,
            OSCSICTRLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSCSICTRLR {
        match value {
            false => OSCSICTRLR::VALUE1,
            true => OSCSICTRLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OSCSICTRLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == OSCSICTRLR::VALUE2
    }
}
#[doc = "Possible values of the field `OSCULCTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCULCTRLR {
    #[doc = "Not updated"]
    VALUE1,
    #[doc = "Update completed"]
    VALUE2,
}
impl OSCULCTRLR {
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
            OSCULCTRLR::VALUE1 => false,
            OSCULCTRLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSCULCTRLR {
        match value {
            false => OSCULCTRLR::VALUE1,
            true => OSCULCTRLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OSCULCTRLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == OSCULCTRLR::VALUE2
    }
}
#[doc = "Possible values of the field `RTC_CTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_CTRR {
    #[doc = "Not updated"]
    VALUE1,
    #[doc = "Update completed"]
    VALUE2,
}
impl RTC_CTRR {
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
            RTC_CTRR::VALUE1 => false,
            RTC_CTRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_CTRR {
        match value {
            false => RTC_CTRR::VALUE1,
            true => RTC_CTRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RTC_CTRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RTC_CTRR::VALUE2
    }
}
#[doc = "Possible values of the field `RTC_ATIM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_ATIM0R {
    #[doc = "Not updated"]
    VALUE1,
    #[doc = "Update completed"]
    VALUE2,
}
impl RTC_ATIM0R {
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
            RTC_ATIM0R::VALUE1 => false,
            RTC_ATIM0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_ATIM0R {
        match value {
            false => RTC_ATIM0R::VALUE1,
            true => RTC_ATIM0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RTC_ATIM0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RTC_ATIM0R::VALUE2
    }
}
#[doc = "Possible values of the field `RTC_ATIM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_ATIM1R {
    #[doc = "Not updated"]
    VALUE1,
    #[doc = "Update completed"]
    VALUE2,
}
impl RTC_ATIM1R {
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
            RTC_ATIM1R::VALUE1 => false,
            RTC_ATIM1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_ATIM1R {
        match value {
            false => RTC_ATIM1R::VALUE1,
            true => RTC_ATIM1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RTC_ATIM1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RTC_ATIM1R::VALUE2
    }
}
#[doc = "Possible values of the field `RTC_TIM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_TIM0R {
    #[doc = "Not updated"]
    VALUE1,
    #[doc = "Update completed"]
    VALUE2,
}
impl RTC_TIM0R {
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
            RTC_TIM0R::VALUE1 => false,
            RTC_TIM0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_TIM0R {
        match value {
            false => RTC_TIM0R::VALUE1,
            true => RTC_TIM0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RTC_TIM0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RTC_TIM0R::VALUE2
    }
}
#[doc = "Possible values of the field `RTC_TIM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_TIM1R {
    #[doc = "Not updated"]
    VALUE1,
    #[doc = "Update completed"]
    VALUE2,
}
impl RTC_TIM1R {
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
            RTC_TIM1R::VALUE1 => false,
            RTC_TIM1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_TIM1R {
        match value {
            false => RTC_TIM1R::VALUE1,
            true => RTC_TIM1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RTC_TIM1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RTC_TIM1R::VALUE2
    }
}
#[doc = "Possible values of the field `RMX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMXR {
    #[doc = "Not updated"]
    VALUE1,
    #[doc = "Update completed"]
    VALUE2,
}
impl RMXR {
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
            RMXR::VALUE1 => false,
            RMXR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RMXR {
        match value {
            false => RMXR::VALUE1,
            true => RMXR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RMXR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RMXR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - WDT pre-warning Interrupt Status Before Masking"]
    #[inline]
    pub fn prwarn(&self) -> PRWARNR {
        PRWARNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - RTC Raw Periodic Interrupt Status Before Masking"]
    #[inline]
    pub fn pi(&self) -> PIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIR { bits }
    }
    #[doc = "Bit 2 - RTC Raw Alarm Interrupt Status Before Masking"]
    #[inline]
    pub fn ai(&self) -> AIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AIR { bits }
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt Status Before Masking"]
    #[inline]
    pub fn dlrovr(&self) -> DLROVRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DLROVRR { bits }
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Status Before Masking"]
    #[inline]
    pub fn hdclr(&self) -> HDCLRR {
        HDCLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Status Before Masking"]
    #[inline]
    pub fn hdset(&self) -> HDSETR {
        HDSETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Status Before Masking"]
    #[inline]
    pub fn hdcr(&self) -> HDCRR {
        HDCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Status Before Masking"]
    #[inline]
    pub fn oscsictrl(&self) -> OSCSICTRLR {
        OSCSICTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Status Before Masking"]
    #[inline]
    pub fn osculctrl(&self) -> OSCULCTRLR {
        OSCULCTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Status Before Masking"]
    #[inline]
    pub fn rtc_ctr(&self) -> RTC_CTRR {
        RTC_CTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Status Before Masking"]
    #[inline]
    pub fn rtc_atim0(&self) -> RTC_ATIM0R {
        RTC_ATIM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Status Before Masking"]
    #[inline]
    pub fn rtc_atim1(&self) -> RTC_ATIM1R {
        RTC_ATIM1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Before Masking Status"]
    #[inline]
    pub fn rtc_tim0(&self) -> RTC_TIM0R {
        RTC_TIM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Status Before Masking"]
    #[inline]
    pub fn rtc_tim1(&self) -> RTC_TIM1R {
        RTC_TIM1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Status Before Masking"]
    #[inline]
    pub fn rmx(&self) -> RMXR {
        RMXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
