#[doc = "Register `PCONF` reader"]
pub type R = crate::R<PconfSpec>;
#[doc = "Register `PCONF` writer"]
pub type W = crate::W<PconfSpec>;
#[doc = "Function Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel {
    #[doc = "0: Hall Sensor Mode enabled"]
    Value1 = 0,
    #[doc = "1: Quadrature Decoder Mode enabled"]
    Value2 = 1,
    #[doc = "2: stand-alone Multi-Channel Mode enabled"]
    Value3 = 2,
    #[doc = "3: Quadrature Decoder and stand-alone Multi-Channel Mode enabled"]
    Value4 = 3,
}
impl From<Fsel> for u8 {
    #[inline(always)]
    fn from(variant: Fsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel {
    type Ux = u8;
}
impl crate::IsEnum for Fsel {}
#[doc = "Field `FSEL` reader - Function Selector"]
pub type FselR = crate::FieldReader<Fsel>;
impl FselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel {
        match self.bits {
            0 => Fsel::Value1,
            1 => Fsel::Value2,
            2 => Fsel::Value3,
            3 => Fsel::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Hall Sensor Mode enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fsel::Value1
    }
    #[doc = "Quadrature Decoder Mode enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fsel::Value2
    }
    #[doc = "stand-alone Multi-Channel Mode enabled"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Fsel::Value3
    }
    #[doc = "Quadrature Decoder and stand-alone Multi-Channel Mode enabled"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Fsel::Value4
    }
}
#[doc = "Field `FSEL` writer - Function Selector"]
pub type FselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fsel, crate::Safe>;
impl<'a, REG> FselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Hall Sensor Mode enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Value1)
    }
    #[doc = "Quadrature Decoder Mode enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Value2)
    }
    #[doc = "stand-alone Multi-Channel Mode enabled"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Value3)
    }
    #[doc = "Quadrature Decoder and stand-alone Multi-Channel Mode enabled"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Value4)
    }
}
#[doc = "Position Decoder Mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qdcm {
    #[doc = "0: Position encoder is in Quadrature Mode"]
    Value1 = 0,
    #[doc = "1: Position encoder is in Direction Count Mode."]
    Value2 = 1,
}
impl From<Qdcm> for bool {
    #[inline(always)]
    fn from(variant: Qdcm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QDCM` reader - Position Decoder Mode selection"]
pub type QdcmR = crate::BitReader<Qdcm>;
impl QdcmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qdcm {
        match self.bits {
            false => Qdcm::Value1,
            true => Qdcm::Value2,
        }
    }
    #[doc = "Position encoder is in Quadrature Mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Qdcm::Value1
    }
    #[doc = "Position encoder is in Direction Count Mode."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Qdcm::Value2
    }
}
#[doc = "Field `QDCM` writer - Position Decoder Mode selection"]
pub type QdcmW<'a, REG> = crate::BitWriter<'a, REG, Qdcm>;
impl<'a, REG> QdcmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Position encoder is in Quadrature Mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Qdcm::Value1)
    }
    #[doc = "Position encoder is in Direction Count Mode."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Qdcm::Value2)
    }
}
#[doc = "Field `HIDG` reader - Idle generation enable"]
pub type HidgR = crate::BitReader;
#[doc = "Field `HIDG` writer - Idle generation enable"]
pub type HidgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Multi-Channel Pattern SW update enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mcue {
    #[doc = "0: Multi-Channel pattern update is controlled via HW"]
    Value1 = 0,
    #[doc = "1: Multi-Channel pattern update is controlled via SW"]
    Value2 = 1,
}
impl From<Mcue> for bool {
    #[inline(always)]
    fn from(variant: Mcue) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCUE` reader - Multi-Channel Pattern SW update enable"]
pub type McueR = crate::BitReader<Mcue>;
impl McueR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mcue {
        match self.bits {
            false => Mcue::Value1,
            true => Mcue::Value2,
        }
    }
    #[doc = "Multi-Channel pattern update is controlled via HW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mcue::Value1
    }
    #[doc = "Multi-Channel pattern update is controlled via SW"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mcue::Value2
    }
}
#[doc = "Field `MCUE` writer - Multi-Channel Pattern SW update enable"]
pub type McueW<'a, REG> = crate::BitWriter<'a, REG, Mcue>;
impl<'a, REG> McueW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multi-Channel pattern update is controlled via HW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mcue::Value1)
    }
    #[doc = "Multi-Channel pattern update is controlled via SW"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mcue::Value2)
    }
}
#[doc = "PhaseA/Hal input 1 selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Insel0 {
    #[doc = "0: POSIFx.IN0A"]
    Value1 = 0,
    #[doc = "1: POSIFx.IN0B"]
    Value2 = 1,
    #[doc = "2: POSIFx.IN0C"]
    Value3 = 2,
    #[doc = "3: POSIFx.IN0D"]
    Value4 = 3,
}
impl From<Insel0> for u8 {
    #[inline(always)]
    fn from(variant: Insel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Insel0 {
    type Ux = u8;
}
impl crate::IsEnum for Insel0 {}
#[doc = "Field `INSEL0` reader - PhaseA/Hal input 1 selector"]
pub type Insel0R = crate::FieldReader<Insel0>;
impl Insel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Insel0 {
        match self.bits {
            0 => Insel0::Value1,
            1 => Insel0::Value2,
            2 => Insel0::Value3,
            3 => Insel0::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "POSIFx.IN0A"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Insel0::Value1
    }
    #[doc = "POSIFx.IN0B"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Insel0::Value2
    }
    #[doc = "POSIFx.IN0C"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Insel0::Value3
    }
    #[doc = "POSIFx.IN0D"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Insel0::Value4
    }
}
#[doc = "Field `INSEL0` writer - PhaseA/Hal input 1 selector"]
pub type Insel0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Insel0, crate::Safe>;
impl<'a, REG> Insel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "POSIFx.IN0A"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0::Value1)
    }
    #[doc = "POSIFx.IN0B"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0::Value2)
    }
    #[doc = "POSIFx.IN0C"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0::Value3)
    }
    #[doc = "POSIFx.IN0D"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0::Value4)
    }
}
#[doc = "PhaseB/Hall input 2 selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Insel1 {
    #[doc = "0: POSIFx.IN1A"]
    Value1 = 0,
    #[doc = "1: POSIFx.IN1B"]
    Value2 = 1,
    #[doc = "2: POSIFx.IN1C"]
    Value3 = 2,
    #[doc = "3: POSIFx.IN1D"]
    Value4 = 3,
}
impl From<Insel1> for u8 {
    #[inline(always)]
    fn from(variant: Insel1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Insel1 {
    type Ux = u8;
}
impl crate::IsEnum for Insel1 {}
#[doc = "Field `INSEL1` reader - PhaseB/Hall input 2 selector"]
pub type Insel1R = crate::FieldReader<Insel1>;
impl Insel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Insel1 {
        match self.bits {
            0 => Insel1::Value1,
            1 => Insel1::Value2,
            2 => Insel1::Value3,
            3 => Insel1::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "POSIFx.IN1A"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Insel1::Value1
    }
    #[doc = "POSIFx.IN1B"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Insel1::Value2
    }
    #[doc = "POSIFx.IN1C"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Insel1::Value3
    }
    #[doc = "POSIFx.IN1D"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Insel1::Value4
    }
}
#[doc = "Field `INSEL1` writer - PhaseB/Hall input 2 selector"]
pub type Insel1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Insel1, crate::Safe>;
impl<'a, REG> Insel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "POSIFx.IN1A"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Insel1::Value1)
    }
    #[doc = "POSIFx.IN1B"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Insel1::Value2)
    }
    #[doc = "POSIFx.IN1C"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Insel1::Value3)
    }
    #[doc = "POSIFx.IN1D"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Insel1::Value4)
    }
}
#[doc = "Index/Hall input 3 selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Insel2 {
    #[doc = "0: POSIFx.IN2A"]
    Value1 = 0,
    #[doc = "1: POSIFx.IN2B"]
    Value2 = 1,
    #[doc = "2: POSIFx.IN2C"]
    Value3 = 2,
    #[doc = "3: POSIFx.IN2D"]
    Value4 = 3,
}
impl From<Insel2> for u8 {
    #[inline(always)]
    fn from(variant: Insel2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Insel2 {
    type Ux = u8;
}
impl crate::IsEnum for Insel2 {}
#[doc = "Field `INSEL2` reader - Index/Hall input 3 selector"]
pub type Insel2R = crate::FieldReader<Insel2>;
impl Insel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Insel2 {
        match self.bits {
            0 => Insel2::Value1,
            1 => Insel2::Value2,
            2 => Insel2::Value3,
            3 => Insel2::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "POSIFx.IN2A"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Insel2::Value1
    }
    #[doc = "POSIFx.IN2B"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Insel2::Value2
    }
    #[doc = "POSIFx.IN2C"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Insel2::Value3
    }
    #[doc = "POSIFx.IN2D"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Insel2::Value4
    }
}
#[doc = "Field `INSEL2` writer - Index/Hall input 3 selector"]
pub type Insel2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Insel2, crate::Safe>;
impl<'a, REG> Insel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "POSIFx.IN2A"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Insel2::Value1)
    }
    #[doc = "POSIFx.IN2B"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Insel2::Value2)
    }
    #[doc = "POSIFx.IN2C"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Insel2::Value3)
    }
    #[doc = "POSIFx.IN2D"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Insel2::Value4)
    }
}
#[doc = "Delay Pin selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsel {
    #[doc = "0: POSIFx.HSDA"]
    Value1 = 0,
    #[doc = "1: POSIFx.HSDB"]
    Value2 = 1,
}
impl From<Dsel> for bool {
    #[inline(always)]
    fn from(variant: Dsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSEL` reader - Delay Pin selector"]
pub type DselR = crate::BitReader<Dsel>;
impl DselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsel {
        match self.bits {
            false => Dsel::Value1,
            true => Dsel::Value2,
        }
    }
    #[doc = "POSIFx.HSDA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dsel::Value1
    }
    #[doc = "POSIFx.HSDB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dsel::Value2
    }
}
#[doc = "Field `DSEL` writer - Delay Pin selector"]
pub type DselW<'a, REG> = crate::BitWriter<'a, REG, Dsel>;
impl<'a, REG> DselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "POSIFx.HSDA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsel::Value1)
    }
    #[doc = "POSIFx.HSDB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dsel::Value2)
    }
}
#[doc = "Edge selector for the sampling trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spes {
    #[doc = "0: Rising edge"]
    Value1 = 0,
    #[doc = "1: Falling edge"]
    Value2 = 1,
}
impl From<Spes> for bool {
    #[inline(always)]
    fn from(variant: Spes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPES` reader - Edge selector for the sampling trigger"]
pub type SpesR = crate::BitReader<Spes>;
impl SpesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spes {
        match self.bits {
            false => Spes::Value1,
            true => Spes::Value2,
        }
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Spes::Value1
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Spes::Value2
    }
}
#[doc = "Field `SPES` writer - Edge selector for the sampling trigger"]
pub type SpesW<'a, REG> = crate::BitWriter<'a, REG, Spes>;
impl<'a, REG> SpesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Spes::Value1)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Spes::Value2)
    }
}
#[doc = "Pattern update signal select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Msets {
    #[doc = "0: POSIFx.MSETA"]
    Value1 = 0,
    #[doc = "1: POSIFx.MSETB"]
    Value2 = 1,
    #[doc = "2: POSIFx.MSETC"]
    Value3 = 2,
    #[doc = "3: POSIFx.MSETD"]
    Value4 = 3,
    #[doc = "4: POSIFx.MSETE"]
    Value5 = 4,
    #[doc = "5: POSIFx.MSETF"]
    Value6 = 5,
    #[doc = "6: POSIFx.MSETG"]
    Value7 = 6,
    #[doc = "7: POSIFx.MSETH"]
    Value8 = 7,
}
impl From<Msets> for u8 {
    #[inline(always)]
    fn from(variant: Msets) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Msets {
    type Ux = u8;
}
impl crate::IsEnum for Msets {}
#[doc = "Field `MSETS` reader - Pattern update signal select"]
pub type MsetsR = crate::FieldReader<Msets>;
impl MsetsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msets {
        match self.bits {
            0 => Msets::Value1,
            1 => Msets::Value2,
            2 => Msets::Value3,
            3 => Msets::Value4,
            4 => Msets::Value5,
            5 => Msets::Value6,
            6 => Msets::Value7,
            7 => Msets::Value8,
            _ => unreachable!(),
        }
    }
    #[doc = "POSIFx.MSETA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Msets::Value1
    }
    #[doc = "POSIFx.MSETB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Msets::Value2
    }
    #[doc = "POSIFx.MSETC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Msets::Value3
    }
    #[doc = "POSIFx.MSETD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Msets::Value4
    }
    #[doc = "POSIFx.MSETE"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Msets::Value5
    }
    #[doc = "POSIFx.MSETF"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Msets::Value6
    }
    #[doc = "POSIFx.MSETG"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Msets::Value7
    }
    #[doc = "POSIFx.MSETH"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Msets::Value8
    }
}
#[doc = "Field `MSETS` writer - Pattern update signal select"]
pub type MsetsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Msets, crate::Safe>;
impl<'a, REG> MsetsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "POSIFx.MSETA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Msets::Value1)
    }
    #[doc = "POSIFx.MSETB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Msets::Value2)
    }
    #[doc = "POSIFx.MSETC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Msets::Value3)
    }
    #[doc = "POSIFx.MSETD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Msets::Value4)
    }
    #[doc = "POSIFx.MSETE"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Msets::Value5)
    }
    #[doc = "POSIFx.MSETF"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Msets::Value6)
    }
    #[doc = "POSIFx.MSETG"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Msets::Value7)
    }
    #[doc = "POSIFx.MSETH"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Msets::Value8)
    }
}
#[doc = "Multi-Channel pattern update trigger edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mses {
    #[doc = "0: The signal used to enable a pattern update is active on the rising edge"]
    Value1 = 0,
    #[doc = "1: The signal used to enable a pattern update is active on the falling edge"]
    Value2 = 1,
}
impl From<Mses> for bool {
    #[inline(always)]
    fn from(variant: Mses) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSES` reader - Multi-Channel pattern update trigger edge"]
pub type MsesR = crate::BitReader<Mses>;
impl MsesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mses {
        match self.bits {
            false => Mses::Value1,
            true => Mses::Value2,
        }
    }
    #[doc = "The signal used to enable a pattern update is active on the rising edge"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mses::Value1
    }
    #[doc = "The signal used to enable a pattern update is active on the falling edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mses::Value2
    }
}
#[doc = "Field `MSES` writer - Multi-Channel pattern update trigger edge"]
pub type MsesW<'a, REG> = crate::BitWriter<'a, REG, Mses>;
impl<'a, REG> MsesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The signal used to enable a pattern update is active on the rising edge"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mses::Value1)
    }
    #[doc = "The signal used to enable a pattern update is active on the falling edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mses::Value2)
    }
}
#[doc = "PWM synchronization signal selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Msyns {
    #[doc = "0: POSIFx.MSYNCA"]
    Value1 = 0,
    #[doc = "1: POSIFx.MSYNCB"]
    Value2 = 1,
    #[doc = "2: POSIFx.MSYNCC"]
    Value3 = 2,
    #[doc = "3: POSIFx.MSYNCD"]
    Value4 = 3,
}
impl From<Msyns> for u8 {
    #[inline(always)]
    fn from(variant: Msyns) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Msyns {
    type Ux = u8;
}
impl crate::IsEnum for Msyns {}
#[doc = "Field `MSYNS` reader - PWM synchronization signal selector"]
pub type MsynsR = crate::FieldReader<Msyns>;
impl MsynsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msyns {
        match self.bits {
            0 => Msyns::Value1,
            1 => Msyns::Value2,
            2 => Msyns::Value3,
            3 => Msyns::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "POSIFx.MSYNCA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Msyns::Value1
    }
    #[doc = "POSIFx.MSYNCB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Msyns::Value2
    }
    #[doc = "POSIFx.MSYNCC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Msyns::Value3
    }
    #[doc = "POSIFx.MSYNCD"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Msyns::Value4
    }
}
#[doc = "Field `MSYNS` writer - PWM synchronization signal selector"]
pub type MsynsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Msyns, crate::Safe>;
impl<'a, REG> MsynsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "POSIFx.MSYNCA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Msyns::Value1)
    }
    #[doc = "POSIFx.MSYNCB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Msyns::Value2)
    }
    #[doc = "POSIFx.MSYNCC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Msyns::Value3)
    }
    #[doc = "POSIFx.MSYNCD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Msyns::Value4)
    }
}
#[doc = "Wrong Hall Event selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ewis {
    #[doc = "0: POSIFx.EWHEA"]
    Value1 = 0,
    #[doc = "1: POSIFx.EWHEB"]
    Value2 = 1,
    #[doc = "2: POSIFx.EWHEC"]
    Value3 = 2,
    #[doc = "3: POSIFx.EWHED"]
    Value4 = 3,
}
impl From<Ewis> for u8 {
    #[inline(always)]
    fn from(variant: Ewis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ewis {
    type Ux = u8;
}
impl crate::IsEnum for Ewis {}
#[doc = "Field `EWIS` reader - Wrong Hall Event selection"]
pub type EwisR = crate::FieldReader<Ewis>;
impl EwisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ewis {
        match self.bits {
            0 => Ewis::Value1,
            1 => Ewis::Value2,
            2 => Ewis::Value3,
            3 => Ewis::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "POSIFx.EWHEA"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ewis::Value1
    }
    #[doc = "POSIFx.EWHEB"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ewis::Value2
    }
    #[doc = "POSIFx.EWHEC"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Ewis::Value3
    }
    #[doc = "POSIFx.EWHED"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Ewis::Value4
    }
}
#[doc = "Field `EWIS` writer - Wrong Hall Event selection"]
pub type EwisW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ewis, crate::Safe>;
impl<'a, REG> EwisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "POSIFx.EWHEA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ewis::Value1)
    }
    #[doc = "POSIFx.EWHEB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ewis::Value2)
    }
    #[doc = "POSIFx.EWHEC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Ewis::Value3)
    }
    #[doc = "POSIFx.EWHED"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Ewis::Value4)
    }
}
#[doc = "External Wrong Hall Event enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ewie {
    #[doc = "0: External wrong hall event emulation signal, POSIFx.EWHE\\[D...A\\], is disabled"]
    Value1 = 0,
    #[doc = "1: External wrong hall event emulation signal, POSIFx.EWHE\\[D...A\\], is enabled."]
    Value2 = 1,
}
impl From<Ewie> for bool {
    #[inline(always)]
    fn from(variant: Ewie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWIE` reader - External Wrong Hall Event enable"]
pub type EwieR = crate::BitReader<Ewie>;
impl EwieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ewie {
        match self.bits {
            false => Ewie::Value1,
            true => Ewie::Value2,
        }
    }
    #[doc = "External wrong hall event emulation signal, POSIFx.EWHE\\[D...A\\], is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ewie::Value1
    }
    #[doc = "External wrong hall event emulation signal, POSIFx.EWHE\\[D...A\\], is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ewie::Value2
    }
}
#[doc = "Field `EWIE` writer - External Wrong Hall Event enable"]
pub type EwieW<'a, REG> = crate::BitWriter<'a, REG, Ewie>;
impl<'a, REG> EwieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External wrong hall event emulation signal, POSIFx.EWHE\\[D...A\\], is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ewie::Value1)
    }
    #[doc = "External wrong hall event emulation signal, POSIFx.EWHE\\[D...A\\], is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ewie::Value2)
    }
}
#[doc = "External Wrong Hall Event active level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ewil {
    #[doc = "0: POSIFx.EWHE\\[D...A\\]
