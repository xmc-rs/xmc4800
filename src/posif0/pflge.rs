#[doc = "Register `PFLGE` reader"]
pub type R = crate::R<PflgeSpec>;
#[doc = "Register `PFLGE` writer"]
pub type W = crate::W<PflgeSpec>;
#[doc = "Correct Hall Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eche {
    #[doc = "0: Correct Hall Event interrupt disabled"]
    Value1 = 0,
    #[doc = "1: Correct Hall Event interrupt enabled"]
    Value2 = 1,
}
impl From<Eche> for bool {
    #[inline(always)]
    fn from(variant: Eche) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECHE` reader - Correct Hall Event Enable"]
pub type EcheR = crate::BitReader<Eche>;
impl EcheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eche {
        match self.bits {
            false => Eche::Value1,
            true => Eche::Value2,
        }
    }
    #[doc = "Correct Hall Event interrupt disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Eche::Value1
    }
    #[doc = "Correct Hall Event interrupt enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Eche::Value2
    }
}
#[doc = "Field `ECHE` writer - Correct Hall Event Enable"]
pub type EcheW<'a, REG> = crate::BitWriter<'a, REG, Eche>;
impl<'a, REG> EcheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Correct Hall Event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Eche::Value1)
    }
    #[doc = "Correct Hall Event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Eche::Value2)
    }
}
#[doc = "Wrong Hall Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ewhe {
    #[doc = "0: Wrong Hall Event interrupt disabled"]
    Value1 = 0,
    #[doc = "1: Wrong Hall Event interrupt enabled"]
    Value2 = 1,
}
impl From<Ewhe> for bool {
    #[inline(always)]
    fn from(variant: Ewhe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWHE` reader - Wrong Hall Event Enable"]
pub type EwheR = crate::BitReader<Ewhe>;
impl EwheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ewhe {
        match self.bits {
            false => Ewhe::Value1,
            true => Ewhe::Value2,
        }
    }
    #[doc = "Wrong Hall Event interrupt disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ewhe::Value1
    }
    #[doc = "Wrong Hall Event interrupt enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ewhe::Value2
    }
}
#[doc = "Field `EWHE` writer - Wrong Hall Event Enable"]
pub type EwheW<'a, REG> = crate::BitWriter<'a, REG, Ewhe>;
impl<'a, REG> EwheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wrong Hall Event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ewhe::Value1)
    }
    #[doc = "Wrong Hall Event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ewhe::Value2)
    }
}
#[doc = "Hall Input Update Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ehie {
    #[doc = "0: Update of the Hall Inputs interrupt is disabled"]
    Value1 = 0,
    #[doc = "1: Update of the Hall Inputs interrupt is enabled"]
    Value2 = 1,
}
impl From<Ehie> for bool {
    #[inline(always)]
    fn from(variant: Ehie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EHIE` reader - Hall Input Update Enable"]
pub type EhieR = crate::BitReader<Ehie>;
impl EhieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ehie {
        match self.bits {
            false => Ehie::Value1,
            true => Ehie::Value2,
        }
    }
    #[doc = "Update of the Hall Inputs interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ehie::Value1
    }
    #[doc = "Update of the Hall Inputs interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ehie::Value2
    }
}
#[doc = "Field `EHIE` writer - Hall Input Update Enable"]
pub type EhieW<'a, REG> = crate::BitWriter<'a, REG, Ehie>;
impl<'a, REG> EhieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update of the Hall Inputs interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ehie::Value1)
    }
    #[doc = "Update of the Hall Inputs interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ehie::Value2)
    }
}
#[doc = "Multi-Channel pattern shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Emst {
    #[doc = "0: Shadow transfer event interrupt disabled"]
    Value1 = 0,
    #[doc = "1: Shadow transfer event interrupt enabled"]
    Value2 = 1,
}
impl From<Emst> for bool {
    #[inline(always)]
    fn from(variant: Emst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMST` reader - Multi-Channel pattern shadow transfer enable"]
pub type EmstR = crate::BitReader<Emst>;
impl EmstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Emst {
        match self.bits {
            false => Emst::Value1,
            true => Emst::Value2,
        }
    }
    #[doc = "Shadow transfer event interrupt disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Emst::Value1
    }
    #[doc = "Shadow transfer event interrupt enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Emst::Value2
    }
}
#[doc = "Field `EMST` writer - Multi-Channel pattern shadow transfer enable"]
pub type EmstW<'a, REG> = crate::BitWriter<'a, REG, Emst>;
impl<'a, REG> EmstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shadow transfer event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Emst::Value1)
    }
    #[doc = "Shadow transfer event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Emst::Value2)
    }
}
#[doc = "Quadrature Index Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eindx {
    #[doc = "0: Index event interrupt disabled"]
    Value1 = 0,
    #[doc = "1: Index event interrupt enabled"]
    Value2 = 1,
}
impl From<Eindx> for bool {
    #[inline(always)]
    fn from(variant: Eindx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EINDX` reader - Quadrature Index Event Enable"]
pub type EindxR = crate::BitReader<Eindx>;
impl EindxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eindx {
        match self.bits {
            false => Eindx::Value1,
            true => Eindx::Value2,
        }
    }
    #[doc = "Index event interrupt disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Eindx::Value1
    }
    #[doc = "Index event interrupt enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Eindx::Value2
    }
}
#[doc = "Field `EINDX` writer - Quadrature Index Event Enable"]
pub type EindxW<'a, REG> = crate::BitWriter<'a, REG, Eindx>;
impl<'a, REG> EindxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Index event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Eindx::Value1)
    }
    #[doc = "Index event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Eindx::Value2)
    }
}
#[doc = "Quadrature Phase Error Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eerr {
    #[doc = "0: Phase error event interrupt disabled"]
    Value1 = 0,
    #[doc = "1: Phase error event interrupt enabled"]
    Value2 = 1,
}
impl From<Eerr> for bool {
    #[inline(always)]
    fn from(variant: Eerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EERR` reader - Quadrature Phase Error Enable"]
pub type EerrR = crate::BitReader<Eerr>;
impl EerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eerr {
        match self.bits {
            false => Eerr::Value1,
            true => Eerr::Value2,
        }
    }
    #[doc = "Phase error event interrupt disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Eerr::Value1
    }
    #[doc = "Phase error event interrupt enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Eerr::Value2
    }
}
#[doc = "Field `EERR` writer - Quadrature Phase Error Enable"]
pub type EerrW<'a, REG> = crate::BitWriter<'a, REG, Eerr>;
impl<'a, REG> EerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Phase error event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Eerr::Value1)
    }
    #[doc = "Phase error event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Eerr::Value2)
    }
}
#[doc = "Quadrature CLK interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecnt {
    #[doc = "0: Quadrature CLK event interrupt disabled"]
    Value1 = 0,
    #[doc = "1: Quadrature CLK event interrupt enabled"]
    Value2 = 1,
}
impl From<Ecnt> for bool {
    #[inline(always)]
    fn from(variant: Ecnt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECNT` reader - Quadrature CLK interrupt Enable"]
pub type EcntR = crate::BitReader<Ecnt>;
impl EcntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecnt {
        match self.bits {
            false => Ecnt::Value1,
            true => Ecnt::Value2,
        }
    }
    #[doc = "Quadrature CLK event interrupt disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ecnt::Value1
    }
    #[doc = "Quadrature CLK event interrupt enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ecnt::Value2
    }
}
#[doc = "Field `ECNT` writer - Quadrature CLK interrupt Enable"]
pub type EcntW<'a, REG> = crate::BitWriter<'a, REG, Ecnt>;
impl<'a, REG> EcntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Quadrature CLK event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ecnt::Value1)
    }
    #[doc = "Quadrature CLK event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ecnt::Value2)
    }
}
#[doc = "Quadrature direction change interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edir {
    #[doc = "0: Direction change event interrupt disabled"]
    Value1 = 0,
    #[doc = "1: Direction change event interrupt enabled"]
    Value2 = 1,
}
impl From<Edir> for bool {
    #[inline(always)]
    fn from(variant: Edir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDIR` reader - Quadrature direction change interrupt Enable"]
pub type EdirR = crate::BitReader<Edir>;
impl EdirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edir {
        match self.bits {
            false => Edir::Value1,
            true => Edir::Value2,
        }
    }
    #[doc = "Direction change event interrupt disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Edir::Value1
    }
    #[doc = "Direction change event interrupt enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Edir::Value2
    }
}
#[doc = "Field `EDIR` writer - Quadrature direction change interrupt Enable"]
pub type EdirW<'a, REG> = crate::BitWriter<'a, REG, Edir>;
impl<'a, REG> EdirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Direction change event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Edir::Value1)
    }
    #[doc = "Direction change event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Edir::Value2)
    }
}
#[doc = "Quadrature Period CLK interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epclk {
    #[doc = "0: Quadrature Period CLK event interrupt disabled"]
    Value1 = 0,
    #[doc = "1: Quadrature Period CLK event interrupt enabled"]
    Value2 = 1,
}
impl From<Epclk> for bool {
    #[inline(always)]
    fn from(variant: Epclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPCLK` reader - Quadrature Period CLK interrupt Enable"]
pub type EpclkR = crate::BitReader<Epclk>;
impl EpclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epclk {
        match self.bits {
            false => Epclk::Value1,
            true => Epclk::Value2,
        }
    }
    #[doc = "Quadrature Period CLK event interrupt disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Epclk::Value1
    }
    #[doc = "Quadrature Period CLK event interrupt enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Epclk::Value2
    }
}
#[doc = "Field `EPCLK` writer - Quadrature Period CLK interrupt Enable"]
pub type EpclkW<'a, REG> = crate::BitWriter<'a, REG, Epclk>;
impl<'a, REG> EpclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Quadrature Period CLK event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Epclk::Value1)
    }
    #[doc = "Quadrature Period CLK event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Epclk::Value2)
    }
}
#[doc = "Correct Hall Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chesel {
    #[doc = "0: Correct Hall Event interrupt forward to POSIFx.SR0"]
    Value1 = 0,
    #[doc = "1: Correct Hall Event interrupt forward to POSIFx.SR1"]
    Value2 = 1,
}
impl From<Chesel> for bool {
    #[inline(always)]
    fn from(variant: Chesel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHESEL` reader - Correct Hall Event Service Request Selector"]
pub type CheselR = crate::BitReader<Chesel>;
impl CheselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chesel {
        match self.bits {
            false => Chesel::Value1,
            true => Chesel::Value2,
        }
    }
    #[doc = "Correct Hall Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chesel::Value1
    }
    #[doc = "Correct Hall Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chesel::Value2
    }
}
#[doc = "Field `CHESEL` writer - Correct Hall Event Service Request Selector"]
pub type CheselW<'a, REG> = crate::BitWriter<'a, REG, Chesel>;
impl<'a, REG> CheselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Correct Hall Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chesel::Value1)
    }
    #[doc = "Correct Hall Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chesel::Value2)
    }
}
#[doc = "Wrong Hall Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Whesel {
    #[doc = "0: Wrong Hall Event interrupt forward to POSIFx.SR0"]
    Value1 = 0,
    #[doc = "1: Wrong Hall Event interrupt forward to POSIFx.SR1"]
    Value2 = 1,
}
impl From<Whesel> for bool {
    #[inline(always)]
    fn from(variant: Whesel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WHESEL` reader - Wrong Hall Event Service Request Selector"]
pub type WheselR = crate::BitReader<Whesel>;
impl WheselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Whesel {
        match self.bits {
            false => Whesel::Value1,
            true => Whesel::Value2,
        }
    }
    #[doc = "Wrong Hall Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Whesel::Value1
    }
    #[doc = "Wrong Hall Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Whesel::Value2
    }
}
#[doc = "Field `WHESEL` writer - Wrong Hall Event Service Request Selector"]
pub type WheselW<'a, REG> = crate::BitWriter<'a, REG, Whesel>;
impl<'a, REG> WheselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wrong Hall Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Whesel::Value1)
    }
    #[doc = "Wrong Hall Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Whesel::Value2)
    }
}
#[doc = "Hall Inputs Update Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hiesel {
    #[doc = "0: Hall Inputs Update Event interrupt forward to POSIFx.SR0"]
    Value1 = 0,
    #[doc = "1: Hall Inputs Update Event interrupt forward to POSIFx.SR1"]
    Value2 = 1,
}
impl From<Hiesel> for bool {
    #[inline(always)]
    fn from(variant: Hiesel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIESEL` reader - Hall Inputs Update Event Service Request Selector"]
pub type HieselR = crate::BitReader<Hiesel>;
impl HieselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hiesel {
        match self.bits {
            false => Hiesel::Value1,
            true => Hiesel::Value2,
        }
    }
    #[doc = "Hall Inputs Update Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hiesel::Value1
    }
    #[doc = "Hall Inputs Update Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hiesel::Value2
    }
}
#[doc = "Field `HIESEL` writer - Hall Inputs Update Event Service Request Selector"]
pub type HieselW<'a, REG> = crate::BitWriter<'a, REG, Hiesel>;
impl<'a, REG> HieselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hall Inputs Update Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hiesel::Value1)
    }
    #[doc = "Hall Inputs Update Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hiesel::Value2)
    }
}
#[doc = "Multi-Channel pattern Update Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstsel {
    #[doc = "0: Multi-Channel pattern Update Event interrupt forward to POSIFx.SR0"]
    Value1 = 0,
    #[doc = "1: Multi-Channel pattern Update Event interrupt forward to POSIFx.SR1"]
    Value2 = 1,
}
impl From<Mstsel> for bool {
    #[inline(always)]
    fn from(variant: Mstsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTSEL` reader - Multi-Channel pattern Update Event Service Request Selector"]
pub type MstselR = crate::BitReader<Mstsel>;
impl MstselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstsel {
        match self.bits {
            false => Mstsel::Value1,
            true => Mstsel::Value2,
        }
    }
    #[doc = "Multi-Channel pattern Update Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mstsel::Value1
    }
    #[doc = "Multi-Channel pattern Update Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mstsel::Value2
    }
}
#[doc = "Field `MSTSEL` writer - Multi-Channel pattern Update Event Service Request Selector"]
pub type MstselW<'a, REG> = crate::BitWriter<'a, REG, Mstsel>;
impl<'a, REG> MstselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multi-Channel pattern Update Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstsel::Value1)
    }
    #[doc = "Multi-Channel pattern Update Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mstsel::Value2)
    }
}
#[doc = "Quadrature Index Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Indsel {
    #[doc = "0: Quadrature Index Event interrupt forward to POSIFx.SR0"]
    Value1 = 0,
    #[doc = "1: Quadrature Index Event interrupt forward to POSIFx.SR1"]
    Value2 = 1,
}
impl From<Indsel> for bool {
    #[inline(always)]
    fn from(variant: Indsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INDSEL` reader - Quadrature Index Event Service Request Selector"]
pub type IndselR = crate::BitReader<Indsel>;
impl IndselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Indsel {
        match self.bits {
            false => Indsel::Value1,
            true => Indsel::Value2,
        }
    }
    #[doc = "Quadrature Index Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Indsel::Value1
    }
    #[doc = "Quadrature Index Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Indsel::Value2
    }
}
#[doc = "Field `INDSEL` writer - Quadrature Index Event Service Request Selector"]
pub type IndselW<'a, REG> = crate::BitWriter<'a, REG, Indsel>;
impl<'a, REG> IndselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Quadrature Index Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Indsel::Value1)
    }
    #[doc = "Quadrature Index Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Indsel::Value2)
    }
}
#[doc = "Quadrature Phase Error Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errsel {
    #[doc = "0: Quadrature Phase error Event interrupt forward to POSIFx.SR0"]
    Value1 = 0,
    #[doc = "1: Quadrature Phase error Event interrupt forward to POSIFx.SR1"]
    Value2 = 1,
}
impl From<Errsel> for bool {
    #[inline(always)]
    fn from(variant: Errsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRSEL` reader - Quadrature Phase Error Event Service Request Selector"]
pub type ErrselR = crate::BitReader<Errsel>;
impl ErrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errsel {
        match self.bits {
            false => Errsel::Value1,
            true => Errsel::Value2,
        }
    }
    #[doc = "Quadrature Phase error Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Errsel::Value1
    }
    #[doc = "Quadrature Phase error Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Errsel::Value2
    }
}
#[doc = "Field `ERRSEL` writer - Quadrature Phase Error Event Service Request Selector"]
pub type ErrselW<'a, REG> = crate::BitWriter<'a, REG, Errsel>;
impl<'a, REG> ErrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Quadrature Phase error Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Errsel::Value1)
    }
    #[doc = "Quadrature Phase error Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Errsel::Value2)
    }
}
#[doc = "Quadrature Clock Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cntsel {
    #[doc = "0: Quadrature Clock Event interrupt forward to POSIFx.SR0"]
    Value1 = 0,
    #[doc = "1: Quadrature Clock Event interrupt forward to POSIFx.SR1"]
    Value2 = 1,
}
impl From<Cntsel> for bool {
    #[inline(always)]
    fn from(variant: Cntsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTSEL` reader - Quadrature Clock Event Service Request Selector"]
pub type CntselR = crate::BitReader<Cntsel>;
impl CntselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cntsel {
        match self.bits {
            false => Cntsel::Value1,
            true => Cntsel::Value2,
        }
    }
    #[doc = "Quadrature Clock Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cntsel::Value1
    }
    #[doc = "Quadrature Clock Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cntsel::Value2
    }
}
#[doc = "Field `CNTSEL` writer - Quadrature Clock Event Service Request Selector"]
pub type CntselW<'a, REG> = crate::BitWriter<'a, REG, Cntsel>;
impl<'a, REG> CntselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Quadrature Clock Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cntsel::Value1)
    }
    #[doc = "Quadrature Clock Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cntsel::Value2)
    }
}
#[doc = "Quadrature Direction Update Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirsel {
    #[doc = "0: Quadrature Direction Update Event interrupt forward to POSIFx.SR0"]
    Value1 = 0,
    #[doc = "1: Quadrature Direction Update Event interrupt forward to POSIFx.SR1"]
    Value2 = 1,
}
impl From<Dirsel> for bool {
    #[inline(always)]
    fn from(variant: Dirsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSEL` reader - Quadrature Direction Update Event Service Request Selector"]
pub type DirselR = crate::BitReader<Dirsel>;
impl DirselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dirsel {
        match self.bits {
            false => Dirsel::Value1,
            true => Dirsel::Value2,
        }
    }
    #[doc = "Quadrature Direction Update Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dirsel::Value1
    }
    #[doc = "Quadrature Direction Update Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dirsel::Value2
    }
}
#[doc = "Field `DIRSEL` writer - Quadrature Direction Update Event Service Request Selector"]
pub type DirselW<'a, REG> = crate::BitWriter<'a, REG, Dirsel>;
impl<'a, REG> DirselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Quadrature Direction Update Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirsel::Value1)
    }
    #[doc = "Quadrature Direction Update Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dirsel::Value2)
    }
}
#[doc = "Quadrature Period clock Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pclsel {
    #[doc = "0: Quadrature Period clock Event interrupt forward to POSIFx.SR0"]
    Value1 = 0,
    #[doc = "1: Quadrature Period clock Event interrupt forward to POSIFx.SR1"]
    Value2 = 1,
}
impl From<Pclsel> for bool {
    #[inline(always)]
    fn from(variant: Pclsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCLSEL` reader - Quadrature Period clock Event Service Request Selector"]
pub type PclselR = crate::BitReader<Pclsel>;
impl PclselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pclsel {
        match self.bits {
            false => Pclsel::Value1,
            true => Pclsel::Value2,
        }
    }
    #[doc = "Quadrature Period clock Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pclsel::Value1
    }
    #[doc = "Quadrature Period clock Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pclsel::Value2
    }
}
#[doc = "Field `PCLSEL` writer - Quadrature Period clock Event Service Request Selector"]
pub type PclselW<'a, REG> = crate::BitWriter<'a, REG, Pclsel>;
impl<'a, REG> PclselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Quadrature Period clock Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pclsel::Value1)
    }
    #[doc = "Quadrature Period clock Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pclsel::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Correct Hall Event Enable"]
    #[inline(always)]
    pub fn eche(&self) -> EcheR {
        EcheR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wrong Hall Event Enable"]
    #[inline(always)]
    pub fn ewhe(&self) -> EwheR {
        EwheR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hall Input Update Enable"]
    #[inline(always)]
    pub fn ehie(&self) -> EhieR {
        EhieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Multi-Channel pattern shadow transfer enable"]
    #[inline(always)]
    pub fn emst(&self) -> EmstR {
        EmstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Quadrature Index Event Enable"]
    #[inline(always)]
    pub fn eindx(&self) -> EindxR {
        EindxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Quadrature Phase Error Enable"]
    #[inline(always)]
    pub fn eerr(&self) -> EerrR {
        EerrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Quadrature CLK interrupt Enable"]
    #[inline(always)]
    pub fn ecnt(&self) -> EcntR {
        EcntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Quadrature direction change interrupt Enable"]
    #[inline(always)]
    pub fn edir(&self) -> EdirR {
        EdirR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Quadrature Period CLK interrupt Enable"]
    #[inline(always)]
    pub fn epclk(&self) -> EpclkR {
        EpclkR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Correct Hall Event Service Request Selector"]
    #[inline(always)]
    pub fn chesel(&self) -> CheselR {
        CheselR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Wrong Hall Event Service Request Selector"]
    #[inline(always)]
    pub fn whesel(&self) -> WheselR {
        WheselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Hall Inputs Update Event Service Request Selector"]
    #[inline(always)]
    pub fn hiesel(&self) -> HieselR {
        HieselR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Multi-Channel pattern Update Event Service Request Selector"]
    #[inline(always)]
    pub fn mstsel(&self) -> MstselR {
        MstselR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Quadrature Index Event Service Request Selector"]
    #[inline(always)]
    pub fn indsel(&self) -> IndselR {
        IndselR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Quadrature Phase Error Event Service Request Selector"]
    #[inline(always)]
    pub fn errsel(&self) -> ErrselR {
        ErrselR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Quadrature Clock Event Service Request Selector"]
    #[inline(always)]
    pub fn cntsel(&self) -> CntselR {
        CntselR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Quadrature Direction Update Event Service Request Selector"]
    #[inline(always)]
    pub fn dirsel(&self) -> DirselR {
        DirselR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Quadrature Period clock Event Service Request Selector"]
    #[inline(always)]
    pub fn pclsel(&self) -> PclselR {
        PclselR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Correct Hall Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eche(&mut self) -> EcheW<PflgeSpec> {
        EcheW::new(self, 0)
    }
    #[doc = "Bit 1 - Wrong Hall Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ewhe(&mut self) -> EwheW<PflgeSpec> {
        EwheW::new(self, 1)
    }
    #[doc = "Bit 2 - Hall Input Update Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ehie(&mut self) -> EhieW<PflgeSpec> {
        EhieW::new(self, 2)
    }
    #[doc = "Bit 4 - Multi-Channel pattern shadow transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn emst(&mut self) -> EmstW<PflgeSpec> {
        EmstW::new(self, 4)
    }
    #[doc = "Bit 8 - Quadrature Index Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eindx(&mut self) -> EindxW<PflgeSpec> {
        EindxW::new(self, 8)
    }
    #[doc = "Bit 9 - Quadrature Phase Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eerr(&mut self) -> EerrW<PflgeSpec> {
        EerrW::new(self, 9)
    }
    #[doc = "Bit 10 - Quadrature CLK interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ecnt(&mut self) -> EcntW<PflgeSpec> {
        EcntW::new(self, 10)
    }
    #[doc = "Bit 11 - Quadrature direction change interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn edir(&mut self) -> EdirW<PflgeSpec> {
        EdirW::new(self, 11)
    }
    #[doc = "Bit 12 - Quadrature Period CLK interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epclk(&mut self) -> EpclkW<PflgeSpec> {
        EpclkW::new(self, 12)
    }
    #[doc = "Bit 16 - Correct Hall Event Service Request Selector"]
    #[inline(always)]
    #[must_use]
    pub fn chesel(&mut self) -> CheselW<PflgeSpec> {
        CheselW::new(self, 16)
    }
    #[doc = "Bit 17 - Wrong Hall Event Service Request Selector"]
    #[inline(always)]
    #[must_use]
    pub fn whesel(&mut self) -> WheselW<PflgeSpec> {
        WheselW::new(self, 17)
    }
    #[doc = "Bit 18 - Hall Inputs Update Event Service Request Selector"]
    #[inline(always)]
    #[must_use]
    pub fn hiesel(&mut self) -> HieselW<PflgeSpec> {
        HieselW::new(self, 18)
    }
    #[doc = "Bit 20 - Multi-Channel pattern Update Event Service Request Selector"]
    #[inline(always)]
    #[must_use]
    pub fn mstsel(&mut self) -> MstselW<PflgeSpec> {
        MstselW::new(self, 20)
    }
    #[doc = "Bit 24 - Quadrature Index Event Service Request Selector"]
    #[inline(always)]
    #[must_use]
    pub fn indsel(&mut self) -> IndselW<PflgeSpec> {
        IndselW::new(self, 24)
    }
    #[doc = "Bit 25 - Quadrature Phase Error Event Service Request Selector"]
    #[inline(always)]
    #[must_use]
    pub fn errsel(&mut self) -> ErrselW<PflgeSpec> {
        ErrselW::new(self, 25)
    }
    #[doc = "Bit 26 - Quadrature Clock Event Service Request Selector"]
    #[inline(always)]
    #[must_use]
    pub fn cntsel(&mut self) -> CntselW<PflgeSpec> {
        CntselW::new(self, 26)
    }
    #[doc = "Bit 27 - Quadrature Direction Update Event Service Request Selector"]
    #[inline(always)]
    #[must_use]
    pub fn dirsel(&mut self) -> DirselW<PflgeSpec> {
        DirselW::new(self, 27)
    }
    #[doc = "Bit 28 - Quadrature Period clock Event Service Request Selector"]
    #[inline(always)]
    #[must_use]
    pub fn pclsel(&mut self) -> PclselW<PflgeSpec> {
        PclselW::new(self, 28)
    }
}
#[doc = "POSIF Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pflge::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pflge::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PflgeSpec;
impl crate::RegisterSpec for PflgeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pflge::R`](R) reader structure"]
impl crate::Readable for PflgeSpec {}
#[doc = "`write(|w| ..)` method takes [`pflge::W`](W) writer structure"]
impl crate::Writable for PflgeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFLGE to value 0"]
impl crate::Resettable for PflgeSpec {
    const RESET_VALUE: u32 = 0;
}
