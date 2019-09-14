#[doc = "Writer for register MCMS"]
pub type W = crate::W<u32, super::MCMS>;
#[doc = "Register MCMS `reset()`'s with value 0"]
impl crate::ResetValue for super::MCMS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `MNPS`"]
pub struct MNPS_W<'a> {
    w: &'a mut W,
}
impl<'a> MNPS_W<'a> {
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
#[doc = "Write proxy for field `STHR`"]
pub struct STHR_W<'a> {
    w: &'a mut W,
}
impl<'a> STHR_W<'a> {
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
#[doc = "Write proxy for field `STMR`"]
pub struct STMR_W<'a> {
    w: &'a mut W,
}
impl<'a> STMR_W<'a> {
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
    #[doc = "Bit 0 - Multi-Channel Pattern Update Enable Set"]
    #[inline(always)]
    pub fn mnps(&mut self) -> MNPS_W {
        MNPS_W { w: self }
    }
    #[doc = "Bit 1 - Hall Pattern Shadow Transfer Request"]
    #[inline(always)]
    pub fn sthr(&mut self) -> STHR_W {
        STHR_W { w: self }
    }
    #[doc = "Bit 2 - Multi-Channel Shadow Transfer Request"]
    #[inline(always)]
    pub fn stmr(&mut self) -> STMR_W {
        STMR_W { w: self }
    }
}
