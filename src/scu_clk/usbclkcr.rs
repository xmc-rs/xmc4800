#[doc = "Reader of register USBCLKCR"]
pub type R = crate::R<u32, super::USBCLKCR>;
#[doc = "Writer for register USBCLKCR"]
pub type W = crate::W<u32, super::USBCLKCR>;
#[doc = "Register USBCLKCR `reset()`'s with value 0"]
impl crate::ResetValue for super::USBCLKCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBDIV`"]
pub type USBDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBDIV`"]
pub struct USBDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "USB Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBSEL_A {
    #[doc = "0: USB PLL Clock"]
    VALUE1,
    #[doc = "1: PLL Clock"]
    VALUE2,
}
impl From<USBSEL_A> for bool {
    #[inline(always)]
    fn from(variant: USBSEL_A) -> Self {
        match variant {
            USBSEL_A::VALUE1 => false,
            USBSEL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `USBSEL`"]
pub type USBSEL_R = crate::R<bool, USBSEL_A>;
impl USBSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBSEL_A {
        match self.bits {
            false => USBSEL_A::VALUE1,
            true => USBSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USBSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USBSEL_A::VALUE2
    }
}
#[doc = "Write proxy for field `USBSEL`"]
pub struct USBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USB PLL Clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USBSEL_A::VALUE1)
    }
    #[doc = "PLL Clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USBSEL_A::VALUE2)
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
impl R {
    #[doc = "Bits 0:2 - USB Clock Divider Value"]
    #[inline(always)]
    pub fn usbdiv(&self) -> USBDIV_R {
        USBDIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 16 - USB Clock Selection Value"]
    #[inline(always)]
    pub fn usbsel(&self) -> USBSEL_R {
        USBSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - USB Clock Divider Value"]
    #[inline(always)]
    pub fn usbdiv(&mut self) -> USBDIV_W {
        USBDIV_W { w: self }
    }
    #[doc = "Bit 16 - USB Clock Selection Value"]
    #[inline(always)]
    pub fn usbsel(&mut self) -> USBSEL_W {
        USBSEL_W { w: self }
    }
}
