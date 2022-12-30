#[doc = "Register `OSCHPCTRL` reader"]
pub struct R(crate::R<OSCHPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCHPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCHPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCHPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCHPCTRL` writer"]
pub struct W(crate::W<OSCHPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCHPCTRL_SPEC>;
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
impl From<crate::W<OSCHPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCHPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `X1DEN` reader - XTAL1 Data Enable"]
pub type X1DEN_R = crate::BitReader<X1DEN_A>;
#[doc = "XTAL1 Data Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum X1DEN_A {
    #[doc = "0: Bit X1D is not updated"]
    VALUE1 = 0,
    #[doc = "1: Bit X1D can be updated"]
    VALUE2 = 1,
}
impl From<X1DEN_A> for bool {
    #[inline(always)]
    fn from(variant: X1DEN_A) -> Self {
        variant as u8 != 0
    }
}
impl X1DEN_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `X1DEN` writer - XTAL1 Data Enable"]
pub type X1DEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCHPCTRL_SPEC, X1DEN_A, O>;
impl<'a, const O: u8> X1DEN_W<'a, O> {
    #[doc = "Bit X1D is not updated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(X1DEN_A::VALUE1)
    }
    #[doc = "Bit X1D can be updated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(X1DEN_A::VALUE2)
    }
}
#[doc = "Field `SHBY` reader - Shaper Bypass"]
pub type SHBY_R = crate::BitReader<SHBY_A>;
#[doc = "Shaper Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHBY_A {
    #[doc = "0: The shaper is not bypassed"]
    VALUE1 = 0,
    #[doc = "1: The shaper is bypassed"]
    VALUE2 = 1,
}
impl From<SHBY_A> for bool {
    #[inline(always)]
    fn from(variant: SHBY_A) -> Self {
        variant as u8 != 0
    }
}
impl SHBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHBY_A {
        match self.bits {
            false => SHBY_A::VALUE1,
            true => SHBY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SHBY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SHBY_A::VALUE2
    }
}
#[doc = "Field `SHBY` writer - Shaper Bypass"]
pub type SHBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCHPCTRL_SPEC, SHBY_A, O>;
impl<'a, const O: u8> SHBY_W<'a, O> {
    #[doc = "The shaper is not bypassed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SHBY_A::VALUE1)
    }
    #[doc = "The shaper is bypassed"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SHBY_A::VALUE2)
    }
}
#[doc = "Field `GAINSEL` reader - Oscillator Gain Selection"]
pub type GAINSEL_R = crate::FieldReader<u8, GAINSEL_A>;
#[doc = "Oscillator Gain Selection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GAINSEL_A {
    #[doc = "0: The gain control is configured for frequencies from 4 MHz to 8 MHz"]
    VALUE1 = 0,
    #[doc = "1: The gain control is configured for frequencies from 4 MHz to 16 MHz"]
    VALUE2 = 1,
    #[doc = "2: The gain control is configured for frequencies from 4 MHz to 20 MHz"]
    VALUE3 = 2,
    #[doc = "3: The gain control is configured for frequencies from 4 MHz to 25 MHz"]
    VALUE4 = 3,
}
impl From<GAINSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: GAINSEL_A) -> Self {
        variant as _
    }
}
impl GAINSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAINSEL_A {
        match self.bits {
            0 => GAINSEL_A::VALUE1,
            1 => GAINSEL_A::VALUE2,
            2 => GAINSEL_A::VALUE3,
            3 => GAINSEL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GAINSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GAINSEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == GAINSEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == GAINSEL_A::VALUE4
    }
}
#[doc = "Field `GAINSEL` writer - Oscillator Gain Selection"]
pub type GAINSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OSCHPCTRL_SPEC, u8, GAINSEL_A, 2, O>;
impl<'a, const O: u8> GAINSEL_W<'a, O> {
    #[doc = "The gain control is configured for frequencies from 4 MHz to 8 MHz"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GAINSEL_A::VALUE1)
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 16 MHz"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GAINSEL_A::VALUE2)
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 20 MHz"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(GAINSEL_A::VALUE3)
    }
    #[doc = "The gain control is configured for frequencies from 4 MHz to 25 MHz"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(GAINSEL_A::VALUE4)
    }
}
#[doc = "Field `MODE` reader - Oscillator Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Oscillator Mode\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: External Crystal Mode and External Input Clock Mode. The oscillator Power-Saving Mode is not entered."]
    VALUE1 = 0,
    #[doc = "1: OSC is disabled. The oscillator Power-Saving Mode is not entered."]
    VALUE2 = 1,
    #[doc = "2: External Input Clock Mode and the oscillator Power-Saving Mode is entered"]
    VALUE3 = 2,
    #[doc = "3: OSC is disabled. The oscillator Power-Saving Mode is entered."]
    VALUE4 = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `MODE` writer - Oscillator Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OSCHPCTRL_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "External Crystal Mode and External Input Clock Mode. The oscillator Power-Saving Mode is not entered."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MODE_A::VALUE1)
    }
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is not entered."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MODE_A::VALUE2)
    }
    #[doc = "External Input Clock Mode and the oscillator Power-Saving Mode is entered"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(MODE_A::VALUE3)
    }
    #[doc = "OSC is disabled. The oscillator Power-Saving Mode is entered."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(MODE_A::VALUE4)
    }
}
#[doc = "Field `OSCVAL` reader - OSC Frequency Value"]
pub type OSCVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSCVAL` writer - OSC Frequency Value"]
pub type OSCVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSCHPCTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - XTAL1 Data Enable"]
    #[inline(always)]
    pub fn x1den(&self) -> X1DEN_R {
        X1DEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shaper Bypass"]
    #[inline(always)]
    pub fn shby(&self) -> SHBY_R {
        SHBY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Oscillator Gain Selection"]
    #[inline(always)]
    pub fn gainsel(&self) -> GAINSEL_R {
        GAINSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:19 - OSC Frequency Value"]
    #[inline(always)]
    pub fn oscval(&self) -> OSCVAL_R {
        OSCVAL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - XTAL1 Data Enable"]
    #[inline(always)]
    #[must_use]
    pub fn x1den(&mut self) -> X1DEN_W<0> {
        X1DEN_W::new(self)
    }
    #[doc = "Bit 1 - Shaper Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn shby(&mut self) -> SHBY_W<1> {
        SHBY_W::new(self)
    }
    #[doc = "Bits 2:3 - Oscillator Gain Selection"]
    #[inline(always)]
    #[must_use]
    pub fn gainsel(&mut self) -> GAINSEL_W<2> {
        GAINSEL_W::new(self)
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<4> {
        MODE_W::new(self)
    }
    #[doc = "Bits 16:19 - OSC Frequency Value"]
    #[inline(always)]
    #[must_use]
    pub fn oscval(&mut self) -> OSCVAL_W<16> {
        OSCVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSC_HP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oschpctrl](index.html) module"]
pub struct OSCHPCTRL_SPEC;
impl crate::RegisterSpec for OSCHPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oschpctrl::R](R) reader structure"]
impl crate::Readable for OSCHPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oschpctrl::W](W) writer structure"]
impl crate::Writable for OSCHPCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCHPCTRL to value 0x3c"]
impl crate::Resettable for OSCHPCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x3c;
}
