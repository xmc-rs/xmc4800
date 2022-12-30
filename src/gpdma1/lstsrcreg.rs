#[doc = "Register `LSTSRCREG` reader"]
pub struct R(crate::R<LSTSRCREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSTSRCREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSTSRCREG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSTSRCREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSTSRCREG` writer"]
pub struct W(crate::W<LSTSRCREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSTSRCREG_SPEC>;
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
impl From<crate::W<LSTSRCREG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSTSRCREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` reader - Source last request for channel 0"]
pub type CH0_R = crate::BitReader<CH0_A>;
#[doc = "Source last request for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1 = 0,
    #[doc = "1: Last transaction in current block"]
    VALUE2 = 1,
}
impl From<CH0_A> for bool {
    #[inline(always)]
    fn from(variant: CH0_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0_A {
        match self.bits {
            false => CH0_A::VALUE1,
            true => CH0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH0_A::VALUE2
    }
}
#[doc = "Field `CH0` writer - Source last request for channel 0"]
pub type CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTSRCREG_SPEC, CH0_A, O>;
impl<'a, const O: u8> CH0_W<'a, O> {
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH0_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH0_A::VALUE2)
    }
}
#[doc = "Field `CH1` reader - Source last request for channel 1"]
pub type CH1_R = crate::BitReader<CH1_A>;
#[doc = "Source last request for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1 = 0,
    #[doc = "1: Last transaction in current block"]
    VALUE2 = 1,
}
impl From<CH1_A> for bool {
    #[inline(always)]
    fn from(variant: CH1_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1_A {
        match self.bits {
            false => CH1_A::VALUE1,
            true => CH1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH1_A::VALUE2
    }
}
#[doc = "Field `CH1` writer - Source last request for channel 1"]
pub type CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTSRCREG_SPEC, CH1_A, O>;
impl<'a, const O: u8> CH1_W<'a, O> {
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH1_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH1_A::VALUE2)
    }
}
#[doc = "Field `CH2` reader - Source last request for channel 2"]
pub type CH2_R = crate::BitReader<CH2_A>;
#[doc = "Source last request for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1 = 0,
    #[doc = "1: Last transaction in current block"]
    VALUE2 = 1,
}
impl From<CH2_A> for bool {
    #[inline(always)]
    fn from(variant: CH2_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2_A {
        match self.bits {
            false => CH2_A::VALUE1,
            true => CH2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH2_A::VALUE2
    }
}
#[doc = "Field `CH2` writer - Source last request for channel 2"]
pub type CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTSRCREG_SPEC, CH2_A, O>;
impl<'a, const O: u8> CH2_W<'a, O> {
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH2_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH2_A::VALUE2)
    }
}
#[doc = "Field `CH3` reader - Source last request for channel 3"]
pub type CH3_R = crate::BitReader<CH3_A>;
#[doc = "Source last request for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1 = 0,
    #[doc = "1: Last transaction in current block"]
    VALUE2 = 1,
}
impl From<CH3_A> for bool {
    #[inline(always)]
    fn from(variant: CH3_A) -> Self {
        variant as u8 != 0
    }
}
impl CH3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3_A {
        match self.bits {
            false => CH3_A::VALUE1,
            true => CH3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH3_A::VALUE2
    }
}
#[doc = "Field `CH3` writer - Source last request for channel 3"]
pub type CH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTSRCREG_SPEC, CH3_A, O>;
impl<'a, const O: u8> CH3_W<'a, O> {
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH3_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH3_A::VALUE2)
    }
}
#[doc = "Source last transaction request write enable for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH0_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH0_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH0` writer - Source last transaction request write enable for channel 0"]
pub type WE_CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTSRCREG_SPEC, WE_CH0_AW, O>;
impl<'a, const O: u8> WE_CH0_W<'a, O> {
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH0_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH0_AW::VALUE2)
    }
}
#[doc = "Source last transaction request write enable for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH1_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH1_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH1` writer - Source last transaction request write enable for channel 1"]
pub type WE_CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTSRCREG_SPEC, WE_CH1_AW, O>;
impl<'a, const O: u8> WE_CH1_W<'a, O> {
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH1_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH1_AW::VALUE2)
    }
}
#[doc = "Source last transaction request write enable for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH2_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH2_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH2` writer - Source last transaction request write enable for channel 2"]
pub type WE_CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTSRCREG_SPEC, WE_CH2_AW, O>;
impl<'a, const O: u8> WE_CH2_W<'a, O> {
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH2_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH2_AW::VALUE2)
    }
}
#[doc = "Source last transaction request write enable for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH3_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH3_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH3` writer - Source last transaction request write enable for channel 3"]
pub type WE_CH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTSRCREG_SPEC, WE_CH3_AW, O>;
impl<'a, const O: u8> WE_CH3_W<'a, O> {
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH3_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH3_AW::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Source last request for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Source last request for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Source last request for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Source last request for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source last request for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Source last request for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - Source last request for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - Source last request for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<3> {
        CH3_W::new(self)
    }
    #[doc = "Bit 8 - Source last transaction request write enable for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch0(&mut self) -> WE_CH0_W<8> {
        WE_CH0_W::new(self)
    }
    #[doc = "Bit 9 - Source last transaction request write enable for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch1(&mut self) -> WE_CH1_W<9> {
        WE_CH1_W::new(self)
    }
    #[doc = "Bit 10 - Source last transaction request write enable for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch2(&mut self) -> WE_CH2_W<10> {
        WE_CH2_W::new(self)
    }
    #[doc = "Bit 11 - Source last transaction request write enable for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch3(&mut self) -> WE_CH3_W<11> {
        WE_CH3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Last Source Transaction Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstsrcreg](index.html) module"]
pub struct LSTSRCREG_SPEC;
impl crate::RegisterSpec for LSTSRCREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lstsrcreg::R](R) reader structure"]
impl crate::Readable for LSTSRCREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lstsrcreg::W](W) writer structure"]
impl crate::Writable for LSTSRCREG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LSTSRCREG to value 0"]
impl crate::Resettable for LSTSRCREG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
