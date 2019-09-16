#[doc = "Reader of register RECTCFG"]
pub type R = crate::R<u32, super::RECTCFG>;
#[doc = "Writer for register RECTCFG"]
pub type W = crate::W<u32, super::RECTCFG>;
#[doc = "Register RECTCFG `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RECTCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Rectification Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFEN_A {
    #[doc = "0: No rectification, data not altered"]
    VALUE1,
    #[doc = "1: Data are rectified according to SGND"]
    VALUE2,
}
impl From<RFEN_A> for bool {
    #[inline(always)]
    fn from(variant: RFEN_A) -> Self {
        match variant {
            RFEN_A::VALUE1 => false,
            RFEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RFEN`"]
pub type RFEN_R = crate::R<bool, RFEN_A>;
impl RFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFEN_A {
        match self.bits {
            false => RFEN_A::VALUE1,
            true => RFEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RFEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RFEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `RFEN`"]
pub struct RFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No rectification, data not altered"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RFEN_A::VALUE1)
    }
    #[doc = "Data are rectified according to SGND"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RFEN_A::VALUE2)
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
#[doc = "Sign Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSRC_A {
    #[doc = "0: On-chip carrier generator"]
    VALUE1,
    #[doc = "1: Sign of result of next channel"]
    VALUE2,
    #[doc = "2: External sign signal A"]
    VALUE3,
    #[doc = "3: External sign signal B"]
    VALUE4,
}
impl From<SSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SSRC_A) -> Self {
        match variant {
            SSRC_A::VALUE1 => 0,
            SSRC_A::VALUE2 => 1,
            SSRC_A::VALUE3 => 2,
            SSRC_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `SSRC`"]
pub type SSRC_R = crate::R<u8, SSRC_A>;
impl SSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSRC_A {
        match self.bits {
            0 => SSRC_A::VALUE1,
            1 => SSRC_A::VALUE2,
            2 => SSRC_A::VALUE3,
            3 => SSRC_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SSRC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SSRC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SSRC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SSRC_A::VALUE4
    }
}
#[doc = "Write proxy for field `SSRC`"]
pub struct SSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "On-chip carrier generator"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SSRC_A::VALUE1)
    }
    #[doc = "Sign of result of next channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SSRC_A::VALUE2)
    }
    #[doc = "External sign signal A"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SSRC_A::VALUE3)
    }
    #[doc = "External sign signal B"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SSRC_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Valid Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDVAL_A {
    #[doc = "0: No new result available"]
    VALUE1,
    #[doc = "1: Bitfield SDCAP has been updated with a new captured value and has not yet been read"]
    VALUE2,
}
impl From<SDVAL_A> for bool {
    #[inline(always)]
    fn from(variant: SDVAL_A) -> Self {
        match variant {
            SDVAL_A::VALUE1 => false,
            SDVAL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SDVAL`"]
pub type SDVAL_R = crate::R<bool, SDVAL_A>;
impl SDVAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDVAL_A {
        match self.bits {
            false => SDVAL_A::VALUE1,
            true => SDVAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SDVAL_A::VALUE2
    }
}
#[doc = "Selected Carrier Sign Signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SGNCS_A {
    #[doc = "0: Positive values"]
    VALUE1,
    #[doc = "1: Negative values"]
    VALUE2,
}
impl From<SGNCS_A> for bool {
    #[inline(always)]
    fn from(variant: SGNCS_A) -> Self {
        match variant {
            SGNCS_A::VALUE1 => false,
            SGNCS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SGNCS`"]
pub type SGNCS_R = crate::R<bool, SGNCS_A>;
impl SGNCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGNCS_A {
        match self.bits {
            false => SGNCS_A::VALUE1,
            true => SGNCS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SGNCS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SGNCS_A::VALUE2
    }
}
#[doc = "Sign Signal Delayed\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SGND_A {
    #[doc = "0: Positive values"]
    VALUE1,
    #[doc = "1: Negative values"]
    VALUE2,
}
impl From<SGND_A> for bool {
    #[inline(always)]
    fn from(variant: SGND_A) -> Self {
        match variant {
            SGND_A::VALUE1 => false,
            SGND_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SGND`"]
pub type SGND_R = crate::R<bool, SGND_A>;
impl SGND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGND_A {
        match self.bits {
            false => SGND_A::VALUE1,
            true => SGND_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SGND_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SGND_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Rectification Enable"]
    #[inline(always)]
    pub fn rfen(&self) -> RFEN_R {
        RFEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Sign Source"]
    #[inline(always)]
    pub fn ssrc(&self) -> SSRC_R {
        SSRC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Valid Flag"]
    #[inline(always)]
    pub fn sdval(&self) -> SDVAL_R {
        SDVAL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Selected Carrier Sign Signal"]
    #[inline(always)]
    pub fn sgncs(&self) -> SGNCS_R {
        SGNCS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Sign Signal Delayed"]
    #[inline(always)]
    pub fn sgnd(&self) -> SGND_R {
        SGND_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rectification Enable"]
    #[inline(always)]
    pub fn rfen(&mut self) -> RFEN_W {
        RFEN_W { w: self }
    }
    #[doc = "Bits 4:5 - Sign Source"]
    #[inline(always)]
    pub fn ssrc(&mut self) -> SSRC_W {
        SSRC_W { w: self }
    }
}
