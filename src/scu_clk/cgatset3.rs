#[doc = "Register `CGATSET3` writer"]
pub struct W(crate::W<CGATSET3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGATSET3_SPEC>;
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
impl From<crate::W<CGATSET3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGATSET3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EBU Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBU_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<EBU_AW> for bool {
    #[inline(always)]
    fn from(variant: EBU_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBU` writer - EBU Gating Set"]
pub struct EBU_W<'a> {
    w: &'a mut W,
}
impl<'a> EBU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EBU_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EBU_AW::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EBU_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl W {
    #[doc = "Bit 2 - EBU Gating Set"]
    #[inline(always)]
    pub fn ebu(&mut self) -> EBU_W {
        EBU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral 3 Clock Gating Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgatset3](index.html) module"]
pub struct CGATSET3_SPEC;
impl crate::RegisterSpec for CGATSET3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cgatset3::W](W) writer structure"]
impl crate::Writable for CGATSET3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CGATSET3 to value 0"]
impl crate::Resettable for CGATSET3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
