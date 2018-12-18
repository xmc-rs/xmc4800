#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCONF {
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
#[doc = "Possible values of the field `FSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSELR {
    #[doc = "Hall Sensor Mode enabled"]
    VALUE1,
    #[doc = "Quadrature Decoder Mode enabled"]
    VALUE2,
    #[doc = "stand-alone Multi-Channel Mode enabled"]
    VALUE3,
    #[doc = "Quadrature Decoder and stand-alone Multi-Channel Mode enabled"]
    VALUE4,
}
impl FSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FSELR::VALUE1 => 0,
            FSELR::VALUE2 => 1,
            FSELR::VALUE3 => 2,
            FSELR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FSELR {
        match value {
            0 => FSELR::VALUE1,
            1 => FSELR::VALUE2,
            2 => FSELR::VALUE3,
            3 => FSELR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FSELR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == FSELR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == FSELR::VALUE4
    }
}
#[doc = "Possible values of the field `QDCM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QDCMR {
    #[doc = "Position encoder is in Quadrature Mode"]
    VALUE1,
    #[doc = "Position encoder is in Direction Count Mode."]
    VALUE2,
}
impl QDCMR {
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
            QDCMR::VALUE1 => false,
            QDCMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QDCMR {
        match value {
            false => QDCMR::VALUE1,
            true => QDCMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == QDCMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == QDCMR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct HIDGR {
    bits: bool,
}
impl HIDGR {
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
#[doc = "Possible values of the field `MCUE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCUER {
    #[doc = "Multi-Channel pattern update is controlled via HW"]
    VALUE1,
    #[doc = "Multi-Channel pattern update is controlled via SW"]
    VALUE2,
}
impl MCUER {
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
            MCUER::VALUE1 => false,
            MCUER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCUER {
        match value {
            false => MCUER::VALUE1,
            true => MCUER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MCUER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MCUER::VALUE2
    }
}
#[doc = "Possible values of the field `INSEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INSEL0R {
    #[doc = "POSIFx.IN0A"]
    VALUE1,
    #[doc = "POSIFx.IN0B"]
    VALUE2,
    #[doc = "POSIFx.IN0C"]
    VALUE3,
    #[doc = "POSIFx.IN0D"]
    VALUE4,
}
impl INSEL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INSEL0R::VALUE1 => 0,
            INSEL0R::VALUE2 => 1,
            INSEL0R::VALUE3 => 2,
            INSEL0R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INSEL0R {
        match value {
            0 => INSEL0R::VALUE1,
            1 => INSEL0R::VALUE2,
            2 => INSEL0R::VALUE3,
            3 => INSEL0R::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == INSEL0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == INSEL0R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == INSEL0R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == INSEL0R::VALUE4
    }
}
#[doc = "Possible values of the field `INSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INSEL1R {
    #[doc = "POSIFx.IN1A"]
    VALUE1,
    #[doc = "POSIFx.IN1B"]
    VALUE2,
    #[doc = "POSIFx.IN1C"]
    VALUE3,
    #[doc = "POSIFx.IN1D"]
    VALUE4,
}
impl INSEL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INSEL1R::VALUE1 => 0,
            INSEL1R::VALUE2 => 1,
            INSEL1R::VALUE3 => 2,
            INSEL1R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INSEL1R {
        match value {
            0 => INSEL1R::VALUE1,
            1 => INSEL1R::VALUE2,
            2 => INSEL1R::VALUE3,
            3 => INSEL1R::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == INSEL1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == INSEL1R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == INSEL1R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == INSEL1R::VALUE4
    }
}
#[doc = "Possible values of the field `INSEL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INSEL2R {
    #[doc = "POSIFx.IN2A"]
    VALUE1,
    #[doc = "POSIFx.IN2B"]
    VALUE2,
    #[doc = "POSIFx.IN2C"]
    VALUE3,
    #[doc = "POSIFx.IN2D"]
    VALUE4,
}
impl INSEL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INSEL2R::VALUE1 => 0,
            INSEL2R::VALUE2 => 1,
            INSEL2R::VALUE3 => 2,
            INSEL2R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INSEL2R {
        match value {
            0 => INSEL2R::VALUE1,
            1 => INSEL2R::VALUE2,
            2 => INSEL2R::VALUE3,
            3 => INSEL2R::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == INSEL2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == INSEL2R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == INSEL2R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == INSEL2R::VALUE4
    }
}
#[doc = "Possible values of the field `DSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSELR {
    #[doc = "POSIFx.HSDA"]
    VALUE1,
    #[doc = "POSIFx.HSDB"]
    VALUE2,
}
impl DSELR {
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
            DSELR::VALUE1 => false,
            DSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DSELR {
        match value {
            false => DSELR::VALUE1,
            true => DSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DSELR::VALUE2
    }
}
#[doc = "Possible values of the field `SPES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPESR {
    #[doc = "Rising edge"]
    VALUE1,
    #[doc = "Falling edge"]
    VALUE2,
}
impl SPESR {
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
            SPESR::VALUE1 => false,
            SPESR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPESR {
        match value {
            false => SPESR::VALUE1,
            true => SPESR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SPESR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SPESR::VALUE2
    }
}
#[doc = "Possible values of the field `MSETS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSETSR {
    #[doc = "POSIFx.MSETA"]
    VALUE1,
    #[doc = "POSIFx.MSETB"]
    VALUE2,
    #[doc = "POSIFx.MSETC"]
    VALUE3,
    #[doc = "POSIFx.MSETD"]
    VALUE4,
    #[doc = "POSIFx.MSETE"]
    VALUE5,
    #[doc = "POSIFx.MSETF"]
    VALUE6,
    #[doc = "POSIFx.MSETG"]
    VALUE7,
    #[doc = "POSIFx.MSETH"]
    VALUE8,
}
impl MSETSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSETSR::VALUE1 => 0,
            MSETSR::VALUE2 => 1,
            MSETSR::VALUE3 => 2,
            MSETSR::VALUE4 => 3,
            MSETSR::VALUE5 => 4,
            MSETSR::VALUE6 => 5,
            MSETSR::VALUE7 => 6,
            MSETSR::VALUE8 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSETSR {
        match value {
            0 => MSETSR::VALUE1,
            1 => MSETSR::VALUE2,
            2 => MSETSR::VALUE3,
            3 => MSETSR::VALUE4,
            4 => MSETSR::VALUE5,
            5 => MSETSR::VALUE6,
            6 => MSETSR::VALUE7,
            7 => MSETSR::VALUE8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MSETSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MSETSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == MSETSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == MSETSR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == MSETSR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == MSETSR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == MSETSR::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == MSETSR::VALUE8
    }
}
#[doc = "Possible values of the field `MSES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSESR {
    #[doc = "The signal used to enable a pattern update is active on the rising edge"]
    VALUE1,
    #[doc = "The signal used to enable a pattern update is active on the falling edge"]
    VALUE2,
}
impl MSESR {
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
            MSESR::VALUE1 => false,
            MSESR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSESR {
        match value {
            false => MSESR::VALUE1,
            true => MSESR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MSESR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MSESR::VALUE2
    }
}
#[doc = "Possible values of the field `MSYNS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSYNSR {
    #[doc = "POSIFx.MSYNCA"]
    VALUE1,
    #[doc = "POSIFx.MSYNCB"]
    VALUE2,
    #[doc = "POSIFx.MSYNCC"]
    VALUE3,
    #[doc = "POSIFx.MSYNCD"]
    VALUE4,
}
impl MSYNSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSYNSR::VALUE1 => 0,
            MSYNSR::VALUE2 => 1,
            MSYNSR::VALUE3 => 2,
            MSYNSR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSYNSR {
        match value {
            0 => MSYNSR::VALUE1,
            1 => MSYNSR::VALUE2,
            2 => MSYNSR::VALUE3,
            3 => MSYNSR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MSYNSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MSYNSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == MSYNSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == MSYNSR::VALUE4
    }
}
#[doc = "Possible values of the field `EWIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWISR {
    #[doc = "POSIFx.EWHEA"]
    VALUE1,
    #[doc = "POSIFx.EWHEB"]
    VALUE2,
    #[doc = "POSIFx.EWHEC"]
    VALUE3,
    #[doc = "POSIFx.EWHED"]
    VALUE4,
}
impl EWISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EWISR::VALUE1 => 0,
            EWISR::VALUE2 => 1,
            EWISR::VALUE3 => 2,
            EWISR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EWISR {
        match value {
            0 => EWISR::VALUE1,
            1 => EWISR::VALUE2,
            2 => EWISR::VALUE3,
            3 => EWISR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EWISR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EWISR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EWISR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EWISR::VALUE4
    }
}
#[doc = "Possible values of the field `EWIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWIER {
    #[doc = "External wrong hall event emulation signal, POSIFx.EWHE\\[D...A\\], is disabled"]
    VALUE1,
    #[doc = "External wrong hall event emulation signal, POSIFx.EWHE\\[D...A\\], is enabled."]
    VALUE2,
}
impl EWIER {
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
            EWIER::VALUE1 => false,
            EWIER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EWIER {
        match value {
            false => EWIER::VALUE1,
            true => EWIER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EWIER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EWIER::VALUE2
    }
}
#[doc = "Possible values of the field `EWIL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWILR {
    #[doc = "POSIFx.EWHE\\[D...A\\] signal is active HIGH"]
    VALUE1,
    #[doc = "POSIFx.EWHE\\[D...A\\] signal is active LOW"]
    VALUE2,
}
impl EWILR {
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
            EWILR::VALUE1 => false,
            EWILR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EWILR {
        match value {
            false => EWILR::VALUE1,
            true => EWILR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EWILR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EWILR::VALUE2
    }
}
#[doc = "Possible values of the field `LPC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCR {
    #[doc = "Low pass filter disabled"]
    VALUE1,
    #[doc = "Low pass of 1 clock cycle"]
    VALUE2,
    #[doc = "Low pass of 2 clock cycles"]
    VALUE3,
    #[doc = "Low pass of 4 clock cycles"]
    VALUE4,
    #[doc = "Low pass of 8 clock cycles"]
    VALUE5,
    #[doc = "Low pass of 16 clock cycles"]
    VALUE6,
    #[doc = "Low pass of 32 clock cycles"]
    VALUE7,
    #[doc = "Low pass of 64 clock cycles"]
    VALUE8,
}
impl LPCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPCR::VALUE1 => 0,
            LPCR::VALUE2 => 1,
            LPCR::VALUE3 => 2,
            LPCR::VALUE4 => 3,
            LPCR::VALUE5 => 4,
            LPCR::VALUE6 => 5,
            LPCR::VALUE7 => 6,
            LPCR::VALUE8 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPCR {
        match value {
            0 => LPCR::VALUE1,
            1 => LPCR::VALUE2,
            2 => LPCR::VALUE3,
            3 => LPCR::VALUE4,
            4 => LPCR::VALUE5,
            5 => LPCR::VALUE6,
            6 => LPCR::VALUE7,
            7 => LPCR::VALUE8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LPCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LPCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == LPCR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == LPCR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == LPCR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == LPCR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == LPCR::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == LPCR::VALUE8
    }
}
#[doc = "Values that can be written to the field `FSEL`"]
pub enum FSELW {
    #[doc = "Hall Sensor Mode enabled"]
    VALUE1,
    #[doc = "Quadrature Decoder Mode enabled"]
    VALUE2,
    #[doc = "stand-alone Multi-Channel Mode enabled"]
    VALUE3,
    #[doc = "Quadrature Decoder and stand-alone Multi-Channel Mode enabled"]
    VALUE4,
}
impl FSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FSELW::VALUE1 => 0,
            FSELW::VALUE2 => 1,
            FSELW::VALUE3 => 2,
            FSELW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Hall Sensor Mode enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FSELW::VALUE1)
    }
    #[doc = "Quadrature Decoder Mode enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FSELW::VALUE2)
    }
    #[doc = "stand-alone Multi-Channel Mode enabled"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(FSELW::VALUE3)
    }
    #[doc = "Quadrature Decoder and stand-alone Multi-Channel Mode enabled"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(FSELW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `QDCM`"]
