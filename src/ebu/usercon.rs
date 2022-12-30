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
pub type DIP_R = crate::BitReader<bool>;
#[doc = "Field `DIP` writer - Disable Internal Pipelining"]
pub type DIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, USERCON_SPEC, bool, O>;
#[doc = "Field `ADDIO` reader - Address Pins to GPIO Mode"]
pub type ADDIO_R = crate::FieldReader<u16, ADDIO_A>;
#[doc = "Address Pins to GPIO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ADDIO_R {
    #[doc = "Get enumerated values variant"]
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
        *self == ADDIO_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ADDIO_A::VALUE2
    }
}
#[doc = "Field `ADDIO` writer - Address Pins to GPIO Mode"]
pub type ADDIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USERCON_SPEC, u16, ADDIO_A, 9, O>;
impl<'a, const O: u8> ADDIO_W<'a, O> {
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
}
#[doc = "Field `ADVIO` reader - ADV Pin to GPIO Mode"]
pub type ADVIO_R = crate::BitReader<ADVIO_A>;
#[doc = "ADV Pin to GPIO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ADVIO_R {
    #[doc = "Get enumerated values variant"]
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
        *self == ADVIO_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ADVIO_A::VALUE2
    }
}
#[doc = "Field `ADVIO` writer - ADV Pin to GPIO Mode"]
pub type ADVIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, USERCON_SPEC, ADVIO_A, O>;
impl<'a, const O: u8> ADVIO_W<'a, O> {
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
}
impl R {
    #[doc = "Bit 0 - Disable Internal Pipelining"]
    #[inline(always)]
    pub fn dip(&self) -> DIP_R {
        DIP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:24 - Address Pins to GPIO Mode"]
    #[inline(always)]
    pub fn addio(&self) -> ADDIO_R {
        ADDIO_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 25 - ADV Pin to GPIO Mode"]
    #[inline(always)]
    pub fn advio(&self) -> ADVIO_R {
        ADVIO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable Internal Pipelining"]
    #[inline(always)]
    #[must_use]
    pub fn dip(&mut self) -> DIP_W<0> {
        DIP_W::new(self)
    }
    #[doc = "Bits 16:24 - Address Pins to GPIO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn addio(&mut self) -> ADDIO_W<16> {
        ADDIO_W::new(self)
    }
    #[doc = "Bit 25 - ADV Pin to GPIO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn advio(&mut self) -> ADVIO_W<25> {
        ADVIO_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USERCON to value 0"]
impl crate::Resettable for USERCON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
