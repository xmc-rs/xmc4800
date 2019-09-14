#[doc = "Writer for register CGATSET3"]
pub type W = crate::W<u32, super::CGATSET3>;
#[doc = "Register CGATSET3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CGATSET3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "EBU Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBU_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Enable gating"]
    VALUE2,
}
impl From<EBU_AW> for bool {
    #[inline(always)]
    fn from(variant: EBU_AW) -> Self {
        match variant {
            EBU_AW::VALUE1 => false,
            EBU_AW::VALUE2 => true,
        }
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
    #[doc = "Enable gating"]
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
    #[doc = "Bit 2 - EBU Gating Set"]
    #[inline(always)]
    pub fn ebu(&mut self) -> EBU_W {
        EBU_W { w: self }
    }
}
