#[doc = "Register `PRCLR3` writer"]
pub struct W(crate::W<PRCLR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRCLR3_SPEC>;
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
impl From<crate::W<PRCLR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRCLR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EBU Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBURS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<EBURS_AW> for bool {
    #[inline(always)]
    fn from(variant: EBURS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBURS` writer - EBU Reset Assert"]
pub struct EBURS_W<'a> {
    w: &'a mut W,
}
impl<'a> EBURS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EBURS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EBURS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EBURS_AW::VALUE2)
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
    #[doc = "Bit 2 - EBU Reset Assert"]
    #[inline(always)]
    pub fn eburs(&mut self) -> EBURS_W {
        EBURS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCU Peripheral 3 Reset Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prclr3](index.html) module"]
pub struct PRCLR3_SPEC;
impl crate::RegisterSpec for PRCLR3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [prclr3::W](W) writer structure"]
impl crate::Writable for PRCLR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRCLR3 to value 0"]
impl crate::Resettable for PRCLR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
