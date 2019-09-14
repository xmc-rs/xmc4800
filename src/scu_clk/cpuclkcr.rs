#[doc = "Reader of register CPUCLKCR"]
pub type R = crate::R<u32, super::CPUCLKCR>;
#[doc = "Writer for register CPUCLKCR"]
pub type W = crate::W<u32, super::CPUCLKCR>;
#[doc = "Register CPUCLKCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CPUCLKCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CPU Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPUDIV_A {
    #[doc = "0: fCPU = fSYS"]
    VALUE1,
    #[doc = "1: fCPU = fSYS / 2"]
    VALUE2,
}
impl From<CPUDIV_A> for bool {
    #[inline(always)]
    fn from(variant: CPUDIV_A) -> Self {
        match variant {
            CPUDIV_A::VALUE1 => false,
            CPUDIV_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CPUDIV`"]
pub type CPUDIV_R = crate::R<bool, CPUDIV_A>;
impl CPUDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPUDIV_A {
        match self.bits {
            false => CPUDIV_A::VALUE1,
            true => CPUDIV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CPUDIV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CPUDIV_A::VALUE2
    }
}
#[doc = "Write proxy for field `CPUDIV`"]
pub struct CPUDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPUDIV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "fCPU = fSYS"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CPUDIV_A::VALUE1)
    }
    #[doc = "fCPU = fSYS / 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CPUDIV_A::VALUE2)
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
impl R {
    #[doc = "Bit 0 - CPU Clock Divider Enable"]
    #[inline(always)]
    pub fn cpudiv(&self) -> CPUDIV_R {
        CPUDIV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU Clock Divider Enable"]
    #[inline(always)]
    pub fn cpudiv(&mut self) -> CPUDIV_W {
        CPUDIV_W { w: self }
    }
}
