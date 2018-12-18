#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HDCR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `WKPEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKPEPR {
    #[doc = "Wake-up event disabled"]
    VALUE1,
    #[doc = "Wake-up event enabled"]
    VALUE2,
}
impl WKPEPR {
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
            WKPEPR::VALUE1 => false,
            WKPEPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKPEPR {
        match value {
            false => WKPEPR::VALUE1,
            true => WKPEPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WKPEPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WKPEPR::VALUE2
    }
}
#[doc = "Possible values of the field `WKPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKPENR {
    #[doc = "Wake-up event disabled"]
    VALUE1,
    #[doc = "Wake-up event enabled"]
    VALUE2,
}
impl WKPENR {
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
            WKPENR::VALUE1 => false,
            WKPENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKPENR {
        match value {
            false => WKPENR::VALUE1,
            true => WKPENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WKPENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WKPENR::VALUE2
    }
}
#[doc = "Possible values of the field `RTCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCER {
    #[doc = "Wake-up event disabled"]
    VALUE1,
    #[doc = "Wake-up event enabled"]
    VALUE2,
}
impl RTCER {
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
            RTCER::VALUE1 => false,
            RTCER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTCER {
        match value {
            false => RTCER::VALUE1,
            true => RTCER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RTCER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RTCER::VALUE2
    }
}
#[doc = "Possible values of the field `ULPWDGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULPWDGENR {
    #[doc = "Wake-up event disabled"]
    VALUE1,
    #[doc = "Wake-up event enabled"]
    VALUE2,
}
impl ULPWDGENR {
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
            ULPWDGENR::VALUE1 => false,
            ULPWDGENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ULPWDGENR {
        match value {
            false => ULPWDGENR::VALUE1,
            true => ULPWDGENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ULPWDGENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ULPWDGENR::VALUE2
    }
}
#[doc = "Possible values of the field `HIB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBR {
    #[doc = "External hibernate request inactive"]
    VALUE1,
    #[doc = "External hibernate request active"]
    VALUE2,
}
impl HIBR {
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
            HIBR::VALUE1 => false,
            HIBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIBR {
        match value {
            false => HIBR::VALUE1,
            true => HIBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HIBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HIBR::VALUE2
    }
}
#[doc = "Possible values of the field `RCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCSR {
    #[doc = "fOSI selected"]
    VALUE1,
    #[doc = "fULP selected"]
    VALUE2,
}
impl RCSR {
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
            RCSR::VALUE1 => false,
            RCSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCSR {
        match value {
            false => RCSR::VALUE1,
            true => RCSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RCSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RCSR::VALUE2
    }
}
#[doc = "Possible values of the field `STDBYSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STDBYSELR {
    #[doc = "fOSI selected"]
    VALUE1,
    #[doc = "fULP selected"]
    VALUE2,
}
impl STDBYSELR {
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
            STDBYSELR::VALUE1 => false,
            STDBYSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STDBYSELR {
        match value {
            false => STDBYSELR::VALUE1,
            true => STDBYSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STDBYSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STDBYSELR::VALUE2
    }
}
#[doc = "Possible values of the field `WKUPSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPSELR {
    #[doc = "HIB_IO_1 pin selected"]
    VALUE1,
    #[doc = "HIB_IO_0 pin selected"]
    VALUE2,
}
impl WKUPSELR {
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
            WKUPSELR::VALUE1 => false,
            WKUPSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPSELR {
        match value {
            false => WKUPSELR::VALUE1,
            true => WKUPSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WKUPSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WKUPSELR::VALUE2
    }
}
#[doc = "Possible values of the field `GPI0SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPI0SELR {
    #[doc = "HIB_IO_1 pin selected"]
    VALUE1,
    #[doc = "HIB_IO_0 pin selected"]
    VALUE2,
}
impl GPI0SELR {
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
            GPI0SELR::VALUE1 => false,
            GPI0SELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPI0SELR {
        match value {
            false => GPI0SELR::VALUE1,
            true => GPI0SELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GPI0SELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GPI0SELR::VALUE2
    }
}
#[doc = "Possible values of the field `HIBIO0POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBIO0POLR {
    #[doc = "Direct value"]
    VALUE1,
    #[doc = "Inverted value"]
    VALUE2,
}
impl HIBIO0POLR {
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
            HIBIO0POLR::VALUE1 => false,
            HIBIO0POLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIBIO0POLR {
        match value {
            false => HIBIO0POLR::VALUE1,
            true => HIBIO0POLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HIBIO0POLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HIBIO0POLR::VALUE2
    }
}
#[doc = "Possible values of the field `HIBIO1POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBIO1POLR {
    #[doc = "Direct value"]
    VALUE1,
    #[doc = "Inverted value"]
    VALUE2,
}
impl HIBIO1POLR {
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
            HIBIO1POLR::VALUE1 => false,
            HIBIO1POLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIBIO1POLR {
        match value {
            false => HIBIO1POLR::VALUE1,
            true => HIBIO1POLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HIBIO1POLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HIBIO1POLR::VALUE2
    }
}
#[doc = "Possible values of the field `HIBIO0SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBIO0SELR {
    #[doc = "Direct input, No input pull device connected"]
    VALUE1,
    #[doc = "Direct input, Input pull-down device connected"]
    VALUE2,
    #[doc = "Direct input, Input pull-up device connected"]
    VALUE3,
    #[doc = "Push-pull HIB Control output"]
    VALUE4,
    #[doc = "Push-pull WDT service output"]
    VALUE5,
    #[doc = "Push-pull GPIO output"]
    VALUE6,
    #[doc = "Open-drain HIB Control output"]
    VALUE7,
    #[doc = "Open-drain WDT service output"]
    VALUE8,
    #[doc = "Open-drain GPIO output"]
    VALUE9,
    #[doc = "Analog input"]
    VALUE10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HIBIO0SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HIBIO0SELR::VALUE1 => 0,
            HIBIO0SELR::VALUE2 => 1,
            HIBIO0SELR::VALUE3 => 2,
            HIBIO0SELR::VALUE4 => 8,
            HIBIO0SELR::VALUE5 => 9,
            HIBIO0SELR::VALUE6 => 10,
            HIBIO0SELR::VALUE7 => 12,
            HIBIO0SELR::VALUE8 => 13,
            HIBIO0SELR::VALUE9 => 14,
            HIBIO0SELR::VALUE10 => 15,
            HIBIO0SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HIBIO0SELR {
        match value {
            0 => HIBIO0SELR::VALUE1,
            1 => HIBIO0SELR::VALUE2,
            2 => HIBIO0SELR::VALUE3,
            8 => HIBIO0SELR::VALUE4,
            9 => HIBIO0SELR::VALUE5,
            10 => HIBIO0SELR::VALUE6,
            12 => HIBIO0SELR::VALUE7,
            13 => HIBIO0SELR::VALUE8,
            14 => HIBIO0SELR::VALUE9,
            15 => HIBIO0SELR::VALUE10,
            i => HIBIO0SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HIBIO0SELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HIBIO0SELR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == HIBIO0SELR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == HIBIO0SELR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == HIBIO0SELR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == HIBIO0SELR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == HIBIO0SELR::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == HIBIO0SELR::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == HIBIO0SELR::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == HIBIO0SELR::VALUE10
    }
}
#[doc = "Possible values of the field `HIBIO1SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBIO1SELR {
    #[doc = "Direct input, No input pull device connected"]
    VALUE1,
    #[doc = "Direct input, Input pull-down device connected"]
    VALUE2,
    #[doc = "Direct input, Input pull-up device connected"]
    VALUE3,
    #[doc = "Push-pull HIB Control output"]
    VALUE4,
    #[doc = "Push-pull WDT service output"]
    VALUE5,
    #[doc = "Push-pull GPIO output"]
    VALUE6,
    #[doc = "Open-drain HIB Control output"]
    VALUE7,
    #[doc = "Open-drain WDT service output"]
    VALUE8,
    #[doc = "Open-drain GPIO output"]
    VALUE9,
    #[doc = "Analog input"]
    VALUE10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HIBIO1SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HIBIO1SELR::VALUE1 => 0,
            HIBIO1SELR::VALUE2 => 1,
            HIBIO1SELR::VALUE3 => 2,
            HIBIO1SELR::VALUE4 => 8,
            HIBIO1SELR::VALUE5 => 9,
            HIBIO1SELR::VALUE6 => 10,
            HIBIO1SELR::VALUE7 => 12,
            HIBIO1SELR::VALUE8 => 13,
            HIBIO1SELR::VALUE9 => 14,
            HIBIO1SELR::VALUE10 => 15,
            HIBIO1SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HIBIO1SELR {
        match value {
            0 => HIBIO1SELR::VALUE1,
            1 => HIBIO1SELR::VALUE2,
            2 => HIBIO1SELR::VALUE3,
            8 => HIBIO1SELR::VALUE4,
            9 => HIBIO1SELR::VALUE5,
            10 => HIBIO1SELR::VALUE6,
            12 => HIBIO1SELR::VALUE7,
            13 => HIBIO1SELR::VALUE8,
            14 => HIBIO1SELR::VALUE9,
            15 => HIBIO1SELR::VALUE10,
            i => HIBIO1SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HIBIO1SELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HIBIO1SELR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == HIBIO1SELR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == HIBIO1SELR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == HIBIO1SELR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == HIBIO1SELR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == HIBIO1SELR::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == HIBIO1SELR::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == HIBIO1SELR::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == HIBIO1SELR::VALUE10
    }
}
#[doc = "Values that can be written to the field `WKPEP`"]
pub enum WKPEPW {
    #[doc = "Wake-up event disabled"]
    VALUE1,
    #[doc = "Wake-up event enabled"]
    VALUE2,
}
impl WKPEPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKPEPW::VALUE1 => false,
            WKPEPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKPEPW<'a> {
    w: &'a mut W,
}
impl<'a> _WKPEPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKPEPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WKPEPW::VALUE1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WKPEPW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKPEN`"]
pub enum WKPENW {
    #[doc = "Wake-up event disabled"]
    VALUE1,
    #[doc = "Wake-up event enabled"]
    VALUE2,
}
impl WKPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKPENW::VALUE1 => false,
            WKPENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKPENW<'a> {
    w: &'a mut W,
}
impl<'a> _WKPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WKPENW::VALUE1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WKPENW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTCE`"]
pub enum RTCEW {
    #[doc = "Wake-up event disabled"]
    VALUE1,
    #[doc = "Wake-up event enabled"]
    VALUE2,
}
impl RTCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCEW::VALUE1 => false,
            RTCEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCEW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTCEW::VALUE1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTCEW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ULPWDGEN`"]
pub enum ULPWDGENW {
    #[doc = "Wake-up event disabled"]
    VALUE1,
    #[doc = "Wake-up event enabled"]
    VALUE2,
}
impl ULPWDGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ULPWDGENW::VALUE1 => false,
            ULPWDGENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ULPWDGENW<'a> {
    w: &'a mut W,
}
impl<'a> _ULPWDGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ULPWDGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ULPWDGENW::VALUE1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ULPWDGENW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HIB`"]
pub enum HIBW {
    #[doc = "External hibernate request inactive"]
    VALUE1,
    #[doc = "External hibernate request active"]
    VALUE2,
}
impl HIBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HIBW::VALUE1 => false,
            HIBW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIBW<'a> {
    w: &'a mut W,
}
impl<'a> _HIBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External hibernate request inactive"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HIBW::VALUE1)
    }
    #[doc = "External hibernate request active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HIBW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCS`"]
