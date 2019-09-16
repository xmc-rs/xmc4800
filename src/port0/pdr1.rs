#[doc = "Reader of register PDR1"]
pub type R = crate::R<u32, super::PDR1>;
#[doc = "Writer for register PDR1"]
pub type W = crate::W<u32, super::PDR1>;
#[doc = "Register PDR1 `reset()`'s with value 0x2222_2222"]
impl crate::ResetValue for super::PDR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2222_2222
    }
}
#[doc = "Pad Driver Mode for Pn.8\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD8_A {
    #[doc = "0: A2 strong driver, sharp edge"]
    SD_SHE,
    #[doc = "1: A2 strong driver, medium edge"]
    SD_MEE,
    #[doc = "2: A2 strong driver, soft edge"]
    SD_SOE,
    #[doc = "4: A2 medium driver"]
    MD,
    #[doc = "7: A2 weak driver"]
    WD,
}
impl From<PD8_A> for u8 {
    #[inline(always)]
    fn from(variant: PD8_A) -> Self {
        match variant {
            PD8_A::SD_SHE => 0,
            PD8_A::SD_MEE => 1,
            PD8_A::SD_SOE => 2,
            PD8_A::MD => 4,
            PD8_A::WD => 7,
        }
    }
}
#[doc = "Reader of field `PD8`"]
pub type PD8_R = crate::R<u8, PD8_A>;
impl PD8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PD8_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PD8_A::SD_SHE),
            1 => Val(PD8_A::SD_MEE),
            2 => Val(PD8_A::SD_SOE),
            4 => Val(PD8_A::MD),
            7 => Val(PD8_A::WD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD8_A::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD8_A::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD8_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD8_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD8_A::WD
    }
}
#[doc = "Write proxy for field `PD8`"]
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
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.9\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD9_A {
    #[doc = "0: A2 strong driver, sharp edge"]
    SD_SHE,
    #[doc = "1: A2 strong driver, medium edge"]
    SD_MEE,
    #[doc = "2: A2 strong driver, soft edge"]
    SD_SOE,
    #[doc = "4: A2 medium driver"]
    MD,
    #[doc = "7: A2 weak driver"]
    WD,
}
impl From<PD9_A> for u8 {
    #[inline(always)]
    fn from(variant: PD9_A) -> Self {
        match variant {
            PD9_A::SD_SHE => 0,
            PD9_A::SD_MEE => 1,
            PD9_A::SD_SOE => 2,
            PD9_A::MD => 4,
            PD9_A::WD => 7,
        }
    }
}
#[doc = "Reader of field `PD9`"]
pub type PD9_R = crate::R<u8, PD9_A>;
impl PD9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PD9_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PD9_A::SD_SHE),
            1 => Val(PD9_A::SD_MEE),
            2 => Val(PD9_A::SD_SOE),
            4 => Val(PD9_A::MD),
            7 => Val(PD9_A::WD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD9_A::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD9_A::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD9_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD9_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD9_A::WD
    }
}
#[doc = "Write proxy for field `PD9`"]
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
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.10\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD10_A {
    #[doc = "2: A1+ strong driver, soft edge"]
    SD_SOE,
    #[doc = "3: A1+ strong driver, slow edge"]
    SD_SLE,
    #[doc = "4: A1+ medium driver"]
    MD,
    #[doc = "7: A1+ weak driver"]
    WD,
    #[doc = "0: A1+ strong driver, soft edge (alternate value)"]
    SD_SOE_ALT,
    #[doc = "1: A1+ strong driver, slow edge (alternate value)"]
    SD_SLE_ALT,
    #[doc = "6: A1+ medium driver (alternate value)"]
    MD_ALT,
    #[doc = "5: A1+ weak driver (alternate value)"]
    WD_ALT,
}
impl From<PD10_A> for u8 {
    #[inline(always)]
    fn from(variant: PD10_A) -> Self {
        match variant {
            PD10_A::SD_SOE => 2,
            PD10_A::SD_SLE => 3,
            PD10_A::MD => 4,
            PD10_A::WD => 7,
            PD10_A::SD_SOE_ALT => 0,
            PD10_A::SD_SLE_ALT => 1,
            PD10_A::MD_ALT => 6,
            PD10_A::WD_ALT => 5,
        }
    }
}
#[doc = "Reader of field `PD10`"]
pub type PD10_R = crate::R<u8, PD10_A>;
impl PD10_R {
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
        *self == PD10_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `SD_SLE`"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == PD10_A::SD_SLE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD10_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD10_A::WD
    }
    #[doc = "Checks if the value of the field is `SD_SOE_ALT`"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == PD10_A::SD_SOE_ALT
    }
    #[doc = "Checks if the value of the field is `SD_SLE_ALT`"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == PD10_A::SD_SLE_ALT
    }
    #[doc = "Checks if the value of the field is `MD_ALT`"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == PD10_A::MD_ALT
    }
    #[doc = "Checks if the value of the field is `WD_ALT`"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == PD10_A::WD_ALT
    }
}
#[doc = "Write proxy for field `PD10`"]
pub struct PD10_W<'a> {
    w: &'a mut W,
}
impl<'a> PD10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD10_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.11\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD11_A {
    #[doc = "2: A1+ strong driver, soft edge"]
    SD_SOE,
    #[doc = "3: A1+ strong driver, slow edge"]
    SD_SLE,
    #[doc = "4: A1+ medium driver"]
    MD,
    #[doc = "7: A1+ weak driver"]
    WD,
    #[doc = "0: A1+ strong driver, soft edge (alternate value)"]
    SD_SOE_ALT,
    #[doc = "1: A1+ strong driver, slow edge (alternate value)"]
    SD_SLE_ALT,
    #[doc = "6: A1+ medium driver (alternate value)"]
    MD_ALT,
    #[doc = "5: A1+ weak driver (alternate value)"]
    WD_ALT,
}
impl From<PD11_A> for u8 {
    #[inline(always)]
    fn from(variant: PD11_A) -> Self {
        match variant {
            PD11_A::SD_SOE => 2,
            PD11_A::SD_SLE => 3,
            PD11_A::MD => 4,
            PD11_A::WD => 7,
            PD11_A::SD_SOE_ALT => 0,
            PD11_A::SD_SLE_ALT => 1,
            PD11_A::MD_ALT => 6,
            PD11_A::WD_ALT => 5,
        }
    }
}
#[doc = "Reader of field `PD11`"]
pub type PD11_R = crate::R<u8, PD11_A>;
impl PD11_R {
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
        *self == PD11_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `SD_SLE`"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == PD11_A::SD_SLE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD11_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD11_A::WD
    }
    #[doc = "Checks if the value of the field is `SD_SOE_ALT`"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == PD11_A::SD_SOE_ALT
    }
    #[doc = "Checks if the value of the field is `SD_SLE_ALT`"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == PD11_A::SD_SLE_ALT
    }
    #[doc = "Checks if the value of the field is `MD_ALT`"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == PD11_A::MD_ALT
    }
    #[doc = "Checks if the value of the field is `WD_ALT`"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == PD11_A::WD_ALT
    }
}
#[doc = "Write proxy for field `PD11`"]
pub struct PD11_W<'a> {
    w: &'a mut W,
}
impl<'a> PD11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD11_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.12\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD12_A {
    #[doc = "2: A1+ strong driver, soft edge"]
    SD_SOE,
    #[doc = "3: A1+ strong driver, slow edge"]
    SD_SLE,
    #[doc = "4: A1+ medium driver"]
    MD,
    #[doc = "7: A1+ weak driver"]
    WD,
    #[doc = "0: A1+ strong driver, soft edge (alternate value)"]
    SD_SOE_ALT,
    #[doc = "1: A1+ strong driver, slow edge (alternate value)"]
    SD_SLE_ALT,
    #[doc = "6: A1+ medium driver (alternate value)"]
    MD_ALT,
    #[doc = "5: A1+ weak driver (alternate value)"]
    WD_ALT,
}
impl From<PD12_A> for u8 {
    #[inline(always)]
    fn from(variant: PD12_A) -> Self {
        match variant {
            PD12_A::SD_SOE => 2,
            PD12_A::SD_SLE => 3,
            PD12_A::MD => 4,
            PD12_A::WD => 7,
            PD12_A::SD_SOE_ALT => 0,
            PD12_A::SD_SLE_ALT => 1,
            PD12_A::MD_ALT => 6,
            PD12_A::WD_ALT => 5,
        }
    }
}
#[doc = "Reader of field `PD12`"]
pub type PD12_R = crate::R<u8, PD12_A>;
impl PD12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD12_A {
        match self.bits {
            2 => PD12_A::SD_SOE,
            3 => PD12_A::SD_SLE,
            4 => PD12_A::MD,
            7 => PD12_A::WD,
            0 => PD12_A::SD_SOE_ALT,
            1 => PD12_A::SD_SLE_ALT,
            6 => PD12_A::MD_ALT,
            5 => PD12_A::WD_ALT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD12_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `SD_SLE`"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == PD12_A::SD_SLE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD12_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD12_A::WD
    }
    #[doc = "Checks if the value of the field is `SD_SOE_ALT`"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == PD12_A::SD_SOE_ALT
    }
    #[doc = "Checks if the value of the field is `SD_SLE_ALT`"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == PD12_A::SD_SLE_ALT
    }
    #[doc = "Checks if the value of the field is `MD_ALT`"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == PD12_A::MD_ALT
    }
    #[doc = "Checks if the value of the field is `WD_ALT`"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == PD12_A::WD_ALT
    }
}
#[doc = "Write proxy for field `PD12`"]
pub struct PD12_W<'a> {
    w: &'a mut W,
}
impl<'a> PD12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD12_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD12_A::SD_SOE)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut W {
        self.variant(PD12_A::SD_SLE)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut W {
        self.variant(PD12_A::MD)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD12_A::WD)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut W {
        self.variant(PD12_A::SD_SOE_ALT)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut W {
        self.variant(PD12_A::SD_SLE_ALT)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut W {
        self.variant(PD12_A::MD_ALT)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut W {
        self.variant(PD12_A::WD_ALT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.13\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD13_A {
    #[doc = "2: A1+ strong driver, soft edge"]
    SD_SOE,
    #[doc = "3: A1+ strong driver, slow edge"]
    SD_SLE,
    #[doc = "4: A1+ medium driver"]
    MD,
    #[doc = "7: A1+ weak driver"]
    WD,
    #[doc = "0: A1+ strong driver, soft edge (alternate value)"]
    SD_SOE_ALT,
    #[doc = "1: A1+ strong driver, slow edge (alternate value)"]
    SD_SLE_ALT,
    #[doc = "6: A1+ medium driver (alternate value)"]
    MD_ALT,
    #[doc = "5: A1+ weak driver (alternate value)"]
    WD_ALT,
}
impl From<PD13_A> for u8 {
    #[inline(always)]
    fn from(variant: PD13_A) -> Self {
        match variant {
            PD13_A::SD_SOE => 2,
            PD13_A::SD_SLE => 3,
            PD13_A::MD => 4,
            PD13_A::WD => 7,
            PD13_A::SD_SOE_ALT => 0,
            PD13_A::SD_SLE_ALT => 1,
            PD13_A::MD_ALT => 6,
            PD13_A::WD_ALT => 5,
        }
    }
}
#[doc = "Reader of field `PD13`"]
pub type PD13_R = crate::R<u8, PD13_A>;
impl PD13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD13_A {
        match self.bits {
            2 => PD13_A::SD_SOE,
            3 => PD13_A::SD_SLE,
            4 => PD13_A::MD,
            7 => PD13_A::WD,
            0 => PD13_A::SD_SOE_ALT,
            1 => PD13_A::SD_SLE_ALT,
            6 => PD13_A::MD_ALT,
            5 => PD13_A::WD_ALT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD13_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `SD_SLE`"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == PD13_A::SD_SLE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD13_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD13_A::WD
    }
    #[doc = "Checks if the value of the field is `SD_SOE_ALT`"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == PD13_A::SD_SOE_ALT
    }
    #[doc = "Checks if the value of the field is `SD_SLE_ALT`"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == PD13_A::SD_SLE_ALT
    }
    #[doc = "Checks if the value of the field is `MD_ALT`"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == PD13_A::MD_ALT
    }
    #[doc = "Checks if the value of the field is `WD_ALT`"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == PD13_A::WD_ALT
    }
}
#[doc = "Write proxy for field `PD13`"]
pub struct PD13_W<'a> {
    w: &'a mut W,
}
impl<'a> PD13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD13_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD13_A::SD_SOE)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut W {
        self.variant(PD13_A::SD_SLE)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut W {
        self.variant(PD13_A::MD)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD13_A::WD)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut W {
        self.variant(PD13_A::SD_SOE_ALT)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut W {
        self.variant(PD13_A::SD_SLE_ALT)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut W {
        self.variant(PD13_A::MD_ALT)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut W {
        self.variant(PD13_A::WD_ALT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.14\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD14_A {
    #[doc = "2: A1+ strong driver, soft edge"]
    SD_SOE,
    #[doc = "3: A1+ strong driver, slow edge"]
    SD_SLE,
    #[doc = "4: A1+ medium driver"]
    MD,
    #[doc = "7: A1+ weak driver"]
    WD,
    #[doc = "0: A1+ strong driver, soft edge (alternate value)"]
    SD_SOE_ALT,
    #[doc = "1: A1+ strong driver, slow edge (alternate value)"]
    SD_SLE_ALT,
    #[doc = "6: A1+ medium driver (alternate value)"]
    MD_ALT,
    #[doc = "5: A1+ weak driver (alternate value)"]
    WD_ALT,
}
impl From<PD14_A> for u8 {
    #[inline(always)]
    fn from(variant: PD14_A) -> Self {
        match variant {
            PD14_A::SD_SOE => 2,
            PD14_A::SD_SLE => 3,
            PD14_A::MD => 4,
            PD14_A::WD => 7,
            PD14_A::SD_SOE_ALT => 0,
            PD14_A::SD_SLE_ALT => 1,
            PD14_A::MD_ALT => 6,
            PD14_A::WD_ALT => 5,
        }
    }
}
#[doc = "Reader of field `PD14`"]
pub type PD14_R = crate::R<u8, PD14_A>;
impl PD14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD14_A {
        match self.bits {
            2 => PD14_A::SD_SOE,
            3 => PD14_A::SD_SLE,
            4 => PD14_A::MD,
            7 => PD14_A::WD,
            0 => PD14_A::SD_SOE_ALT,
            1 => PD14_A::SD_SLE_ALT,
            6 => PD14_A::MD_ALT,
            5 => PD14_A::WD_ALT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD14_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `SD_SLE`"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == PD14_A::SD_SLE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD14_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD14_A::WD
    }
    #[doc = "Checks if the value of the field is `SD_SOE_ALT`"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == PD14_A::SD_SOE_ALT
    }
    #[doc = "Checks if the value of the field is `SD_SLE_ALT`"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == PD14_A::SD_SLE_ALT
    }
    #[doc = "Checks if the value of the field is `MD_ALT`"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == PD14_A::MD_ALT
    }
    #[doc = "Checks if the value of the field is `WD_ALT`"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == PD14_A::WD_ALT
    }
}
#[doc = "Write proxy for field `PD14`"]
pub struct PD14_W<'a> {
    w: &'a mut W,
}
impl<'a> PD14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD14_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD14_A::SD_SOE)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut W {
        self.variant(PD14_A::SD_SLE)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut W {
        self.variant(PD14_A::MD)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD14_A::WD)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut W {
        self.variant(PD14_A::SD_SOE_ALT)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut W {
        self.variant(PD14_A::SD_SLE_ALT)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut W {
        self.variant(PD14_A::MD_ALT)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut W {
        self.variant(PD14_A::WD_ALT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Pad Driver Mode for Pn.15\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD15_A {
    #[doc = "2: A1+ strong driver, soft edge"]
    SD_SOE,
    #[doc = "3: A1+ strong driver, slow edge"]
    SD_SLE,
    #[doc = "4: A1+ medium driver"]
    MD,
    #[doc = "7: A1+ weak driver"]
    WD,
    #[doc = "0: A1+ strong driver, soft edge (alternate value)"]
    SD_SOE_ALT,
    #[doc = "1: A1+ strong driver, slow edge (alternate value)"]
    SD_SLE_ALT,
    #[doc = "6: A1+ medium driver (alternate value)"]
    MD_ALT,
    #[doc = "5: A1+ weak driver (alternate value)"]
    WD_ALT,
}
impl From<PD15_A> for u8 {
    #[inline(always)]
    fn from(variant: PD15_A) -> Self {
        match variant {
            PD15_A::SD_SOE => 2,
            PD15_A::SD_SLE => 3,
            PD15_A::MD => 4,
            PD15_A::WD => 7,
            PD15_A::SD_SOE_ALT => 0,
            PD15_A::SD_SLE_ALT => 1,
            PD15_A::MD_ALT => 6,
            PD15_A::WD_ALT => 5,
        }
    }
}
#[doc = "Reader of field `PD15`"]
pub type PD15_R = crate::R<u8, PD15_A>;
impl PD15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD15_A {
        match self.bits {
            2 => PD15_A::SD_SOE,
            3 => PD15_A::SD_SLE,
            4 => PD15_A::MD,
            7 => PD15_A::WD,
            0 => PD15_A::SD_SOE_ALT,
            1 => PD15_A::SD_SLE_ALT,
            6 => PD15_A::MD_ALT,
            5 => PD15_A::WD_ALT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD15_A::SD_SOE
    }
    #[doc = "Checks if the value of the field is `SD_SLE`"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == PD15_A::SD_SLE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD15_A::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD15_A::WD
    }
    #[doc = "Checks if the value of the field is `SD_SOE_ALT`"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == PD15_A::SD_SOE_ALT
    }
    #[doc = "Checks if the value of the field is `SD_SLE_ALT`"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == PD15_A::SD_SLE_ALT
    }
    #[doc = "Checks if the value of the field is `MD_ALT`"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == PD15_A::MD_ALT
    }
    #[doc = "Checks if the value of the field is `WD_ALT`"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == PD15_A::WD_ALT
    }
}
#[doc = "Write proxy for field `PD15`"]
pub struct PD15_W<'a> {
    w: &'a mut W,
}
impl<'a> PD15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD15_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD15_A::SD_SOE)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut W {
        self.variant(PD15_A::SD_SLE)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut W {
        self.variant(PD15_A::MD)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD15_A::WD)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut W {
        self.variant(PD15_A::SD_SOE_ALT)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut W {
        self.variant(PD15_A::SD_SLE_ALT)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut W {
        self.variant(PD15_A::MD_ALT)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut W {
        self.variant(PD15_A::WD_ALT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
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
}
