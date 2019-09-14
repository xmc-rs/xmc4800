#[doc = "Writer for register CGATSET2"]
pub type W = crate::W<u32, super::CGATSET2>;
#[doc = "Register CGATSET2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CGATSET2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "WDT Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Enable gating"]
    VALUE2,
}
impl From<WDT_AW> for bool {
    #[inline(always)]
    fn from(variant: WDT_AW) -> Self {
        match variant {
            WDT_AW::VALUE1 => false,
            WDT_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `WDT`"]
pub struct WDT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WDT_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WDT_AW::VALUE2)
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
#[doc = "ETH0 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETH0_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Enable gating"]
    VALUE2,
}
impl From<ETH0_AW> for bool {
    #[inline(always)]
    fn from(variant: ETH0_AW) -> Self {
        match variant {
            ETH0_AW::VALUE1 => false,
            ETH0_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `ETH0`"]
pub struct ETH0_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETH0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ETH0_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ETH0_AW::VALUE2)
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
#[doc = "DMA0 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA0_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Enable gating"]
    VALUE2,
}
impl From<DMA0_AW> for bool {
    #[inline(always)]
    fn from(variant: DMA0_AW) -> Self {
        match variant {
            DMA0_AW::VALUE1 => false,
            DMA0_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `DMA0`"]
pub struct DMA0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DMA0_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DMA0_AW::VALUE2)
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
#[doc = "DMA1 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Enable gating"]
    VALUE2,
}
impl From<DMA1_AW> for bool {
    #[inline(always)]
    fn from(variant: DMA1_AW) -> Self {
        match variant {
            DMA1_AW::VALUE1 => false,
            DMA1_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `DMA1`"]
pub struct DMA1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DMA1_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DMA1_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "FCE Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCE_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Enable gating"]
    VALUE2,
}
impl From<FCE_AW> for bool {
    #[inline(always)]
    fn from(variant: FCE_AW) -> Self {
        match variant {
            FCE_AW::VALUE1 => false,
            FCE_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `FCE`"]
pub struct FCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FCE_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FCE_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "USB Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Enable gating"]
    VALUE2,
}
impl From<USB_AW> for bool {
    #[inline(always)]
    fn from(variant: USB_AW) -> Self {
        match variant {
            USB_AW::VALUE1 => false,
            USB_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `USB`"]
pub struct USB_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USB_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USB_AW::VALUE2)
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
#[doc = "ECAT0 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECAT0_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Enable gating"]
    VALUE2,
}
impl From<ECAT0_AW> for bool {
    #[inline(always)]
    fn from(variant: ECAT0_AW) -> Self {
        match variant {
            ECAT0_AW::VALUE1 => false,
            ECAT0_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `ECAT0`"]
pub struct ECAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> ECAT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECAT0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECAT0_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECAT0_AW::VALUE2)
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
impl W {
    #[doc = "Bit 1 - WDT Gating Set"]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W {
        WDT_W { w: self }
    }
    #[doc = "Bit 2 - ETH0 Gating Set"]
    #[inline(always)]
    pub fn eth0(&mut self) -> ETH0_W {
        ETH0_W { w: self }
    }
    #[doc = "Bit 4 - DMA0 Gating Set"]
    #[inline(always)]
    pub fn dma0(&mut self) -> DMA0_W {
        DMA0_W { w: self }
    }
    #[doc = "Bit 5 - DMA1 Gating Set"]
    #[inline(always)]
    pub fn dma1(&mut self) -> DMA1_W {
        DMA1_W { w: self }
    }
    #[doc = "Bit 6 - FCE Gating Set"]
    #[inline(always)]
    pub fn fce(&mut self) -> FCE_W {
        FCE_W { w: self }
    }
    #[doc = "Bit 7 - USB Gating Set"]
    #[inline(always)]
    pub fn usb(&mut self) -> USB_W {
        USB_W { w: self }
    }
    #[doc = "Bit 10 - ECAT0 Gating Set"]
    #[inline(always)]
    pub fn ecat0(&mut self) -> ECAT0_W {
        ECAT0_W { w: self }
    }
}