pub enum RCSW {
    #[doc = "fOSI selected"]
    VALUE1,
    #[doc = "fULP selected"]
    VALUE2,
}
impl RCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCSW::VALUE1 => false,
            RCSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCSW<'a> {
    w: &'a mut W,
}
impl<'a> _RCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "fOSI selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RCSW::VALUE1)
    }
    #[doc = "fULP selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RCSW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STDBYSEL`"]
pub enum STDBYSELW {
    #[doc = "fOSI selected"]
    VALUE1,
    #[doc = "fULP selected"]
    VALUE2,
}
impl STDBYSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STDBYSELW::VALUE1 => false,
            STDBYSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STDBYSELW<'a> {
    w: &'a mut W,
}
impl<'a> _STDBYSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STDBYSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "fOSI selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STDBYSELW::VALUE1)
    }
    #[doc = "fULP selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STDBYSELW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPSEL`"]
pub enum WKUPSELW {
    #[doc = "HIB_IO_1 pin selected"]
    VALUE1,
    #[doc = "HIB_IO_0 pin selected"]
    VALUE2,
}
impl WKUPSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPSELW::VALUE1 => false,
            WKUPSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HIB_IO_1 pin selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WKUPSELW::VALUE1)
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WKUPSELW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPI0SEL`"]
pub enum GPI0SELW {
    #[doc = "HIB_IO_1 pin selected"]
    VALUE1,
    #[doc = "HIB_IO_0 pin selected"]
    VALUE2,
}
impl GPI0SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPI0SELW::VALUE1 => false,
            GPI0SELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPI0SELW<'a> {
    w: &'a mut W,
}
impl<'a> _GPI0SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPI0SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HIB_IO_1 pin selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GPI0SELW::VALUE1)
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GPI0SELW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HIBIO0POL`"]
pub enum HIBIO0POLW {
    #[doc = "Direct value"]
    VALUE1,
    #[doc = "Inverted value"]
    VALUE2,
}
impl HIBIO0POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HIBIO0POLW::VALUE1 => false,
            HIBIO0POLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIBIO0POLW<'a> {
    w: &'a mut W,
}
impl<'a> _HIBIO0POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIBIO0POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Direct value"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HIBIO0POLW::VALUE1)
    }
    #[doc = "Inverted value"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HIBIO0POLW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HIBIO1POL`"]
