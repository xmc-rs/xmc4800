#[doc = "Reader of register PRSTAT3"]
pub type R = crate::R<u32, super::PRSTAT3>;
#[doc = "EBU Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBURS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<EBURS_A> for bool {
    #[inline(always)]
    fn from(variant: EBURS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EBURS`"]
pub type EBURS_R = crate::R<bool, EBURS_A>;
impl EBURS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EBURS_A {
        match self.bits {
            false => EBURS_A::VALUE1,
            true => EBURS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EBURS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EBURS_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 2 - EBU Reset Status"]
    #[inline(always)]
    pub fn eburs(&self) -> EBURS_R {
        EBURS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
