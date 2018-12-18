#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PFLGE {
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
#[doc = "Possible values of the field `ECHE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECHER {
    #[doc = "Correct Hall Event interrupt disabled"]
    VALUE1,
    #[doc = "Correct Hall Event interrupt enabled"]
    VALUE2,
}
impl ECHER {
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
            ECHER::VALUE1 => false,
            ECHER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECHER {
        match value {
            false => ECHER::VALUE1,
            true => ECHER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ECHER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ECHER::VALUE2
    }
}
#[doc = "Possible values of the field `EWHE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWHER {
    #[doc = "Wrong Hall Event interrupt disabled"]
    VALUE1,
    #[doc = "Wrong Hall Event interrupt enabled"]
    VALUE2,
}
impl EWHER {
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
            EWHER::VALUE1 => false,
            EWHER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EWHER {
        match value {
            false => EWHER::VALUE1,
            true => EWHER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EWHER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EWHER::VALUE2
    }
}
#[doc = "Possible values of the field `EHIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EHIER {
    #[doc = "Update of the Hall Inputs interrupt is disabled"]
    VALUE1,
    #[doc = "Update of the Hall Inputs interrupt is enabled"]
    VALUE2,
}
impl EHIER {
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
            EHIER::VALUE1 => false,
            EHIER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EHIER {
        match value {
            false => EHIER::VALUE1,
            true => EHIER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EHIER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EHIER::VALUE2
    }
}
#[doc = "Possible values of the field `EMST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMSTR {
    #[doc = "Shadow transfer event interrupt disabled"]
    VALUE1,
    #[doc = "Shadow transfer event interrupt enabled"]
    VALUE2,
}
impl EMSTR {
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
            EMSTR::VALUE1 => false,
            EMSTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EMSTR {
        match value {
            false => EMSTR::VALUE1,
            true => EMSTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EMSTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EMSTR::VALUE2
    }
}
#[doc = "Possible values of the field `EINDX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EINDXR {
    #[doc = "Index event interrupt disabled"]
    VALUE1,
    #[doc = "Index event interrupt enabled"]
    VALUE2,
}
impl EINDXR {
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
            EINDXR::VALUE1 => false,
            EINDXR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EINDXR {
        match value {
            false => EINDXR::VALUE1,
            true => EINDXR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EINDXR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EINDXR::VALUE2
    }
}
#[doc = "Possible values of the field `EERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EERRR {
    #[doc = "Phase error event interrupt disabled"]
    VALUE1,
    #[doc = "Phase error event interrupt enabled"]
    VALUE2,
}
impl EERRR {
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
            EERRR::VALUE1 => false,
            EERRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EERRR {
        match value {
            false => EERRR::VALUE1,
            true => EERRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EERRR::VALUE2
    }
}
#[doc = "Possible values of the field `ECNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECNTR {
    #[doc = "Quadrature CLK event interrupt disabled"]
    VALUE1,
    #[doc = "Quadrature CLK event interrupt enabled"]
    VALUE2,
}
impl ECNTR {
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
            ECNTR::VALUE1 => false,
            ECNTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECNTR {
        match value {
            false => ECNTR::VALUE1,
            true => ECNTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ECNTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ECNTR::VALUE2
    }
}
#[doc = "Possible values of the field `EDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDIRR {
    #[doc = "Direction change event interrupt disabled"]
    VALUE1,
    #[doc = "Direction change event interrupt enabled"]
    VALUE2,
}
impl EDIRR {
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
            EDIRR::VALUE1 => false,
            EDIRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDIRR {
        match value {
            false => EDIRR::VALUE1,
            true => EDIRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EDIRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EDIRR::VALUE2
    }
}
#[doc = "Possible values of the field `EPCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPCLKR {
    #[doc = "Quadrature Period CLK event interrupt disabled"]
    VALUE1,
    #[doc = "Quadrature Period CLK event interrupt enabled"]
    VALUE2,
}
impl EPCLKR {
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
            EPCLKR::VALUE1 => false,
            EPCLKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPCLKR {
        match value {
            false => EPCLKR::VALUE1,
            true => EPCLKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EPCLKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EPCLKR::VALUE2
    }
}
#[doc = "Possible values of the field `CHESEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHESELR {
    #[doc = "Correct Hall Event interrupt forward to POSIFx.SR0"]
    VALUE1,
    #[doc = "Correct Hall Event interrupt forward to POSIFx.SR1"]
    VALUE2,
}
impl CHESELR {
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
            CHESELR::VALUE1 => false,
            CHESELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHESELR {
        match value {
            false => CHESELR::VALUE1,
            true => CHESELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHESELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHESELR::VALUE2
    }
}
#[doc = "Possible values of the field `WHESEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WHESELR {
    #[doc = "Wrong Hall Event interrupt forward to POSIFx.SR0"]
    VALUE1,
    #[doc = "Wrong Hall Event interrupt forward to POSIFx.SR1"]
    VALUE2,
}
impl WHESELR {
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
            WHESELR::VALUE1 => false,
            WHESELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WHESELR {
        match value {
            false => WHESELR::VALUE1,
            true => WHESELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WHESELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WHESELR::VALUE2
    }
}
#[doc = "Possible values of the field `HIESEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIESELR {
    #[doc = "Hall Inputs Update Event interrupt forward to POSIFx.SR0"]
    VALUE1,
    #[doc = "Hall Inputs Update Event interrupt forward to POSIFx.SR1"]
    VALUE2,
}
impl HIESELR {
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
            HIESELR::VALUE1 => false,
            HIESELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIESELR {
        match value {
            false => HIESELR::VALUE1,
            true => HIESELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HIESELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HIESELR::VALUE2
    }
}
#[doc = "Possible values of the field `MSTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSELR {
    #[doc = "Multi-Channel pattern Update Event interrupt forward to POSIFx.SR0"]
    VALUE1,
    #[doc = "Multi-Channel pattern Update Event interrupt forward to POSIFx.SR1"]
    VALUE2,
}
impl MSTSELR {
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
            MSTSELR::VALUE1 => false,
            MSTSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTSELR {
        match value {
            false => MSTSELR::VALUE1,
            true => MSTSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MSTSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MSTSELR::VALUE2
    }
}
#[doc = "Possible values of the field `INDSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INDSELR {
    #[doc = "Quadrature Index Event interrupt forward to POSIFx.SR0"]
    VALUE1,
    #[doc = "Quadrature Index Event interrupt forward to POSIFx.SR1"]
    VALUE2,
}
impl INDSELR {
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
            INDSELR::VALUE1 => false,
            INDSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INDSELR {
        match value {
            false => INDSELR::VALUE1,
            true => INDSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == INDSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == INDSELR::VALUE2
    }
}
#[doc = "Possible values of the field `ERRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRSELR {
    #[doc = "Quadrature Phase error Event interrupt forward to POSIFx.SR0"]
    VALUE1,
    #[doc = "Quadrature Phase error Event interrupt forward to POSIFx.SR1"]
    VALUE2,
}
impl ERRSELR {
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
            ERRSELR::VALUE1 => false,
            ERRSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRSELR {
        match value {
            false => ERRSELR::VALUE1,
            true => ERRSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ERRSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ERRSELR::VALUE2
    }
}
#[doc = "Possible values of the field `CNTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTSELR {
    #[doc = "Quadrature Clock Event interrupt forward to POSIFx.SR0"]
    VALUE1,
    #[doc = "Quadrature Clock Event interrupt forward to POSIFx.SR1"]
    VALUE2,
}
impl CNTSELR {
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
            CNTSELR::VALUE1 => false,
            CNTSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNTSELR {
        match value {
            false => CNTSELR::VALUE1,
            true => CNTSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CNTSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CNTSELR::VALUE2
    }
}
#[doc = "Possible values of the field `DIRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRSELR {
    #[doc = "Quadrature Direction Update Event interrupt forward to POSIFx.SR0"]
    VALUE1,
    #[doc = "Quadrature Direction Update Event interrupt forward to POSIFx.SR1"]
    VALUE2,
}
impl DIRSELR {
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
            DIRSELR::VALUE1 => false,
            DIRSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIRSELR {
        match value {
            false => DIRSELR::VALUE1,
            true => DIRSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIRSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIRSELR::VALUE2
    }
}
#[doc = "Possible values of the field `PCLSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLSELR {
    #[doc = "Quadrature Period clock Event interrupt forward to POSIFx.SR0"]
    VALUE1,
    #[doc = "Quadrature Period clock Event interrupt forward to POSIFx.SR1"]
    VALUE2,
}
impl PCLSELR {
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
            PCLSELR::VALUE1 => false,
            PCLSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCLSELR {
        match value {
            false => PCLSELR::VALUE1,
            true => PCLSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PCLSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PCLSELR::VALUE2
    }
}
#[doc = "Values that can be written to the field `ECHE`"]
pub enum ECHEW {
    #[doc = "Correct Hall Event interrupt disabled"]
    VALUE1,
    #[doc = "Correct Hall Event interrupt enabled"]
    VALUE2,
}
impl ECHEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECHEW::VALUE1 => false,
            ECHEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECHEW<'a> {
    w: &'a mut W,
}
impl<'a> _ECHEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECHEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Correct Hall Event interrupt disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECHEW::VALUE1)
    }
    #[doc = "Correct Hall Event interrupt enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECHEW::VALUE2)
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
#[doc = "Values that can be written to the field `EWHE`"]
pub enum EWHEW {
    #[doc = "Wrong Hall Event interrupt disabled"]
    VALUE1,
    #[doc = "Wrong Hall Event interrupt enabled"]
    VALUE2,
}
impl EWHEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EWHEW::VALUE1 => false,
            EWHEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EWHEW<'a> {
    w: &'a mut W,
}
impl<'a> _EWHEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EWHEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wrong Hall Event interrupt disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EWHEW::VALUE1)
    }
    #[doc = "Wrong Hall Event interrupt enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EWHEW::VALUE2)
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
#[doc = "Values that can be written to the field `EHIE`"]
pub enum EHIEW {
    #[doc = "Update of the Hall Inputs interrupt is disabled"]
    VALUE1,
    #[doc = "Update of the Hall Inputs interrupt is enabled"]
    VALUE2,
}
impl EHIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EHIEW::VALUE1 => false,
            EHIEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EHIEW<'a> {
    w: &'a mut W,
}
impl<'a> _EHIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EHIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Update of the Hall Inputs interrupt is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EHIEW::VALUE1)
    }
    #[doc = "Update of the Hall Inputs interrupt is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EHIEW::VALUE2)
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
#[doc = "Values that can be written to the field `EMST`"]
pub enum EMSTW {
    #[doc = "Shadow transfer event interrupt disabled"]
    VALUE1,
    #[doc = "Shadow transfer event interrupt enabled"]
    VALUE2,
}
impl EMSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EMSTW::VALUE1 => false,
            EMSTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMSTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shadow transfer event interrupt disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EMSTW::VALUE1)
    }
    #[doc = "Shadow transfer event interrupt enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EMSTW::VALUE2)
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
#[doc = "Values that can be written to the field `EINDX`"]
pub enum EINDXW {
    #[doc = "Index event interrupt disabled"]
    VALUE1,
    #[doc = "Index event interrupt enabled"]
    VALUE2,
}
impl EINDXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EINDXW::VALUE1 => false,
            EINDXW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EINDXW<'a> {
    w: &'a mut W,
}
impl<'a> _EINDXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EINDXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Index event interrupt disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EINDXW::VALUE1)
    }
    #[doc = "Index event interrupt enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EINDXW::VALUE2)
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
#[doc = "Values that can be written to the field `EERR`"]
pub enum EERRW {
    #[doc = "Phase error event interrupt disabled"]
    VALUE1,
    #[doc = "Phase error event interrupt enabled"]
    VALUE2,
}
impl EERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EERRW::VALUE1 => false,
            EERRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EERRW<'a> {
    w: &'a mut W,
}
impl<'a> _EERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Phase error event interrupt disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EERRW::VALUE1)
    }
    #[doc = "Phase error event interrupt enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EERRW::VALUE2)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ECNT`"]