pub enum HIBIO1POLW {
    #[doc = "Direct value"]
    VALUE1,
    #[doc = "Inverted value"]
    VALUE2,
}
impl HIBIO1POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HIBIO1POLW::VALUE1 => false,
            HIBIO1POLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIBIO1POLW<'a> {
    w: &'a mut W,
}
impl<'a> _HIBIO1POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIBIO1POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Direct value"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HIBIO1POLW::VALUE1)
    }
    #[doc = "Inverted value"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HIBIO1POLW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HIBIO0SEL`"]
pub enum HIBIO0SELW {
    #[doc = "Direct input, No input pull device connected"]
    VALUE1,
    #[doc = "Direct input, Input pull-down device connected"]
    VALUE2,
    #[doc = "Direct input, Input pull-up device connected"]
    VALUE3,
    #[doc = "Push-pull HIB Control output"]
    VALUE4,
    #[doc = "Push-pull WDT service output"]
    VALUE5,
    #[doc = "Push-pull GPIO output"]
    VALUE6,
    #[doc = "Open-drain HIB Control output"]
    VALUE7,
    #[doc = "Open-drain WDT service output"]
    VALUE8,
    #[doc = "Open-drain GPIO output"]
    VALUE9,
    #[doc = "Analog input"]
    VALUE10,
}
impl HIBIO0SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HIBIO0SELW::VALUE1 => 0,
            HIBIO0SELW::VALUE2 => 1,
            HIBIO0SELW::VALUE3 => 2,
            HIBIO0SELW::VALUE4 => 8,
            HIBIO0SELW::VALUE5 => 9,
            HIBIO0SELW::VALUE6 => 10,
            HIBIO0SELW::VALUE7 => 12,
            HIBIO0SELW::VALUE8 => 13,
            HIBIO0SELW::VALUE9 => 14,
            HIBIO0SELW::VALUE10 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIBIO0SELW<'a> {
    w: &'a mut W,
}
impl<'a> _HIBIO0SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIBIO0SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Direct input, No input pull device connected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HIBIO0SELW::VALUE1)
    }
    #[doc = "Direct input, Input pull-down device connected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HIBIO0SELW::VALUE2)
    }
    #[doc = "Direct input, Input pull-up device connected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(HIBIO0SELW::VALUE3)
    }
    #[doc = "Push-pull HIB Control output"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(HIBIO0SELW::VALUE4)
    }
    #[doc = "Push-pull WDT service output"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(HIBIO0SELW::VALUE5)
    }
    #[doc = "Push-pull GPIO output"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(HIBIO0SELW::VALUE6)
    }
    #[doc = "Open-drain HIB Control output"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(HIBIO0SELW::VALUE7)
    }
    #[doc = "Open-drain WDT service output"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(HIBIO0SELW::VALUE8)
    }
    #[doc = "Open-drain GPIO output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(HIBIO0SELW::VALUE9)
    }
    #[doc = "Analog input"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(HIBIO0SELW::VALUE10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HIBIO1SEL`"]
