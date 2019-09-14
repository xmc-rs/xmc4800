#[doc = "Writer for register PRCLR2"]
pub type W = crate::W<u32, super::PRCLR2>;
#[doc = "Register PRCLR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRCLR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "WDT Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTRS_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: De-assert reset"]
    VALUE2,
}
impl From<WDTRS_AW> for bool {
    #[inline(always)]
    fn from(variant: WDTRS_AW) -> Self {
        match variant {
            WDTRS_AW::VALUE1 => false,
            WDTRS_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `WDTRS`"]
pub struct WDTRS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTRS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WDTRS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WDTRS_AW::VALUE2)
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
#[doc = "ETH0 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETH0RS_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: De-assert reset"]
    VALUE2,
}
impl From<ETH0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: ETH0RS_AW) -> Self {
        match variant {
            ETH0RS_AW::VALUE1 => false,
            ETH0RS_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `ETH0RS`"]
pub struct ETH0RS_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH0RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETH0RS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ETH0RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ETH0RS_AW::VALUE2)
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
#[doc = "DMA0 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA0RS_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: De-assert reset"]
    VALUE2,
}
impl From<DMA0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: DMA0RS_AW) -> Self {
        match variant {
            DMA0RS_AW::VALUE1 => false,
            DMA0RS_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `DMA0RS`"]
pub struct DMA0RS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA0RS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DMA0RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DMA0RS_AW::VALUE2)
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
#[doc = "DMA1 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1RS_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: De-assert reset"]
    VALUE2,
}
impl From<DMA1RS_AW> for bool {
    #[inline(always)]
    fn from(variant: DMA1RS_AW) -> Self {
        match variant {
            DMA1RS_AW::VALUE1 => false,
            DMA1RS_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `DMA1RS`"]
pub struct DMA1RS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA1RS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DMA1RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DMA1RS_AW::VALUE2)
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
#[doc = "FCE Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCERS_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: De-assert reset"]
    VALUE2,
}
impl From<FCERS_AW> for bool {
    #[inline(always)]
    fn from(variant: FCERS_AW) -> Self {
        match variant {
            FCERS_AW::VALUE1 => false,
            FCERS_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `FCERS`"]
pub struct FCERS_W<'a> {
    w: &'a mut W,
}
impl<'a> FCERS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCERS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FCERS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FCERS_AW::VALUE2)
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
#[doc = "USB Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRS_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: De-assert reset"]
    VALUE2,
}
impl From<USBRS_AW> for bool {
    #[inline(always)]
    fn from(variant: USBRS_AW) -> Self {
        match variant {
            USBRS_AW::VALUE1 => false,
            USBRS_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `USBRS`"]
pub struct USBRS_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBRS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USBRS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USBRS_AW::VALUE2)
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
#[doc = "ECAT0 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECAT0RS_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: De-assert reset"]
    VALUE2,
}
impl From<ECAT0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: ECAT0RS_AW) -> Self {
        match variant {
            ECAT0RS_AW::VALUE1 => false,
            ECAT0RS_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `ECAT0RS`"]
pub struct ECAT0RS_W<'a> {
    w: &'a mut W,
}
impl<'a> ECAT0RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECAT0RS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECAT0RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECAT0RS_AW::VALUE2)
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
    #[doc = "Bit 1 - WDT Reset Clear"]
    #[inline(always)]
    pub fn wdtrs(&mut self) -> WDTRS_W {
        WDTRS_W { w: self }
    }
    #[doc = "Bit 2 - ETH0 Reset Clear"]
    #[inline(always)]
    pub fn eth0rs(&mut self) -> ETH0RS_W {
        ETH0RS_W { w: self }
    }
    #[doc = "Bit 4 - DMA0 Reset Clear"]
    #[inline(always)]
    pub fn dma0rs(&mut self) -> DMA0RS_W {
        DMA0RS_W { w: self }
    }
    #[doc = "Bit 5 - DMA1 Reset Clear"]
    #[inline(always)]
    pub fn dma1rs(&mut self) -> DMA1RS_W {
        DMA1RS_W { w: self }
    }
    #[doc = "Bit 6 - FCE Reset Clear"]
    #[inline(always)]
    pub fn fcers(&mut self) -> FCERS_W {
        FCERS_W { w: self }
    }
    #[doc = "Bit 7 - USB Reset Clear"]
    #[inline(always)]
    pub fn usbrs(&mut self) -> USBRS_W {
        USBRS_W { w: self }
    }
    #[doc = "Bit 10 - ECAT0 Reset Clear"]
    #[inline(always)]
    pub fn ecat0rs(&mut self) -> ECAT0RS_W {
        ECAT0RS_W { w: self }
    }
}
