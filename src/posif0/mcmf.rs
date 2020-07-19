#[doc = "Reader of register MCMF"]
pub type R = crate::R<u32, super::MCMF>;
#[doc = "Multi-Channel Pattern update status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSS_A {
    #[doc = "0: Update of the Multi-Channel pattern is set"]
    VALUE1 = 0,
    #[doc = "1: Update of the Multi-Channel pattern is not set"]
    VALUE2 = 1,
}
impl From<MSS_A> for bool {
    #[inline(always)]
    fn from(variant: MSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSS`"]
pub type MSS_R = crate::R<bool, MSS_A>;
impl MSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSS_A {
        match self.bits {
            false => MSS_A::VALUE1,
            true => MSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSS_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Multi-Channel Pattern update status"]
    #[inline(always)]
    pub fn mss(&self) -> MSS_R {
        MSS_R::new((self.bits & 0x01) != 0)
    }
}
