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
#[doc = "Field `DIVM` reader - Divider Factor for Modulator Clock"]
pub type DIVM_R = crate::FieldReader<u8, DIVM_A>;
#[doc = "Divider Factor for Modulator Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DIVM_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DIVM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIVM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DIVM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DIVM_A::VALUE4
    }
}
#[doc = "Field `DIVM` writer - Divider Factor for Modulator Clock"]
pub type DIVM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODCFG_SPEC, u8, DIVM_A, 4, O>;
impl<'a, const O: u8> DIVM_W<'a, O> {
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
}
#[doc = "Write Control for Divider Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type DWC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODCFG_SPEC, DWC_AW, O>;
impl<'a, const O: u8> DWC_W<'a, O> {
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
    #[must_use]
    pub fn divm(&mut self) -> DIVM_W<16> {
        DIVM_W::new(self)
    }
    #[doc = "Bit 23 - Write Control for Divider Factor"]
    #[inline(always)]
    #[must_use]
    pub fn dwc(&mut self) -> DWC_W<23> {
        DWC_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODCFG to value 0"]
impl crate::Resettable for MODCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
