#[doc = "Reader of register CGATSTAT3"]
pub type R = crate::R<u32, super::CGATSTAT3>;
#[doc = "EBU Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBU_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1,
    #[doc = "1: Gating asserted"]
    VALUE2,
}
impl From<EBU_A> for bool {
    #[inline(always)]
    fn from(variant: EBU_A) -> Self {
        match variant {
            EBU_A::VALUE1 => false,
            EBU_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `EBU`"]
pub type EBU_R = crate::R<bool, EBU_A>;
impl EBU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EBU_A {
        match self.bits {
            false => EBU_A::VALUE1,
            true => EBU_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EBU_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EBU_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 2 - EBU Gating Status"]
    #[inline(always)]
    pub fn ebu(&self) -> EBU_R {
        EBU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
