#[doc = "Reader of register CLKMXSTAT"]
pub type R = crate::R<u32, super::CLKMXSTAT>;
#[doc = "Status of System Clock Multiplexing Upon Source Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCLKMUX_A {
    #[doc = "1: fOFI clock active"]
    VALUE1 = 1,
    #[doc = "2: fPLL clock active"]
    VALUE2 = 2,
}
impl From<SYSCLKMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCLKMUX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCLKMUX`"]
pub type SYSCLKMUX_R = crate::R<u8, SYSCLKMUX_A>;
impl SYSCLKMUX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCLKMUX_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SYSCLKMUX_A::VALUE1),
            2 => Val(SYSCLKMUX_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYSCLKMUX_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYSCLKMUX_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:1 - Status of System Clock Multiplexing Upon Source Switching"]
    #[inline(always)]
    pub fn sysclkmux(&self) -> SYSCLKMUX_R {
        SYSCLKMUX_R::new((self.bits & 0x03) as u8)
    }
}
