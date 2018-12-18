#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PFLG {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `CHES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHESR {
    #[doc = "Correct Hall Event not detected"]
    VALUE1,
    #[doc = "Correct Hall Event detected"]
    VALUE2,
}
impl CHESR {
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
            CHESR::VALUE1 => false,
            CHESR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHESR {
        match value {
            false => CHESR::VALUE1,
            true => CHESR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHESR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHESR::VALUE2
    }
}
#[doc = "Possible values of the field `WHES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WHESR {
    #[doc = "Wrong Hall Event not detected"]
    VALUE1,
    #[doc = "Wrong Hall Event detected"]
    VALUE2,
}
impl WHESR {
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
            WHESR::VALUE1 => false,
            WHESR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WHESR {
        match value {
            false => WHESR::VALUE1,
            true => WHESR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WHESR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WHESR::VALUE2
    }
}
#[doc = "Possible values of the field `HIES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIESR {
    #[doc = "Transition on the Hall Inputs not detected"]
    VALUE1,
    #[doc = "Transition on the Hall Inputs detected"]
    VALUE2,
}
impl HIESR {
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
            HIESR::VALUE1 => false,
            HIESR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIESR {
        match value {
            false => HIESR::VALUE1,
            true => HIESR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HIESR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HIESR::VALUE2
    }
}
#[doc = "Possible values of the field `MSTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSR {
    #[doc = "Shadow transfer not done"]
    VALUE1,
    #[doc = "Shadow transfer done"]
    VALUE2,
}
impl MSTSR {
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
            MSTSR::VALUE1 => false,
            MSTSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTSR {
        match value {
            false => MSTSR::VALUE1,
            true => MSTSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MSTSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MSTSR::VALUE2
    }
}
#[doc = "Possible values of the field `INDXS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INDXSR {
    #[doc = "Index event not detected"]
    VALUE1,
    #[doc = "Index event detected"]
    VALUE2,
}
impl INDXSR {
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
            INDXSR::VALUE1 => false,
            INDXSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INDXSR {
        match value {
            false => INDXSR::VALUE1,
            true => INDXSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == INDXSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == INDXSR::VALUE2
    }
}
#[doc = "Possible values of the field `ERRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRSR {
    #[doc = "Phase Error event not detected"]
    VALUE1,
    #[doc = "Phase Error event detected"]
    VALUE2,
}
impl ERRSR {
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
            ERRSR::VALUE1 => false,
            ERRSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRSR {
        match value {
            false => ERRSR::VALUE1,
            true => ERRSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ERRSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ERRSR::VALUE2
    }
}
#[doc = "Possible values of the field `CNTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTSR {
    #[doc = "Quadrature clock not generated"]
    VALUE1,
    #[doc = "Quadrature clock generated"]
    VALUE2,
}
impl CNTSR {
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
            CNTSR::VALUE1 => false,
            CNTSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNTSR {
        match value {
            false => CNTSR::VALUE1,
            true => CNTSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CNTSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CNTSR::VALUE2
    }
}
#[doc = "Possible values of the field `DIRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSR {
    #[doc = "Change on direction not detected"]
    VALUE1,
    #[doc = "Change on direction detected"]
    VALUE2,
}
impl DIRSR {
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
            DIRSR::VALUE1 => false,
            DIRSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIRSR {
        match value {
            false => DIRSR::VALUE1,
            true => DIRSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIRSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIRSR::VALUE2
    }
}
#[doc = "Possible values of the field `PCLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLKSR {
    #[doc = "Period clock not generated"]
    VALUE1,
    #[doc = "Period clock generated"]
    VALUE2,
}
impl PCLKSR {
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
            PCLKSR::VALUE1 => false,
            PCLKSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCLKSR {
        match value {
            false => PCLKSR::VALUE1,
            true => PCLKSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PCLKSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PCLKSR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Correct Hall Event Status"]
    #[inline]
    pub fn ches(&self) -> CHESR {
        CHESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Wrong Hall Event Status"]
    #[inline]
    pub fn whes(&self) -> WHESR {
        WHESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Hall Inputs Update Status"]
    #[inline]
    pub fn hies(&self) -> HIESR {
        HIESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Multi-Channel pattern shadow transfer status"]
    #[inline]
    pub fn msts(&self) -> MSTSR {
        MSTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Quadrature Index Status"]
    #[inline]
    pub fn indxs(&self) -> INDXSR {
        INDXSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Quadrature Phase Error Status"]
    #[inline]
    pub fn errs(&self) -> ERRSR {
        ERRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Quadrature CLK Status"]
    #[inline]
    pub fn cnts(&self) -> CNTSR {
        CNTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Quadrature Direction Change"]
    #[inline]
    pub fn dirs(&self) -> DIRSR {
        DIRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Quadrature Period Clk Status"]
    #[inline]
    pub fn pclks(&self) -> PCLKSR {
        PCLKSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
