#[doc = "Register `OSCULCTRL` reader"]
pub struct R(crate::R<OSCULCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCULCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCULCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCULCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCULCTRL` writer"]
pub struct W(crate::W<OSCULCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCULCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<OSCULCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCULCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "XTAL1 Data General Purpose Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum X1DEN_A {
    #[doc = "0: Data input inactivated, power down"]
    VALUE1 = 0,
    #[doc = "1: Data input active"]
    VALUE2 = 1,
}
impl From<X1DEN_A> for bool {
    #[inline(always)]
    fn from(variant: X1DEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `X1DEN` reader - XTAL1 Data General Purpose Input Enable"]
pub struct X1DEN_R(crate::FieldReader<bool, X1DEN_A>);
impl X1DEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        X1DEN_R(crate::FieldReader::new(bits))
    }
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
        **self == X1DEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == X1DEN_A::VALUE2
    }
}
impl core::ops::Deref for X1DEN_R {
    type Target = crate::FieldReader<bool, X1DEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X1DEN` writer - XTAL1 Data General Purpose Input Enable"]
pub struct X1DEN_W<'a> {
    w: &'a mut W,
}
impl<'a> X1DEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: X1DEN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Oscillator Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Oscillator is enabled, in operation"]
    VALUE1 = 0,
    #[doc = "1: Oscillator is enabled, in bypass mode"]
    VALUE2 = 1,
    #[doc = "2: Oscillator in power down"]
    VALUE3 = 2,
    #[doc = "3: Oscillator in power down, can be used as GPI"]
    VALUE4 = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Oscillator Mode"]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
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
        **self == MODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == MODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == MODE_A::VALUE4
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Oscillator Mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSC_ULP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osculctrl](index.html) module"]
pub struct OSCULCTRL_SPEC;
impl crate::RegisterSpec for OSCULCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osculctrl::R](R) reader structure"]
impl crate::Readable for OSCULCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osculctrl::W](W) writer structure"]
impl crate::Writable for OSCULCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSCULCTRL to value 0x20"]
impl crate::Resettable for OSCULCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
