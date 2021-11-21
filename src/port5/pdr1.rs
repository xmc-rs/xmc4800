#[doc = "Register `PDR1` reader"]
pub struct R(crate::R<PDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDR1` writer"]
pub struct W(crate::W<PDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDR1_SPEC>;
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
impl From<crate::W<PDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad Driver Mode for Pn.8\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD8_A {
    #[doc = "0: A2 strong driver, sharp edge"]
    SD_SHE = 0,
    #[doc = "1: A2 strong driver, medium edge"]
    SD_MEE = 1,
    #[doc = "2: A2 strong driver, soft edge"]
    SD_SOE = 2,
    #[doc = "4: A2 medium driver"]
    MD = 4,
    #[doc = "7: A2 weak driver"]
    WD = 7,
}
impl From<PD8_A> for u8 {
    #[inline(always)]
    fn from(variant: PD8_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PD8` reader - Pad Driver Mode for Pn.8"]
pub struct PD8_R(crate::FieldReader<u8, PD8_A>);
impl PD8_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PD8_A> {
        match self.bits {
            0 => Some(PD8_A::SD_SHE),
            1 => Some(PD8_A::SD_MEE),
            2 => Some(PD8_A::SD_SOE),
            4 => Some(PD8_A::MD),
            7 => Some(PD8_A::WD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        **self == PD8_A::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        **self == PD8_A::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        **self == PD8_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        **self == PD8_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        **self == PD8_A::WD
    }
}
impl core::ops::Deref for PD8_R {
    type Target = crate::FieldReader<u8, PD8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD8` writer - Pad Driver Mode for Pn.8"]
pub struct PD8_W<'a> {
    w: &'a mut W,
}
impl<'a> PD8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD8_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD8_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD8_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut W {
        self.variant(PD8_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD8_A::WD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.9\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD9_A {
    #[doc = "0: A2 strong driver, sharp edge"]
    SD_SHE = 0,
    #[doc = "1: A2 strong driver, medium edge"]
    SD_MEE = 1,
    #[doc = "2: A2 strong driver, soft edge"]
    SD_SOE = 2,
    #[doc = "4: A2 medium driver"]
    MD = 4,
    #[doc = "7: A2 weak driver"]
    WD = 7,
}
impl From<PD9_A> for u8 {
    #[inline(always)]
    fn from(variant: PD9_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PD9` reader - Pad Driver Mode for Pn.9"]
pub struct PD9_R(crate::FieldReader<u8, PD9_A>);
impl PD9_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PD9_A> {
        match self.bits {
            0 => Some(PD9_A::SD_SHE),
            1 => Some(PD9_A::SD_MEE),
            2 => Some(PD9_A::SD_SOE),
            4 => Some(PD9_A::MD),
            7 => Some(PD9_A::WD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        **self == PD9_A::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        **self == PD9_A::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        **self == PD9_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        **self == PD9_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        **self == PD9_A::WD
    }
}
impl core::ops::Deref for PD9_R {
    type Target = crate::FieldReader<u8, PD9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD9` writer - Pad Driver Mode for Pn.9"]
pub struct PD9_W<'a> {
    w: &'a mut W,
}
impl<'a> PD9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD9_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD9_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD9_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD9_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut W {
        self.variant(PD9_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD9_A::WD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.10\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD10_A {
    #[doc = "2: A1+ strong driver, soft edge"]
    SD_SOE = 2,
    #[doc = "3: A1+ strong driver, slow edge"]
    SD_SLE = 3,
    #[doc = "4: A1+ medium driver"]
    MD = 4,
    #[doc = "7: A1+ weak driver"]
    WD = 7,
    #[doc = "0: A1+ strong driver, soft edge (alternate value)"]
    SD_SOE_ALT = 0,
    #[doc = "1: A1+ strong driver, slow edge (alternate value)"]
    SD_SLE_ALT = 1,
    #[doc = "6: A1+ medium driver (alternate value)"]
    MD_ALT = 6,
    #[doc = "5: A1+ weak driver (alternate value)"]
    WD_ALT = 5,
}
impl From<PD10_A> for u8 {
    #[inline(always)]
    fn from(variant: PD10_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PD10` reader - Pad Driver Mode for Pn.10"]
pub struct PD10_R(crate::FieldReader<u8, PD10_A>);
impl PD10_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD10_A {
        match self.bits {
            2 => PD10_A::SD_SOE,
            3 => PD10_A::SD_SLE,
            4 => PD10_A::MD,
            7 => PD10_A::WD,
            0 => PD10_A::SD_SOE_ALT,
            1 => PD10_A::SD_SLE_ALT,
            6 => PD10_A::MD_ALT,
            5 => PD10_A::WD_ALT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        **self == PD10_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `SD_SLE`"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        **self == PD10_A::SD_SLE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        **self == PD10_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        **self == PD10_A::WD
    }
    #[doc = "Checks if the value of the field is `SD_SOE_ALT`"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        **self == PD10_A::SD_SOE_ALT
    }
    #[doc = "Checks if the value of the field is `SD_SLE_ALT`"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        **self == PD10_A::SD_SLE_ALT
    }
    #[doc = "Checks if the value of the field is `MD_ALT`"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        **self == PD10_A::MD_ALT
    }
    #[doc = "Checks if the value of the field is `WD_ALT`"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        **self == PD10_A::WD_ALT
    }
}
impl core::ops::Deref for PD10_R {
    type Target = crate::FieldReader<u8, PD10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD10` writer - Pad Driver Mode for Pn.10"]
pub struct PD10_W<'a> {
    w: &'a mut W,
}
impl<'a> PD10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD10_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD10_A::SD_SOE)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut W {
        self.variant(PD10_A::SD_SLE)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut W {
        self.variant(PD10_A::MD)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD10_A::WD)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut W {
        self.variant(PD10_A::SD_SOE_ALT)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut W {
        self.variant(PD10_A::SD_SLE_ALT)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut W {
        self.variant(PD10_A::MD_ALT)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut W {
        self.variant(PD10_A::WD_ALT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.11\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD11_A {
    #[doc = "2: A1+ strong driver, soft edge"]
    SD_SOE = 2,
    #[doc = "3: A1+ strong driver, slow edge"]
    SD_SLE = 3,
    #[doc = "4: A1+ medium driver"]
    MD = 4,
    #[doc = "7: A1+ weak driver"]
    WD = 7,
    #[doc = "0: A1+ strong driver, soft edge (alternate value)"]
    SD_SOE_ALT = 0,
    #[doc = "1: A1+ strong driver, slow edge (alternate value)"]
    SD_SLE_ALT = 1,
    #[doc = "6: A1+ medium driver (alternate value)"]
    MD_ALT = 6,
    #[doc = "5: A1+ weak driver (alternate value)"]
    WD_ALT = 5,
}
impl From<PD11_A> for u8 {
    #[inline(always)]
    fn from(variant: PD11_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PD11` reader - Pad Driver Mode for Pn.11"]
pub struct PD11_R(crate::FieldReader<u8, PD11_A>);
impl PD11_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD11_A {
        match self.bits {
            2 => PD11_A::SD_SOE,
            3 => PD11_A::SD_SLE,
            4 => PD11_A::MD,
            7 => PD11_A::WD,
            0 => PD11_A::SD_SOE_ALT,
            1 => PD11_A::SD_SLE_ALT,
            6 => PD11_A::MD_ALT,
            5 => PD11_A::WD_ALT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        **self == PD11_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `SD_SLE`"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        **self == PD11_A::SD_SLE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        **self == PD11_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        **self == PD11_A::WD
    }
    #[doc = "Checks if the value of the field is `SD_SOE_ALT`"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        **self == PD11_A::SD_SOE_ALT
    }
    #[doc = "Checks if the value of the field is `SD_SLE_ALT`"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        **self == PD11_A::SD_SLE_ALT
    }
    #[doc = "Checks if the value of the field is `MD_ALT`"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        **self == PD11_A::MD_ALT
    }
    #[doc = "Checks if the value of the field is `WD_ALT`"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        **self == PD11_A::WD_ALT
    }
}
impl core::ops::Deref for PD11_R {
    type Target = crate::FieldReader<u8, PD11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD11` writer - Pad Driver Mode for Pn.11"]
pub struct PD11_W<'a> {
    w: &'a mut W,
}
impl<'a> PD11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD11_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD11_A::SD_SOE)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut W {
        self.variant(PD11_A::SD_SLE)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut W {
        self.variant(PD11_A::MD)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD11_A::WD)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut W {
        self.variant(PD11_A::SD_SOE_ALT)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut W {
        self.variant(PD11_A::SD_SLE_ALT)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut W {
        self.variant(PD11_A::MD_ALT)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut W {
        self.variant(PD11_A::WD_ALT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.12\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD12_A {
    #[doc = "0: A2 strong driver, sharp edge"]
    SD_SHE = 0,
    #[doc = "1: A2 strong driver, medium edge"]
    SD_MEE = 1,
    #[doc = "2: A2 strong driver, soft edge"]
    SD_SOE = 2,
    #[doc = "4: A2 medium driver"]
    MD = 4,
    #[doc = "7: A2 weak driver"]
    WD = 7,
}
impl From<PD12_A> for u8 {
    #[inline(always)]
    fn from(variant: PD12_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PD12` reader - Pad Driver Mode for Pn.12"]
pub struct PD12_R(crate::FieldReader<u8, PD12_A>);
impl PD12_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PD12_A> {
        match self.bits {
            0 => Some(PD12_A::SD_SHE),
            1 => Some(PD12_A::SD_MEE),
            2 => Some(PD12_A::SD_SOE),
            4 => Some(PD12_A::MD),
            7 => Some(PD12_A::WD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        **self == PD12_A::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        **self == PD12_A::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        **self == PD12_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        **self == PD12_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        **self == PD12_A::WD
    }
}
impl core::ops::Deref for PD12_R {
    type Target = crate::FieldReader<u8, PD12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD12` writer - Pad Driver Mode for Pn.12"]
pub struct PD12_W<'a> {
    w: &'a mut W,
}
impl<'a> PD12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD12_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD12_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD12_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD12_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut W {
        self.variant(PD12_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD12_A::WD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.13\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD13_A {
    #[doc = "0: A2 strong driver, sharp edge"]
    SD_SHE = 0,
    #[doc = "1: A2 strong driver, medium edge"]
    SD_MEE = 1,
    #[doc = "2: A2 strong driver, soft edge"]
    SD_SOE = 2,
    #[doc = "4: A2 medium driver"]
    MD = 4,
    #[doc = "7: A2 weak driver"]
    WD = 7,
}
impl From<PD13_A> for u8 {
    #[inline(always)]
    fn from(variant: PD13_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PD13` reader - Pad Driver Mode for Pn.13"]
pub struct PD13_R(crate::FieldReader<u8, PD13_A>);
impl PD13_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PD13_A> {
        match self.bits {
            0 => Some(PD13_A::SD_SHE),
            1 => Some(PD13_A::SD_MEE),
            2 => Some(PD13_A::SD_SOE),
            4 => Some(PD13_A::MD),
            7 => Some(PD13_A::WD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        **self == PD13_A::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        **self == PD13_A::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        **self == PD13_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        **self == PD13_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        **self == PD13_A::WD
    }
}
impl core::ops::Deref for PD13_R {
    type Target = crate::FieldReader<u8, PD13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD13` writer - Pad Driver Mode for Pn.13"]
pub struct PD13_W<'a> {
    w: &'a mut W,
}
impl<'a> PD13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD13_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD13_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD13_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD13_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut W {
        self.variant(PD13_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD13_A::WD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.14\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD14_A {
    #[doc = "0: A2 strong driver, sharp edge"]
    SD_SHE = 0,
    #[doc = "1: A2 strong driver, medium edge"]
    SD_MEE = 1,
    #[doc = "2: A2 strong driver, soft edge"]
    SD_SOE = 2,
    #[doc = "4: A2 medium driver"]
    MD = 4,
    #[doc = "7: A2 weak driver"]
    WD = 7,
}
impl From<PD14_A> for u8 {
    #[inline(always)]
    fn from(variant: PD14_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PD14` reader - Pad Driver Mode for Pn.14"]
pub struct PD14_R(crate::FieldReader<u8, PD14_A>);
impl PD14_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PD14_A> {
        match self.bits {
            0 => Some(PD14_A::SD_SHE),
            1 => Some(PD14_A::SD_MEE),
            2 => Some(PD14_A::SD_SOE),
            4 => Some(PD14_A::MD),
            7 => Some(PD14_A::WD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        **self == PD14_A::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        **self == PD14_A::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        **self == PD14_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        **self == PD14_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        **self == PD14_A::WD
    }
}
impl core::ops::Deref for PD14_R {
    type Target = crate::FieldReader<u8, PD14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD14` writer - Pad Driver Mode for Pn.14"]
pub struct PD14_W<'a> {
    w: &'a mut W,
}
impl<'a> PD14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD14_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD14_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD14_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD14_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut W {
        self.variant(PD14_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD14_A::WD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.15\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD15_A {
    #[doc = "0: A2 strong driver, sharp edge"]
    SD_SHE = 0,
    #[doc = "1: A2 strong driver, medium edge"]
    SD_MEE = 1,
    #[doc = "2: A2 strong driver, soft edge"]
    SD_SOE = 2,
    #[doc = "4: A2 medium driver"]
    MD = 4,
    #[doc = "7: A2 weak driver"]
    WD = 7,
}
impl From<PD15_A> for u8 {
    #[inline(always)]
    fn from(variant: PD15_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PD15` reader - Pad Driver Mode for Pn.15"]
pub struct PD15_R(crate::FieldReader<u8, PD15_A>);
impl PD15_R {
    pub(crate) fn new(bits: u8) -> Self {
        PD15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PD15_A> {
        match self.bits {
            0 => Some(PD15_A::SD_SHE),
            1 => Some(PD15_A::SD_MEE),
            2 => Some(PD15_A::SD_SOE),
            4 => Some(PD15_A::MD),
            7 => Some(PD15_A::WD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        **self == PD15_A::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        **self == PD15_A::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        **self == PD15_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        **self == PD15_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        **self == PD15_A::WD
    }
}
impl core::ops::Deref for PD15_R {
    type Target = crate::FieldReader<u8, PD15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD15` writer - Pad Driver Mode for Pn.15"]
pub struct PD15_W<'a> {
    w: &'a mut W,
}
impl<'a> PD15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD15_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD15_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD15_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD15_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut W {
        self.variant(PD15_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD15_A::WD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.8"]
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.9"]
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.10"]
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.11"]
    #[inline(always)]
    pub fn pd11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.12"]
    #[inline(always)]
    pub fn pd12(&self) -> PD12_R {
        PD12_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.13"]
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.14"]
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.15"]
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.8"]
    #[inline(always)]
    pub fn pd8(&mut self) -> PD8_W {
        PD8_W { w: self }
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.9"]
    #[inline(always)]
    pub fn pd9(&mut self) -> PD9_W {
        PD9_W { w: self }
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.10"]
    #[inline(always)]
    pub fn pd10(&mut self) -> PD10_W {
        PD10_W { w: self }
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.11"]
    #[inline(always)]
    pub fn pd11(&mut self) -> PD11_W {
        PD11_W { w: self }
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.12"]
    #[inline(always)]
    pub fn pd12(&mut self) -> PD12_W {
        PD12_W { w: self }
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.13"]
    #[inline(always)]
    pub fn pd13(&mut self) -> PD13_W {
        PD13_W { w: self }
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.14"]
    #[inline(always)]
    pub fn pd14(&mut self) -> PD14_W {
        PD14_W { w: self }
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.15"]
    #[inline(always)]
    pub fn pd15(&mut self) -> PD15_W {
        PD15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 5 Pad Driver Mode 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdr1](index.html) module"]
pub struct PDR1_SPEC;
impl crate::RegisterSpec for PDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdr1::R](R) reader structure"]
impl crate::Readable for PDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdr1::W](W) writer structure"]
impl crate::Writable for PDR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDR1 to value 0x2222_2222"]
impl crate::Resettable for PDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2222_2222
    }
}
