#[doc = "Writer for register PRCLR3"]
pub type W = crate::W<u32, super::PRCLR3>;
#[doc = "Register PRCLR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRCLR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "EBU Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBURS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<EBURS_AW> for bool {
    #[inline(always)]
    fn from(variant: EBURS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `EBURS`"]
pub struct EBURS_W<'a> {
    w: &'a mut W,
}
impl<'a> EBURS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EBURS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EBURS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EBURS_AW::VALUE2)
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
    #[doc = "Bit 2 - EBU Reset Assert"]
    #[inline(always)]
    pub fn eburs(&mut self) -> EBURS_W {
        EBURS_W { w: self }
    }
}
