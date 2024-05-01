#[doc = "Register `QDC` reader"]
pub type R = crate::R<QdcSpec>;
#[doc = "Register `QDC` writer"]
pub type W = crate::W<QdcSpec>;
#[doc = "Phase A Level selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pals {
    #[doc = "0: Phase A is active HIGH"]
    Value1 = 0,
    #[doc = "1: Phase A is active LOW"]
    Value2 = 1,
}
impl From<Pals> for bool {
    #[inline(always)]
    fn from(variant: Pals) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PALS` reader - Phase A Level selector"]
pub type PalsR = crate::BitReader<Pals>;
impl PalsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pals {
        match self.bits {
            false => Pals::Value1,
            true => Pals::Value2,
        }
    }
    #[doc = "Phase A is active HIGH"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pals::Value1
    }
    #[doc = "Phase A is active LOW"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pals::Value2
    }
}
#[doc = "Field `PALS` writer - Phase A Level selector"]
pub type PalsW<'a, REG> = crate::BitWriter<'a, REG, Pals>;
impl<'a, REG> PalsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Phase A is active HIGH"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pals::Value1)
    }
    #[doc = "Phase A is active LOW"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pals::Value2)
    }
}
#[doc = "Phase B Level selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbls {
    #[doc = "0: Phase B is active HIGH"]
    Value1 = 0,
    #[doc = "1: Phase B is active LOW"]
    Value2 = 1,
}
impl From<Pbls> for bool {
    #[inline(always)]
    fn from(variant: Pbls) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBLS` reader - Phase B Level selector"]
pub type PblsR = crate::BitReader<Pbls>;
impl PblsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbls {
        match self.bits {
            false => Pbls::Value1,
            true => Pbls::Value2,
        }
    }
    #[doc = "Phase B is active HIGH"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pbls::Value1
    }
    #[doc = "Phase B is active LOW"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pbls::Value2
    }
}
#[doc = "Field `PBLS` writer - Phase B Level selector"]
pub type PblsW<'a, REG> = crate::BitWriter<'a, REG, Pbls>;
impl<'a, REG> PblsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Phase B is active HIGH"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pbls::Value1)
    }
    #[doc = "Phase B is active LOW"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pbls::Value2)
    }
}
#[doc = "Phase signals swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phs {
    #[doc = "0: Phase A is the leading signal for clockwise rotation"]
    Value1 = 0,
    #[doc = "1: Phase B is the leading signal for clockwise rotation"]
    Value2 = 1,
}
impl From<Phs> for bool {
    #[inline(always)]
    fn from(variant: Phs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHS` reader - Phase signals swap"]
pub type PhsR = crate::BitReader<Phs>;
impl PhsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Phs {
        match self.bits {
            false => Phs::Value1,
            true => Phs::Value2,
        }
    }
    #[doc = "Phase A is the leading signal for clockwise rotation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Phs::Value1
    }
    #[doc = "Phase B is the leading signal for clockwise rotation"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Phs::Value2
    }
}
#[doc = "Field `PHS` writer - Phase signals swap"]
pub type PhsW<'a, REG> = crate::BitWriter<'a, REG, Phs>;
impl<'a, REG> PhsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Phase A is the leading signal for clockwise rotation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Phs::Value1)
    }
    #[doc = "Phase B is the leading signal for clockwise rotation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Phs::Value2)
    }
}
#[doc = "Index Marker generations control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Icm {
    #[doc = "0: No index marker generation on POSIFx.OUT3"]
    Value1 = 0,
    #[doc = "1: Only first index occurrence generated on POSIFx.OUT3"]
    Value2 = 1,
    #[doc = "2: All index occurrences generated on POSIFx.OUT3"]
    Value3 = 2,
}
impl From<Icm> for u8 {
    #[inline(always)]
    fn from(variant: Icm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Icm {
    type Ux = u8;
}
impl crate::IsEnum for Icm {}
#[doc = "Field `ICM` reader - Index Marker generations control"]
pub type IcmR = crate::FieldReader<Icm>;
impl IcmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Icm> {
        match self.bits {
            0 => Some(Icm::Value1),
            1 => Some(Icm::Value2),
            2 => Some(Icm::Value3),
            _ => None,
        }
    }
    #[doc = "No index marker generation on POSIFx.OUT3"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Icm::Value1
    }
    #[doc = "Only first index occurrence generated on POSIFx.OUT3"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Icm::Value2
    }
    #[doc = "All index occurrences generated on POSIFx.OUT3"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Icm::Value3
    }
}
#[doc = "Field `ICM` writer - Index Marker generations control"]
pub type IcmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Icm>;
impl<'a, REG> IcmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No index marker generation on POSIFx.OUT3"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Icm::Value1)
    }
    #[doc = "Only first index occurrence generated on POSIFx.OUT3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Icm::Value2)
    }
    #[doc = "All index occurrences generated on POSIFx.OUT3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Icm::Value3)
    }
}
#[doc = "Current rotation direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dval {
    #[doc = "0: Counterclockwise rotation"]
    Value1 = 0,
    #[doc = "1: Clockwise rotation"]
    Value2 = 1,
}
impl From<Dval> for bool {
    #[inline(always)]
    fn from(variant: Dval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DVAL` reader - Current rotation direction"]
pub type DvalR = crate::BitReader<Dval>;
impl DvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dval {
        match self.bits {
            false => Dval::Value1,
            true => Dval::Value2,
        }
    }
    #[doc = "Counterclockwise rotation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dval::Value1
    }
    #[doc = "Clockwise rotation"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dval::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Phase A Level selector"]
    #[inline(always)]
    pub fn pals(&self) -> PalsR {
        PalsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Phase B Level selector"]
    #[inline(always)]
    pub fn pbls(&self) -> PblsR {
        PblsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Phase signals swap"]
    #[inline(always)]
    pub fn phs(&self) -> PhsR {
        PhsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Index Marker generations control"]
    #[inline(always)]
    pub fn icm(&self) -> IcmR {
        IcmR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Current rotation direction"]
    #[inline(always)]
    pub fn dval(&self) -> DvalR {
        DvalR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Phase A Level selector"]
    #[inline(always)]
    #[must_use]
    pub fn pals(&mut self) -> PalsW<QdcSpec> {
        PalsW::new(self, 0)
    }
    #[doc = "Bit 1 - Phase B Level selector"]
    #[inline(always)]
    #[must_use]
    pub fn pbls(&mut self) -> PblsW<QdcSpec> {
        PblsW::new(self, 1)
    }
    #[doc = "Bit 2 - Phase signals swap"]
    #[inline(always)]
    #[must_use]
    pub fn phs(&mut self) -> PhsW<QdcSpec> {
        PhsW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Index Marker generations control"]
    #[inline(always)]
    #[must_use]
    pub fn icm(&mut self) -> IcmW<QdcSpec> {
        IcmW::new(self, 4)
    }
}
#[doc = "Quadrature Decoder Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qdc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qdc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QdcSpec;
impl crate::RegisterSpec for QdcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qdc::R`](R) reader structure"]
impl crate::Readable for QdcSpec {}
#[doc = "`write(|w| ..)` method takes [`qdc::W`](W) writer structure"]
impl crate::Writable for QdcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QDC to value 0"]
impl crate::Resettable for QdcSpec {
    const RESET_VALUE: u32 = 0;
}
