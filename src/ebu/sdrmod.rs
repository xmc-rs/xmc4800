#[doc = "Register `SDRMOD` reader"]
pub type R = crate::R<SdrmodSpec>;
#[doc = "Register `SDRMOD` writer"]
pub type W = crate::W<SdrmodSpec>;
#[doc = "Burst length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Burstl {
    #[doc = "0: 1 (default after reset)"]
    Value1 = 0,
    #[doc = "1: 2"]
    Value2 = 1,
    #[doc = "2: 4"]
    Value3 = 2,
    #[doc = "3: 8"]
    Value4 = 3,
    #[doc = "4: 16"]
    Value5 = 4,
}
impl From<Burstl> for u8 {
    #[inline(always)]
    fn from(variant: Burstl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Burstl {
    type Ux = u8;
}
impl crate::IsEnum for Burstl {}
#[doc = "Field `BURSTL` reader - Burst length"]
pub type BurstlR = crate::FieldReader<Burstl>;
impl BurstlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Burstl> {
        match self.bits {
            0 => Some(Burstl::Value1),
            1 => Some(Burstl::Value2),
            2 => Some(Burstl::Value3),
            3 => Some(Burstl::Value4),
            4 => Some(Burstl::Value5),
            _ => None,
        }
    }
    #[doc = "1 (default after reset)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Burstl::Value1
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Burstl::Value2
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Burstl::Value3
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Burstl::Value4
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Burstl::Value5
    }
}
#[doc = "Field `BURSTL` writer - Burst length"]
pub type BurstlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Burstl>;
impl<'a, REG> BurstlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Burstl::Value1)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Burstl::Value2)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Burstl::Value3)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Burstl::Value4)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Burstl::Value5)
    }
}
#[doc = "Burst type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Btyp {
    #[doc = "0: Only this value should be written (default after reset)"]
    Value1 = 0,
}
impl From<Btyp> for bool {
    #[inline(always)]
    fn from(variant: Btyp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BTYP` reader - Burst type"]
pub type BtypR = crate::BitReader<Btyp>;
impl BtypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Btyp> {
        match self.bits {
            false => Some(Btyp::Value1),
            _ => None,
        }
    }
    #[doc = "Only this value should be written (default after reset)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Btyp::Value1
    }
}
#[doc = "Field `BTYP` writer - Burst type"]
pub type BtypW<'a, REG> = crate::BitWriter<'a, REG, Btyp>;
impl<'a, REG> BtypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Only this value should be written (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Btyp::Value1)
    }
}
#[doc = "CAS latency\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Caslat {
    #[doc = "2: Two clocks (default after reset)"]
    Value1 = 2,
    #[doc = "3: Three clocks"]
    Value2 = 3,
}
impl From<Caslat> for u8 {
    #[inline(always)]
    fn from(variant: Caslat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Caslat {
    type Ux = u8;
}
impl crate::IsEnum for Caslat {}
#[doc = "Field `CASLAT` reader - CAS latency"]
pub type CaslatR = crate::FieldReader<Caslat>;
impl CaslatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Caslat> {
        match self.bits {
            2 => Some(Caslat::Value1),
            3 => Some(Caslat::Value2),
            _ => None,
        }
    }
    #[doc = "Two clocks (default after reset)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Caslat::Value1
    }
    #[doc = "Three clocks"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Caslat::Value2
    }
}
#[doc = "Field `CASLAT` writer - CAS latency"]
pub type CaslatW<'a, REG> = crate::FieldWriter<'a, REG, 3, Caslat>;
impl<'a, REG> CaslatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Two clocks (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Caslat::Value1)
    }
    #[doc = "Three clocks"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Caslat::Value2)
    }
}
#[doc = "Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Opmode {
    #[doc = "0: Only this value must be written (default after reset)"]
    Value1 = 0,
}
impl From<Opmode> for u8 {
    #[inline(always)]
    fn from(variant: Opmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Opmode {
    type Ux = u8;
}
impl crate::IsEnum for Opmode {}
#[doc = "Field `OPMODE` reader - Operation Mode"]
pub type OpmodeR = crate::FieldReader<Opmode>;
impl OpmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Opmode> {
        match self.bits {
            0 => Some(Opmode::Value1),
            _ => None,
        }
    }
    #[doc = "Only this value must be written (default after reset)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Opmode::Value1
    }
}
#[doc = "Field `OPMODE` writer - Operation Mode"]
pub type OpmodeW<'a, REG> = crate::FieldWriter<'a, REG, 7, Opmode>;
impl<'a, REG> OpmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Only this value must be written (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Opmode::Value1)
    }
}
#[doc = "Field `COLDSTART` writer - SDRAM coldstart"]
pub type ColdstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOPM` reader - Extended Operation Mode"]
pub type XopmR = crate::FieldReader<u16>;
#[doc = "Field `XOPM` writer - Extended Operation Mode"]
pub type XopmW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `XBA` reader - Extended Operation Bank Select"]
pub type XbaR = crate::FieldReader;
#[doc = "Field `XBA` writer - Extended Operation Bank Select"]
pub type XbaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Burst length"]
    #[inline(always)]
    pub fn burstl(&self) -> BurstlR {
        BurstlR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Burst type"]
    #[inline(always)]
    pub fn btyp(&self) -> BtypR {
        BtypR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - CAS latency"]
    #[inline(always)]
    pub fn caslat(&self) -> CaslatR {
        CaslatR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:13 - Operation Mode"]
    #[inline(always)]
    pub fn opmode(&self) -> OpmodeR {
        OpmodeR::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 16:27 - Extended Operation Mode"]
    #[inline(always)]
    pub fn xopm(&self) -> XopmR {
        XopmR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31 - Extended Operation Bank Select"]
    #[inline(always)]
    pub fn xba(&self) -> XbaR {
        XbaR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Burst length"]
    #[inline(always)]
    #[must_use]
    pub fn burstl(&mut self) -> BurstlW<SdrmodSpec> {
        BurstlW::new(self, 0)
    }
    #[doc = "Bit 3 - Burst type"]
    #[inline(always)]
    #[must_use]
    pub fn btyp(&mut self) -> BtypW<SdrmodSpec> {
        BtypW::new(self, 3)
    }
    #[doc = "Bits 4:6 - CAS latency"]
    #[inline(always)]
    #[must_use]
    pub fn caslat(&mut self) -> CaslatW<SdrmodSpec> {
        CaslatW::new(self, 4)
    }
    #[doc = "Bits 7:13 - Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn opmode(&mut self) -> OpmodeW<SdrmodSpec> {
        OpmodeW::new(self, 7)
    }
    #[doc = "Bit 15 - SDRAM coldstart"]
    #[inline(always)]
    #[must_use]
    pub fn coldstart(&mut self) -> ColdstartW<SdrmodSpec> {
        ColdstartW::new(self, 15)
    }
    #[doc = "Bits 16:27 - Extended Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn xopm(&mut self) -> XopmW<SdrmodSpec> {
        XopmW::new(self, 16)
    }
    #[doc = "Bits 28:31 - Extended Operation Bank Select"]
    #[inline(always)]
    #[must_use]
    pub fn xba(&mut self) -> XbaW<SdrmodSpec> {
        XbaW::new(self, 28)
    }
}
#[doc = "EBU SDRAM Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdrmod::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdrmod::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdrmodSpec;
impl crate::RegisterSpec for SdrmodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdrmod::R`](R) reader structure"]
impl crate::Readable for SdrmodSpec {}
#[doc = "`write(|w| ..)` method takes [`sdrmod::W`](W) writer structure"]
impl crate::Writable for SdrmodSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDRMOD to value 0x20"]
impl crate::Resettable for SdrmodSpec {
    const RESET_VALUE: u32 = 0x20;
}
