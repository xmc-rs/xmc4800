#[doc = "Reader of register STCON"]
pub type R = crate::R<u32, super::STCON>;
#[doc = "Writer for register STCON"]
pub type W = crate::W<u32, super::STCON>;
#[doc = "Register STCON `reset()`'s with value 0"]
impl crate::ResetValue for super::STCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "HW Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HWCON_A {
    #[doc = "0: Normal mode, JTAG"]
    VALUE1 = 0,
    #[doc = "1: ASC BSL enabled"]
    VALUE2 = 1,
    #[doc = "2: BMI customized boot enabled"]
    VALUE3 = 2,
    #[doc = "3: CAN BSL enabled"]
    VALUE4 = 3,
}
impl From<HWCON_A> for u8 {
    #[inline(always)]
    fn from(variant: HWCON_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HWCON`"]
pub type HWCON_R = crate::R<u8, HWCON_A>;
impl HWCON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWCON_A {
        match self.bits {
            0 => HWCON_A::VALUE1,
            1 => HWCON_A::VALUE2,
            2 => HWCON_A::VALUE3,
            3 => HWCON_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HWCON_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HWCON_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == HWCON_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == HWCON_A::VALUE4
    }
}
#[doc = "SW Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWCON_A {
    #[doc = "0: Normal mode, boot from Boot ROM"]
    VALUE1 = 0,
    #[doc = "1: ASC BSL enabled"]
    VALUE2 = 1,
    #[doc = "2: BMI customized boot enabled"]
    VALUE3 = 2,
    #[doc = "3: CAN BSL enabled"]
    VALUE4 = 3,
    #[doc = "4: Boot from Code SRAM"]
    VALUE5 = 4,
    #[doc = "8: Boot from alternate Flash Address 0"]
    VALUE6 = 8,
    #[doc = "12: Boot from alternate Flash Address 1"]
    VALUE7 = 12,
    #[doc = "14: Enable fallback Alternate Boot Mode (ABM)"]
    VALUE8 = 14,
}
impl From<SWCON_A> for u8 {
    #[inline(always)]
    fn from(variant: SWCON_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SWCON`"]
pub type SWCON_R = crate::R<u8, SWCON_A>;
impl SWCON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SWCON_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SWCON_A::VALUE1),
            1 => Val(SWCON_A::VALUE2),
            2 => Val(SWCON_A::VALUE3),
            3 => Val(SWCON_A::VALUE4),
            4 => Val(SWCON_A::VALUE5),
            8 => Val(SWCON_A::VALUE6),
            12 => Val(SWCON_A::VALUE7),
            14 => Val(SWCON_A::VALUE8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SWCON_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SWCON_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SWCON_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SWCON_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == SWCON_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == SWCON_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == SWCON_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == SWCON_A::VALUE8
    }
}
#[doc = "Write proxy for field `SWCON`"]
pub struct SWCON_W<'a> {
    w: &'a mut W,
}
impl<'a> SWCON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWCON_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal mode, boot from Boot ROM"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SWCON_A::VALUE1)
    }
    #[doc = "ASC BSL enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SWCON_A::VALUE2)
    }
    #[doc = "BMI customized boot enabled"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SWCON_A::VALUE3)
    }
    #[doc = "CAN BSL enabled"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SWCON_A::VALUE4)
    }
    #[doc = "Boot from Code SRAM"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(SWCON_A::VALUE5)
    }
    #[doc = "Boot from alternate Flash Address 0"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(SWCON_A::VALUE6)
    }
    #[doc = "Boot from alternate Flash Address 1"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(SWCON_A::VALUE7)
    }
    #[doc = "Enable fallback Alternate Boot Mode (ABM)"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(SWCON_A::VALUE8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - HW Configuration"]
    #[inline(always)]
    pub fn hwcon(&self) -> HWCON_R {
        HWCON_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - SW Configuration"]
    #[inline(always)]
    pub fn swcon(&self) -> SWCON_R {
        SWCON_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - SW Configuration"]
    #[inline(always)]
    pub fn swcon(&mut self) -> SWCON_W {
        SWCON_W { w: self }
    }
}
