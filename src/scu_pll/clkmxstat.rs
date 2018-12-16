#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CLKMXSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `SYSCLKMUX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCLKMUXR {
    #[doc = "fOFI clock active"]
    VALUE1,
    #[doc = "fPLL clock active"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SYSCLKMUXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCLKMUXR::VALUE1 => 1,
            SYSCLKMUXR::VALUE2 => 2,
            SYSCLKMUXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYSCLKMUXR {
        match value {
            1 => SYSCLKMUXR::VALUE1,
            2 => SYSCLKMUXR::VALUE2,
            i => SYSCLKMUXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SYSCLKMUXR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SYSCLKMUXR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Status of System Clock Multiplexing Upon Source Switching"]
    #[inline]
    pub fn sysclkmux(&self) -> SYSCLKMUXR {
        SYSCLKMUXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
