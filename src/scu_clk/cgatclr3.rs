#[doc = "Writer for register CGATCLR3"]
pub type W = crate::W<u32, super::CGATCLR3>;
#[doc = "Register CGATCLR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CGATCLR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "EBU Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBU_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<EBU_AW> for bool {
    #[inline(always)]
    fn from(variant: EBU_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `EBU`"]
pub struct EBU_W<'a> {
    w: &'a mut W,
}
impl<'a> EBU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EBU_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EBU_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EBU_AW::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl W {
    #[doc = "Bit 2 - EBU Gating Clear"]
    #[inline(always)]
    pub fn ebu(&mut self) -> EBU_W {
        EBU_W { w: self }
    }
}
