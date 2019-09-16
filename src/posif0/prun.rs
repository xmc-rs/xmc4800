#[doc = "Reader of register PRUN"]
pub type R = crate::R<u32, super::PRUN>;
#[doc = "Run Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RB_A {
    #[doc = "0: IDLE"]
    VALUE1,
    #[doc = "1: Running"]
    VALUE2,
}
impl From<RB_A> for bool {
    #[inline(always)]
    fn from(variant: RB_A) -> Self {
        match variant {
            RB_A::VALUE1 => false,
            RB_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RB`"]
pub type RB_R = crate::R<bool, RB_A>;
impl RB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RB_A {
        match self.bits {
            false => RB_A::VALUE1,
            true => RB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RB_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Run Bit"]
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new((self.bits & 0x01) != 0)
    }
}
