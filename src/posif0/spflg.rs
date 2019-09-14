#[doc = "Writer for register SPFLG"]
pub type W = crate::W<u32, super::SPFLG>;
#[doc = "Register SPFLG `reset()`'s with value 0"]
impl crate::ResetValue for super::SPFLG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SCHE`"]
pub struct SCHE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHE_W<'a> {
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
#[doc = "Write proxy for field `SWHE`"]
pub struct SWHE_W<'a> {
    w: &'a mut W,
}
impl<'a> SWHE_W<'a> {
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
#[doc = "Write proxy for field `SHIE`"]
pub struct SHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIE_W<'a> {
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
#[doc = "Write proxy for field `SMST`"]
pub struct SMST_W<'a> {
    w: &'a mut W,
}
impl<'a> SMST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `SINDX`"]
pub struct SINDX_W<'a> {
    w: &'a mut W,
}
impl<'a> SINDX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `SERR`"]
pub struct SERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `SCNT`"]
pub struct SCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCNT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `SDIR`"]
pub struct SDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Write proxy for field `SPCLK`"]
pub struct SPCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SPCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Correct Hall Event flag set"]
    #[inline(always)]
    pub fn sche(&mut self) -> SCHE_W {
        SCHE_W { w: self }
    }
    #[doc = "Bit 1 - Wrong Hall Event flag set"]
    #[inline(always)]
    pub fn swhe(&mut self) -> SWHE_W {
        SWHE_W { w: self }
    }
    #[doc = "Bit 2 - Hall Inputs Update Event flag set"]
    #[inline(always)]
    pub fn shie(&mut self) -> SHIE_W {
        SHIE_W { w: self }
    }
    #[doc = "Bit 4 - Multi-Channel Pattern shadow transfer flag set"]
    #[inline(always)]
    pub fn smst(&mut self) -> SMST_W {
        SMST_W { w: self }
    }
    #[doc = "Bit 8 - Quadrature Index flag set"]
    #[inline(always)]
    pub fn sindx(&mut self) -> SINDX_W {
        SINDX_W { w: self }
    }
    #[doc = "Bit 9 - Quadrature Phase Error flag set"]
    #[inline(always)]
    pub fn serr(&mut self) -> SERR_W {
        SERR_W { w: self }
    }
    #[doc = "Bit 10 - Quadrature CLK flag set"]
    #[inline(always)]
    pub fn scnt(&mut self) -> SCNT_W {
        SCNT_W { w: self }
    }
    #[doc = "Bit 11 - Quadrature Direction flag set"]
    #[inline(always)]
    pub fn sdir(&mut self) -> SDIR_W {
        SDIR_W { w: self }
    }
    #[doc = "Bit 12 - Quadrature period clock flag set"]
    #[inline(always)]
    pub fn spclk(&mut self) -> SPCLK_W {
        SPCLK_W { w: self }
    }
}
