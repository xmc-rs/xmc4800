#[doc = "Register `SDRMCON` reader"]
pub type R = crate::R<SdrmconSpec>;
#[doc = "Register `SDRMCON` writer"]
pub type W = crate::W<SdrmconSpec>;
#[doc = "Field `CRAS` reader - Row to precharge delay counter"]
pub type CrasR = crate::FieldReader;
#[doc = "Field `CRAS` writer - Row to precharge delay counter"]
pub type CrasW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CRFSH` reader - Initialization refresh commands counter"]
pub type CrfshR = crate::FieldReader;
#[doc = "Field `CRFSH` writer - Initialization refresh commands counter"]
pub type CrfshW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CRSC` reader - Mode register set-up time"]
pub type CrscR = crate::FieldReader;
#[doc = "Field `CRSC` writer - Mode register set-up time"]
pub type CrscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CRP` reader - Row precharge time counter"]
pub type CrpR = crate::FieldReader;
#[doc = "Field `CRP` writer - Row precharge time counter"]
pub type CrpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Width of column address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Awidth {
    #[doc = "0: do not use"]
    Value1 = 0,
    #[doc = "1: Address(8:0)"]
    Value2 = 1,
    #[doc = "2: Address(9:0)"]
    Value3 = 2,
    #[doc = "3: Address(10:0)"]
    Value4 = 3,
}
impl From<Awidth> for u8 {
    #[inline(always)]
    fn from(variant: Awidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Awidth {
    type Ux = u8;
}
impl crate::IsEnum for Awidth {}
#[doc = "Field `AWIDTH` reader - Width of column address"]
pub type AwidthR = crate::FieldReader<Awidth>;
impl AwidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awidth {
        match self.bits {
            0 => Awidth::Value1,
            1 => Awidth::Value2,
            2 => Awidth::Value3,
            3 => Awidth::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "do not use"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Awidth::Value1
    }
    #[doc = "Address(8:0)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Awidth::Value2
    }
    #[doc = "Address(9:0)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Awidth::Value3
    }
    #[doc = "Address(10:0)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Awidth::Value4
    }
}
#[doc = "Field `AWIDTH` writer - Width of column address"]
pub type AwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, Awidth, crate::Safe>;
impl<'a, REG> AwidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "do not use"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Awidth::Value1)
    }
    #[doc = "Address(8:0)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Awidth::Value2)
    }
    #[doc = "Address(9:0)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Awidth::Value3)
    }
    #[doc = "Address(10:0)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Awidth::Value4)
    }
}
#[doc = "Field `CRCD` reader - Row to column delay counter"]
pub type CrcdR = crate::FieldReader;
#[doc = "Field `CRCD` writer - Row to column delay counter"]
pub type CrcdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CRC` reader - Row cycle time counter"]
pub type CrcR = crate::FieldReader;
#[doc = "Field `CRC` writer - Row cycle time counter"]
pub type CrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Mask for row tag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rowm {
    #[doc = "1: Address bit 26 to 9"]
    Value2 = 1,
    #[doc = "2: Address bit 26 to 10"]
    Value3 = 2,
    #[doc = "3: Address bit 26 to 11"]
    Value4 = 3,
    #[doc = "4: Address bit 26 to 12"]
    Value5 = 4,
    #[doc = "5: Address bit 26 to 13"]
    Value6 = 5,
}
impl From<Rowm> for u8 {
    #[inline(always)]
    fn from(variant: Rowm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rowm {
    type Ux = u8;
}
impl crate::IsEnum for Rowm {}
#[doc = "Field `ROWM` reader - Mask for row tag"]
pub type RowmR = crate::FieldReader<Rowm>;
impl RowmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rowm> {
        match self.bits {
            1 => Some(Rowm::Value2),
            2 => Some(Rowm::Value3),
            3 => Some(Rowm::Value4),
            4 => Some(Rowm::Value5),
            5 => Some(Rowm::Value6),
            _ => None,
        }
    }
    #[doc = "Address bit 26 to 9"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rowm::Value2
    }
    #[doc = "Address bit 26 to 10"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rowm::Value3
    }
    #[doc = "Address bit 26 to 11"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rowm::Value4
    }
    #[doc = "Address bit 26 to 12"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Rowm::Value5
    }
    #[doc = "Address bit 26 to 13"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Rowm::Value6
    }
}
#[doc = "Field `ROWM` writer - Mask for row tag"]
pub type RowmW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rowm>;
impl<'a, REG> RowmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Address bit 26 to 9"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rowm::Value2)
    }
    #[doc = "Address bit 26 to 10"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rowm::Value3)
    }
    #[doc = "Address bit 26 to 11"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rowm::Value4)
    }
    #[doc = "Address bit 26 to 12"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Rowm::Value5)
    }
    #[doc = "Address bit 26 to 13"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Rowm::Value6)
    }
}
#[doc = "Mask for bank tag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bankm {
    #[doc = "1: Address bit 21 to 20"]
    Value2 = 1,
    #[doc = "2: Address bit 22 to 21"]
    Value3 = 2,
    #[doc = "3: Address bit 23 to 22"]
    Value4 = 3,
    #[doc = "4: Address bit 24 to 23"]
    Value5 = 4,
    #[doc = "5: Address bit 25 to 24"]
    Value6 = 5,
    #[doc = "6: Address bit 26 to 25"]
    Value7 = 6,
    #[doc = "7: Address bit 26"]
    Value8 = 7,
}
impl From<Bankm> for u8 {
    #[inline(always)]
    fn from(variant: Bankm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bankm {
    type Ux = u8;
}
impl crate::IsEnum for Bankm {}
#[doc = "Field `BANKM` reader - Mask for bank tag"]
pub type BankmR = crate::FieldReader<Bankm>;
impl BankmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bankm> {
        match self.bits {
            1 => Some(Bankm::Value2),
            2 => Some(Bankm::Value3),
            3 => Some(Bankm::Value4),
            4 => Some(Bankm::Value5),
            5 => Some(Bankm::Value6),
            6 => Some(Bankm::Value7),
            7 => Some(Bankm::Value8),
            _ => None,
        }
    }
    #[doc = "Address bit 21 to 20"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bankm::Value2
    }
    #[doc = "Address bit 22 to 21"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Bankm::Value3
    }
    #[doc = "Address bit 23 to 22"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Bankm::Value4
    }
    #[doc = "Address bit 24 to 23"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Bankm::Value5
    }
    #[doc = "Address bit 25 to 24"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Bankm::Value6
    }
    #[doc = "Address bit 26 to 25"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Bankm::Value7
    }
    #[doc = "Address bit 26"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Bankm::Value8
    }
}
#[doc = "Field `BANKM` writer - Mask for bank tag"]
pub type BankmW<'a, REG> = crate::FieldWriter<'a, REG, 3, Bankm>;
impl<'a, REG> BankmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Address bit 21 to 20"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bankm::Value2)
    }
    #[doc = "Address bit 22 to 21"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Bankm::Value3)
    }
    #[doc = "Address bit 23 to 22"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Bankm::Value4)
    }
    #[doc = "Address bit 24 to 23"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Bankm::Value5)
    }
    #[doc = "Address bit 25 to 24"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Bankm::Value6)
    }
    #[doc = "Address bit 26 to 25"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Bankm::Value7)
    }
    #[doc = "Address bit 26"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Bankm::Value8)
    }
}
#[doc = "Field `CRCE` reader - Row cycle time counter extension"]
pub type CrceR = crate::FieldReader;
#[doc = "Field `CRCE` writer - Row cycle time counter extension"]
pub type CrceW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Disable SDRAM clock output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkdis {
    #[doc = "0: clock enabled"]
    Value1 = 0,
    #[doc = "1: clock disabled"]
    Value2 = 1,
}
impl From<Clkdis> for bool {
    #[inline(always)]
    fn from(variant: Clkdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKDIS` reader - Disable SDRAM clock output"]
pub type ClkdisR = crate::BitReader<Clkdis>;
impl ClkdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkdis {
        match self.bits {
            false => Clkdis::Value1,
            true => Clkdis::Value2,
        }
    }
    #[doc = "clock enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Clkdis::Value1
    }
    #[doc = "clock disabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Clkdis::Value2
    }
}
#[doc = "Field `CLKDIS` writer - Disable SDRAM clock output"]
pub type ClkdisW<'a, REG> = crate::BitWriter<'a, REG, Clkdis>;
impl<'a, REG> ClkdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clock enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdis::Value1)
    }
    #[doc = "clock disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdis::Value2)
    }
}
#[doc = "Power Save Mode used for gated clock mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PwrMode {
    #[doc = "0: precharge before clock stop (default after reset)"]
    Value1 = 0,
    #[doc = "1: auto-precharge before clock stop"]
    Value2 = 1,
    #[doc = "2: active power down (stop clock without precharge)"]
    Value3 = 2,
    #[doc = "3: clock stop power down"]
    Value4 = 3,
}
impl From<PwrMode> for u8 {
    #[inline(always)]
    fn from(variant: PwrMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PwrMode {
    type Ux = u8;
}
impl crate::IsEnum for PwrMode {}
#[doc = "Field `PWR_MODE` reader - Power Save Mode used for gated clock mode"]
pub type PwrModeR = crate::FieldReader<PwrMode>;
impl PwrModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrMode {
        match self.bits {
            0 => PwrMode::Value1,
            1 => PwrMode::Value2,
            2 => PwrMode::Value3,
            3 => PwrMode::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "precharge before clock stop (default after reset)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PwrMode::Value1
    }
    #[doc = "auto-precharge before clock stop"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PwrMode::Value2
    }
    #[doc = "active power down (stop clock without precharge)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PwrMode::Value3
    }
    #[doc = "clock stop power down"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PwrMode::Value4
    }
}
#[doc = "Field `PWR_MODE` writer - Power Save Mode used for gated clock mode"]
pub type PwrModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, PwrMode, crate::Safe>;
impl<'a, REG> PwrModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "precharge before clock stop (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Value1)
    }
    #[doc = "auto-precharge before clock stop"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Value2)
    }
    #[doc = "active power down (stop clock without precharge)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Value3)
    }
    #[doc = "clock stop power down"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Value4)
    }
}
#[doc = "SDRAM clock mode select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdcmsel {
    #[doc = "1: clock disabled between accesses"]
    Value1 = 1,
    #[doc = "0: clock continuously runs"]
    Value2 = 0,
}
impl From<Sdcmsel> for bool {
    #[inline(always)]
    fn from(variant: Sdcmsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDCMSEL` reader - SDRAM clock mode select"]
pub type SdcmselR = crate::BitReader<Sdcmsel>;
impl SdcmselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdcmsel {
        match self.bits {
            true => Sdcmsel::Value1,
            false => Sdcmsel::Value2,
        }
    }
    #[doc = "clock disabled between accesses"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sdcmsel::Value1
    }
    #[doc = "clock continuously runs"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sdcmsel::Value2
    }
}
#[doc = "Field `SDCMSEL` writer - SDRAM clock mode select"]
pub type SdcmselW<'a, REG> = crate::BitWriter<'a, REG, Sdcmsel>;
impl<'a, REG> SdcmselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clock disabled between accesses"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcmsel::Value1)
    }
    #[doc = "clock continuously runs"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcmsel::Value2)
    }
}
impl R {
    #[doc = "Bits 0:3 - Row to precharge delay counter"]
    #[inline(always)]
    pub fn cras(&self) -> CrasR {
        CrasR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Initialization refresh commands counter"]
    #[inline(always)]
    pub fn crfsh(&self) -> CrfshR {
        CrfshR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Mode register set-up time"]
    #[inline(always)]
    pub fn crsc(&self) -> CrscR {
        CrscR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Row precharge time counter"]
    #[inline(always)]
    pub fn crp(&self) -> CrpR {
        CrpR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Width of column address"]
    #[inline(always)]
    pub fn awidth(&self) -> AwidthR {
        AwidthR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Row to column delay counter"]
    #[inline(always)]
    pub fn crcd(&self) -> CrcdR {
        CrcdR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Row cycle time counter"]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Mask for row tag"]
    #[inline(always)]
    pub fn rowm(&self) -> RowmR {
        RowmR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - Mask for bank tag"]
    #[inline(always)]
    pub fn bankm(&self) -> BankmR {
        BankmR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - Row cycle time counter extension"]
    #[inline(always)]
    pub fn crce(&self) -> CrceR {
        CrceR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - Disable SDRAM clock output"]
    #[inline(always)]
    pub fn clkdis(&self) -> ClkdisR {
        ClkdisR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Power Save Mode used for gated clock mode"]
    #[inline(always)]
    pub fn pwr_mode(&self) -> PwrModeR {
        PwrModeR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - SDRAM clock mode select"]
    #[inline(always)]
    pub fn sdcmsel(&self) -> SdcmselR {
        SdcmselR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Row to precharge delay counter"]
    #[inline(always)]
    #[must_use]
    pub fn cras(&mut self) -> CrasW<SdrmconSpec> {
        CrasW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Initialization refresh commands counter"]
    #[inline(always)]
    #[must_use]
    pub fn crfsh(&mut self) -> CrfshW<SdrmconSpec> {
        CrfshW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Mode register set-up time"]
    #[inline(always)]
    #[must_use]
    pub fn crsc(&mut self) -> CrscW<SdrmconSpec> {
        CrscW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Row precharge time counter"]
    #[inline(always)]
    #[must_use]
    pub fn crp(&mut self) -> CrpW<SdrmconSpec> {
        CrpW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Width of column address"]
    #[inline(always)]
    #[must_use]
    pub fn awidth(&mut self) -> AwidthW<SdrmconSpec> {
        AwidthW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Row to column delay counter"]
    #[inline(always)]
    #[must_use]
    pub fn crcd(&mut self) -> CrcdW<SdrmconSpec> {
        CrcdW::new(self, 14)
    }
    #[doc = "Bits 16:18 - Row cycle time counter"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CrcW<SdrmconSpec> {
        CrcW::new(self, 16)
    }
    #[doc = "Bits 19:21 - Mask for row tag"]
    #[inline(always)]
    #[must_use]
    pub fn rowm(&mut self) -> RowmW<SdrmconSpec> {
        RowmW::new(self, 19)
    }
    #[doc = "Bits 22:24 - Mask for bank tag"]
    #[inline(always)]
    #[must_use]
    pub fn bankm(&mut self) -> BankmW<SdrmconSpec> {
        BankmW::new(self, 22)
    }
    #[doc = "Bits 25:27 - Row cycle time counter extension"]
    #[inline(always)]
    #[must_use]
    pub fn crce(&mut self) -> CrceW<SdrmconSpec> {
        CrceW::new(self, 25)
    }
    #[doc = "Bit 28 - Disable SDRAM clock output"]
    #[inline(always)]
    #[must_use]
    pub fn clkdis(&mut self) -> ClkdisW<SdrmconSpec> {
        ClkdisW::new(self, 28)
    }
    #[doc = "Bits 29:30 - Power Save Mode used for gated clock mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_mode(&mut self) -> PwrModeW<SdrmconSpec> {
        PwrModeW::new(self, 29)
    }
    #[doc = "Bit 31 - SDRAM clock mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sdcmsel(&mut self) -> SdcmselW<SdrmconSpec> {
        SdcmselW::new(self, 31)
    }
}
#[doc = "EBU SDRAM Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdrmcon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdrmcon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdrmconSpec;
impl crate::RegisterSpec for SdrmconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdrmcon::R`](R) reader structure"]
impl crate::Readable for SdrmconSpec {}
#[doc = "`write(|w| ..)` method takes [`sdrmcon::W`](W) writer structure"]
impl crate::Writable for SdrmconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDRMCON to value 0x8000_0000"]
impl crate::Resettable for SdrmconSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
