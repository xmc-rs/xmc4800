#[doc = "Register `SRSEL1` reader"]
pub struct R(crate::R<SRSEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRSEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRSEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRSEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRSEL1` writer"]
pub struct W(crate::W<SRSEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRSEL1_SPEC>;
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
impl From<crate::W<SRSEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRSEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RS8` reader - Request Source for Line 8"]
pub type RS8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RS8` writer - Request Source for Line 8"]
pub type RS8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRSEL1_SPEC, u8, u8, 4, O>;
#[doc = "Field `RS9` reader - Request Source for Line 9"]
pub type RS9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RS9` writer - Request Source for Line 9"]
pub type RS9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRSEL1_SPEC, u8, u8, 4, O>;
#[doc = "Field `RS10` reader - Request Source for Line 10"]
pub type RS10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RS10` writer - Request Source for Line 10"]
pub type RS10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRSEL1_SPEC, u8, u8, 4, O>;
#[doc = "Field `RS11` reader - Request Source for Line 11"]
pub type RS11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RS11` writer - Request Source for Line 11"]
pub type RS11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRSEL1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Request Source for Line 8"]
    #[inline(always)]
    pub fn rs8(&self) -> RS8_R {
        RS8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Request Source for Line 9"]
    #[inline(always)]
    pub fn rs9(&self) -> RS9_R {
        RS9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Request Source for Line 10"]
    #[inline(always)]
    pub fn rs10(&self) -> RS10_R {
        RS10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Request Source for Line 11"]
    #[inline(always)]
    pub fn rs11(&self) -> RS11_R {
        RS11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Request Source for Line 8"]
    #[inline(always)]
    #[must_use]
    pub fn rs8(&mut self) -> RS8_W<0> {
        RS8_W::new(self)
    }
    #[doc = "Bits 4:7 - Request Source for Line 9"]
    #[inline(always)]
    #[must_use]
    pub fn rs9(&mut self) -> RS9_W<4> {
        RS9_W::new(self)
    }
    #[doc = "Bits 8:11 - Request Source for Line 10"]
    #[inline(always)]
    #[must_use]
    pub fn rs10(&mut self) -> RS10_W<8> {
        RS10_W::new(self)
    }
    #[doc = "Bits 12:15 - Request Source for Line 11"]
    #[inline(always)]
    #[must_use]
    pub fn rs11(&mut self) -> RS11_W<12> {
        RS11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Service Request Selection 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srsel1](index.html) module"]
pub struct SRSEL1_SPEC;
impl crate::RegisterSpec for SRSEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srsel1::R](R) reader structure"]
impl crate::Readable for SRSEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srsel1::W](W) writer structure"]
impl crate::Writable for SRSEL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRSEL1 to value 0"]
impl crate::Resettable for SRSEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
