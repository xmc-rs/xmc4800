#[doc = "Register `PDR0` reader"]
pub type R = crate::R<PDR0_SPEC>;
#[doc = "Register `PDR0` writer"]
pub type W = crate::W<PDR0_SPEC>;
#[doc = "Field `PD0` reader - Pad Driver Mode for Pn.0"]
pub type PD0_R = crate::FieldReader<PD0_A>;
#[doc = "Pad Driver Mode for Pn.0\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for PD0_A {
    type Ux = u8;
}
impl PD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD0_A> {
        match self.bits {
            0 => Some(PD0_A::SD_SHE),
            1 => Some(PD0_A::SD_MEE),
            2 => Some(PD0_A::SD_SOE),
            4 => Some(PD0_A::MD),
            7 => Some(PD0_A::WD),
            _ => None,
        }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD0_A::SD_SHE
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD0_A::SD_MEE
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD0_A::SD_SOE
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD0_A::MD
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD0_A::WD
    }
}
#[doc = "Field `PD0` writer - Pad Driver Mode for Pn.0"]
pub type PD0_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PD0_A>;
impl<'a, REG> PD0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut crate::W<REG> {
        self.variant(PD0_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut crate::W<REG> {
        self.variant(PD0_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(PD0_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(PD0_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(PD0_A::WD)
    }
}
#[doc = "Field `PD1` reader - Pad Driver Mode for Pn.1"]
pub type PD1_R = crate::FieldReader<PD1_A>;
#[doc = "Pad Driver Mode for Pn.1\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for PD1_A {
    type Ux = u8;
}
impl PD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD1_A> {
        match self.bits {
            0 => Some(PD1_A::SD_SHE),
            1 => Some(PD1_A::SD_MEE),
            2 => Some(PD1_A::SD_SOE),
            4 => Some(PD1_A::MD),
            7 => Some(PD1_A::WD),
            _ => None,
        }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD1_A::SD_SHE
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD1_A::SD_MEE
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD1_A::SD_SOE
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD1_A::MD
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD1_A::WD
    }
}
#[doc = "Field `PD1` writer - Pad Driver Mode for Pn.1"]
pub type PD1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PD1_A>;
impl<'a, REG> PD1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut crate::W<REG> {
        self.variant(PD1_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut crate::W<REG> {
        self.variant(PD1_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(PD1_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(PD1_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(PD1_A::WD)
    }
}
#[doc = "Field `PD2` reader - Pad Driver Mode for Pn.2"]
pub type PD2_R = crate::FieldReader<PD2_A>;
#[doc = "Pad Driver Mode for Pn.2\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for PD2_A {
    type Ux = u8;
}
impl PD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD2_A> {
        match self.bits {
            0 => Some(PD2_A::SD_SHE),
            1 => Some(PD2_A::SD_MEE),
            2 => Some(PD2_A::SD_SOE),
            4 => Some(PD2_A::MD),
            7 => Some(PD2_A::WD),
            _ => None,
        }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD2_A::SD_SHE
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD2_A::SD_MEE
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD2_A::SD_SOE
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD2_A::MD
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD2_A::WD
    }
}
#[doc = "Field `PD2` writer - Pad Driver Mode for Pn.2"]
pub type PD2_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PD2_A>;
impl<'a, REG> PD2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut crate::W<REG> {
        self.variant(PD2_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut crate::W<REG> {
        self.variant(PD2_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(PD2_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(PD2_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(PD2_A::WD)
    }
}
#[doc = "Field `PD3` reader - Pad Driver Mode for Pn.3"]
pub type PD3_R = crate::FieldReader<PD3_A>;
#[doc = "Pad Driver Mode for Pn.3\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD3_A {
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
impl From<PD3_A> for u8 {
    #[inline(always)]
    fn from(variant: PD3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD3_A {
    type Ux = u8;
}
impl PD3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD3_A> {
        match self.bits {
            0 => Some(PD3_A::SD_SHE),
            1 => Some(PD3_A::SD_MEE),
            2 => Some(PD3_A::SD_SOE),
            4 => Some(PD3_A::MD),
            7 => Some(PD3_A::WD),
            _ => None,
        }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD3_A::SD_SHE
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD3_A::SD_MEE
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD3_A::SD_SOE
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD3_A::MD
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD3_A::WD
    }
}
#[doc = "Field `PD3` writer - Pad Driver Mode for Pn.3"]
pub type PD3_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PD3_A>;
impl<'a, REG> PD3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut crate::W<REG> {
        self.variant(PD3_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut crate::W<REG> {
        self.variant(PD3_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(PD3_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(PD3_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(PD3_A::WD)
    }
}
#[doc = "Field `PD4` reader - Pad Driver Mode for Pn.4"]
pub type PD4_R = crate::FieldReader<PD4_A>;
#[doc = "Pad Driver Mode for Pn.4\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for PD4_A {
    type Ux = u8;
}
impl PD4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD4_A> {
        match self.bits {
            0 => Some(PD4_A::SD_SHE),
            1 => Some(PD4_A::SD_MEE),
            2 => Some(PD4_A::SD_SOE),
            4 => Some(PD4_A::MD),
            7 => Some(PD4_A::WD),
            _ => None,
        }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD4_A::SD_SHE
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD4_A::SD_MEE
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD4_A::SD_SOE
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD4_A::MD
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD4_A::WD
    }
}
#[doc = "Field `PD4` writer - Pad Driver Mode for Pn.4"]
pub type PD4_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PD4_A>;
impl<'a, REG> PD4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut crate::W<REG> {
        self.variant(PD4_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut crate::W<REG> {
        self.variant(PD4_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(PD4_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(PD4_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(PD4_A::WD)
    }
}
#[doc = "Field `PD5` reader - Pad Driver Mode for Pn.5"]
pub type PD5_R = crate::FieldReader<PD5_A>;
#[doc = "Pad Driver Mode for Pn.5\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for PD5_A {
    type Ux = u8;
}
impl PD5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PD5_A> {
        match self.bits {
            0 => Some(PD5_A::SD_SHE),
            1 => Some(PD5_A::SD_MEE),
            2 => Some(PD5_A::SD_SOE),
            4 => Some(PD5_A::MD),
            7 => Some(PD5_A::WD),
            _ => None,
        }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == PD5_A::SD_SHE
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD5_A::SD_MEE
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD5_A::SD_SOE
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD5_A::MD
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD5_A::WD
    }
}
#[doc = "Field `PD5` writer - Pad Driver Mode for Pn.5"]
pub type PD5_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PD5_A>;
impl<'a, REG> PD5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut crate::W<REG> {
        self.variant(PD5_A::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut crate::W<REG> {
        self.variant(PD5_A::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(PD5_A::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(PD5_A::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(PD5_A::WD)
    }
}
#[doc = "Field `PD6` reader - Pad Driver Mode for Pn.6"]
pub type PD6_R = crate::FieldReader<PD6_A>;
#[doc = "Pad Driver Mode for Pn.6\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD6_A {
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
impl From<PD6_A> for u8 {
    #[inline(always)]
    fn from(variant: PD6_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD6_A {
    type Ux = u8;
}
impl PD6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PD6_A {
        match self.bits {
            2 => PD6_A::SD_SOE,
            3 => PD6_A::SD_SLE,
            4 => PD6_A::MD,
            7 => PD6_A::WD,
            0 => PD6_A::SD_SOE_ALT,
            1 => PD6_A::SD_SLE_ALT,
            6 => PD6_A::MD_ALT,
            5 => PD6_A::WD_ALT,
            _ => unreachable!(),
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD6_A::SD_SOE
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == PD6_A::SD_SLE
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD6_A::MD
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD6_A::WD
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == PD6_A::SD_SOE_ALT
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == PD6_A::SD_SLE_ALT
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == PD6_A::MD_ALT
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == PD6_A::WD_ALT
    }
}
#[doc = "Field `PD6` writer - Pad Driver Mode for Pn.6"]
pub type PD6_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, PD6_A>;
impl<'a, REG> PD6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(PD6_A::SD_SOE)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut crate::W<REG> {
        self.variant(PD6_A::SD_SLE)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(PD6_A::MD)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(PD6_A::WD)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut crate::W<REG> {
        self.variant(PD6_A::SD_SOE_ALT)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut crate::W<REG> {
        self.variant(PD6_A::SD_SLE_ALT)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut crate::W<REG> {
        self.variant(PD6_A::MD_ALT)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut crate::W<REG> {
        self.variant(PD6_A::WD_ALT)
    }
}
#[doc = "Field `PD7` reader - Pad Driver Mode for Pn.7"]
pub type PD7_R = crate::FieldReader<PD7_A>;
#[doc = "Pad Driver Mode for Pn.7\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD7_A {
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
impl From<PD7_A> for u8 {
    #[inline(always)]
    fn from(variant: PD7_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD7_A {
    type Ux = u8;
}
impl PD7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PD7_A {
        match self.bits {
            2 => PD7_A::SD_SOE,
            3 => PD7_A::SD_SLE,
            4 => PD7_A::MD,
            7 => PD7_A::WD,
            0 => PD7_A::SD_SOE_ALT,
            1 => PD7_A::SD_SLE_ALT,
            6 => PD7_A::MD_ALT,
            5 => PD7_A::WD_ALT,
            _ => unreachable!(),
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD7_A::SD_SOE
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == PD7_A::SD_SLE
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == PD7_A::MD
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == PD7_A::WD
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == PD7_A::SD_SOE_ALT
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == PD7_A::SD_SLE_ALT
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == PD7_A::MD_ALT
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == PD7_A::WD_ALT
    }
}
#[doc = "Field `PD7` writer - Pad Driver Mode for Pn.7"]
pub type PD7_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, PD7_A>;
impl<'a, REG> PD7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(PD7_A::SD_SOE)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut crate::W<REG> {
        self.variant(PD7_A::SD_SLE)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(PD7_A::MD)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(PD7_A::WD)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut crate::W<REG> {
        self.variant(PD7_A::SD_SOE_ALT)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut crate::W<REG> {
        self.variant(PD7_A::SD_SLE_ALT)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut crate::W<REG> {
        self.variant(PD7_A::MD_ALT)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut crate::W<REG> {
        self.variant(PD7_A::WD_ALT)
    }
}
impl R {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.0"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.1"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.2"]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.3"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.4"]
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.5"]
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.6"]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.7"]
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.0"]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<PDR0_SPEC> {
        PD0_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.1"]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<PDR0_SPEC> {
        PD1_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.2"]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<PDR0_SPEC> {
        PD2_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.3"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<PDR0_SPEC> {
        PD3_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.4"]
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> PD4_W<PDR0_SPEC> {
        PD4_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.5"]
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> PD5_W<PDR0_SPEC> {
        PD5_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.6"]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> PD6_W<PDR0_SPEC> {
        PD6_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.7"]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> PD7_W<PDR0_SPEC> {
        PD7_W::new(self, 28)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 2 Pad Driver Mode 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDR0_SPEC;
impl crate::RegisterSpec for PDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdr0::R`](R) reader structure"]
impl crate::Readable for PDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pdr0::W`](W) writer structure"]
impl crate::Writable for PDR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDR0 to value 0x2222_2222"]
impl crate::Resettable for PDR0_SPEC {
    const RESET_VALUE: u32 = 0x2222_2222;
}
