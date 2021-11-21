#[doc = "Register `USERCON` reader"]
pub struct R(crate::R<USERCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USERCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USERCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USERCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USERCON` writer"]
pub struct W(crate::W<USERCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USERCON_SPEC>;
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
impl From<crate::W<USERCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USERCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIP` reader - Disable Internal Pipelining"]
pub struct DIP_R(crate::FieldReader<bool, bool>);
impl DIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIP` writer - Disable Internal Pipelining"]
pub struct DIP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIP_W<'a> {
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
#[doc = "Address Pins to GPIO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ADDIO_A {
    #[doc = "0: Address Bit is required for addressing memory"]
    VALUE1 = 0,
    #[doc = "1: Address Bit is available for GPIO function"]
    VALUE2 = 1,
}
impl From<ADDIO_A> for u16 {
    #[inline(always)]
    fn from(variant: ADDIO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADDIO` reader - Address Pins to GPIO Mode"]
pub struct ADDIO_R(crate::FieldReader<u16, ADDIO_A>);
impl ADDIO_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADDIO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADDIO_A> {
        match self.bits {
            0 => Some(ADDIO_A::VALUE1),
            1 => Some(ADDIO_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ADDIO_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ADDIO_A::VALUE2
    }
}
impl core::ops::Deref for ADDIO_R {
    type Target = crate::FieldReader<u16, ADDIO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDIO` writer - Address Pins to GPIO Mode"]
pub struct ADDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDIO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Address Bit is required for addressing memory"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ADDIO_A::VALUE1)
    }
    #[doc = "Address Bit is available for GPIO function"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ADDIO_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
#[doc = "ADV Pin to GPIO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADVIO_A {
    #[doc = "0: ADV pin is required for controlling memory"]
    VALUE1 = 0,
    #[doc = "1: ADV pin is available for GPIO function"]
    VALUE2 = 1,
}
impl From<ADVIO_A> for bool {
    #[inline(always)]
    fn from(variant: ADVIO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADVIO` reader - ADV Pin to GPIO Mode"]
pub struct ADVIO_R(crate::FieldReader<bool, ADVIO_A>);
impl ADVIO_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADVIO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADVIO_A {
        match self.bits {
            false => ADVIO_A::VALUE1,
            true => ADVIO_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ADVIO_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ADVIO_A::VALUE2
    }
}
impl core::ops::Deref for ADVIO_R {
    type Target = crate::FieldReader<bool, ADVIO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADVIO` writer - ADV Pin to GPIO Mode"]
pub struct ADVIO_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADVIO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADV pin is required for controlling memory"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ADVIO_A::VALUE1)
    }
    #[doc = "ADV pin is available for GPIO function"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ADVIO_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Disable Internal Pipelining"]
    #[inline(always)]
    pub fn dip(&self) -> DIP_R {
        DIP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 16:24 - Address Pins to GPIO Mode"]
    #[inline(always)]
    pub fn addio(&self) -> ADDIO_R {
        ADDIO_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 25 - ADV Pin to GPIO Mode"]
    #[inline(always)]
    pub fn advio(&self) -> ADVIO_R {
        ADVIO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable Internal Pipelining"]
    #[inline(always)]
    pub fn dip(&mut self) -> DIP_W {
        DIP_W { w: self }
    }
    #[doc = "Bits 16:24 - Address Pins to GPIO Mode"]
    #[inline(always)]
    pub fn addio(&mut self) -> ADDIO_W {
        ADDIO_W { w: self }
    }
    #[doc = "Bit 25 - ADV Pin to GPIO Mode"]
    #[inline(always)]
    pub fn advio(&mut self) -> ADVIO_W {
        ADVIO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBU Test/Control Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usercon](index.html) module"]
pub struct USERCON_SPEC;
impl crate::RegisterSpec for USERCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usercon::R](R) reader structure"]
impl crate::Readable for USERCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usercon::W](W) writer structure"]
impl crate::Writable for USERCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USERCON to value 0"]
impl crate::Resettable for USERCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
