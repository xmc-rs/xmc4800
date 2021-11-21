#[doc = "Register `PLLCON2` reader"]
pub struct R(crate::R<PLLCON2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCON2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCON2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCON2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCON2` writer"]
pub struct W(crate::W<PLLCON2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCON2_SPEC>;
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
impl From<crate::W<PLLCON2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCON2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "P-Divider Input Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINSEL_A {
    #[doc = "0: PLL external oscillator selected"]
    VALUE1 = 0,
    #[doc = "1: Backup clock fofi selected"]
    VALUE2 = 1,
}
impl From<PINSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PINSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINSEL` reader - P-Divider Input Selection"]
pub struct PINSEL_R(crate::FieldReader<bool, PINSEL_A>);
impl PINSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINSEL_A {
        match self.bits {
            false => PINSEL_A::VALUE1,
            true => PINSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PINSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PINSEL_A::VALUE2
    }
}
impl core::ops::Deref for PINSEL_R {
    type Target = crate::FieldReader<bool, PINSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINSEL` writer - P-Divider Input Selection"]
pub struct PINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PLL external oscillator selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PINSEL_A::VALUE1)
    }
    #[doc = "Backup clock fofi selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PINSEL_A::VALUE2)
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
#[doc = "K1-Divider Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum K1INSEL_A {
    #[doc = "0: PLL external oscillator selected"]
    VALUE1 = 0,
    #[doc = "1: Backup clock fofi selected"]
    VALUE2 = 1,
}
impl From<K1INSEL_A> for bool {
    #[inline(always)]
    fn from(variant: K1INSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `K1INSEL` reader - K1-Divider Input Selection"]
pub struct K1INSEL_R(crate::FieldReader<bool, K1INSEL_A>);
impl K1INSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        K1INSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> K1INSEL_A {
        match self.bits {
            false => K1INSEL_A::VALUE1,
            true => K1INSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == K1INSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == K1INSEL_A::VALUE2
    }
}
impl core::ops::Deref for K1INSEL_R {
    type Target = crate::FieldReader<bool, K1INSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `K1INSEL` writer - K1-Divider Input Selection"]
pub struct K1INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> K1INSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: K1INSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PLL external oscillator selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(K1INSEL_A::VALUE1)
    }
    #[doc = "Backup clock fofi selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(K1INSEL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - P-Divider Input Selection"]
    #[inline(always)]
    pub fn pinsel(&self) -> PINSEL_R {
        PINSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - K1-Divider Input Selection"]
    #[inline(always)]
    pub fn k1insel(&self) -> K1INSEL_R {
        K1INSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P-Divider Input Selection"]
    #[inline(always)]
    pub fn pinsel(&mut self) -> PINSEL_W {
        PINSEL_W { w: self }
    }
    #[doc = "Bit 8 - K1-Divider Input Selection"]
    #[inline(always)]
    pub fn k1insel(&mut self) -> K1INSEL_W {
        K1INSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Configuration 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcon2](index.html) module"]
pub struct PLLCON2_SPEC;
impl crate::RegisterSpec for PLLCON2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcon2::R](R) reader structure"]
impl crate::Readable for PLLCON2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcon2::W](W) writer structure"]
impl crate::Writable for PLLCON2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLCON2 to value 0x01"]
impl crate::Resettable for PLLCON2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
