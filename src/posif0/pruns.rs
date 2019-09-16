#[doc = "Writer for register PRUNS"]
pub type W = crate::W<u32, super::PRUNS>;
#[doc = "Register PRUNS `reset()`'s with value 0"]
impl crate::ResetValue for super::PRUNS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SRB`"]
pub struct SRB_W<'a> {
    w: &'a mut W,
}
impl<'a> SRB_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Set Run bit"]
    #[inline(always)]
    pub fn srb(&mut self) -> SRB_W {
        SRB_W { w: self }
    }
}
