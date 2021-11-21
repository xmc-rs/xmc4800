#[doc = "Register `EXTCLKCR` reader"]
pub struct R(crate::R<EXTCLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTCLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTCLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTCLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTCLKCR` writer"]
pub struct W(crate::W<EXTCLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTCLKCR_SPEC>;
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
impl From<crate::W<EXTCLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTCLKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "External Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ECKSEL_A {
    #[doc = "0: fSYS clock"]
    VALUE1 = 0,
    #[doc = "2: fUSB clock"]
    VALUE3 = 2,
    #[doc = "3: fPLL clock divided according to ECKDIV bit field configuration"]
    VALUE4 = 3,
}
impl From<ECKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ECKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ECKSEL` reader - External Clock Selection Value"]
pub struct ECKSEL_R(crate::FieldReader<u8, ECKSEL_A>);
impl ECKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ECKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ECKSEL_A> {
        match self.bits {
            0 => Some(ECKSEL_A::VALUE1),
            2 => Some(ECKSEL_A::VALUE3),
            3 => Some(ECKSEL_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ECKSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == ECKSEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == ECKSEL_A::VALUE4
    }
}
impl core::ops::Deref for ECKSEL_R {
    type Target = crate::FieldReader<u8, ECKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECKSEL` writer - External Clock Selection Value"]
pub struct ECKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ECKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "fSYS clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECKSEL_A::VALUE1)
    }
    #[doc = "fUSB clock"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ECKSEL_A::VALUE3)
    }
    #[doc = "fPLL clock divided according to ECKDIV bit field configuration"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ECKSEL_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `ECKDIV` reader - External Clock Divider Value"]
pub struct ECKDIV_R(crate::FieldReader<u16, u16>);
impl ECKDIV_R {
    pub(crate) fn new(bits: u16) -> Self {
        ECKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECKDIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECKDIV` writer - External Clock Divider Value"]
pub struct ECKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ECKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - External Clock Selection Value"]
    #[inline(always)]
    pub fn ecksel(&self) -> ECKSEL_R {
        ECKSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 16:24 - External Clock Divider Value"]
    #[inline(always)]
    pub fn eckdiv(&self) -> ECKDIV_R {
        ECKDIV_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Clock Selection Value"]
    #[inline(always)]
    pub fn ecksel(&mut self) -> ECKSEL_W {
        ECKSEL_W { w: self }
    }
    #[doc = "Bits 16:24 - External Clock Divider Value"]
    #[inline(always)]
    pub fn eckdiv(&mut self) -> ECKDIV_W {
        ECKDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extclkcr](index.html) module"]
pub struct EXTCLKCR_SPEC;
impl crate::RegisterSpec for EXTCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extclkcr::R](R) reader structure"]
impl crate::Readable for EXTCLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extclkcr::W](W) writer structure"]
impl crate::Writable for EXTCLKCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTCLKCR to value 0"]
impl crate::Resettable for EXTCLKCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
