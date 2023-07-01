#[doc = "Register `GPR1` reader"]
pub struct R(crate::R<GPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR1` writer"]
pub struct W(crate::W<GPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR1_SPEC>;
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
impl From<crate::W<GPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAT` reader - User Data"]
pub type DAT_R = crate::FieldReader<u32>;
#[doc = "Field `DAT` writer - User Data"]
pub type DAT_W<'a, const O: u8> = crate::FieldWriter<'a, GPR1_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - User Data"]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User Data"]
    #[inline(always)]
    #[must_use]
    pub fn dat(&mut self) -> DAT_W<0> {
        DAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr1](index.html) module"]
pub struct GPR1_SPEC;
impl crate::RegisterSpec for GPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr1::R](R) reader structure"]
impl crate::Readable for GPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr1::W](W) writer structure"]
impl crate::Writable for GPR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR1 to value 0"]
impl crate::Resettable for GPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
