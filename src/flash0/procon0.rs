#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PROCON0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `S0L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0LR {
    #[doc = "No write protection is configured for sector n."]
    VALUE1,
    #[doc = "Write protection is configured for sector n."]
    VALUE2,
}
impl S0LR {
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
            S0LR::VALUE1 => false,
            S0LR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S0LR {
        match value {
            false => S0LR::VALUE1,
            true => S0LR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S0LR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S0LR::VALUE2
    }
}
#[doc = "Possible values of the field `S1L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1LR {
    #[doc = "No write protection is configured for sector n."]
    VALUE1,
    #[doc = "Write protection is configured for sector n."]
    VALUE2,
}
impl S1LR {
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
            S1LR::VALUE1 => false,
            S1LR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S1LR {
        match value {
            false => S1LR::VALUE1,
            true => S1LR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S1LR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S1LR::VALUE2
    }
}
#[doc = "Possible values of the field `S2L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2LR {
    #[doc = "No write protection is configured for sector n."]
    VALUE1,
    #[doc = "Write protection is configured for sector n."]
    VALUE2,
}
impl S2LR {
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
            S2LR::VALUE1 => false,
            S2LR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S2LR {
        match value {
            false => S2LR::VALUE1,
            true => S2LR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S2LR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S2LR::VALUE2
    }
}
#[doc = "Possible values of the field `S3L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3LR {
    #[doc = "No write protection is configured for sector n."]
    VALUE1,
    #[doc = "Write protection is configured for sector n."]
    VALUE2,
}
impl S3LR {
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
            S3LR::VALUE1 => false,
            S3LR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S3LR {
        match value {
            false => S3LR::VALUE1,
            true => S3LR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S3LR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S3LR::VALUE2
    }
}
#[doc = "Possible values of the field `S4L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S4LR {
    #[doc = "No write protection is configured for sector n."]
    VALUE1,
    #[doc = "Write protection is configured for sector n."]
    VALUE2,
}
impl S4LR {
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
            S4LR::VALUE1 => false,
            S4LR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S4LR {
        match value {
            false => S4LR::VALUE1,
            true => S4LR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S4LR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S4LR::VALUE2
    }
}
#[doc = "Possible values of the field `S5L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S5LR {
    #[doc = "No write protection is configured for sector n."]
    VALUE1,
    #[doc = "Write protection is configured for sector n."]
    VALUE2,
}
impl S5LR {
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
            S5LR::VALUE1 => false,
            S5LR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S5LR {
        match value {
            false => S5LR::VALUE1,
            true => S5LR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S5LR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S5LR::VALUE2
    }
}
#[doc = "Possible values of the field `S6L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S6LR {
    #[doc = "No write protection is configured for sector n."]
    VALUE1,
    #[doc = "Write protection is configured for sector n."]
    VALUE2,
}
impl S6LR {
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
            S6LR::VALUE1 => false,
            S6LR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S6LR {
        match value {
            false => S6LR::VALUE1,
            true => S6LR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S6LR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S6LR::VALUE2
    }
}
#[doc = "Possible values of the field `S7L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S7LR {
    #[doc = "No write protection is configured for sector n."]
    VALUE1,
    #[doc = "Write protection is configured for sector n."]
    VALUE2,
}
impl S7LR {
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
            S7LR::VALUE1 => false,
            S7LR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S7LR {
        match value {
            false => S7LR::VALUE1,
            true => S7LR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S7LR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S7LR::VALUE2
    }
}
#[doc = "Possible values of the field `S8L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S8LR {
    #[doc = "No write protection is configured for sector n."]
    VALUE1,
    #[doc = "Write protection is configured for sector n."]
    VALUE2,
}
impl S8LR {
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
            S8LR::VALUE1 => false,
            S8LR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S8LR {
        match value {
            false => S8LR::VALUE1,
            true => S8LR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S8LR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S8LR::VALUE2
    }
}
#[doc = "Possible values of the field `S9L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S9LR {
    #[doc = "No write protection is configured for sector n."]
    VALUE1,
    #[doc = "Write protection is configured for sector n."]
    VALUE2,
}
impl S9LR {
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
            S9LR::VALUE1 => false,
            S9LR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S9LR {
        match value {
            false => S9LR::VALUE1,
            true => S9LR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S9LR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S9LR::VALUE2
    }
}
#[doc = "Possible values of the field `S10_S11L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S10_S11LR {
    #[doc = "No write protection is configured for sectors 10+11."]
    VALUE1,
    #[doc = "Write protection is configured for sectors 10+11."]
    VALUE2,
}
impl S10_S11LR {
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
            S10_S11LR::VALUE1 => false,
            S10_S11LR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S10_S11LR {
        match value {
            false => S10_S11LR::VALUE1,
            true => S10_S11LR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S10_S11LR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S10_S11LR::VALUE2
    }
}
#[doc = "Possible values of the field `S12_S13L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S12_S13LR {
    #[doc = "No write protection is configured for sectors 12+13."]
    VALUE1,
    #[doc = "Write protection is configured for sectors 12+13."]
    VALUE2,
}
impl S12_S13LR {
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
            S12_S13LR::VALUE1 => false,
            S12_S13LR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S12_S13LR {
        match value {
            false => S12_S13LR::VALUE1,
            true => S12_S13LR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S12_S13LR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S12_S13LR::VALUE2
    }
}
#[doc = "Possible values of the field `S14_S15L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S14_S15LR {
    #[doc = "No write protection is configured for sectors 14+15."]
    VALUE1,
    #[doc = "Write protection is configured for sectors 14+15."]
    VALUE2,
}
impl S14_S15LR {
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
            S14_S15LR::VALUE1 => false,
            S14_S15LR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S14_S15LR {
        match value {
            false => S14_S15LR::VALUE1,
            true => S14_S15LR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == S14_S15LR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == S14_S15LR::VALUE2
    }
}
#[doc = "Possible values of the field `RPRO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPROR {
    #[doc = "No read protection configured"]
    VALUE1,
    #[doc = "Read protection and global write protection is configured by user 0 (master user)"]
    VALUE2,
}
impl RPROR {
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
            RPROR::VALUE1 => false,
            RPROR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RPROR {
        match value {
            false => RPROR::VALUE1,
            true => RPROR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RPROR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RPROR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Sector 0 Locked for Write Protection by User 0"]
    #[inline]
    pub fn s0l(&self) -> S0LR {
        S0LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Sector 1 Locked for Write Protection by User 0"]
    #[inline]
    pub fn s1l(&self) -> S1LR {
        S1LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Sector 2 Locked for Write Protection by User 0"]
    #[inline]
    pub fn s2l(&self) -> S2LR {
        S2LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Sector 3 Locked for Write Protection by User 0"]
    #[inline]
    pub fn s3l(&self) -> S3LR {
        S3LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Sector 4 Locked for Write Protection by User 0"]
    #[inline]
    pub fn s4l(&self) -> S4LR {
        S4LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Sector 5 Locked for Write Protection by User 0"]
    #[inline]
    pub fn s5l(&self) -> S5LR {
        S5LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Sector 6 Locked for Write Protection by User 0"]
    #[inline]
    pub fn s6l(&self) -> S6LR {
        S6LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Sector 7 Locked for Write Protection by User 0"]
    #[inline]
    pub fn s7l(&self) -> S7LR {
        S7LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Sector 8 Locked for Write Protection by User 0"]
    #[inline]
    pub fn s8l(&self) -> S8LR {
        S8LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Sector 9 Locked for Write Protection by User 0"]
    #[inline]
    pub fn s9l(&self) -> S9LR {
        S9LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Sectors 10 and 11 Locked for Write Protection by User 0"]
    #[inline]
    pub fn s10_s11l(&self) -> S10_S11LR {
        S10_S11LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Sectors 12 and 13 Locked for Write Protection by User 0"]
    #[inline]
    pub fn s12_s13l(&self) -> S12_S13LR {
        S12_S13LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Sectors 14 and 15 Locked for Write Protection by User 0"]
    #[inline]
    pub fn s14_s15l(&self) -> S14_S15LR {
        S14_S15LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Read Protection Configuration"]
    #[inline]
    pub fn rpro(&self) -> RPROR {
        RPROR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
