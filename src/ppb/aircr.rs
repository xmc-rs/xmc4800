#[doc = "Reader of register AIRCR"]
pub type R = crate::R<u32, super::AIRCR>;
#[doc = "Writer for register AIRCR"]
pub type W = crate::W<u32, super::AIRCR>;
#[doc = "Register AIRCR `reset()`'s with value 0xfa05_0000"]
impl crate::ResetValue for super::AIRCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfa05_0000
    }
}
#[doc = "Write proxy for field `VECTRESET`"]
pub struct VECTRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTRESET_W<'a> {
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
#[doc = "Write proxy for field `VECTCLRACTIVE`"]
pub struct VECTCLRACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTCLRACTIVE_W<'a> {
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
#[doc = "System reset request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRESETREQ_AW {
    #[doc = "0: no system reset request"]
    VALUE1,
    #[doc = "1: asserts a signal to the outer system that requests a reset."]
    VALUE2,
}
impl From<SYSRESETREQ_AW> for bool {
    #[inline(always)]
    fn from(variant: SYSRESETREQ_AW) -> Self {
        match variant {
            SYSRESETREQ_AW::VALUE1 => false,
            SYSRESETREQ_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `SYSRESETREQ`"]
pub struct SYSRESETREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRESETREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRESETREQ_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no system reset request"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SYSRESETREQ_AW::VALUE1)
    }
    #[doc = "asserts a signal to the outer system that requests a reset."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SYSRESETREQ_AW::VALUE2)
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
#[doc = "Reader of field `PRIGROUP`"]
pub type PRIGROUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRIGROUP`"]
pub struct PRIGROUP_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIGROUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Data endianness bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIANNESS_A {
    #[doc = "0: Little-endian"]
    VALUE1,
    #[doc = "1: Big-endian."]
    VALUE2,
}
impl From<ENDIANNESS_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIANNESS_A) -> Self {
        match variant {
            ENDIANNESS_A::VALUE1 => false,
            ENDIANNESS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ENDIANNESS`"]
pub type ENDIANNESS_R = crate::R<bool, ENDIANNESS_A>;
impl ENDIANNESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDIANNESS_A {
        match self.bits {
            false => ENDIANNESS_A::VALUE1,
            true => ENDIANNESS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENDIANNESS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENDIANNESS_A::VALUE2
    }
}
#[doc = "Reader of field `VECTKEY`"]
pub type VECTKEY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VECTKEY`"]
pub struct VECTKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:10 - Interrupt priority grouping field"]
    #[inline(always)]
    pub fn prigroup(&self) -> PRIGROUP_R {
        PRIGROUP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Data endianness bit"]
    #[inline(always)]
    pub fn endianness(&self) -> ENDIANNESS_R {
        ENDIANNESS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline(always)]
    pub fn vectkey(&self) -> VECTKEY_R {
        VECTKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved for Debug use."]
    #[inline(always)]
    pub fn vectreset(&mut self) -> VECTRESET_W {
        VECTRESET_W { w: self }
    }
    #[doc = "Bit 1 - Reserved for Debug use."]
    #[inline(always)]
    pub fn vectclractive(&mut self) -> VECTCLRACTIVE_W {
        VECTCLRACTIVE_W { w: self }
    }
    #[doc = "Bit 2 - System reset request"]
    #[inline(always)]
    pub fn sysresetreq(&mut self) -> SYSRESETREQ_W {
        SYSRESETREQ_W { w: self }
    }
    #[doc = "Bits 8:10 - Interrupt priority grouping field"]
    #[inline(always)]
    pub fn prigroup(&mut self) -> PRIGROUP_W {
        PRIGROUP_W { w: self }
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline(always)]
    pub fn vectkey(&mut self) -> VECTKEY_W {
        VECTKEY_W { w: self }
    }
}