pub enum QDCMW {
    #[doc = "Position encoder is in Quadrature Mode"]
    VALUE1,
    #[doc = "Position encoder is in Direction Count Mode."]
    VALUE2,
}
impl QDCMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QDCMW::VALUE1 => false,
            QDCMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QDCMW<'a> {
    w: &'a mut W,
}
impl<'a> _QDCMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QDCMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Position encoder is in Quadrature Mode"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(QDCMW::VALUE1)
    }
    #[doc = "Position encoder is in Direction Count Mode."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(QDCMW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _HIDGW<'a> {
    w: &'a mut W,
}
impl<'a> _HIDGW<'a> {
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
#[doc = "Values that can be written to the field `MCUE`"]
pub enum MCUEW {
    #[doc = "Multi-Channel pattern update is controlled via HW"]
    VALUE1,
    #[doc = "Multi-Channel pattern update is controlled via SW"]
    VALUE2,
}
impl MCUEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCUEW::VALUE1 => false,
            MCUEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCUEW<'a> {
    w: &'a mut W,
}
impl<'a> _MCUEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCUEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Multi-Channel pattern update is controlled via HW"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCUEW::VALUE1)
    }
    #[doc = "Multi-Channel pattern update is controlled via SW"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCUEW::VALUE2)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INSEL0`"]
