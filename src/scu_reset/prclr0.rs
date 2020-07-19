#[doc = "Writer for register PRCLR0"]
pub type W = crate::W<u32, super::PRCLR0>;
#[doc = "Register PRCLR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRCLR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "VADC Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VADCRS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<VADCRS_AW> for bool {
    #[inline(always)]
    fn from(variant: VADCRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `VADCRS`"]
pub struct VADCRS_W<'a> {
    w: &'a mut W,
}
impl<'a> VADCRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VADCRS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VADCRS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VADCRS_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "DSD Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSDRS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<DSDRS_AW> for bool {
    #[inline(always)]
    fn from(variant: DSDRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DSDRS`"]
pub struct DSDRS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSDRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSDRS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DSDRS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DSDRS_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "CCU40 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU40RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<CCU40RS_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU40RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CCU40RS`"]
pub struct CCU40RS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCU40RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCU40RS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU40RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU40RS_AW::VALUE2)
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
#[doc = "CCU41 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU41RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<CCU41RS_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU41RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CCU41RS`"]
pub struct CCU41RS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCU41RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCU41RS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU41RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU41RS_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "CCU42 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU42RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<CCU42RS_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU42RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CCU42RS`"]
pub struct CCU42RS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCU42RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCU42RS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU42RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU42RS_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "CCU80 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU80RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<CCU80RS_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU80RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CCU80RS`"]
pub struct CCU80RS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCU80RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCU80RS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU80RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU80RS_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "CCU81 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU81RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<CCU81RS_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU81RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CCU81RS`"]
pub struct CCU81RS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCU81RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCU81RS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU81RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU81RS_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "POSIF0 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSIF0RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<POSIF0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: POSIF0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `POSIF0RS`"]
pub struct POSIF0RS_W<'a> {
    w: &'a mut W,
}
impl<'a> POSIF0RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POSIF0RS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(POSIF0RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(POSIF0RS_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "POSIF1 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSIF1RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<POSIF1RS_AW> for bool {
    #[inline(always)]
    fn from(variant: POSIF1RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `POSIF1RS`"]
pub struct POSIF1RS_W<'a> {
    w: &'a mut W,
}
impl<'a> POSIF1RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POSIF1RS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(POSIF1RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(POSIF1RS_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "USIC0 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC0RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<USIC0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: USIC0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `USIC0RS`"]
pub struct USIC0RS_W<'a> {
    w: &'a mut W,
}
impl<'a> USIC0RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USIC0RS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USIC0RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USIC0RS_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "ERU1 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERU1RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<ERU1RS_AW> for bool {
    #[inline(always)]
    fn from(variant: ERU1RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ERU1RS`"]
pub struct ERU1RS_W<'a> {
    w: &'a mut W,
}
impl<'a> ERU1RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERU1RS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ERU1RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ERU1RS_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - VADC Reset Clear"]
    #[inline(always)]
    pub fn vadcrs(&mut self) -> VADCRS_W {
        VADCRS_W { w: self }
    }
    #[doc = "Bit 1 - DSD Reset Clear"]
    #[inline(always)]
    pub fn dsdrs(&mut self) -> DSDRS_W {
        DSDRS_W { w: self }
    }
    #[doc = "Bit 2 - CCU40 Reset Clear"]
    #[inline(always)]
    pub fn ccu40rs(&mut self) -> CCU40RS_W {
        CCU40RS_W { w: self }
    }
    #[doc = "Bit 3 - CCU41 Reset Clear"]
    #[inline(always)]
    pub fn ccu41rs(&mut self) -> CCU41RS_W {
        CCU41RS_W { w: self }
    }
    #[doc = "Bit 4 - CCU42 Reset Clear"]
    #[inline(always)]
    pub fn ccu42rs(&mut self) -> CCU42RS_W {
        CCU42RS_W { w: self }
    }
    #[doc = "Bit 7 - CCU80 Reset Clear"]
    #[inline(always)]
    pub fn ccu80rs(&mut self) -> CCU80RS_W {
        CCU80RS_W { w: self }
    }
    #[doc = "Bit 8 - CCU81 Reset Clear"]
    #[inline(always)]
    pub fn ccu81rs(&mut self) -> CCU81RS_W {
        CCU81RS_W { w: self }
    }
    #[doc = "Bit 9 - POSIF0 Reset Clear"]
    #[inline(always)]
    pub fn posif0rs(&mut self) -> POSIF0RS_W {
        POSIF0RS_W { w: self }
    }
    #[doc = "Bit 10 - POSIF1 Reset Clear"]
    #[inline(always)]
    pub fn posif1rs(&mut self) -> POSIF1RS_W {
        POSIF1RS_W { w: self }
    }
    #[doc = "Bit 11 - USIC0 Reset Clear"]
    #[inline(always)]
    pub fn usic0rs(&mut self) -> USIC0RS_W {
        USIC0RS_W { w: self }
    }
    #[doc = "Bit 16 - ERU1 Reset Clear"]
    #[inline(always)]
    pub fn eru1rs(&mut self) -> ERU1RS_W {
        ERU1RS_W { w: self }
    }
}
