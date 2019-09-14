#[doc = "Writer for register CGATCLR0"]
pub type W = crate::W<u32, super::CGATCLR0>;
#[doc = "Register CGATCLR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CGATCLR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "VADC Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VADC_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Disable gating"]
    VALUE2,
}
impl From<VADC_AW> for bool {
    #[inline(always)]
    fn from(variant: VADC_AW) -> Self {
        match variant {
            VADC_AW::VALUE1 => false,
            VADC_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `VADC`"]
pub struct VADC_W<'a> {
    w: &'a mut W,
}
impl<'a> VADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VADC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VADC_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VADC_AW::VALUE2)
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
#[doc = "DSD Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSD_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Disable gating"]
    VALUE2,
}
impl From<DSD_AW> for bool {
    #[inline(always)]
    fn from(variant: DSD_AW) -> Self {
        match variant {
            DSD_AW::VALUE1 => false,
            DSD_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `DSD`"]
pub struct DSD_W<'a> {
    w: &'a mut W,
}
impl<'a> DSD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSD_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DSD_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DSD_AW::VALUE2)
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
#[doc = "CCU40 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU40_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Disable gating"]
    VALUE2,
}
impl From<CCU40_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU40_AW) -> Self {
        match variant {
            CCU40_AW::VALUE1 => false,
            CCU40_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CCU40`"]
pub struct CCU40_W<'a> {
    w: &'a mut W,
}
impl<'a> CCU40_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCU40_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU40_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU40_AW::VALUE2)
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
#[doc = "CCU41 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU41_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Disable gating"]
    VALUE2,
}
impl From<CCU41_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU41_AW) -> Self {
        match variant {
            CCU41_AW::VALUE1 => false,
            CCU41_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CCU41`"]
pub struct CCU41_W<'a> {
    w: &'a mut W,
}
impl<'a> CCU41_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCU41_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU41_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU41_AW::VALUE2)
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
#[doc = "CCU42 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU42_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Disable gating"]
    VALUE2,
}
impl From<CCU42_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU42_AW) -> Self {
        match variant {
            CCU42_AW::VALUE1 => false,
            CCU42_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CCU42`"]
pub struct CCU42_W<'a> {
    w: &'a mut W,
}
impl<'a> CCU42_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCU42_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU42_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU42_AW::VALUE2)
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
#[doc = "CCU80 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU80_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Disable gating"]
    VALUE2,
}
impl From<CCU80_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU80_AW) -> Self {
        match variant {
            CCU80_AW::VALUE1 => false,
            CCU80_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CCU80`"]
pub struct CCU80_W<'a> {
    w: &'a mut W,
}
impl<'a> CCU80_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCU80_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU80_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU80_AW::VALUE2)
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
#[doc = "CCU81 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU81_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Disable gating"]
    VALUE2,
}
impl From<CCU81_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU81_AW) -> Self {
        match variant {
            CCU81_AW::VALUE1 => false,
            CCU81_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CCU81`"]
pub struct CCU81_W<'a> {
    w: &'a mut W,
}
impl<'a> CCU81_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCU81_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU81_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU81_AW::VALUE2)
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
#[doc = "POSIF0 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSIF0_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Disable gating"]
    VALUE2,
}
impl From<POSIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: POSIF0_AW) -> Self {
        match variant {
            POSIF0_AW::VALUE1 => false,
            POSIF0_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `POSIF0`"]
pub struct POSIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> POSIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POSIF0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(POSIF0_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(POSIF0_AW::VALUE2)
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
#[doc = "POSIF1 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSIF1_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Disable gating"]
    VALUE2,
}
impl From<POSIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: POSIF1_AW) -> Self {
        match variant {
            POSIF1_AW::VALUE1 => false,
            POSIF1_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `POSIF1`"]
pub struct POSIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> POSIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POSIF1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(POSIF1_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(POSIF1_AW::VALUE2)
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
#[doc = "USIC0 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC0_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Disable gating"]
    VALUE2,
}
impl From<USIC0_AW> for bool {
    #[inline(always)]
    fn from(variant: USIC0_AW) -> Self {
        match variant {
            USIC0_AW::VALUE1 => false,
            USIC0_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `USIC0`"]
pub struct USIC0_W<'a> {
    w: &'a mut W,
}
impl<'a> USIC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USIC0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USIC0_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USIC0_AW::VALUE2)
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
#[doc = "ERU1 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERU1_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Disable gating"]
    VALUE2,
}
impl From<ERU1_AW> for bool {
    #[inline(always)]
    fn from(variant: ERU1_AW) -> Self {
        match variant {
            ERU1_AW::VALUE1 => false,
            ERU1_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `ERU1`"]
pub struct ERU1_W<'a> {
    w: &'a mut W,
}
impl<'a> ERU1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERU1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ERU1_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ERU1_AW::VALUE2)
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
    #[doc = "Bit 0 - VADC Gating Clear"]
    #[inline(always)]
    pub fn vadc(&mut self) -> VADC_W {
        VADC_W { w: self }
    }
    #[doc = "Bit 1 - DSD Gating Clear"]
    #[inline(always)]
    pub fn dsd(&mut self) -> DSD_W {
        DSD_W { w: self }
    }
    #[doc = "Bit 2 - CCU40 Gating Clear"]
    #[inline(always)]
    pub fn ccu40(&mut self) -> CCU40_W {
        CCU40_W { w: self }
    }
    #[doc = "Bit 3 - CCU41 Gating Clear"]
    #[inline(always)]
    pub fn ccu41(&mut self) -> CCU41_W {
        CCU41_W { w: self }
    }
    #[doc = "Bit 4 - CCU42 Gating Clear"]
    #[inline(always)]
    pub fn ccu42(&mut self) -> CCU42_W {
        CCU42_W { w: self }
    }
    #[doc = "Bit 7 - CCU80 Gating Clear"]
    #[inline(always)]
    pub fn ccu80(&mut self) -> CCU80_W {
        CCU80_W { w: self }
    }
    #[doc = "Bit 8 - CCU81 Gating Clear"]
    #[inline(always)]
    pub fn ccu81(&mut self) -> CCU81_W {
        CCU81_W { w: self }
    }
    #[doc = "Bit 9 - POSIF0 Gating Clear"]
    #[inline(always)]
    pub fn posif0(&mut self) -> POSIF0_W {
        POSIF0_W { w: self }
    }
    #[doc = "Bit 10 - POSIF1 Gating Clear"]
    #[inline(always)]
    pub fn posif1(&mut self) -> POSIF1_W {
        POSIF1_W { w: self }
    }
    #[doc = "Bit 11 - USIC0 Gating Clear"]
    #[inline(always)]
    pub fn usic0(&mut self) -> USIC0_W {
        USIC0_W { w: self }
    }
    #[doc = "Bit 16 - ERU1 Gating Clear"]
    #[inline(always)]
    pub fn eru1(&mut self) -> ERU1_W {
        ERU1_W { w: self }
    }
}