pub enum INSEL0W {
    #[doc = "POSIFx.IN0A"]
    VALUE1,
    #[doc = "POSIFx.IN0B"]
    VALUE2,
    #[doc = "POSIFx.IN0C"]
    VALUE3,
    #[doc = "POSIFx.IN0D"]
    VALUE4,
}
impl INSEL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INSEL0W::VALUE1 => 0,
            INSEL0W::VALUE2 => 1,
            INSEL0W::VALUE3 => 2,
            INSEL0W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INSEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _INSEL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INSEL0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "POSIFx.IN0A"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(INSEL0W::VALUE1)
    }
    #[doc = "POSIFx.IN0B"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(INSEL0W::VALUE2)
    }
    #[doc = "POSIFx.IN0C"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(INSEL0W::VALUE3)
    }
    #[doc = "POSIFx.IN0D"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(INSEL0W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INSEL1`"]
pub enum INSEL1W {
    #[doc = "POSIFx.IN1A"]
    VALUE1,
    #[doc = "POSIFx.IN1B"]
    VALUE2,
    #[doc = "POSIFx.IN1C"]
    VALUE3,
    #[doc = "POSIFx.IN1D"]
    VALUE4,
}
impl INSEL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INSEL1W::VALUE1 => 0,
            INSEL1W::VALUE2 => 1,
            INSEL1W::VALUE3 => 2,
            INSEL1W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _INSEL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INSEL1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "POSIFx.IN1A"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(INSEL1W::VALUE1)
    }
    #[doc = "POSIFx.IN1B"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(INSEL1W::VALUE2)
    }
    #[doc = "POSIFx.IN1C"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(INSEL1W::VALUE3)
    }
    #[doc = "POSIFx.IN1D"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(INSEL1W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INSEL2`"]
