#[doc = "Reader of register PSUS"]
pub type R = crate::R<u32, super::PSUS>;
#[doc = "Writer for register PSUS"]
pub type W = crate::W<u32, super::PSUS>;
#[doc = "Register PSUS `reset()`'s with value 0"]
impl crate::ResetValue for super::PSUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Quadrature Mode Suspend Config\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum QSUS_A {
    #[doc = "0: Suspend request ignored"]
    VALUE1 = 0,
    #[doc = "1: Stop immediately"]
    VALUE2 = 1,
    #[doc = "2: Suspend in the next index occurrence"]
    VALUE3 = 2,
    #[doc = "3: Suspend in the next phase (PhaseA or PhaseB) occurrence"]
    VALUE4 = 3,
}
impl From<QSUS_A> for u8 {
    #[inline(always)]
    fn from(variant: QSUS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `QSUS`"]
pub type QSUS_R = crate::R<u8, QSUS_A>;
impl QSUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QSUS_A {
        match self.bits {
            0 => QSUS_A::VALUE1,
            1 => QSUS_A::VALUE2,
            2 => QSUS_A::VALUE3,
            3 => QSUS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == QSUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == QSUS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == QSUS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == QSUS_A::VALUE4
    }
}
#[doc = "Write proxy for field `QSUS`"]
pub struct QSUS_W<'a> {
    w: &'a mut W,
}
impl<'a> QSUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QSUS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Suspend request ignored"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(QSUS_A::VALUE1)
    }
    #[doc = "Stop immediately"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(QSUS_A::VALUE2)
    }
    #[doc = "Suspend in the next index occurrence"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(QSUS_A::VALUE3)
    }
    #[doc = "Suspend in the next phase (PhaseA or PhaseB) occurrence"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(QSUS_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Multi-Channel Mode Suspend Config\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSUS_A {
    #[doc = "0: Suspend request ignored"]
    VALUE1 = 0,
    #[doc = "1: Stop immediately. Multi-Channel pattern is not set to the reset value."]
    VALUE2 = 1,
    #[doc = "2: Stop immediately. Multi-Channel pattern is set to the reset value."]
    VALUE3 = 2,
    #[doc = "3: Suspend with the synchronization of the PWM signal. Multi-Channel pattern is set to the reset value at the same time of the synchronization."]
    VALUE4 = 3,
}
impl From<MSUS_A> for u8 {
    #[inline(always)]
    fn from(variant: MSUS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MSUS`"]
pub type MSUS_R = crate::R<u8, MSUS_A>;
impl MSUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSUS_A {
        match self.bits {
            0 => MSUS_A::VALUE1,
            1 => MSUS_A::VALUE2,
            2 => MSUS_A::VALUE3,
            3 => MSUS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MSUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSUS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MSUS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MSUS_A::VALUE4
    }
}
#[doc = "Write proxy for field `MSUS`"]
pub struct MSUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MSUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSUS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Suspend request ignored"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSUS_A::VALUE1)
    }
    #[doc = "Stop immediately. Multi-Channel pattern is not set to the reset value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSUS_A::VALUE2)
    }
    #[doc = "Stop immediately. Multi-Channel pattern is set to the reset value."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(MSUS_A::VALUE3)
    }
    #[doc = "Suspend with the synchronization of the PWM signal. Multi-Channel pattern is set to the reset value at the same time of the synchronization."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(MSUS_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Quadrature Mode Suspend Config"]
    #[inline(always)]
    pub fn qsus(&self) -> QSUS_R {
        QSUS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Multi-Channel Mode Suspend Config"]
    #[inline(always)]
    pub fn msus(&self) -> MSUS_R {
        MSUS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Quadrature Mode Suspend Config"]
    #[inline(always)]
    pub fn qsus(&mut self) -> QSUS_W {
        QSUS_W { w: self }
    }
    #[doc = "Bits 2:3 - Multi-Channel Mode Suspend Config"]
    #[inline(always)]
    pub fn msus(&mut self) -> MSUS_W {
        MSUS_W { w: self }
    }
}
