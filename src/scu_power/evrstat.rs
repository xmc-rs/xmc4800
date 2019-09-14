#[doc = "Reader of register EVRSTAT"]
pub type R = crate::R<u32, super::EVRSTAT>;
#[doc = "Regulator Overvoltage for 1.3 V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OV13_A {
    #[doc = "0: No overvoltage condition"]
    VALUE1,
    #[doc = "1: Regulator is in overvoltage"]
    VALUE2,
}
impl From<OV13_A> for bool {
    #[inline(always)]
    fn from(variant: OV13_A) -> Self {
        match variant {
            OV13_A::VALUE1 => false,
            OV13_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `OV13`"]
pub type OV13_R = crate::R<bool, OV13_A>;
impl OV13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OV13_A {
        match self.bits {
            false => OV13_A::VALUE1,
            true => OV13_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OV13_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OV13_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 1 - Regulator Overvoltage for 1.3 V"]
    #[inline(always)]
    pub fn ov13(&self) -> OV13_R {
        OV13_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