pub enum INSEL2W {
    #[doc = "POSIFx.IN2A"]
    VALUE1,
    #[doc = "POSIFx.IN2B"]
    VALUE2,
    #[doc = "POSIFx.IN2C"]
    VALUE3,
    #[doc = "POSIFx.IN2D"]
    VALUE4,
}
impl INSEL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INSEL2W::VALUE1 => 0,
            INSEL2W::VALUE2 => 1,
            INSEL2W::VALUE3 => 2,
            INSEL2W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INSEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _INSEL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INSEL2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "POSIFx.IN2A"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(INSEL2W::VALUE1)
    }
    #[doc = "POSIFx.IN2B"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(INSEL2W::VALUE2)
    }
    #[doc = "POSIFx.IN2C"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(INSEL2W::VALUE3)
    }
    #[doc = "POSIFx.IN2D"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(INSEL2W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DSEL`"]
pub enum DSELW {
    #[doc = "POSIFx.HSDA"]
    VALUE1,
    #[doc = "POSIFx.HSDB"]
    VALUE2,
}
impl DSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DSELW::VALUE1 => false,
            DSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "POSIFx.HSDA"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DSELW::VALUE1)
    }
    #[doc = "POSIFx.HSDB"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DSELW::VALUE2)
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
#[doc = "Values that can be written to the field `SPES`"]
pub enum SPESW {
    #[doc = "Rising edge"]
    VALUE1,
    #[doc = "Falling edge"]
    VALUE2,
}
impl SPESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPESW::VALUE1 => false,
            SPESW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPESW<'a> {
    w: &'a mut W,
}
impl<'a> _SPESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rising edge"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SPESW::VALUE1)
    }
    #[doc = "Falling edge"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SPESW::VALUE2)
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
#[doc = "Values that can be written to the field `MSETS`"]
pub enum MSETSW {
    #[doc = "POSIFx.MSETA"]
    VALUE1,
    #[doc = "POSIFx.MSETB"]
    VALUE2,
    #[doc = "POSIFx.MSETC"]
    VALUE3,
    #[doc = "POSIFx.MSETD"]
    VALUE4,
    #[doc = "POSIFx.MSETE"]
    VALUE5,
    #[doc = "POSIFx.MSETF"]
    VALUE6,
    #[doc = "POSIFx.MSETG"]
    VALUE7,
    #[doc = "POSIFx.MSETH"]
    VALUE8,
}
impl MSETSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSETSW::VALUE1 => 0,
            MSETSW::VALUE2 => 1,
            MSETSW::VALUE3 => 2,
            MSETSW::VALUE4 => 3,
            MSETSW::VALUE5 => 4,
            MSETSW::VALUE6 => 5,
            MSETSW::VALUE7 => 6,
            MSETSW::VALUE8 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSETSW<'a> {
    w: &'a mut W,
}
impl<'a> _MSETSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSETSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "POSIFx.MSETA"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSETSW::VALUE1)
    }
    #[doc = "POSIFx.MSETB"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSETSW::VALUE2)
    }
    #[doc = "POSIFx.MSETC"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(MSETSW::VALUE3)
    }
    #[doc = "POSIFx.MSETD"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(MSETSW::VALUE4)
    }
    #[doc = "POSIFx.MSETE"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(MSETSW::VALUE5)
    }
    #[doc = "POSIFx.MSETF"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(MSETSW::VALUE6)
    }
    #[doc = "POSIFx.MSETG"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(MSETSW::VALUE7)
    }
    #[doc = "POSIFx.MSETH"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(MSETSW::VALUE8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MSES`"]
