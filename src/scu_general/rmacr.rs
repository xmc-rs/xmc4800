#[doc = "Reader of register RMACR"]
pub type R = crate::R<u32, super::RMACR>;
#[doc = "Writer for register RMACR"]
pub type W = crate::W<u32, super::RMACR>;
#[doc = "Register RMACR `reset()`'s with value 0"]
impl crate::ResetValue for super::RMACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Hibernate Retention Memory Register Update Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDWR_A {
    #[doc = "0: transfer data from Retention Memory in Hibernate domain to RMDATA register"]
    VALUE1 = 0,
    #[doc = "1: transfer data from RMDATA into Retention Memory in Hibernate domain"]
    VALUE2 = 1,
}
impl From<RDWR_A> for bool {
    #[inline(always)]
    fn from(variant: RDWR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDWR`"]
pub type RDWR_R = crate::R<bool, RDWR_A>;
impl RDWR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDWR_A {
        match self.bits {
            false => RDWR_A::VALUE1,
            true => RDWR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RDWR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RDWR_A::VALUE2
    }
}
#[doc = "Write proxy for field `RDWR`"]
pub struct RDWR_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDWR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "transfer data from Retention Memory in Hibernate domain to RMDATA register"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RDWR_A::VALUE1)
    }
    #[doc = "transfer data from RMDATA into Retention Memory in Hibernate domain"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RDWR_A::VALUE2)
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
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Hibernate Retention Memory Register Update Control"]
    #[inline(always)]
    pub fn rdwr(&self) -> RDWR_R {
        RDWR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Hibernate Retention Memory Register Address Select"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Hibernate Retention Memory Register Update Control"]
    #[inline(always)]
    pub fn rdwr(&mut self) -> RDWR_W {
        RDWR_W { w: self }
    }
    #[doc = "Bits 16:19 - Hibernate Retention Memory Register Address Select"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
}
