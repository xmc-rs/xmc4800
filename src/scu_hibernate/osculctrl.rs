#[doc = "Reader of register OSCULCTRL"]
pub type R = crate::R<u32, super::OSCULCTRL>;
#[doc = "Writer for register OSCULCTRL"]
pub type W = crate::W<u32, super::OSCULCTRL>;
#[doc = "Register OSCULCTRL `reset()`'s with value 0x20"]
impl crate::ResetValue for super::OSCULCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "XTAL1 Data General Purpose Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum X1DEN_A {
    #[doc = "0: Data input inactivated, power down"]
    VALUE1,
    #[doc = "1: Data input active"]
    VALUE2,
}
impl From<X1DEN_A> for bool {
    #[inline(always)]
    fn from(variant: X1DEN_A) -> Self {
        match variant {
            X1DEN_A::VALUE1 => false,
            X1DEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `X1DEN`"]
pub type X1DEN_R = crate::R<bool, X1DEN_A>;
impl X1DEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> X1DEN_A {
        match self.bits {
            false => X1DEN_A::VALUE1,
            true => X1DEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == X1DEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == X1DEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `X1DEN`"]
pub struct X1DEN_W<'a> {
    w: &'a mut W,
}
impl<'a> X1DEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: X1DEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data input inactivated, power down"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(X1DEN_A::VALUE1)
    }
    #[doc = "Data input active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(X1DEN_A::VALUE2)
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
#[doc = "Oscillator Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Oscillator is enabled, in operation"]
    VALUE1,
    #[doc = "1: Oscillator is enabled, in bypass mode"]
    VALUE2,
    #[doc = "2: Oscillator in power down"]
    VALUE3,
    #[doc = "3: Oscillator in power down, can be used as GPI"]
    VALUE4,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        match variant {
            MODE_A::VALUE1 => 0,
            MODE_A::VALUE2 => 1,
            MODE_A::VALUE3 => 2,
            MODE_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::VALUE1,
            1 => MODE_A::VALUE2,
            2 => MODE_A::VALUE3,
            3 => MODE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MODE_A::VALUE4
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Oscillator is enabled, in operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MODE_A::VALUE1)
    }
    #[doc = "Oscillator is enabled, in bypass mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MODE_A::VALUE2)
    }
    #[doc = "Oscillator in power down"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(MODE_A::VALUE3)
    }
    #[doc = "Oscillator in power down, can be used as GPI"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(MODE_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - XTAL1 Data General Purpose Input Enable"]
    #[inline(always)]
    pub fn x1den(&self) -> X1DEN_R {
        X1DEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - XTAL1 Data General Purpose Input Enable"]
    #[inline(always)]
    pub fn x1den(&mut self) -> X1DEN_W {
        X1DEN_W { w: self }
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