pub enum MSESW {
    #[doc = "The signal used to enable a pattern update is active on the rising edge"]
    VALUE1,
    #[doc = "The signal used to enable a pattern update is active on the falling edge"]
    VALUE2,
}
impl MSESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSESW::VALUE1 => false,
            MSESW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSESW<'a> {
    w: &'a mut W,
}
impl<'a> _MSESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The signal used to enable a pattern update is active on the rising edge"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSESW::VALUE1)
    }
    #[doc = "The signal used to enable a pattern update is active on the falling edge"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSESW::VALUE2)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MSYNS`"]
pub enum MSYNSW {
    #[doc = "POSIFx.MSYNCA"]
    VALUE1,
    #[doc = "POSIFx.MSYNCB"]
    VALUE2,
    #[doc = "POSIFx.MSYNCC"]
    VALUE3,
    #[doc = "POSIFx.MSYNCD"]
    VALUE4,
}
impl MSYNSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSYNSW::VALUE1 => 0,
            MSYNSW::VALUE2 => 1,
            MSYNSW::VALUE3 => 2,
            MSYNSW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSYNSW<'a> {
    w: &'a mut W,
}
impl<'a> _MSYNSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSYNSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "POSIFx.MSYNCA"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSYNSW::VALUE1)
    }
    #[doc = "POSIFx.MSYNCB"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSYNSW::VALUE2)
    }
    #[doc = "POSIFx.MSYNCC"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(MSYNSW::VALUE3)
    }
    #[doc = "POSIFx.MSYNCD"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(MSYNSW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EWIS`"]
pub enum EWISW {
    #[doc = "POSIFx.EWHEA"]
    VALUE1,
    #[doc = "POSIFx.EWHEB"]
    VALUE2,
    #[doc = "POSIFx.EWHEC"]
    VALUE3,
    #[doc = "POSIFx.EWHED"]
    VALUE4,
}
impl EWISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EWISW::VALUE1 => 0,
            EWISW::VALUE2 => 1,
            EWISW::VALUE3 => 2,
            EWISW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EWISW<'a> {
    w: &'a mut W,
}
impl<'a> _EWISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EWISW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "POSIFx.EWHEA"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EWISW::VALUE1)
    }
    #[doc = "POSIFx.EWHEB"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EWISW::VALUE2)
    }
    #[doc = "POSIFx.EWHEC"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(EWISW::VALUE3)
    }
    #[doc = "POSIFx.EWHED"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(EWISW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EWIE`"]
