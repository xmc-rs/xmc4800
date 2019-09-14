#[doc = "Writer for register OVRCLR"]
pub type W = crate::W<u32, super::OVRCLR>;
#[doc = "Register OVRCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::OVRCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `LN0`"]
pub struct LN0_W<'a> {
    w: &'a mut W,
}
impl<'a> LN0_W<'a> {
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
#[doc = "Write proxy for field `LN1`"]
pub struct LN1_W<'a> {
    w: &'a mut W,
}
impl<'a> LN1_W<'a> {
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
#[doc = "Write proxy for field `LN2`"]
pub struct LN2_W<'a> {
    w: &'a mut W,
}
impl<'a> LN2_W<'a> {
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
#[doc = "Write proxy for field `LN3`"]
pub struct LN3_W<'a> {
    w: &'a mut W,
}
impl<'a> LN3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `LN4`"]
pub struct LN4_W<'a> {
    w: &'a mut W,
}
impl<'a> LN4_W<'a> {
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
#[doc = "Write proxy for field `LN5`"]
pub struct LN5_W<'a> {
    w: &'a mut W,
}
impl<'a> LN5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `LN6`"]
pub struct LN6_W<'a> {
    w: &'a mut W,
}
impl<'a> LN6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `LN7`"]
pub struct LN7_W<'a> {
    w: &'a mut W,
}
impl<'a> LN7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Write proxy for field `LN8`"]
pub struct LN8_W<'a> {
    w: &'a mut W,
}
impl<'a> LN8_W<'a> {
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
#[doc = "Write proxy for field `LN9`"]
pub struct LN9_W<'a> {
    w: &'a mut W,
}
impl<'a> LN9_W<'a> {
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
#[doc = "Write proxy for field `LN10`"]
pub struct LN10_W<'a> {
    w: &'a mut W,
}
impl<'a> LN10_W<'a> {
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
#[doc = "Write proxy for field `LN11`"]
pub struct LN11_W<'a> {
    w: &'a mut W,
}
impl<'a> LN11_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Line 0 Overrun Status Clear"]
    #[inline(always)]
    pub fn ln0(&mut self) -> LN0_W {
        LN0_W { w: self }
    }
    #[doc = "Bit 1 - Line 1 Overrun Status Clear"]
    #[inline(always)]
    pub fn ln1(&mut self) -> LN1_W {
        LN1_W { w: self }
    }
    #[doc = "Bit 2 - Line 2 Overrun Status Clear"]
    #[inline(always)]
    pub fn ln2(&mut self) -> LN2_W {
        LN2_W { w: self }
    }
    #[doc = "Bit 3 - Line 3 Overrun Status Clear"]
    #[inline(always)]
    pub fn ln3(&mut self) -> LN3_W {
        LN3_W { w: self }
    }
    #[doc = "Bit 4 - Line 4 Overrun Status Clear"]
    #[inline(always)]
    pub fn ln4(&mut self) -> LN4_W {
        LN4_W { w: self }
    }
    #[doc = "Bit 5 - Line 5 Overrun Status Clear"]
    #[inline(always)]
    pub fn ln5(&mut self) -> LN5_W {
        LN5_W { w: self }
    }
    #[doc = "Bit 6 - Line 6 Overrun Status Clear"]
    #[inline(always)]
    pub fn ln6(&mut self) -> LN6_W {
        LN6_W { w: self }
    }
    #[doc = "Bit 7 - Line 7 Overrun Status Clear"]
    #[inline(always)]
    pub fn ln7(&mut self) -> LN7_W {
        LN7_W { w: self }
    }
    #[doc = "Bit 8 - Line 8 Overrun Status Clear"]
    #[inline(always)]
    pub fn ln8(&mut self) -> LN8_W {
        LN8_W { w: self }
    }
    #[doc = "Bit 9 - Line 9 Overrun Status Clear"]
    #[inline(always)]
    pub fn ln9(&mut self) -> LN9_W {
        LN9_W { w: self }
    }
    #[doc = "Bit 10 - Line 10 Overrun Status Clear"]
    #[inline(always)]
    pub fn ln10(&mut self) -> LN10_W {
        LN10_W { w: self }
    }
    #[doc = "Bit 11 - Line 11 Overrun Status Clear"]
    #[inline(always)]
    pub fn ln11(&mut self) -> LN11_W {
        LN11_W { w: self }
    }
}