signal is active HIGH"]
    Value1 = 0,
    #[doc = "1: POSIFx.EWHE\\[D...A\\]
signal is active LOW"]
    Value2 = 1,
}
impl From<Ewil> for bool {
    #[inline(always)]
    fn from(variant: Ewil) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWIL` reader - External Wrong Hall Event active level"]
pub type EwilR = crate::BitReader<Ewil>;
impl EwilR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ewil {
        match self.bits {
            false => Ewil::Value1,
            true => Ewil::Value2,
        }
    }
    #[doc = "POSIFx.EWHE\\[D...A\\]
signal is active HIGH"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ewil::Value1
    }
    #[doc = "POSIFx.EWHE\\[D...A\\]
signal is active LOW"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ewil::Value2
    }
}
#[doc = "Field `EWIL` writer - External Wrong Hall Event active level"]
pub type EwilW<'a, REG> = crate::BitWriter<'a, REG, Ewil>;
impl<'a, REG> EwilW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "POSIFx.EWHE\\[D...A\\]
signal is active HIGH"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ewil::Value1)
    }
    #[doc = "POSIFx.EWHE\\[D...A\\]
signal is active LOW"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ewil::Value2)
    }
}
#[doc = "Low Pass Filters Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpc {
    #[doc = "0: Low pass filter disabled"]
    Value1 = 0,
    #[doc = "1: Low pass of 1 clock cycle"]
    Value2 = 1,
    #[doc = "2: Low pass of 2 clock cycles"]
    Value3 = 2,
    #[doc = "3: Low pass of 4 clock cycles"]
    Value4 = 3,
    #[doc = "4: Low pass of 8 clock cycles"]
    Value5 = 4,
    #[doc = "5: Low pass of 16 clock cycles"]
    Value6 = 5,
    #[doc = "6: Low pass of 32 clock cycles"]
    Value7 = 6,
    #[doc = "7: Low pass of 64 clock cycles"]
    Value8 = 7,
}
impl From<Lpc> for u8 {
    #[inline(always)]
    fn from(variant: Lpc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpc {
    type Ux = u8;
}
impl crate::IsEnum for Lpc {}
#[doc = "Field `LPC` reader - Low Pass Filters Configuration"]
pub type LpcR = crate::FieldReader<Lpc>;
impl LpcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpc {
        match self.bits {
            0 => Lpc::Value1,
            1 => Lpc::Value2,
            2 => Lpc::Value3,
            3 => Lpc::Value4,
            4 => Lpc::Value5,
            5 => Lpc::Value6,
            6 => Lpc::Value7,
            7 => Lpc::Value8,
            _ => unreachable!(),
        }
    }
    #[doc = "Low pass filter disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lpc::Value1
    }
    #[doc = "Low pass of 1 clock cycle"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lpc::Value2
    }
    #[doc = "Low pass of 2 clock cycles"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Lpc::Value3
    }
    #[doc = "Low pass of 4 clock cycles"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Lpc::Value4
    }
    #[doc = "Low pass of 8 clock cycles"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Lpc::Value5
    }
    #[doc = "Low pass of 16 clock cycles"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Lpc::Value6
    }
    #[doc = "Low pass of 32 clock cycles"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Lpc::Value7
    }
    #[doc = "Low pass of 64 clock cycles"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Lpc::Value8
    }
}
#[doc = "Field `LPC` writer - Low Pass Filters Configuration"]
pub type LpcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lpc, crate::Safe>;
impl<'a, REG> LpcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low pass filter disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpc::Value1)
    }
    #[doc = "Low pass of 1 clock cycle"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpc::Value2)
    }
    #[doc = "Low pass of 2 clock cycles"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Lpc::Value3)
    }
    #[doc = "Low pass of 4 clock cycles"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Lpc::Value4)
    }
    #[doc = "Low pass of 8 clock cycles"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Lpc::Value5)
    }
    #[doc = "Low pass of 16 clock cycles"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Lpc::Value6)
    }
    #[doc = "Low pass of 32 clock cycles"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Lpc::Value7)
    }
    #[doc = "Low pass of 64 clock cycles"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Lpc::Value8)
    }
}
impl R {
    #[doc = "Bits 0:1 - Function Selector"]
    #[inline(always)]
    pub fn fsel(&self) -> FselR {
        FselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Position Decoder Mode selection"]
    #[inline(always)]
    pub fn qdcm(&self) -> QdcmR {
        QdcmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Idle generation enable"]
    #[inline(always)]
    pub fn hidg(&self) -> HidgR {
        HidgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi-Channel Pattern SW update enable"]
    #[inline(always)]
    pub fn mcue(&self) -> McueR {
        McueR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9 - PhaseA/Hal input 1 selector"]
    #[inline(always)]
    pub fn insel0(&self) -> Insel0R {
        Insel0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PhaseB/Hall input 2 selector"]
    #[inline(always)]
    pub fn insel1(&self) -> Insel1R {
        Insel1R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Index/Hall input 3 selector"]
    #[inline(always)]
    pub fn insel2(&self) -> Insel2R {
        Insel2R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - Delay Pin selector"]
    #[inline(always)]
    pub fn dsel(&self) -> DselR {
        DselR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Edge selector for the sampling trigger"]
    #[inline(always)]
    pub fn spes(&self) -> SpesR {
        SpesR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Pattern update signal select"]
    #[inline(always)]
    pub fn msets(&self) -> MsetsR {
        MsetsR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - Multi-Channel pattern update trigger edge"]
    #[inline(always)]
    pub fn mses(&self) -> MsesR {
        MsesR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - PWM synchronization signal selector"]
    #[inline(always)]
    pub fn msyns(&self) -> MsynsR {
        MsynsR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Wrong Hall Event selection"]
    #[inline(always)]
    pub fn ewis(&self) -> EwisR {
        EwisR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - External Wrong Hall Event enable"]
    #[inline(always)]
    pub fn ewie(&self) -> EwieR {
        EwieR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - External Wrong Hall Event active level"]
    #[inline(always)]
    pub fn ewil(&self) -> EwilR {
        EwilR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Low Pass Filters Configuration"]
    #[inline(always)]
    pub fn lpc(&self) -> LpcR {
        LpcR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Function Selector"]
    #[inline(always)]
    #[must_use]
    pub fn fsel(&mut self) -> FselW<PconfSpec> {
        FselW::new(self, 0)
    }
    #[doc = "Bit 2 - Position Decoder Mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn qdcm(&mut self) -> QdcmW<PconfSpec> {
        QdcmW::new(self, 2)
    }
    #[doc = "Bit 4 - Idle generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn hidg(&mut self) -> HidgW<PconfSpec> {
        HidgW::new(self, 4)
    }
    #[doc = "Bit 5 - Multi-Channel Pattern SW update enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcue(&mut self) -> McueW<PconfSpec> {
        McueW::new(self, 5)
    }
    #[doc = "Bits 8:9 - PhaseA/Hal input 1 selector"]
    #[inline(always)]
    #[must_use]
    pub fn insel0(&mut self) -> Insel0W<PconfSpec> {
        Insel0W::new(self, 8)
    }
    #[doc = "Bits 10:11 - PhaseB/Hall input 2 selector"]
    #[inline(always)]
    #[must_use]
    pub fn insel1(&mut self) -> Insel1W<PconfSpec> {
        Insel1W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Index/Hall input 3 selector"]
    #[inline(always)]
    #[must_use]
    pub fn insel2(&mut self) -> Insel2W<PconfSpec> {
        Insel2W::new(self, 12)
    }
    #[doc = "Bit 16 - Delay Pin selector"]
    #[inline(always)]
    #[must_use]
    pub fn dsel(&mut self) -> DselW<PconfSpec> {
        DselW::new(self, 16)
    }
    #[doc = "Bit 17 - Edge selector for the sampling trigger"]
    #[inline(always)]
    #[must_use]
    pub fn spes(&mut self) -> SpesW<PconfSpec> {
        SpesW::new(self, 17)
    }
    #[doc = "Bits 18:20 - Pattern update signal select"]
    #[inline(always)]
    #[must_use]
    pub fn msets(&mut self) -> MsetsW<PconfSpec> {
        MsetsW::new(self, 18)
    }
    #[doc = "Bit 21 - Multi-Channel pattern update trigger edge"]
    #[inline(always)]
    #[must_use]
    pub fn mses(&mut self) -> MsesW<PconfSpec> {
        MsesW::new(self, 21)
    }
    #[doc = "Bits 22:23 - PWM synchronization signal selector"]
    #[inline(always)]
    #[must_use]
    pub fn msyns(&mut self) -> MsynsW<PconfSpec> {
        MsynsW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Wrong Hall Event selection"]
    #[inline(always)]
    #[must_use]
    pub fn ewis(&mut self) -> EwisW<PconfSpec> {
        EwisW::new(self, 24)
    }
    #[doc = "Bit 26 - External Wrong Hall Event enable"]
    #[inline(always)]
    #[must_use]
    pub fn ewie(&mut self) -> EwieW<PconfSpec> {
        EwieW::new(self, 26)
    }
    #[doc = "Bit 27 - External Wrong Hall Event active level"]
    #[inline(always)]
    #[must_use]
    pub fn ewil(&mut self) -> EwilW<PconfSpec> {
        EwilW::new(self, 27)
    }
    #[doc = "Bits 28:30 - Low Pass Filters Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn lpc(&mut self) -> LpcW<PconfSpec> {
        LpcW::new(self, 28)
    }
}
#[doc = "POSIF configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pconf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pconf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PconfSpec;
impl crate::RegisterSpec for PconfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pconf::R`](R) reader structure"]
impl crate::Readable for PconfSpec {}
#[doc = "`write(|w| ..)` method takes [`pconf::W`](W) writer structure"]
impl crate::Writable for PconfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCONF to value 0"]
impl crate::Resettable for PconfSpec {
    const RESET_VALUE: u32 = 0;
}