pub enum EWIEW {
    #[doc = "External wrong hall event emulation signal, POSIFx.EWHE\\[D...A\\], is disabled"]
    VALUE1,
    #[doc = "External wrong hall event emulation signal, POSIFx.EWHE\\[D...A\\], is enabled."]
    VALUE2,
}
impl EWIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EWIEW::VALUE1 => false,
            EWIEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EWIEW<'a> {
    w: &'a mut W,
}
impl<'a> _EWIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EWIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External wrong hall event emulation signal, POSIFx.EWHE\\[D...A\\], is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EWIEW::VALUE1)
    }
    #[doc = "External wrong hall event emulation signal, POSIFx.EWHE\\[D...A\\], is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EWIEW::VALUE2)
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
#[doc = "Values that can be written to the field `EWIL`"]
pub enum EWILW {
    #[doc = "POSIFx.EWHE\\[D...A\\] signal is active HIGH"]
    VALUE1,
    #[doc = "POSIFx.EWHE\\[D...A\\] signal is active LOW"]
    VALUE2,
}
impl EWILW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EWILW::VALUE1 => false,
            EWILW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EWILW<'a> {
    w: &'a mut W,
}
impl<'a> _EWILW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EWILW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "POSIFx.EWHE\\[D...A\\] signal is active HIGH"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EWILW::VALUE1)
    }
    #[doc = "POSIFx.EWHE\\[D...A\\] signal is active LOW"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EWILW::VALUE2)
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
#[doc = "Values that can be written to the field `LPC`"]
pub enum LPCW {
    #[doc = "Low pass filter disabled"]
    VALUE1,
    #[doc = "Low pass of 1 clock cycle"]
    VALUE2,
    #[doc = "Low pass of 2 clock cycles"]
    VALUE3,
    #[doc = "Low pass of 4 clock cycles"]
    VALUE4,
    #[doc = "Low pass of 8 clock cycles"]
    VALUE5,
    #[doc = "Low pass of 16 clock cycles"]
    VALUE6,
    #[doc = "Low pass of 32 clock cycles"]
    VALUE7,
    #[doc = "Low pass of 64 clock cycles"]
    VALUE8,
}
impl LPCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPCW::VALUE1 => 0,
            LPCW::VALUE2 => 1,
            LPCW::VALUE3 => 2,
            LPCW::VALUE4 => 3,
            LPCW::VALUE5 => 4,
            LPCW::VALUE6 => 5,
            LPCW::VALUE7 => 6,
            LPCW::VALUE8 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPCW<'a> {
    w: &'a mut W,
}
impl<'a> _LPCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low pass filter disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPCW::VALUE1)
    }
    #[doc = "Low pass of 1 clock cycle"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPCW::VALUE2)
    }
    #[doc = "Low pass of 2 clock cycles"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(LPCW::VALUE3)
    }
    #[doc = "Low pass of 4 clock cycles"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(LPCW::VALUE4)
    }
    #[doc = "Low pass of 8 clock cycles"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(LPCW::VALUE5)
    }
    #[doc = "Low pass of 16 clock cycles"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(LPCW::VALUE6)
    }
    #[doc = "Low pass of 32 clock cycles"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(LPCW::VALUE7)
    }
    #[doc = "Low pass of 64 clock cycles"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(LPCW::VALUE8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 0:1 - Function Selector"]
    #[inline]
    pub fn fsel(&self) -> FSELR {
        FSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Position Decoder Mode selection"]
    #[inline]
    pub fn qdcm(&self) -> QDCMR {
        QDCMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Idle generation enable"]
    #[inline]
    pub fn hidg(&self) -> HIDGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HIDGR { bits }
    }
    #[doc = "Bit 5 - Multi-Channel Pattern SW update enable"]
    #[inline]
    pub fn mcue(&self) -> MCUER {
        MCUER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - PhaseA/Hal input 1 selector"]
    #[inline]
    pub fn insel0(&self) -> INSEL0R {
        INSEL0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - PhaseB/Hall input 2 selector"]
    #[inline]
    pub fn insel1(&self) -> INSEL1R {
        INSEL1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Index/Hall input 3 selector"]
    #[inline]
    pub fn insel2(&self) -> INSEL2R {
        INSEL2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Delay Pin selector"]
    #[inline]
    pub fn dsel(&self) -> DSELR {
        DSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Edge selector for the sampling trigger"]
    #[inline]
    pub fn spes(&self) -> SPESR {
        SPESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 18:20 - Pattern update signal select"]
    #[inline]
    pub fn msets(&self) -> MSETSR {
        MSETSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 21 - Multi-Channel pattern update trigger edge"]
    #[inline]
    pub fn mses(&self) -> MSESR {
        MSESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 22:23 - PWM synchronization signal selector"]
    #[inline]
    pub fn msyns(&self) -> MSYNSR {
        MSYNSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Wrong Hall Event selection"]
    #[inline]
    pub fn ewis(&self) -> EWISR {
        EWISR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - External Wrong Hall Event enable"]
    #[inline]
    pub fn ewie(&self) -> EWIER {
        EWIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - External Wrong Hall Event active level"]
    #[inline]
    pub fn ewil(&self) -> EWILR {
        EWILR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 28:30 - Low Pass Filters Configuration"]
    #[inline]
    pub fn lpc(&self) -> LPCR {
        LPCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:1 - Function Selector"]
    #[inline]
    pub fn fsel(&mut self) -> _FSELW {
        _FSELW { w: self }
    }
    #[doc = "Bit 2 - Position Decoder Mode selection"]
    #[inline]
    pub fn qdcm(&mut self) -> _QDCMW {
        _QDCMW { w: self }
    }
    #[doc = "Bit 4 - Idle generation enable"]
    #[inline]
    pub fn hidg(&mut self) -> _HIDGW {
        _HIDGW { w: self }
    }
    #[doc = "Bit 5 - Multi-Channel Pattern SW update enable"]
    #[inline]
    pub fn mcue(&mut self) -> _MCUEW {
        _MCUEW { w: self }
    }
    #[doc = "Bits 8:9 - PhaseA/Hal input 1 selector"]
    #[inline]
    pub fn insel0(&mut self) -> _INSEL0W {
        _INSEL0W { w: self }
    }
    #[doc = "Bits 10:11 - PhaseB/Hall input 2 selector"]
    #[inline]
    pub fn insel1(&mut self) -> _INSEL1W {
        _INSEL1W { w: self }
    }
    #[doc = "Bits 12:13 - Index/Hall input 3 selector"]
    #[inline]
    pub fn insel2(&mut self) -> _INSEL2W {
        _INSEL2W { w: self }
    }
    #[doc = "Bit 16 - Delay Pin selector"]
    #[inline]
    pub fn dsel(&mut self) -> _DSELW {
        _DSELW { w: self }
    }
    #[doc = "Bit 17 - Edge selector for the sampling trigger"]
    #[inline]
    pub fn spes(&mut self) -> _SPESW {
        _SPESW { w: self }
    }
    #[doc = "Bits 18:20 - Pattern update signal select"]
    #[inline]
    pub fn msets(&mut self) -> _MSETSW {
        _MSETSW { w: self }
    }
    #[doc = "Bit 21 - Multi-Channel pattern update trigger edge"]
    #[inline]
    pub fn mses(&mut self) -> _MSESW {
        _MSESW { w: self }
    }
    #[doc = "Bits 22:23 - PWM synchronization signal selector"]
    #[inline]
    pub fn msyns(&mut self) -> _MSYNSW {
        _MSYNSW { w: self }
    }
    #[doc = "Bits 24:25 - Wrong Hall Event selection"]
    #[inline]
    pub fn ewis(&mut self) -> _EWISW {
        _EWISW { w: self }
    }
    #[doc = "Bit 26 - External Wrong Hall Event enable"]
    #[inline]
    pub fn ewie(&mut self) -> _EWIEW {
        _EWIEW { w: self }
    }
    #[doc = "Bit 27 - External Wrong Hall Event active level"]
    #[inline]
    pub fn ewil(&mut self) -> _EWILW {
        _EWILW { w: self }
    }
    #[doc = "Bits 28:30 - Low Pass Filters Configuration"]
    #[inline]
    pub fn lpc(&mut self) -> _LPCW {
        _LPCW { w: self }
    }
}
