#[doc = "Writer for register HDSET"]
pub type W = crate::W<u32, super::HDSET>;
#[doc = "Register HDSET `reset()`'s with value 0"]
impl crate::ResetValue for super::HDSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Wake-up Pin Event Positive Edge Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPEV_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Set wake-up event"]
    VALUE2,
}
impl From<EPEV_AW> for bool {
    #[inline(always)]
    fn from(variant: EPEV_AW) -> Self {
        match variant {
            EPEV_AW::VALUE1 => false,
            EPEV_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `EPEV`"]
pub struct EPEV_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPEV_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EPEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EPEV_AW::VALUE2)
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
#[doc = "Wake-up Pin Event Negative Edge Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENEV_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Set wake-up event"]
    VALUE2,
}
impl From<ENEV_AW> for bool {
    #[inline(always)]
    fn from(variant: ENEV_AW) -> Self {
        match variant {
            ENEV_AW::VALUE1 => false,
            ENEV_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `ENEV`"]
pub struct ENEV_W<'a> {
    w: &'a mut W,
}
impl<'a> ENEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENEV_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENEV_AW::VALUE2)
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
#[doc = "RTC Event Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCEV_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Set wake-up event"]
    VALUE2,
}
impl From<RTCEV_AW> for bool {
    #[inline(always)]
    fn from(variant: RTCEV_AW) -> Self {
        match variant {
            RTCEV_AW::VALUE1 => false,
            RTCEV_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `RTCEV`"]
pub struct RTCEV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCEV_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTCEV_AW::VALUE1)
    }
    #[doc = "Set wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTCEV_AW::VALUE2)
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
#[doc = "ULP WDG Alarm Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULPWDG_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Set watchdog alarm"]
    VALUE2,
}
impl From<ULPWDG_AW> for bool {
    #[inline(always)]
    fn from(variant: ULPWDG_AW) -> Self {
        match variant {
            ULPWDG_AW::VALUE1 => false,
            ULPWDG_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `ULPWDG`"]
pub struct ULPWDG_W<'a> {
    w: &'a mut W,
}
impl<'a> ULPWDG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ULPWDG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ULPWDG_AW::VALUE1)
    }
    #[doc = "Set watchdog alarm"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ULPWDG_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up Pin Event Positive Edge Set"]
    #[inline(always)]
    pub fn epev(&mut self) -> EPEV_W {
        EPEV_W { w: self }
    }
    #[doc = "Bit 1 - Wake-up Pin Event Negative Edge Set"]
    #[inline(always)]
    pub fn enev(&mut self) -> ENEV_W {
        ENEV_W { w: self }
    }
    #[doc = "Bit 2 - RTC Event Set"]
    #[inline(always)]
    pub fn rtcev(&mut self) -> RTCEV_W {
        RTCEV_W { w: self }
    }
    #[doc = "Bit 3 - ULP WDG Alarm Set"]
    #[inline(always)]
    pub fn ulpwdg(&mut self) -> ULPWDG_W {
        ULPWDG_W { w: self }
    }
}
