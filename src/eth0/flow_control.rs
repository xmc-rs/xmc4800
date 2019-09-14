#[doc = "Reader of register FLOW_CONTROL"]
pub type R = crate::R<u32, super::FLOW_CONTROL>;
#[doc = "Writer for register FLOW_CONTROL"]
pub type W = crate::W<u32, super::FLOW_CONTROL>;
#[doc = "Register FLOW_CONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::FLOW_CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FCA_BPA`"]
pub type FCA_BPA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FCA_BPA`"]
pub struct FCA_BPA_W<'a> {
    w: &'a mut W,
}
impl<'a> FCA_BPA_W<'a> {
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
#[doc = "Reader of field `TFE`"]
pub type TFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFE`"]
pub struct TFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFE_W<'a> {
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
#[doc = "Reader of field `RFE`"]
pub type RFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFE`"]
pub struct RFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFE_W<'a> {
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
#[doc = "Reader of field `UP`"]
pub type UP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UP`"]
pub struct UP_W<'a> {
    w: &'a mut W,
}
impl<'a> UP_W<'a> {
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
#[doc = "Reader of field `PLT`"]
pub type PLT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLT`"]
pub struct PLT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `DZPQ`"]
pub type DZPQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DZPQ`"]
pub struct DZPQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DZPQ_W<'a> {
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
#[doc = "Reader of field `PT`"]
pub type PT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PT`"]
pub struct PT_W<'a> {
    w: &'a mut W,
}
impl<'a> PT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flow Control Busy or Backpressure Activate"]
    #[inline(always)]
    pub fn fca_bpa(&self) -> FCA_BPA_R {
        FCA_BPA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable"]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause"]
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Pause Time"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow Control Busy or Backpressure Activate"]
    #[inline(always)]
    pub fn fca_bpa(&mut self) -> FCA_BPA_W {
        FCA_BPA_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W {
        TFE_W { w: self }
    }
    #[doc = "Bit 2 - Receive Flow Control Enable"]
    #[inline(always)]
    pub fn rfe(&mut self) -> RFE_W {
        RFE_W { w: self }
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect"]
    #[inline(always)]
    pub fn up(&mut self) -> UP_W {
        UP_W { w: self }
    }
    #[doc = "Bits 4:5 - Pause Low Threshold"]
    #[inline(always)]
    pub fn plt(&mut self) -> PLT_W {
        PLT_W { w: self }
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause"]
    #[inline(always)]
    pub fn dzpq(&mut self) -> DZPQ_W {
        DZPQ_W { w: self }
    }
    #[doc = "Bits 16:31 - Pause Time"]
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W {
        PT_W { w: self }
    }
}
