#[doc = "Register `MODCFG` reader"]
pub struct R(crate::R<MODCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODCFG` writer"]
pub struct W(crate::W<MODCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODCFG_SPEC>;
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
impl From<crate::W<MODCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Divider Factor for Modulator Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVM_A {
    #[doc = "0: fMOD = fCLK / 2"]
    VALUE1 = 0,
    #[doc = "1: fMOD = fCLK / 4"]
    VALUE2 = 1,
    #[doc = "2: fMOD = fCLK / 6"]
    VALUE3 = 2,
    #[doc = "15: fMOD = fCLK / 32"]
    VALUE4 = 15,
}
impl From<DIVM_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIVM` reader - Divider Factor for Modulator Clock"]
pub struct DIVM_R(crate::FieldReader<u8, DIVM_A>);
impl DIVM_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVM_A> {
        match self.bits {
            0 => Some(DIVM_A::VALUE1),
            1 => Some(DIVM_A::VALUE2),
            2 => Some(DIVM_A::VALUE3),
            15 => Some(DIVM_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DIVM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DIVM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == DIVM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == DIVM_A::VALUE4
    }
}
impl core::ops::Deref for DIVM_R {
    type Target = crate::FieldReader<u8, DIVM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVM` writer - Divider Factor for Modulator Clock"]
pub struct DIVM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "fMOD = fCLK / 2"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVM_A::VALUE1)
    }
    #[doc = "fMOD = fCLK / 4"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVM_A::VALUE2)
    }
    #[doc = "fMOD = fCLK / 6"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DIVM_A::VALUE3)
    }
    #[doc = "fMOD = fCLK / 32"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(DIVM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Write Control for Divider Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DWC_AW {
    #[doc = "0: No write access to divider factor"]
    VALUE1 = 0,
    #[doc = "1: Bitfield DIVM can be written"]
    VALUE2 = 1,
}
impl From<DWC_AW> for bool {
    #[inline(always)]
    fn from(variant: DWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DWC` writer - Write Control for Divider Factor"]
pub struct DWC_W<'a> {
    w: &'a mut W,
}
impl<'a> DWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DWC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No write access to divider factor"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DWC_AW::VALUE1)
    }
    #[doc = "Bitfield DIVM can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DWC_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:19 - Divider Factor for Modulator Clock"]
    #[inline(always)]
    pub fn divm(&self) -> DIVM_R {
        DIVM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - Divider Factor for Modulator Clock"]
    #[inline(always)]
    pub fn divm(&mut self) -> DIVM_W {
        DIVM_W { w: self }
    }
    #[doc = "Bit 23 - Write Control for Divider Factor"]
    #[inline(always)]
    pub fn dwc(&mut self) -> DWC_W {
        DWC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Modulator Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modcfg](index.html) module"]
pub struct MODCFG_SPEC;
impl crate::RegisterSpec for MODCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modcfg::R](R) reader structure"]
impl crate::Readable for MODCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modcfg::W](W) writer structure"]
impl crate::Writable for MODCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODCFG to value 0"]
impl crate::Resettable for MODCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