pub enum ECNTW {
    #[doc = "Quadrature CLK event interrupt disabled"]
    VALUE1,
    #[doc = "Quadrature CLK event interrupt enabled"]
    VALUE2,
}
impl ECNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECNTW::VALUE1 => false,
            ECNTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECNTW<'a> {
    w: &'a mut W,
}
impl<'a> _ECNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECNTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Quadrature CLK event interrupt disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECNTW::VALUE1)
    }
    #[doc = "Quadrature CLK event interrupt enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECNTW::VALUE2)
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
#[doc = "Values that can be written to the field `EDIR`"]
pub enum EDIRW {
    #[doc = "Direction change event interrupt disabled"]
    VALUE1,
    #[doc = "Direction change event interrupt enabled"]
    VALUE2,
}
impl EDIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDIRW::VALUE1 => false,
            EDIRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _EDIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Direction change event interrupt disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EDIRW::VALUE1)
    }
    #[doc = "Direction change event interrupt enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EDIRW::VALUE2)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EPCLK`"]
pub enum EPCLKW {
    #[doc = "Quadrature Period CLK event interrupt disabled"]
    VALUE1,
    #[doc = "Quadrature Period CLK event interrupt enabled"]
    VALUE2,
}
impl EPCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPCLKW::VALUE1 => false,
            EPCLKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _EPCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPCLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Quadrature Period CLK event interrupt disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EPCLKW::VALUE1)
    }
    #[doc = "Quadrature Period CLK event interrupt enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EPCLKW::VALUE2)
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
#[doc = "Values that can be written to the field `CHESEL`"]
pub enum CHESELW {
    #[doc = "Correct Hall Event interrupt forward to POSIFx.SR0"]
    VALUE1,
    #[doc = "Correct Hall Event interrupt forward to POSIFx.SR1"]
    VALUE2,
}
impl CHESELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHESELW::VALUE1 => false,
            CHESELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHESELW<'a> {
    w: &'a mut W,
}
impl<'a> _CHESELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHESELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Correct Hall Event interrupt forward to POSIFx.SR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHESELW::VALUE1)
    }
    #[doc = "Correct Hall Event interrupt forward to POSIFx.SR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHESELW::VALUE2)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WHESEL`"]
