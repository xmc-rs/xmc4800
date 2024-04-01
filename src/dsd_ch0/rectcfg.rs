#[doc = "Register `RECTCFG` reader"]
pub type R = crate::R<RectcfgSpec>;
#[doc = "Register `RECTCFG` writer"]
pub type W = crate::W<RectcfgSpec>;
#[doc = "Rectification Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfen {
    #[doc = "0: No rectification, data not altered"]
    Value1 = 0,
    #[doc = "1: Data are rectified according to SGND"]
    Value2 = 1,
}
impl From<Rfen> for bool {
    #[inline(always)]
    fn from(variant: Rfen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFEN` reader - Rectification Enable"]
pub type RfenR = crate::BitReader<Rfen>;
impl RfenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfen {
        match self.bits {
            false => Rfen::Value1,
            true => Rfen::Value2,
        }
    }
    #[doc = "No rectification, data not altered"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rfen::Value1
    }
    #[doc = "Data are rectified according to SGND"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rfen::Value2
    }
}
#[doc = "Field `RFEN` writer - Rectification Enable"]
pub type RfenW<'a, REG> = crate::BitWriter<'a, REG, Rfen>;
impl<'a, REG> RfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rectification, data not altered"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfen::Value1)
    }
    #[doc = "Data are rectified according to SGND"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rfen::Value2)
    }
}
#[doc = "Sign Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ssrc {
    #[doc = "0: On-chip carrier generator"]
    Value1 = 0,
    #[doc = "1: Sign of result of next channel"]
    Value2 = 1,
    #[doc = "2: External sign signal A"]
    Value3 = 2,
    #[doc = "3: External sign signal B"]
    Value4 = 3,
}
impl From<Ssrc> for u8 {
    #[inline(always)]
    fn from(variant: Ssrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ssrc {
    type Ux = u8;
}
impl crate::IsEnum for Ssrc {}
#[doc = "Field `SSRC` reader - Sign Source"]
pub type SsrcR = crate::FieldReader<Ssrc>;
impl SsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssrc {
        match self.bits {
            0 => Ssrc::Value1,
            1 => Ssrc::Value2,
            2 => Ssrc::Value3,
            3 => Ssrc::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "On-chip carrier generator"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ssrc::Value1
    }
    #[doc = "Sign of result of next channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ssrc::Value2
    }
    #[doc = "External sign signal A"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Ssrc::Value3
    }
    #[doc = "External sign signal B"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Ssrc::Value4
    }
}
#[doc = "Field `SSRC` writer - Sign Source"]
pub type SsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ssrc, crate::Safe>;
impl<'a, REG> SsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "On-chip carrier generator"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssrc::Value1)
    }
    #[doc = "Sign of result of next channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ssrc::Value2)
    }
    #[doc = "External sign signal A"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Ssrc::Value3)
    }
    #[doc = "External sign signal B"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Ssrc::Value4)
    }
}
#[doc = "Valid Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdval {
    #[doc = "0: No new result available"]
    Value1 = 0,
    #[doc = "1: Bitfield SDCAP has been updated with a new captured value and has not yet been read"]
    Value2 = 1,
}
impl From<Sdval> for bool {
    #[inline(always)]
    fn from(variant: Sdval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDVAL` reader - Valid Flag"]
pub type SdvalR = crate::BitReader<Sdval>;
impl SdvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdval {
        match self.bits {
            false => Sdval::Value1,
            true => Sdval::Value2,
        }
    }
    #[doc = "No new result available"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sdval::Value1
    }
    #[doc = "Bitfield SDCAP has been updated with a new captured value and has not yet been read"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sdval::Value2
    }
}
#[doc = "Selected Carrier Sign Signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sgncs {
    #[doc = "0: Positive values"]
    Value1 = 0,
    #[doc = "1: Negative values"]
    Value2 = 1,
}
impl From<Sgncs> for bool {
    #[inline(always)]
    fn from(variant: Sgncs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SGNCS` reader - Selected Carrier Sign Signal"]
pub type SgncsR = crate::BitReader<Sgncs>;
impl SgncsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sgncs {
        match self.bits {
            false => Sgncs::Value1,
            true => Sgncs::Value2,
        }
    }
    #[doc = "Positive values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sgncs::Value1
    }
    #[doc = "Negative values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sgncs::Value2
    }
}
#[doc = "Sign Signal Delayed\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sgnd {
    #[doc = "0: Positive values"]
    Value1 = 0,
    #[doc = "1: Negative values"]
    Value2 = 1,
}
impl From<Sgnd> for bool {
    #[inline(always)]
    fn from(variant: Sgnd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SGND` reader - Sign Signal Delayed"]
pub type SgndR = crate::BitReader<Sgnd>;
impl SgndR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sgnd {
        match self.bits {
            false => Sgnd::Value1,
            true => Sgnd::Value2,
        }
    }
    #[doc = "Positive values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sgnd::Value1
    }
    #[doc = "Negative values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sgnd::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Rectification Enable"]
    #[inline(always)]
    pub fn rfen(&self) -> RfenR {
        RfenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Sign Source"]
    #[inline(always)]
    pub fn ssrc(&self) -> SsrcR {
        SsrcR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 15 - Valid Flag"]
    #[inline(always)]
    pub fn sdval(&self) -> SdvalR {
        SdvalR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 30 - Selected Carrier Sign Signal"]
    #[inline(always)]
    pub fn sgncs(&self) -> SgncsR {
        SgncsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Sign Signal Delayed"]
    #[inline(always)]
    pub fn sgnd(&self) -> SgndR {
        SgndR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rectification Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfen(&mut self) -> RfenW<RectcfgSpec> {
        RfenW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Sign Source"]
    #[inline(always)]
    #[must_use]
    pub fn ssrc(&mut self) -> SsrcW<RectcfgSpec> {
        SsrcW::new(self, 4)
    }
}
#[doc = "Rectification Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rectcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rectcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RectcfgSpec;
impl crate::RegisterSpec for RectcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rectcfg::R`](R) reader structure"]
impl crate::Readable for RectcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`rectcfg::W`](W) writer structure"]
impl crate::Writable for RectcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RECTCFG to value 0x8000_0000"]
impl crate::Resettable for RectcfgSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
