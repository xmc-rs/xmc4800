#[doc = "Reader of register MODCFG"]
pub type R = crate::R<u32, super::MODCFG>;
#[doc = "Writer for register MODCFG"]
pub type W = crate::W<u32, super::MODCFG>;
#[doc = "Register MODCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::MODCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Divider Factor for Modulator Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVM_A {
    #[doc = "0: fMOD = fCLK / 2"]
    VALUE1,
    #[doc = "1: fMOD = fCLK / 4"]
    VALUE2,
    #[doc = "2: fMOD = fCLK / 6"]
    VALUE3,
    #[doc = "15: fMOD = fCLK / 32"]
    VALUE4,
}
impl From<DIVM_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVM_A) -> Self {
        match variant {
            DIVM_A::VALUE1 => 0,
            DIVM_A::VALUE2 => 1,
            DIVM_A::VALUE3 => 2,
            DIVM_A::VALUE4 => 15,
        }
    }
}
#[doc = "Reader of field `DIVM`"]
pub type DIVM_R = crate::R<u8, DIVM_A>;
impl DIVM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIVM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIVM_A::VALUE1),
            1 => Val(DIVM_A::VALUE2),
            2 => Val(DIVM_A::VALUE3),
            15 => Val(DIVM_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIVM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIVM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DIVM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DIVM_A::VALUE4
    }
}
#[doc = "Write proxy for field `DIVM`"]
pub struct DIVM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "fMOD = fCLK / 2"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVM_A::VALUE1)
    }
    #[doc = "fMOD = fCLK / 4"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVM_A::VALUE2)
    }
    #[doc = "fMOD = fCLK / 6"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DIVM_A::VALUE3)
    }
    #[doc = "fMOD = fCLK / 32"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(DIVM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Write Control for Divider Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DWC_AW {
    #[doc = "0: No write access to divider factor"]
    VALUE1,
    #[doc = "1: Bitfield DIVM can be written"]
    VALUE2,
}
impl From<DWC_AW> for bool {
    #[inline(always)]
    fn from(variant: DWC_AW) -> Self {
        match variant {
            DWC_AW::VALUE1 => false,
            DWC_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `DWC`"]
pub struct DWC_W<'a> {
    w: &'a mut W,
}
impl<'a> DWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DWC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No write access to divider factor"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DWC_AW::VALUE1)
    }
    #[doc = "Bitfield DIVM can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DWC_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:19 - Divider Factor for Modulator Clock"]
    #[inline(always)]
    pub fn divm(&self) -> DIVM_R {
        DIVM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - Divider Factor for Modulator Clock"]
    #[inline(always)]
    pub fn divm(&mut self) -> DIVM_W {
        DIVM_W { w: self }
    }
    #[doc = "Bit 23 - Write Control for Divider Factor"]
    #[inline(always)]
    pub fn dwc(&mut self) -> DWC_W {
        DWC_W { w: self }
    }
}
