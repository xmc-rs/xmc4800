#[doc = "Writer for register PWRCLR"]
pub type W = crate::W<u32, super::PWRCLR>;
#[doc = "Register PWRCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clear Disable Hibernate Domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIB_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable Hibernate domain"]
    VALUE2 = 1,
}
impl From<HIB_AW> for bool {
    #[inline(always)]
    fn from(variant: HIB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `HIB`"]
pub struct HIB_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIB_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HIB_AW::VALUE1)
    }
    #[doc = "Disable Hibernate domain"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HIB_AW::VALUE2)
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
#[doc = "Clear USB PHY Transceiver Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPHYPDQ_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Power-down"]
    VALUE2 = 1,
}
impl From<USBPHYPDQ_AW> for bool {
    #[inline(always)]
    fn from(variant: USBPHYPDQ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `USBPHYPDQ`"]
pub struct USBPHYPDQ_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPHYPDQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBPHYPDQ_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USBPHYPDQ_AW::VALUE1)
    }
    #[doc = "Power-down"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USBPHYPDQ_AW::VALUE2)
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
#[doc = "Clear USB On-The-Go Comparators Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBOTGEN_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Power-down"]
    VALUE2 = 1,
}
impl From<USBOTGEN_AW> for bool {
    #[inline(always)]
    fn from(variant: USBOTGEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `USBOTGEN`"]
pub struct USBOTGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBOTGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBOTGEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USBOTGEN_AW::VALUE1)
    }
    #[doc = "Power-down"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USBOTGEN_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Clear USB Weak Pull-Up at PADN Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPUWQ_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Pull-up active"]
    VALUE2 = 1,
}
impl From<USBPUWQ_AW> for bool {
    #[inline(always)]
    fn from(variant: USBPUWQ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `USBPUWQ`"]
pub struct USBPUWQ_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPUWQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBPUWQ_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USBPUWQ_AW::VALUE1)
    }
    #[doc = "Pull-up active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USBPUWQ_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Clear Disable Hibernate Domain"]
    #[inline(always)]
    pub fn hib(&mut self) -> HIB_W {
        HIB_W { w: self }
    }
    #[doc = "Bit 16 - Clear USB PHY Transceiver Disable"]
    #[inline(always)]
    pub fn usbphypdq(&mut self) -> USBPHYPDQ_W {
        USBPHYPDQ_W { w: self }
    }
    #[doc = "Bit 17 - Clear USB On-The-Go Comparators Enable"]
    #[inline(always)]
    pub fn usbotgen(&mut self) -> USBOTGEN_W {
        USBOTGEN_W { w: self }
    }
    #[doc = "Bit 18 - Clear USB Weak Pull-Up at PADN Enable"]
    #[inline(always)]
    pub fn usbpuwq(&mut self) -> USBPUWQ_W {
        USBPUWQ_W { w: self }
    }
}
