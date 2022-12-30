#[doc = "Register `BOUNDSEL` reader"]
pub struct R(crate::R<BOUNDSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOUNDSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOUNDSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOUNDSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOUNDSEL` writer"]
pub struct W(crate::W<BOUNDSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOUNDSEL_SPEC>;
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
impl From<crate::W<BOUNDSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOUNDSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOUNDARYL` reader - Lower Boundary Value for Limit Checking"]
pub type BOUNDARYL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BOUNDARYL` writer - Lower Boundary Value for Limit Checking"]
pub type BOUNDARYL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BOUNDSEL_SPEC, u16, u16, 16, O>;
#[doc = "Field `BOUNDARYU` reader - Upper Boundary Value for Limit Checking"]
pub type BOUNDARYU_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BOUNDARYU` writer - Upper Boundary Value for Limit Checking"]
pub type BOUNDARYU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BOUNDSEL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Lower Boundary Value for Limit Checking"]
    #[inline(always)]
    pub fn boundaryl(&self) -> BOUNDARYL_R {
        BOUNDARYL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Upper Boundary Value for Limit Checking"]
    #[inline(always)]
    pub fn boundaryu(&self) -> BOUNDARYU_R {
        BOUNDARYU_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower Boundary Value for Limit Checking"]
    #[inline(always)]
    #[must_use]
    pub fn boundaryl(&mut self) -> BOUNDARYL_W<0> {
        BOUNDARYL_W::new(self)
    }
    #[doc = "Bits 16:31 - Upper Boundary Value for Limit Checking"]
    #[inline(always)]
    #[must_use]
    pub fn boundaryu(&mut self) -> BOUNDARYU_W<16> {
        BOUNDARYU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Boundary Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boundsel](index.html) module"]
pub struct BOUNDSEL_SPEC;
impl crate::RegisterSpec for BOUNDSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [boundsel::R](R) reader structure"]
impl crate::Readable for BOUNDSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [boundsel::W](W) writer structure"]
impl crate::Writable for BOUNDSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOUNDSEL to value 0"]
impl crate::Resettable for BOUNDSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
