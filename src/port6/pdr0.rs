#[doc = "Reader of register PDR0"]
pub type R = crate::R<u32, super::PDR0>;
#[doc = "Writer for register PDR0"]
pub type W = crate::W<u32, super::PDR0>;
#[doc = "Register PDR0 `reset()`'s with value 0x2222_2222"]
impl crate::ResetValue for super::PDR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2222_2222
    }
}
#[doc = "Pad Driver Mode for Pn.0\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD0_A {
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
impl From<PD0_A> for u8 {
    #[inline(always)]
    fn from(variant: PD0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PD0`"]
pub type PD0_R = crate::R<u8, PD0_A>;
impl PD0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PD0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PD0_A::SD_SHE),
            1 => Val(PD0_A::SD_MEE),
            2 => Val(PD0_A::SD_SOE),
            4 => Val(PD0_A::MD),
            7 => Val(PD0_A::WD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD0_A::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD0_A::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD0_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD0_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD0_A::WD
    }
}
#[doc = "Write proxy for field `PD0`"]
pub struct PD0_W<'a> {
    w: &'a mut W,
}
impl<'a> PD0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD0_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD0_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD0_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut W {
        self.variant(PD0_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD0_A::WD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.1\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD1_A {
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
impl From<PD1_A> for u8 {
    #[inline(always)]
    fn from(variant: PD1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PD1`"]
pub type PD1_R = crate::R<u8, PD1_A>;
impl PD1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PD1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PD1_A::SD_SHE),
            1 => Val(PD1_A::SD_MEE),
            2 => Val(PD1_A::SD_SOE),
            4 => Val(PD1_A::MD),
            7 => Val(PD1_A::WD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD1_A::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD1_A::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD1_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD1_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD1_A::WD
    }
}
#[doc = "Write proxy for field `PD1`"]
pub struct PD1_W<'a> {
    w: &'a mut W,
}
impl<'a> PD1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD1_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD1_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD1_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut W {
        self.variant(PD1_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD1_A::WD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.2\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD2_A {
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
impl From<PD2_A> for u8 {
    #[inline(always)]
    fn from(variant: PD2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PD2`"]
pub type PD2_R = crate::R<u8, PD2_A>;
impl PD2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PD2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PD2_A::SD_SHE),
            1 => Val(PD2_A::SD_MEE),
            2 => Val(PD2_A::SD_SOE),
            4 => Val(PD2_A::MD),
            7 => Val(PD2_A::WD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD2_A::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD2_A::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD2_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD2_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD2_A::WD
    }
}
#[doc = "Write proxy for field `PD2`"]
pub struct PD2_W<'a> {
    w: &'a mut W,
}
impl<'a> PD2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD2_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD2_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD2_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut W {
        self.variant(PD2_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD2_A::WD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.3\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD3_A {
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
impl From<PD3_A> for u8 {
    #[inline(always)]
    fn from(variant: PD3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PD3`"]
pub type PD3_R = crate::R<u8, PD3_A>;
impl PD3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD3_A {
        match self.bits {
            2 => PD3_A::SD_SOE,
            3 => PD3_A::SD_SLE,
            4 => PD3_A::MD,
            7 => PD3_A::WD,
            0 => PD3_A::SD_SOE_ALT,
            1 => PD3_A::SD_SLE_ALT,
            6 => PD3_A::MD_ALT,
            5 => PD3_A::WD_ALT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD3_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `SD_SLE`"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == PD3_A::SD_SLE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD3_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD3_A::WD
    }
    #[doc = "Checks if the value of the field is `SD_SOE_ALT`"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == PD3_A::SD_SOE_ALT
    }
    #[doc = "Checks if the value of the field is `SD_SLE_ALT`"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == PD3_A::SD_SLE_ALT
    }
    #[doc = "Checks if the value of the field is `MD_ALT`"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == PD3_A::MD_ALT
    }
    #[doc = "Checks if the value of the field is `WD_ALT`"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == PD3_A::WD_ALT
    }
}
#[doc = "Write proxy for field `PD3`"]
pub struct PD3_W<'a> {
    w: &'a mut W,
}
impl<'a> PD3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD3_A::SD_SOE)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut W {
        self.variant(PD3_A::SD_SLE)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut W {
        self.variant(PD3_A::MD)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD3_A::WD)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut W {
        self.variant(PD3_A::SD_SOE_ALT)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut W {
        self.variant(PD3_A::SD_SLE_ALT)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut W {
        self.variant(PD3_A::MD_ALT)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut W {
        self.variant(PD3_A::WD_ALT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.4\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD4_A {
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
impl From<PD4_A> for u8 {
    #[inline(always)]
    fn from(variant: PD4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PD4`"]
pub type PD4_R = crate::R<u8, PD4_A>;
impl PD4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PD4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PD4_A::SD_SHE),
            1 => Val(PD4_A::SD_MEE),
            2 => Val(PD4_A::SD_SOE),
            4 => Val(PD4_A::MD),
            7 => Val(PD4_A::WD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD4_A::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD4_A::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD4_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD4_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD4_A::WD
    }
}
#[doc = "Write proxy for field `PD4`"]
pub struct PD4_W<'a> {
    w: &'a mut W,
}
impl<'a> PD4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD4_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD4_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD4_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut W {
        self.variant(PD4_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD4_A::WD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.5\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD5_A {
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
impl From<PD5_A> for u8 {
    #[inline(always)]
    fn from(variant: PD5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PD5`"]
pub type PD5_R = crate::R<u8, PD5_A>;
impl PD5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PD5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PD5_A::SD_SHE),
            1 => Val(PD5_A::SD_MEE),
            2 => Val(PD5_A::SD_SOE),
            4 => Val(PD5_A::MD),
            7 => Val(PD5_A::WD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD5_A::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD5_A::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD5_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD5_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD5_A::WD
    }
}
#[doc = "Write proxy for field `PD5`"]
pub struct PD5_W<'a> {
    w: &'a mut W,
}
impl<'a> PD5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD5_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD5_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD5_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut W {
        self.variant(PD5_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD5_A::WD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.6\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD6_A {
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
impl From<PD6_A> for u8 {
    #[inline(always)]
    fn from(variant: PD6_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PD6`"]
pub type PD6_R = crate::R<u8, PD6_A>;
impl PD6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PD6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PD6_A::SD_SHE),
            1 => Val(PD6_A::SD_MEE),
            2 => Val(PD6_A::SD_SOE),
            4 => Val(PD6_A::MD),
            7 => Val(PD6_A::WD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD6_A::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD6_A::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD6_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD6_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD6_A::WD
    }
}
#[doc = "Write proxy for field `PD6`"]
pub struct PD6_W<'a> {
    w: &'a mut W,
}
impl<'a> PD6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD6_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD6_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD6_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut W {
        self.variant(PD6_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD6_A::WD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.7\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD7_A {
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
impl From<PD7_A> for u8 {
    #[inline(always)]
    fn from(variant: PD7_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PD7`"]
pub type PD7_R = crate::R<u8, PD7_A>;
impl PD7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PD7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PD7_A::SD_SHE),
            1 => Val(PD7_A::SD_MEE),
            2 => Val(PD7_A::SD_SOE),
            4 => Val(PD7_A::MD),
            7 => Val(PD7_A::WD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD7_A::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD7_A::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD7_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD7_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD7_A::WD
    }
}
#[doc = "Write proxy for field `PD7`"]
pub struct PD7_W<'a> {
    w: &'a mut W,
}
impl<'a> PD7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD7_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD7_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD7_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut W {
        self.variant(PD7_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD7_A::WD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.0"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.1"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.2"]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.3"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.4"]
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.5"]
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.6"]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.7"]
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.0"]
    #[inline(always)]
    pub fn pd0(&mut self) -> PD0_W {
        PD0_W { w: self }
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.1"]
    #[inline(always)]
    pub fn pd1(&mut self) -> PD1_W {
        PD1_W { w: self }
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.2"]
    #[inline(always)]
    pub fn pd2(&mut self) -> PD2_W {
        PD2_W { w: self }
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.3"]
    #[inline(always)]
    pub fn pd3(&mut self) -> PD3_W {
        PD3_W { w: self }
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.4"]
    #[inline(always)]
    pub fn pd4(&mut self) -> PD4_W {
        PD4_W { w: self }
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.5"]
    #[inline(always)]
    pub fn pd5(&mut self) -> PD5_W {
        PD5_W { w: self }
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.6"]
    #[inline(always)]
    pub fn pd6(&mut self) -> PD6_W {
        PD6_W { w: self }
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.7"]
    #[inline(always)]
    pub fn pd7(&mut self) -> PD7_W {
        PD7_W { w: self }
    }
}