pub enum WHESELW {
    #[doc = "Wrong Hall Event interrupt forward to POSIFx.SR0"]
    VALUE1,
    #[doc = "Wrong Hall Event interrupt forward to POSIFx.SR1"]
    VALUE2,
}
impl WHESELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WHESELW::VALUE1 => false,
            WHESELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WHESELW<'a> {
    w: &'a mut W,
}
impl<'a> _WHESELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WHESELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wrong Hall Event interrupt forward to POSIFx.SR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WHESELW::VALUE1)
    }
    #[doc = "Wrong Hall Event interrupt forward to POSIFx.SR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WHESELW::VALUE2)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HIESEL`"]
pub enum HIESELW {
    #[doc = "Hall Inputs Update Event interrupt forward to POSIFx.SR0"]
    VALUE1,
    #[doc = "Hall Inputs Update Event interrupt forward to POSIFx.SR1"]
    VALUE2,
}
impl HIESELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HIESELW::VALUE1 => false,
            HIESELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIESELW<'a> {
    w: &'a mut W,
}
impl<'a> _HIESELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIESELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hall Inputs Update Event interrupt forward to POSIFx.SR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HIESELW::VALUE1)
    }
    #[doc = "Hall Inputs Update Event interrupt forward to POSIFx.SR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HIESELW::VALUE2)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MSTSEL`"]
