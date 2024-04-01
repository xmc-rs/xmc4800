#[doc = "Register `PDR1` reader"]
pub type R = crate::R<Pdr1Spec>;
#[doc = "Register `PDR1` writer"]
pub type W = crate::W<Pdr1Spec>;
#[doc = "Pad Driver Mode for Pn.8\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pd8 {
    #[doc = "0: A2 strong driver, sharp edge"]
    SdShe = 0,
    #[doc = "1: A2 strong driver, medium edge"]
    SdMee = 1,
    #[doc = "2: A2 strong driver, soft edge"]
    SdSoe = 2,
    #[doc = "4: A2 medium driver"]
    Md = 4,
    #[doc = "7: A2 weak driver"]
    Wd = 7,
}
impl From<Pd8> for u8 {
    #[inline(always)]
    fn from(variant: Pd8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pd8 {
    type Ux = u8;
}
impl crate::IsEnum for Pd8 {}
#[doc = "Field `PD8` reader - Pad Driver Mode for Pn.8"]
pub type Pd8R = crate::FieldReader<Pd8>;
impl Pd8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pd8> {
        match self.bits {
            0 => Some(Pd8::SdShe),
            1 => Some(Pd8::SdMee),
            2 => Some(Pd8::SdSoe),
            4 => Some(Pd8::Md),
            7 => Some(Pd8::Wd),
            _ => None,
        }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == Pd8::SdShe
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == Pd8::SdMee
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == Pd8::SdSoe
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == Pd8::Md
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == Pd8::Wd
    }
}
#[doc = "Field `PD8` writer - Pad Driver Mode for Pn.8"]
pub type Pd8W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pd8>;
impl<'a, REG> Pd8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut crate::W<REG> {
        self.variant(Pd8::SdShe)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut crate::W<REG> {
        self.variant(Pd8::SdMee)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(Pd8::SdSoe)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(Pd8::Md)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(Pd8::Wd)
    }
}
#[doc = "Pad Driver Mode for Pn.9\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pd9 {
    #[doc = "0: A2 strong driver, sharp edge"]
    SdShe = 0,
    #[doc = "1: A2 strong driver, medium edge"]
    SdMee = 1,
    #[doc = "2: A2 strong driver, soft edge"]
    SdSoe = 2,
    #[doc = "4: A2 medium driver"]
    Md = 4,
    #[doc = "7: A2 weak driver"]
    Wd = 7,
}
impl From<Pd9> for u8 {
    #[inline(always)]
    fn from(variant: Pd9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pd9 {
    type Ux = u8;
}
impl crate::IsEnum for Pd9 {}
#[doc = "Field `PD9` reader - Pad Driver Mode for Pn.9"]
pub type Pd9R = crate::FieldReader<Pd9>;
impl Pd9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pd9> {
        match self.bits {
            0 => Some(Pd9::SdShe),
            1 => Some(Pd9::SdMee),
            2 => Some(Pd9::SdSoe),
            4 => Some(Pd9::Md),
            7 => Some(Pd9::Wd),
            _ => None,
        }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == Pd9::SdShe
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == Pd9::SdMee
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == Pd9::SdSoe
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == Pd9::Md
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == Pd9::Wd
    }
}
#[doc = "Field `PD9` writer - Pad Driver Mode for Pn.9"]
pub type Pd9W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pd9>;
impl<'a, REG> Pd9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut crate::W<REG> {
        self.variant(Pd9::SdShe)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut crate::W<REG> {
        self.variant(Pd9::SdMee)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(Pd9::SdSoe)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(Pd9::Md)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(Pd9::Wd)
    }
}
#[doc = "Pad Driver Mode for Pn.10\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pd10 {
    #[doc = "2: A1+ strong driver, soft edge"]
    SdSoe = 2,
    #[doc = "3: A1+ strong driver, slow edge"]
    SdSle = 3,
    #[doc = "4: A1+ medium driver"]
    Md = 4,
    #[doc = "7: A1+ weak driver"]
    Wd = 7,
    #[doc = "0: A1+ strong driver, soft edge (alternate value)"]
    SdSoeAlt = 0,
    #[doc = "1: A1+ strong driver, slow edge (alternate value)"]
    SdSleAlt = 1,
    #[doc = "6: A1+ medium driver (alternate value)"]
    MdAlt = 6,
    #[doc = "5: A1+ weak driver (alternate value)"]
    WdAlt = 5,
}
impl From<Pd10> for u8 {
    #[inline(always)]
    fn from(variant: Pd10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pd10 {
    type Ux = u8;
}
impl crate::IsEnum for Pd10 {}
#[doc = "Field `PD10` reader - Pad Driver Mode for Pn.10"]
pub type Pd10R = crate::FieldReader<Pd10>;
impl Pd10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pd10 {
        match self.bits {
            2 => Pd10::SdSoe,
            3 => Pd10::SdSle,
            4 => Pd10::Md,
            7 => Pd10::Wd,
            0 => Pd10::SdSoeAlt,
            1 => Pd10::SdSleAlt,
            6 => Pd10::MdAlt,
            5 => Pd10::WdAlt,
            _ => unreachable!(),
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == Pd10::SdSoe
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == Pd10::SdSle
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == Pd10::Md
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == Pd10::Wd
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == Pd10::SdSoeAlt
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == Pd10::SdSleAlt
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == Pd10::MdAlt
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == Pd10::WdAlt
    }
}
#[doc = "Field `PD10` writer - Pad Driver Mode for Pn.10"]
pub type Pd10W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pd10, crate::Safe>;
impl<'a, REG> Pd10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(Pd10::SdSoe)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut crate::W<REG> {
        self.variant(Pd10::SdSle)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(Pd10::Md)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(Pd10::Wd)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd10::SdSoeAlt)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd10::SdSleAlt)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd10::MdAlt)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd10::WdAlt)
    }
}
#[doc = "Pad Driver Mode for Pn.11\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pd11 {
    #[doc = "2: A1+ strong driver, soft edge"]
    SdSoe = 2,
    #[doc = "3: A1+ strong driver, slow edge"]
    SdSle = 3,
    #[doc = "4: A1+ medium driver"]
    Md = 4,
    #[doc = "7: A1+ weak driver"]
    Wd = 7,
    #[doc = "0: A1+ strong driver, soft edge (alternate value)"]
    SdSoeAlt = 0,
    #[doc = "1: A1+ strong driver, slow edge (alternate value)"]
    SdSleAlt = 1,
    #[doc = "6: A1+ medium driver (alternate value)"]
    MdAlt = 6,
    #[doc = "5: A1+ weak driver (alternate value)"]
    WdAlt = 5,
}
impl From<Pd11> for u8 {
    #[inline(always)]
    fn from(variant: Pd11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pd11 {
    type Ux = u8;
}
impl crate::IsEnum for Pd11 {}
#[doc = "Field `PD11` reader - Pad Driver Mode for Pn.11"]
pub type Pd11R = crate::FieldReader<Pd11>;
impl Pd11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pd11 {
        match self.bits {
            2 => Pd11::SdSoe,
            3 => Pd11::SdSle,
            4 => Pd11::Md,
            7 => Pd11::Wd,
            0 => Pd11::SdSoeAlt,
            1 => Pd11::SdSleAlt,
            6 => Pd11::MdAlt,
            5 => Pd11::WdAlt,
            _ => unreachable!(),
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == Pd11::SdSoe
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == Pd11::SdSle
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == Pd11::Md
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == Pd11::Wd
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == Pd11::SdSoeAlt
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == Pd11::SdSleAlt
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == Pd11::MdAlt
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == Pd11::WdAlt
    }
}
#[doc = "Field `PD11` writer - Pad Driver Mode for Pn.11"]
pub type Pd11W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pd11, crate::Safe>;
impl<'a, REG> Pd11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(Pd11::SdSoe)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut crate::W<REG> {
        self.variant(Pd11::SdSle)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(Pd11::Md)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(Pd11::Wd)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd11::SdSoeAlt)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd11::SdSleAlt)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd11::MdAlt)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd11::WdAlt)
    }
}
#[doc = "Pad Driver Mode for Pn.12\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pd12 {
    #[doc = "2: A1+ strong driver, soft edge"]
    SdSoe = 2,
    #[doc = "3: A1+ strong driver, slow edge"]
    SdSle = 3,
    #[doc = "4: A1+ medium driver"]
    Md = 4,
    #[doc = "7: A1+ weak driver"]
    Wd = 7,
    #[doc = "0: A1+ strong driver, soft edge (alternate value)"]
    SdSoeAlt = 0,
    #[doc = "1: A1+ strong driver, slow edge (alternate value)"]
    SdSleAlt = 1,
    #[doc = "6: A1+ medium driver (alternate value)"]
    MdAlt = 6,
    #[doc = "5: A1+ weak driver (alternate value)"]
    WdAlt = 5,
}
impl From<Pd12> for u8 {
    #[inline(always)]
    fn from(variant: Pd12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pd12 {
    type Ux = u8;
}
impl crate::IsEnum for Pd12 {}
#[doc = "Field `PD12` reader - Pad Driver Mode for Pn.12"]
pub type Pd12R = crate::FieldReader<Pd12>;
impl Pd12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pd12 {
        match self.bits {
            2 => Pd12::SdSoe,
            3 => Pd12::SdSle,
            4 => Pd12::Md,
            7 => Pd12::Wd,
            0 => Pd12::SdSoeAlt,
            1 => Pd12::SdSleAlt,
            6 => Pd12::MdAlt,
            5 => Pd12::WdAlt,
            _ => unreachable!(),
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == Pd12::SdSoe
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == Pd12::SdSle
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == Pd12::Md
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == Pd12::Wd
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == Pd12::SdSoeAlt
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == Pd12::SdSleAlt
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == Pd12::MdAlt
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == Pd12::WdAlt
    }
}
#[doc = "Field `PD12` writer - Pad Driver Mode for Pn.12"]
pub type Pd12W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pd12, crate::Safe>;
impl<'a, REG> Pd12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(Pd12::SdSoe)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut crate::W<REG> {
        self.variant(Pd12::SdSle)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(Pd12::Md)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(Pd12::Wd)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd12::SdSoeAlt)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd12::SdSleAlt)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd12::MdAlt)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd12::WdAlt)
    }
}
#[doc = "Pad Driver Mode for Pn.13\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pd13 {
    #[doc = "2: A1+ strong driver, soft edge"]
    SdSoe = 2,
    #[doc = "3: A1+ strong driver, slow edge"]
    SdSle = 3,
    #[doc = "4: A1+ medium driver"]
    Md = 4,
    #[doc = "7: A1+ weak driver"]
    Wd = 7,
    #[doc = "0: A1+ strong driver, soft edge (alternate value)"]
    SdSoeAlt = 0,
    #[doc = "1: A1+ strong driver, slow edge (alternate value)"]
    SdSleAlt = 1,
    #[doc = "6: A1+ medium driver (alternate value)"]
    MdAlt = 6,
    #[doc = "5: A1+ weak driver (alternate value)"]
    WdAlt = 5,
}
impl From<Pd13> for u8 {
    #[inline(always)]
    fn from(variant: Pd13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pd13 {
    type Ux = u8;
}
impl crate::IsEnum for Pd13 {}
#[doc = "Field `PD13` reader - Pad Driver Mode for Pn.13"]
pub type Pd13R = crate::FieldReader<Pd13>;
impl Pd13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pd13 {
        match self.bits {
            2 => Pd13::SdSoe,
            3 => Pd13::SdSle,
            4 => Pd13::Md,
            7 => Pd13::Wd,
            0 => Pd13::SdSoeAlt,
            1 => Pd13::SdSleAlt,
            6 => Pd13::MdAlt,
            5 => Pd13::WdAlt,
            _ => unreachable!(),
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == Pd13::SdSoe
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == Pd13::SdSle
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == Pd13::Md
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == Pd13::Wd
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == Pd13::SdSoeAlt
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == Pd13::SdSleAlt
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == Pd13::MdAlt
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == Pd13::WdAlt
    }
}
#[doc = "Field `PD13` writer - Pad Driver Mode for Pn.13"]
pub type Pd13W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pd13, crate::Safe>;
impl<'a, REG> Pd13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(Pd13::SdSoe)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut crate::W<REG> {
        self.variant(Pd13::SdSle)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(Pd13::Md)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(Pd13::Wd)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd13::SdSoeAlt)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd13::SdSleAlt)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd13::MdAlt)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd13::WdAlt)
    }
}
#[doc = "Pad Driver Mode for Pn.14\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pd14 {
    #[doc = "2: A1+ strong driver, soft edge"]
    SdSoe = 2,
    #[doc = "3: A1+ strong driver, slow edge"]
    SdSle = 3,
    #[doc = "4: A1+ medium driver"]
    Md = 4,
    #[doc = "7: A1+ weak driver"]
    Wd = 7,
    #[doc = "0: A1+ strong driver, soft edge (alternate value)"]
    SdSoeAlt = 0,
    #[doc = "1: A1+ strong driver, slow edge (alternate value)"]
    SdSleAlt = 1,
    #[doc = "6: A1+ medium driver (alternate value)"]
    MdAlt = 6,
    #[doc = "5: A1+ weak driver (alternate value)"]
    WdAlt = 5,
}
impl From<Pd14> for u8 {
    #[inline(always)]
    fn from(variant: Pd14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pd14 {
    type Ux = u8;
}
impl crate::IsEnum for Pd14 {}
#[doc = "Field `PD14` reader - Pad Driver Mode for Pn.14"]
pub type Pd14R = crate::FieldReader<Pd14>;
impl Pd14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pd14 {
        match self.bits {
            2 => Pd14::SdSoe,
            3 => Pd14::SdSle,
            4 => Pd14::Md,
            7 => Pd14::Wd,
            0 => Pd14::SdSoeAlt,
            1 => Pd14::SdSleAlt,
            6 => Pd14::MdAlt,
            5 => Pd14::WdAlt,
            _ => unreachable!(),
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == Pd14::SdSoe
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == Pd14::SdSle
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == Pd14::Md
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == Pd14::Wd
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == Pd14::SdSoeAlt
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == Pd14::SdSleAlt
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == Pd14::MdAlt
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == Pd14::WdAlt
    }
}
#[doc = "Field `PD14` writer - Pad Driver Mode for Pn.14"]
pub type Pd14W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pd14, crate::Safe>;
impl<'a, REG> Pd14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(Pd14::SdSoe)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut crate::W<REG> {
        self.variant(Pd14::SdSle)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(Pd14::Md)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(Pd14::Wd)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd14::SdSoeAlt)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd14::SdSleAlt)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd14::MdAlt)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd14::WdAlt)
    }
}
#[doc = "Pad Driver Mode for Pn.15\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pd15 {
    #[doc = "2: A1+ strong driver, soft edge"]
    SdSoe = 2,
    #[doc = "3: A1+ strong driver, slow edge"]
    SdSle = 3,
    #[doc = "4: A1+ medium driver"]
    Md = 4,
    #[doc = "7: A1+ weak driver"]
    Wd = 7,
    #[doc = "0: A1+ strong driver, soft edge (alternate value)"]
    SdSoeAlt = 0,
    #[doc = "1: A1+ strong driver, slow edge (alternate value)"]
    SdSleAlt = 1,
    #[doc = "6: A1+ medium driver (alternate value)"]
    MdAlt = 6,
    #[doc = "5: A1+ weak driver (alternate value)"]
    WdAlt = 5,
}
impl From<Pd15> for u8 {
    #[inline(always)]
    fn from(variant: Pd15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pd15 {
    type Ux = u8;
}
impl crate::IsEnum for Pd15 {}
#[doc = "Field `PD15` reader - Pad Driver Mode for Pn.15"]
pub type Pd15R = crate::FieldReader<Pd15>;
impl Pd15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pd15 {
        match self.bits {
            2 => Pd15::SdSoe,
            3 => Pd15::SdSle,
            4 => Pd15::Md,
            7 => Pd15::Wd,
            0 => Pd15::SdSoeAlt,
            1 => Pd15::SdSleAlt,
            6 => Pd15::MdAlt,
            5 => Pd15::WdAlt,
            _ => unreachable!(),
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == Pd15::SdSoe
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == Pd15::SdSle
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == Pd15::Md
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == Pd15::Wd
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == Pd15::SdSoeAlt
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == Pd15::SdSleAlt
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == Pd15::MdAlt
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == Pd15::WdAlt
    }
}
#[doc = "Field `PD15` writer - Pad Driver Mode for Pn.15"]
pub type Pd15W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pd15, crate::Safe>;
impl<'a, REG> Pd15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(Pd15::SdSoe)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut crate::W<REG> {
        self.variant(Pd15::SdSle)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(Pd15::Md)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(Pd15::Wd)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd15::SdSoeAlt)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd15::SdSleAlt)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd15::MdAlt)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd15::WdAlt)
    }
}
impl R {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.8"]
    #[inline(always)]
    pub fn pd8(&self) -> Pd8R {
        Pd8R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.9"]
    #[inline(always)]
    pub fn pd9(&self) -> Pd9R {
        Pd9R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.10"]
    #[inline(always)]
    pub fn pd10(&self) -> Pd10R {
        Pd10R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.11"]
    #[inline(always)]
    pub fn pd11(&self) -> Pd11R {
        Pd11R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.12"]
    #[inline(always)]
    pub fn pd12(&self) -> Pd12R {
        Pd12R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.13"]
    #[inline(always)]
    pub fn pd13(&self) -> Pd13R {
        Pd13R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.14"]
    #[inline(always)]
    pub fn pd14(&self) -> Pd14R {
        Pd14R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.15"]
    #[inline(always)]
    pub fn pd15(&self) -> Pd15R {
        Pd15R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.8"]
    #[inline(always)]
    #[must_use]
    pub fn pd8(&mut self) -> Pd8W<Pdr1Spec> {
        Pd8W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.9"]
    #[inline(always)]
    #[must_use]
    pub fn pd9(&mut self) -> Pd9W<Pdr1Spec> {
        Pd9W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.10"]
    #[inline(always)]
    #[must_use]
    pub fn pd10(&mut self) -> Pd10W<Pdr1Spec> {
        Pd10W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.11"]
    #[inline(always)]
    #[must_use]
    pub fn pd11(&mut self) -> Pd11W<Pdr1Spec> {
        Pd11W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.12"]
    #[inline(always)]
    #[must_use]
    pub fn pd12(&mut self) -> Pd12W<Pdr1Spec> {
        Pd12W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.13"]
    #[inline(always)]
    #[must_use]
    pub fn pd13(&mut self) -> Pd13W<Pdr1Spec> {
        Pd13W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.14"]
    #[inline(always)]
    #[must_use]
    pub fn pd14(&mut self) -> Pd14W<Pdr1Spec> {
        Pd14W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.15"]
    #[inline(always)]
    #[must_use]
    pub fn pd15(&mut self) -> Pd15W<Pdr1Spec> {
        Pd15W::new(self, 28)
    }
}
#[doc = "Port 0 Pad Driver Mode 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdr1Spec;
impl crate::RegisterSpec for Pdr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdr1::R`](R) reader structure"]
impl crate::Readable for Pdr1Spec {}
#[doc = "`write(|w| ..)` method takes [`pdr1::W`](W) writer structure"]
impl crate::Writable for Pdr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDR1 to value 0x2222_2222"]
impl crate::Resettable for Pdr1Spec {
    const RESET_VALUE: u32 = 0x2222_2222;
}
