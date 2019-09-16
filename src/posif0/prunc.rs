#[doc = "Writer for register PRUNC"]
pub type W = crate::W<u32, super::PRUNC>;
#[doc = "Register PRUNC `reset()`'s with value 0"]
impl crate::ResetValue for super::PRUNC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CRB`"]
pub struct CRB_W<'a> {
    w: &'a mut W,
}
impl<'a> CRB_W<'a> {
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
#[doc = "Write proxy for field `CSM`"]
pub struct CSM_W<'a> {
    w: &'a mut W,
}
impl<'a> CSM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Clear Run bit"]
    #[inline(always)]
    pub fn crb(&mut self) -> CRB_W {
        CRB_W { w: self }
    }
    #[doc = "Bit 1 - Clear Current internal status"]
    #[inline(always)]
    pub fn csm(&mut self) -> CSM_W {
        CSM_W { w: self }
    }
}