pub enum MSTSELW {
    #[doc = "Multi-Channel pattern Update Event interrupt forward to POSIFx.SR0"]
    VALUE1,
    #[doc = "Multi-Channel pattern Update Event interrupt forward to POSIFx.SR1"]
    VALUE2,
}
impl MSTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTSELW::VALUE1 => false,
            MSTSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Multi-Channel pattern Update Event interrupt forward to POSIFx.SR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSTSELW::VALUE1)
    }
    #[doc = "Multi-Channel pattern Update Event interrupt forward to POSIFx.SR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSTSELW::VALUE2)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INDSEL`"]
pub enum INDSELW {
    #[doc = "Quadrature Index Event interrupt forward to POSIFx.SR0"]
    VALUE1,
    #[doc = "Quadrature Index Event interrupt forward to POSIFx.SR1"]
    VALUE2,
}
impl INDSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INDSELW::VALUE1 => false,
            INDSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INDSELW<'a> {
    w: &'a mut W,
}
impl<'a> _INDSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INDSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Quadrature Index Event interrupt forward to POSIFx.SR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(INDSELW::VALUE1)
    }
    #[doc = "Quadrature Index Event interrupt forward to POSIFx.SR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(INDSELW::VALUE2)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERRSEL`"]
pub enum ERRSELW {
    #[doc = "Quadrature Phase error Event interrupt forward to POSIFx.SR0"]
    VALUE1,
    #[doc = "Quadrature Phase error Event interrupt forward to POSIFx.SR1"]
    VALUE2,
}
impl ERRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRSELW::VALUE1 => false,
            ERRSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERRSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Quadrature Phase error Event interrupt forward to POSIFx.SR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ERRSELW::VALUE1)
    }
    #[doc = "Quadrature Phase error Event interrupt forward to POSIFx.SR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ERRSELW::VALUE2)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CNTSEL`"]
