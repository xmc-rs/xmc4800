#[doc = "Register `PDR0` reader"]
pub type R = crate::R<Pdr0Spec>;
#[doc = "Register `PDR0` writer"]
pub type W = crate::W<Pdr0Spec>;
#[doc = "Pad Driver Mode for Pn.0\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pd0 {
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
impl From<Pd0> for u8 {
    #[inline(always)]
    fn from(variant: Pd0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pd0 {
    type Ux = u8;
}
impl crate::IsEnum for Pd0 {}
#[doc = "Field `PD0` reader - Pad Driver Mode for Pn.0"]
pub type Pd0R = crate::FieldReader<Pd0>;
impl Pd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pd0> {
        match self.bits {
            0 => Some(Pd0::SdShe),
            1 => Some(Pd0::SdMee),
            2 => Some(Pd0::SdSoe),
            4 => Some(Pd0::Md),
            7 => Some(Pd0::Wd),
            _ => None,
        }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == Pd0::SdShe
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == Pd0::SdMee
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == Pd0::SdSoe
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == Pd0::Md
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == Pd0::Wd
    }
}
#[doc = "Field `PD0` writer - Pad Driver Mode for Pn.0"]
pub type Pd0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pd0>;
impl<'a, REG> Pd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut crate::W<REG> {
        self.variant(Pd0::SdShe)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut crate::W<REG> {
        self.variant(Pd0::SdMee)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(Pd0::SdSoe)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(Pd0::Md)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(Pd0::Wd)
    }
}
#[doc = "Pad Driver Mode for Pn.1\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pd1 {
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
impl From<Pd1> for u8 {
    #[inline(always)]
    fn from(variant: Pd1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pd1 {
    type Ux = u8;
}
impl crate::IsEnum for Pd1 {}
#[doc = "Field `PD1` reader - Pad Driver Mode for Pn.1"]
pub type Pd1R = crate::FieldReader<Pd1>;
impl Pd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pd1> {
        match self.bits {
            0 => Some(Pd1::SdShe),
            1 => Some(Pd1::SdMee),
            2 => Some(Pd1::SdSoe),
            4 => Some(Pd1::Md),
            7 => Some(Pd1::Wd),
            _ => None,
        }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn is_sd_she(&self) -> bool {
        *self == Pd1::SdShe
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn is_sd_mee(&self) -> bool {
        *self == Pd1::SdMee
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == Pd1::SdSoe
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == Pd1::Md
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == Pd1::Wd
    }
}
#[doc = "Field `PD1` writer - Pad Driver Mode for Pn.1"]
pub type Pd1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pd1>;
impl<'a, REG> Pd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A2 strong driver, sharp edge"]
    #[inline(always)]
    pub fn sd_she(self) -> &'a mut crate::W<REG> {
        self.variant(Pd1::SdShe)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline(always)]
    pub fn sd_mee(self) -> &'a mut crate::W<REG> {
        self.variant(Pd1::SdMee)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(Pd1::SdSoe)
    }
    #[doc = "A2 medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(Pd1::Md)
    }
    #[doc = "A2 weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(Pd1::Wd)
    }
}
#[doc = "Pad Driver Mode for Pn.2\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pd2 {
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
impl From<Pd2> for u8 {
    #[inline(always)]
    fn from(variant: Pd2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pd2 {
    type Ux = u8;
}
impl crate::IsEnum for Pd2 {}
#[doc = "Field `PD2` reader - Pad Driver Mode for Pn.2"]
pub type Pd2R = crate::FieldReader<Pd2>;
impl Pd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pd2 {
        match self.bits {
            2 => Pd2::SdSoe,
            3 => Pd2::SdSle,
            4 => Pd2::Md,
            7 => Pd2::Wd,
            0 => Pd2::SdSoeAlt,
            1 => Pd2::SdSleAlt,
            6 => Pd2::MdAlt,
            5 => Pd2::WdAlt,
            _ => unreachable!(),
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == Pd2::SdSoe
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == Pd2::SdSle
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == Pd2::Md
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == Pd2::Wd
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == Pd2::SdSoeAlt
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == Pd2::SdSleAlt
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == Pd2::MdAlt
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == Pd2::WdAlt
    }
}
#[doc = "Field `PD2` writer - Pad Driver Mode for Pn.2"]
pub type Pd2W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pd2, crate::Safe>;
impl<'a, REG> Pd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(Pd2::SdSoe)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut crate::W<REG> {
        self.variant(Pd2::SdSle)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(Pd2::Md)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(Pd2::Wd)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd2::SdSoeAlt)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd2::SdSleAlt)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd2::MdAlt)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd2::WdAlt)
    }
}
#[doc = "Pad Driver Mode for Pn.3\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pd3 {
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
impl From<Pd3> for u8 {
    #[inline(always)]
    fn from(variant: Pd3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pd3 {
    type Ux = u8;
}
impl crate::IsEnum for Pd3 {}
#[doc = "Field `PD3` reader - Pad Driver Mode for Pn.3"]
pub type Pd3R = crate::FieldReader<Pd3>;
impl Pd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pd3 {
        match self.bits {
            2 => Pd3::SdSoe,
            3 => Pd3::SdSle,
            4 => Pd3::Md,
            7 => Pd3::Wd,
            0 => Pd3::SdSoeAlt,
            1 => Pd3::SdSleAlt,
            6 => Pd3::MdAlt,
            5 => Pd3::WdAlt,
            _ => unreachable!(),
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == Pd3::SdSoe
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == Pd3::SdSle
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == Pd3::Md
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == Pd3::Wd
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == Pd3::SdSoeAlt
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == Pd3::SdSleAlt
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == Pd3::MdAlt
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == Pd3::WdAlt
    }
}
#[doc = "Field `PD3` writer - Pad Driver Mode for Pn.3"]
pub type Pd3W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pd3, crate::Safe>;
impl<'a, REG> Pd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(Pd3::SdSoe)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut crate::W<REG> {
        self.variant(Pd3::SdSle)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(Pd3::Md)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(Pd3::Wd)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd3::SdSoeAlt)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd3::SdSleAlt)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd3::MdAlt)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd3::WdAlt)
    }
}
#[doc = "Pad Driver Mode for Pn.4\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pd4 {
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
impl From<Pd4> for u8 {
    #[inline(always)]
    fn from(variant: Pd4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pd4 {
    type Ux = u8;
}
impl crate::IsEnum for Pd4 {}
#[doc = "Field `PD4` reader - Pad Driver Mode for Pn.4"]
pub type Pd4R = crate::FieldReader<Pd4>;
impl Pd4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pd4 {
        match self.bits {
            2 => Pd4::SdSoe,
            3 => Pd4::SdSle,
            4 => Pd4::Md,
            7 => Pd4::Wd,
            0 => Pd4::SdSoeAlt,
            1 => Pd4::SdSleAlt,
            6 => Pd4::MdAlt,
            5 => Pd4::WdAlt,
            _ => unreachable!(),
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == Pd4::SdSoe
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == Pd4::SdSle
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == Pd4::Md
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == Pd4::Wd
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == Pd4::SdSoeAlt
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == Pd4::SdSleAlt
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == Pd4::MdAlt
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == Pd4::WdAlt
    }
}
#[doc = "Field `PD4` writer - Pad Driver Mode for Pn.4"]
pub type Pd4W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pd4, crate::Safe>;
impl<'a, REG> Pd4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(Pd4::SdSoe)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut crate::W<REG> {
        self.variant(Pd4::SdSle)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(Pd4::Md)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(Pd4::Wd)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd4::SdSoeAlt)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd4::SdSleAlt)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd4::MdAlt)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd4::WdAlt)
    }
}
#[doc = "Pad Driver Mode for Pn.5\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pd5 {
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
impl From<Pd5> for u8 {
    #[inline(always)]
    fn from(variant: Pd5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pd5 {
    type Ux = u8;
}
impl crate::IsEnum for Pd5 {}
#[doc = "Field `PD5` reader - Pad Driver Mode for Pn.5"]
pub type Pd5R = crate::FieldReader<Pd5>;
impl Pd5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pd5 {
        match self.bits {
            2 => Pd5::SdSoe,
            3 => Pd5::SdSle,
            4 => Pd5::Md,
            7 => Pd5::Wd,
            0 => Pd5::SdSoeAlt,
            1 => Pd5::SdSleAlt,
            6 => Pd5::MdAlt,
            5 => Pd5::WdAlt,
            _ => unreachable!(),
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == Pd5::SdSoe
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == Pd5::SdSle
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == Pd5::Md
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == Pd5::Wd
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == Pd5::SdSoeAlt
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == Pd5::SdSleAlt
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == Pd5::MdAlt
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == Pd5::WdAlt
    }
}
#[doc = "Field `PD5` writer - Pad Driver Mode for Pn.5"]
pub type Pd5W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pd5, crate::Safe>;
impl<'a, REG> Pd5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(Pd5::SdSoe)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut crate::W<REG> {
        self.variant(Pd5::SdSle)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(Pd5::Md)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(Pd5::Wd)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd5::SdSoeAlt)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd5::SdSleAlt)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd5::MdAlt)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd5::WdAlt)
    }
}
#[doc = "Pad Driver Mode for Pn.6\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pd6 {
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
impl From<Pd6> for u8 {
    #[inline(always)]
    fn from(variant: Pd6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pd6 {
    type Ux = u8;
}
impl crate::IsEnum for Pd6 {}
#[doc = "Field `PD6` reader - Pad Driver Mode for Pn.6"]
pub type Pd6R = crate::FieldReader<Pd6>;
impl Pd6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pd6 {
        match self.bits {
            2 => Pd6::SdSoe,
            3 => Pd6::SdSle,
            4 => Pd6::Md,
            7 => Pd6::Wd,
            0 => Pd6::SdSoeAlt,
            1 => Pd6::SdSleAlt,
            6 => Pd6::MdAlt,
            5 => Pd6::WdAlt,
            _ => unreachable!(),
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == Pd6::SdSoe
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == Pd6::SdSle
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == Pd6::Md
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == Pd6::Wd
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == Pd6::SdSoeAlt
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == Pd6::SdSleAlt
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == Pd6::MdAlt
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == Pd6::WdAlt
    }
}
#[doc = "Field `PD6` writer - Pad Driver Mode for Pn.6"]
pub type Pd6W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pd6, crate::Safe>;
impl<'a, REG> Pd6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(Pd6::SdSoe)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut crate::W<REG> {
        self.variant(Pd6::SdSle)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(Pd6::Md)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(Pd6::Wd)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd6::SdSoeAlt)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd6::SdSleAlt)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd6::MdAlt)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd6::WdAlt)
    }
}
#[doc = "Pad Driver Mode for Pn.7\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pd7 {
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
impl From<Pd7> for u8 {
    #[inline(always)]
    fn from(variant: Pd7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pd7 {
    type Ux = u8;
}
impl crate::IsEnum for Pd7 {}
#[doc = "Field `PD7` reader - Pad Driver Mode for Pn.7"]
pub type Pd7R = crate::FieldReader<Pd7>;
impl Pd7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pd7 {
        match self.bits {
            2 => Pd7::SdSoe,
            3 => Pd7::SdSle,
            4 => Pd7::Md,
            7 => Pd7::Wd,
            0 => Pd7::SdSoeAlt,
            1 => Pd7::SdSleAlt,
            6 => Pd7::MdAlt,
            5 => Pd7::WdAlt,
            _ => unreachable!(),
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn is_sd_soe(&self) -> bool {
        *self == Pd7::SdSoe
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn is_sd_sle(&self) -> bool {
        *self == Pd7::SdSle
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn is_md(&self) -> bool {
        *self == Pd7::Md
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn is_wd(&self) -> bool {
        *self == Pd7::Wd
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == Pd7::SdSoeAlt
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == Pd7::SdSleAlt
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn is_md_alt(&self) -> bool {
        *self == Pd7::MdAlt
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn is_wd_alt(&self) -> bool {
        *self == Pd7::WdAlt
    }
}
#[doc = "Field `PD7` writer - Pad Driver Mode for Pn.7"]
pub type Pd7W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pd7, crate::Safe>;
impl<'a, REG> Pd7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A1+ strong driver, soft edge"]
    #[inline(always)]
    pub fn sd_soe(self) -> &'a mut crate::W<REG> {
        self.variant(Pd7::SdSoe)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline(always)]
    pub fn sd_sle(self) -> &'a mut crate::W<REG> {
        self.variant(Pd7::SdSle)
    }
    #[doc = "A1+ medium driver"]
    #[inline(always)]
    pub fn md(self) -> &'a mut crate::W<REG> {
        self.variant(Pd7::Md)
    }
    #[doc = "A1+ weak driver"]
    #[inline(always)]
    pub fn wd(self) -> &'a mut crate::W<REG> {
        self.variant(Pd7::Wd)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline(always)]
    pub fn sd_soe_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd7::SdSoeAlt)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline(always)]
    pub fn sd_sle_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd7::SdSleAlt)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline(always)]
    pub fn md_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd7::MdAlt)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline(always)]
    pub fn wd_alt(self) -> &'a mut crate::W<REG> {
        self.variant(Pd7::WdAlt)
    }
}
impl R {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.0"]
    #[inline(always)]
    pub fn pd0(&self) -> Pd0R {
        Pd0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.1"]
    #[inline(always)]
    pub fn pd1(&self) -> Pd1R {
        Pd1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.2"]
    #[inline(always)]
    pub fn pd2(&self) -> Pd2R {
        Pd2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.3"]
    #[inline(always)]
    pub fn pd3(&self) -> Pd3R {
        Pd3R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.4"]
    #[inline(always)]
    pub fn pd4(&self) -> Pd4R {
        Pd4R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.5"]
    #[inline(always)]
    pub fn pd5(&self) -> Pd5R {
        Pd5R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.6"]
    #[inline(always)]
    pub fn pd6(&self) -> Pd6R {
        Pd6R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.7"]
    #[inline(always)]
    pub fn pd7(&self) -> Pd7R {
        Pd7R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.0"]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> Pd0W<Pdr0Spec> {
        Pd0W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.1"]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> Pd1W<Pdr0Spec> {
        Pd1W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.2"]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> Pd2W<Pdr0Spec> {
        Pd2W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.3"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> Pd3W<Pdr0Spec> {
        Pd3W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.4"]
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> Pd4W<Pdr0Spec> {
        Pd4W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.5"]
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> Pd5W<Pdr0Spec> {
        Pd5W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.6"]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> Pd6W<Pdr0Spec> {
        Pd6W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.7"]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> Pd7W<Pdr0Spec> {
        Pd7W::new(self, 28)
    }
}
#[doc = "Port 4 Pad Driver Mode 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdr0Spec;
impl crate::RegisterSpec for Pdr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdr0::R`](R) reader structure"]
impl crate::Readable for Pdr0Spec {}
#[doc = "`write(|w| ..)` method takes [`pdr0::W`](W) writer structure"]
impl crate::Writable for Pdr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDR0 to value 0x2222_2222"]
impl crate::Resettable for Pdr0Spec {
    const RESET_VALUE: u32 = 0x2222_2222;
}