pub enum HIBIO1SELW {
    #[doc = "Direct input, No input pull device connected"]
    VALUE1,
    #[doc = "Direct input, Input pull-down device connected"]
    VALUE2,
    #[doc = "Direct input, Input pull-up device connected"]
    VALUE3,
    #[doc = "Push-pull HIB Control output"]
    VALUE4,
    #[doc = "Push-pull WDT service output"]
    VALUE5,
    #[doc = "Push-pull GPIO output"]
    VALUE6,
    #[doc = "Open-drain HIB Control output"]
    VALUE7,
    #[doc = "Open-drain WDT service output"]
    VALUE8,
    #[doc = "Open-drain GPIO output"]
    VALUE9,
    #[doc = "Analog input"]
    VALUE10,
}
impl HIBIO1SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HIBIO1SELW::VALUE1 => 0,
            HIBIO1SELW::VALUE2 => 1,
            HIBIO1SELW::VALUE3 => 2,
            HIBIO1SELW::VALUE4 => 8,
            HIBIO1SELW::VALUE5 => 9,
            HIBIO1SELW::VALUE6 => 10,
            HIBIO1SELW::VALUE7 => 12,
            HIBIO1SELW::VALUE8 => 13,
            HIBIO1SELW::VALUE9 => 14,
            HIBIO1SELW::VALUE10 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIBIO1SELW<'a> {
    w: &'a mut W,
}
impl<'a> _HIBIO1SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIBIO1SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Direct input, No input pull device connected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HIBIO1SELW::VALUE1)
    }
    #[doc = "Direct input, Input pull-down device connected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HIBIO1SELW::VALUE2)
    }
    #[doc = "Direct input, Input pull-up device connected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(HIBIO1SELW::VALUE3)
    }
    #[doc = "Push-pull HIB Control output"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(HIBIO1SELW::VALUE4)
    }
    #[doc = "Push-pull WDT service output"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(HIBIO1SELW::VALUE5)
    }
    #[doc = "Push-pull GPIO output"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(HIBIO1SELW::VALUE6)
    }
    #[doc = "Open-drain HIB Control output"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(HIBIO1SELW::VALUE7)
    }
    #[doc = "Open-drain WDT service output"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(HIBIO1SELW::VALUE8)
    }
    #[doc = "Open-drain GPIO output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(HIBIO1SELW::VALUE9)
    }
    #[doc = "Analog input"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(HIBIO1SELW::VALUE10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Wake-Up on Pin Event Positive Edge Enable"]
    #[inline]
    pub fn wkpep(&self) -> WKPEPR {
        WKPEPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Wake-up on Pin Event Negative Edge Enable"]
    #[inline]
    pub fn wkpen(&self) -> WKPENR {
        WKPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Wake-up on RTC Event Enable"]
    #[inline]
    pub fn rtce(&self) -> RTCER {
        RTCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - ULP WDG Alarm Enable"]
    #[inline]
    pub fn ulpwdgen(&self) -> ULPWDGENR {
        ULPWDGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Hibernate Request Value Set"]
    #[inline]
    pub fn hib(&self) -> HIBR {
        HIBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - fRTC Clock Selection"]
    #[inline]
    pub fn rcs(&self) -> RCSR {
        RCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - fSTDBY Clock Selection"]
    #[inline]
    pub fn stdbysel(&self) -> STDBYSELR {
        STDBYSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Wake-Up from Hibernate Trigger Input Selection"]
    #[inline]
    pub fn wkupsel(&self) -> WKUPSELR {
        WKUPSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - General Purpose Input 0 Selection"]
    #[inline]
    pub fn gpi0sel(&self) -> GPI0SELR {
        GPI0SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - HIBIO0 Polarity Set"]
    #[inline]
    pub fn hibio0pol(&self) -> HIBIO0POLR {
        HIBIO0POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - HIBIO1 Polarity Set"]
    #[inline]
    pub fn hibio1pol(&self) -> HIBIO1POLR {
        HIBIO1POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
    #[inline]
    pub fn hibio0sel(&self) -> HIBIO0SELR {
        HIBIO0SELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - HIB_IO_1 Pin I/O Control (Default WKUP)"]
    #[inline]
    pub fn hibio1sel(&self) -> HIBIO1SELR {
        HIBIO1SELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 794624 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Wake-Up on Pin Event Positive Edge Enable"]
    #[inline]
    pub fn wkpep(&mut self) -> _WKPEPW {
        _WKPEPW { w: self }
    }
    #[doc = "Bit 1 - Wake-up on Pin Event Negative Edge Enable"]
    #[inline]
    pub fn wkpen(&mut self) -> _WKPENW {
        _WKPENW { w: self }
    }
    #[doc = "Bit 2 - Wake-up on RTC Event Enable"]
    #[inline]
    pub fn rtce(&mut self) -> _RTCEW {
        _RTCEW { w: self }
    }
    #[doc = "Bit 3 - ULP WDG Alarm Enable"]
    #[inline]
    pub fn ulpwdgen(&mut self) -> _ULPWDGENW {
        _ULPWDGENW { w: self }
    }
    #[doc = "Bit 4 - Hibernate Request Value Set"]
    #[inline]
    pub fn hib(&mut self) -> _HIBW {
        _HIBW { w: self }
    }
    #[doc = "Bit 6 - fRTC Clock Selection"]
    #[inline]
    pub fn rcs(&mut self) -> _RCSW {
        _RCSW { w: self }
    }
    #[doc = "Bit 7 - fSTDBY Clock Selection"]
    #[inline]
    pub fn stdbysel(&mut self) -> _STDBYSELW {
        _STDBYSELW { w: self }
    }
    #[doc = "Bit 8 - Wake-Up from Hibernate Trigger Input Selection"]
    #[inline]
    pub fn wkupsel(&mut self) -> _WKUPSELW {
        _WKUPSELW { w: self }
    }
    #[doc = "Bit 10 - General Purpose Input 0 Selection"]
    #[inline]
    pub fn gpi0sel(&mut self) -> _GPI0SELW {
        _GPI0SELW { w: self }
    }
    #[doc = "Bit 12 - HIBIO0 Polarity Set"]
    #[inline]
    pub fn hibio0pol(&mut self) -> _HIBIO0POLW {
        _HIBIO0POLW { w: self }
    }
    #[doc = "Bit 13 - HIBIO1 Polarity Set"]
    #[inline]
    pub fn hibio1pol(&mut self) -> _HIBIO1POLW {
        _HIBIO1POLW { w: self }
    }
    #[doc = "Bits 16:19 - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
    #[inline]
    pub fn hibio0sel(&mut self) -> _HIBIO0SELW {
        _HIBIO0SELW { w: self }
    }
    #[doc = "Bits 20:23 - HIB_IO_1 Pin I/O Control (Default WKUP)"]
    #[inline]
    pub fn hibio1sel(&mut self) -> _HIBIO1SELW {
        _HIBIO1SELW { w: self }
    }
}