pub enum CNTSELW {
    #[doc = "Quadrature Clock Event interrupt forward to POSIFx.SR0"]
    VALUE1,
    #[doc = "Quadrature Clock Event interrupt forward to POSIFx.SR1"]
    VALUE2,
}
impl CNTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNTSELW::VALUE1 => false,
            CNTSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNTSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Quadrature Clock Event interrupt forward to POSIFx.SR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CNTSELW::VALUE1)
    }
    #[doc = "Quadrature Clock Event interrupt forward to POSIFx.SR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CNTSELW::VALUE2)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIRSEL`"]
pub enum DIRSELW {
    #[doc = "Quadrature Direction Update Event interrupt forward to POSIFx.SR0"]
    VALUE1,
    #[doc = "Quadrature Direction Update Event interrupt forward to POSIFx.SR1"]
    VALUE2,
}
impl DIRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIRSELW::VALUE1 => false,
            DIRSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIRSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Quadrature Direction Update Event interrupt forward to POSIFx.SR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIRSELW::VALUE1)
    }
    #[doc = "Quadrature Direction Update Event interrupt forward to POSIFx.SR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIRSELW::VALUE2)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCLSEL`"]
pub enum PCLSELW {
    #[doc = "Quadrature Period clock Event interrupt forward to POSIFx.SR0"]
    VALUE1,
    #[doc = "Quadrature Period clock Event interrupt forward to POSIFx.SR1"]
    VALUE2,
}
impl PCLSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCLSELW::VALUE1 => false,
            PCLSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PCLSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Quadrature Period clock Event interrupt forward to POSIFx.SR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PCLSELW::VALUE1)
    }
    #[doc = "Quadrature Period clock Event interrupt forward to POSIFx.SR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PCLSELW::VALUE2)
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
        const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - Correct Hall Event Enable"]
    #[inline]
    pub fn eche(&self) -> ECHER {
        ECHER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Wrong Hall Event Enable"]
    #[inline]
    pub fn ewhe(&self) -> EWHER {
        EWHER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Hall Input Update Enable"]
    #[inline]
    pub fn ehie(&self) -> EHIER {
        EHIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Multi-Channel pattern shadow transfer enable"]
    #[inline]
    pub fn emst(&self) -> EMSTR {
        EMSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Quadrature Index Event Enable"]
    #[inline]
    pub fn eindx(&self) -> EINDXR {
        EINDXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Quadrature Phase Error Enable"]
    #[inline]
    pub fn eerr(&self) -> EERRR {
        EERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Quadrature CLK interrupt Enable"]
    #[inline]
    pub fn ecnt(&self) -> ECNTR {
        ECNTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Quadrature direction change interrupt Enable"]
    #[inline]
    pub fn edir(&self) -> EDIRR {
        EDIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Quadrature Period CLK interrupt Enable"]
    #[inline]
    pub fn epclk(&self) -> EPCLKR {
        EPCLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Correct Hall Event Service Request Selector"]
    #[inline]
    pub fn chesel(&self) -> CHESELR {
        CHESELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Wrong Hall Event Service Request Selector"]
    #[inline]
    pub fn whesel(&self) -> WHESELR {
        WHESELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Hall Inputs Update Event Service Request Selector"]
    #[inline]
    pub fn hiesel(&self) -> HIESELR {
        HIESELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Multi-Channel pattern Update Event Service Request Selector"]
    #[inline]
    pub fn mstsel(&self) -> MSTSELR {
        MSTSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Quadrature Index Event Service Request Selector"]
    #[inline]
    pub fn indsel(&self) -> INDSELR {
        INDSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Quadrature Phase Error Event Service Request Selector"]
    #[inline]
    pub fn errsel(&self) -> ERRSELR {
        ERRSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Quadrature Clock Event Service Request Selector"]
    #[inline]
    pub fn cntsel(&self) -> CNTSELR {
        CNTSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Quadrature Direction Update Event Service Request Selector"]
    #[inline]
    pub fn dirsel(&self) -> DIRSELR {
        DIRSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Quadrature Period clock Event Service Request Selector"]
    #[inline]
    pub fn pclsel(&self) -> PCLSELR {
        PCLSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Correct Hall Event Enable"]
    #[inline]
    pub fn eche(&mut self) -> _ECHEW {
        _ECHEW { w: self }
    }
    #[doc = "Bit 1 - Wrong Hall Event Enable"]
    #[inline]
    pub fn ewhe(&mut self) -> _EWHEW {
        _EWHEW { w: self }
    }
    #[doc = "Bit 2 - Hall Input Update Enable"]
    #[inline]
    pub fn ehie(&mut self) -> _EHIEW {
        _EHIEW { w: self }
    }
    #[doc = "Bit 4 - Multi-Channel pattern shadow transfer enable"]
    #[inline]
    pub fn emst(&mut self) -> _EMSTW {
        _EMSTW { w: self }
    }
    #[doc = "Bit 8 - Quadrature Index Event Enable"]
    #[inline]
    pub fn eindx(&mut self) -> _EINDXW {
        _EINDXW { w: self }
    }
    #[doc = "Bit 9 - Quadrature Phase Error Enable"]
    #[inline]
    pub fn eerr(&mut self) -> _EERRW {
        _EERRW { w: self }
    }
    #[doc = "Bit 10 - Quadrature CLK interrupt Enable"]
    #[inline]
    pub fn ecnt(&mut self) -> _ECNTW {
        _ECNTW { w: self }
    }
    #[doc = "Bit 11 - Quadrature direction change interrupt Enable"]
    #[inline]
    pub fn edir(&mut self) -> _EDIRW {
        _EDIRW { w: self }
    }
    #[doc = "Bit 12 - Quadrature Period CLK interrupt Enable"]
    #[inline]
    pub fn epclk(&mut self) -> _EPCLKW {
        _EPCLKW { w: self }
    }
    #[doc = "Bit 16 - Correct Hall Event Service Request Selector"]
    #[inline]
    pub fn chesel(&mut self) -> _CHESELW {
        _CHESELW { w: self }
    }
    #[doc = "Bit 17 - Wrong Hall Event Service Request Selector"]
    #[inline]
    pub fn whesel(&mut self) -> _WHESELW {
        _WHESELW { w: self }
    }
    #[doc = "Bit 18 - Hall Inputs Update Event Service Request Selector"]
    #[inline]
    pub fn hiesel(&mut self) -> _HIESELW {
        _HIESELW { w: self }
    }
    #[doc = "Bit 20 - Multi-Channel pattern Update Event Service Request Selector"]
    #[inline]
    pub fn mstsel(&mut self) -> _MSTSELW {
        _MSTSELW { w: self }
    }
    #[doc = "Bit 24 - Quadrature Index Event Service Request Selector"]
    #[inline]
    pub fn indsel(&mut self) -> _INDSELW {
        _INDSELW { w: self }
    }
    #[doc = "Bit 25 - Quadrature Phase Error Event Service Request Selector"]
    #[inline]
    pub fn errsel(&mut self) -> _ERRSELW {
        _ERRSELW { w: self }
    }
    #[doc = "Bit 26 - Quadrature Clock Event Service Request Selector"]
    #[inline]
    pub fn cntsel(&mut self) -> _CNTSELW {
        _CNTSELW { w: self }
    }
    #[doc = "Bit 27 - Quadrature Direction Update Event Service Request Selector"]
    #[inline]
    pub fn dirsel(&mut self) -> _DIRSELW {
        _DIRSELW { w: self }
    }
    #[doc = "Bit 28 - Quadrature Period clock Event Service Request Selector"]
    #[inline]
    pub fn pclsel(&mut self) -> _PCLSELW {
        _PCLSELW { w: self }
    }
}
