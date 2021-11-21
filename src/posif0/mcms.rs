#[doc = "Register `MCMS` writer"]
pub struct W(crate::W<MCMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCMS_SPEC>;
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
impl From<crate::W<MCMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MNPS` writer - Multi-Channel Pattern Update Enable Set"]
pub struct MNPS_W<'a> {
    w: &'a mut W,
}
impl<'a> MNPS_W<'a> {
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
#[doc = "Field `STHR` writer - Hall Pattern Shadow Transfer Request"]
pub struct STHR_W<'a> {
    w: &'a mut W,
}
impl<'a> STHR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `STMR` writer - Multi-Channel Shadow Transfer Request"]
pub struct STMR_W<'a> {
    w: &'a mut W,
}
impl<'a> STMR_W<'a> {
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
    #[doc = "Bit 0 - Multi-Channel Pattern Update Enable Set"]
    #[inline(always)]
    pub fn mnps(&mut self) -> MNPS_W {
        MNPS_W { w: self }
    }
    #[doc = "Bit 1 - Hall Pattern Shadow Transfer Request"]
    #[inline(always)]
    pub fn sthr(&mut self) -> STHR_W {
        STHR_W { w: self }
    }
    #[doc = "Bit 2 - Multi-Channel Shadow Transfer Request"]
    #[inline(always)]
    pub fn stmr(&mut self) -> STMR_W {
        STMR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Multi-Channel Pattern Control set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcms](index.html) module"]
pub struct MCMS_SPEC;
impl crate::RegisterSpec for MCMS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mcms::W](W) writer structure"]
impl crate::Writable for MCMS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCMS to value 0"]
impl crate::Resettable for MCMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
