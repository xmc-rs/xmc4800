#[doc = "Register `PFLG` reader"]
pub type R = crate::R<PflgSpec>;
#[doc = "Correct Hall Event Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ches {
    #[doc = "0: Correct Hall Event not detected"]
    Value1 = 0,
    #[doc = "1: Correct Hall Event detected"]
    Value2 = 1,
}
impl From<Ches> for bool {
    #[inline(always)]
    fn from(variant: Ches) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHES` reader - Correct Hall Event Status"]
pub type ChesR = crate::BitReader<Ches>;
impl ChesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ches {
        match self.bits {
            false => Ches::Value1,
            true => Ches::Value2,
        }
    }
    #[doc = "Correct Hall Event not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ches::Value1
    }
    #[doc = "Correct Hall Event detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ches::Value2
    }
}
#[doc = "Wrong Hall Event Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Whes {
    #[doc = "0: Wrong Hall Event not detected"]
    Value1 = 0,
    #[doc = "1: Wrong Hall Event detected"]
    Value2 = 1,
}
impl From<Whes> for bool {
    #[inline(always)]
    fn from(variant: Whes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WHES` reader - Wrong Hall Event Status"]
pub type WhesR = crate::BitReader<Whes>;
impl WhesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Whes {
        match self.bits {
            false => Whes::Value1,
            true => Whes::Value2,
        }
    }
    #[doc = "Wrong Hall Event not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Whes::Value1
    }
    #[doc = "Wrong Hall Event detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Whes::Value2
    }
}
#[doc = "Hall Inputs Update Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hies {
    #[doc = "0: Transition on the Hall Inputs not detected"]
    Value1 = 0,
    #[doc = "1: Transition on the Hall Inputs detected"]
    Value2 = 1,
}
impl From<Hies> for bool {
    #[inline(always)]
    fn from(variant: Hies) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIES` reader - Hall Inputs Update Status"]
pub type HiesR = crate::BitReader<Hies>;
impl HiesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hies {
        match self.bits {
            false => Hies::Value1,
            true => Hies::Value2,
        }
    }
    #[doc = "Transition on the Hall Inputs not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hies::Value1
    }
    #[doc = "Transition on the Hall Inputs detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hies::Value2
    }
}
#[doc = "Multi-Channel pattern shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msts {
    #[doc = "0: Shadow transfer not done"]
    Value1 = 0,
    #[doc = "1: Shadow transfer done"]
    Value2 = 1,
}
impl From<Msts> for bool {
    #[inline(always)]
    fn from(variant: Msts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTS` reader - Multi-Channel pattern shadow transfer status"]
pub type MstsR = crate::BitReader<Msts>;
impl MstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msts {
        match self.bits {
            false => Msts::Value1,
            true => Msts::Value2,
        }
    }
    #[doc = "Shadow transfer not done"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Msts::Value1
    }
    #[doc = "Shadow transfer done"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Msts::Value2
    }
}
#[doc = "Quadrature Index Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Indxs {
    #[doc = "0: Index event not detected"]
    Value1 = 0,
    #[doc = "1: Index event detected"]
    Value2 = 1,
}
impl From<Indxs> for bool {
    #[inline(always)]
    fn from(variant: Indxs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INDXS` reader - Quadrature Index Status"]
pub type IndxsR = crate::BitReader<Indxs>;
impl IndxsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Indxs {
        match self.bits {
            false => Indxs::Value1,
            true => Indxs::Value2,
        }
    }
    #[doc = "Index event not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Indxs::Value1
    }
    #[doc = "Index event detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Indxs::Value2
    }
}
#[doc = "Quadrature Phase Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errs {
    #[doc = "0: Phase Error event not detected"]
    Value1 = 0,
    #[doc = "1: Phase Error event detected"]
    Value2 = 1,
}
impl From<Errs> for bool {
    #[inline(always)]
    fn from(variant: Errs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRS` reader - Quadrature Phase Error Status"]
pub type ErrsR = crate::BitReader<Errs>;
impl ErrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errs {
        match self.bits {
            false => Errs::Value1,
            true => Errs::Value2,
        }
    }
    #[doc = "Phase Error event not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Errs::Value1
    }
    #[doc = "Phase Error event detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Errs::Value2
    }
}
#[doc = "Quadrature CLK Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cnts {
    #[doc = "0: Quadrature clock not generated"]
    Value1 = 0,
    #[doc = "1: Quadrature clock generated"]
    Value2 = 1,
}
impl From<Cnts> for bool {
    #[inline(always)]
    fn from(variant: Cnts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTS` reader - Quadrature CLK Status"]
pub type CntsR = crate::BitReader<Cnts>;
impl CntsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cnts {
        match self.bits {
            false => Cnts::Value1,
            true => Cnts::Value2,
        }
    }
    #[doc = "Quadrature clock not generated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cnts::Value1
    }
    #[doc = "Quadrature clock generated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cnts::Value2
    }
}
#[doc = "Quadrature Direction Change\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirs {
    #[doc = "0: Change on direction not detected"]
    Value1 = 0,
    #[doc = "1: Change on direction detected"]
    Value2 = 1,
}
impl From<Dirs> for bool {
    #[inline(always)]
    fn from(variant: Dirs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRS` reader - Quadrature Direction Change"]
pub type DirsR = crate::BitReader<Dirs>;
impl DirsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dirs {
        match self.bits {
            false => Dirs::Value1,
            true => Dirs::Value2,
        }
    }
    #[doc = "Change on direction not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dirs::Value1
    }
    #[doc = "Change on direction detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dirs::Value2
    }
}
#[doc = "Quadrature Period Clk Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pclks {
    #[doc = "0: Period clock not generated"]
    Value1 = 0,
    #[doc = "1: Period clock generated"]
    Value2 = 1,
}
impl From<Pclks> for bool {
    #[inline(always)]
    fn from(variant: Pclks) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCLKS` reader - Quadrature Period Clk Status"]
pub type PclksR = crate::BitReader<Pclks>;
impl PclksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pclks {
        match self.bits {
            false => Pclks::Value1,
            true => Pclks::Value2,
        }
    }
    #[doc = "Period clock not generated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pclks::Value1
    }
    #[doc = "Period clock generated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pclks::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Correct Hall Event Status"]
    #[inline(always)]
    pub fn ches(&self) -> ChesR {
        ChesR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wrong Hall Event Status"]
    #[inline(always)]
    pub fn whes(&self) -> WhesR {
        WhesR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hall Inputs Update Status"]
    #[inline(always)]
    pub fn hies(&self) -> HiesR {
        HiesR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Multi-Channel pattern shadow transfer status"]
    #[inline(always)]
    pub fn msts(&self) -> MstsR {
        MstsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Quadrature Index Status"]
    #[inline(always)]
    pub fn indxs(&self) -> IndxsR {
        IndxsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Quadrature Phase Error Status"]
    #[inline(always)]
    pub fn errs(&self) -> ErrsR {
        ErrsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Quadrature CLK Status"]
    #[inline(always)]
    pub fn cnts(&self) -> CntsR {
        CntsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Quadrature Direction Change"]
    #[inline(always)]
    pub fn dirs(&self) -> DirsR {
        DirsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Quadrature Period Clk Status"]
    #[inline(always)]
    pub fn pclks(&self) -> PclksR {
        PclksR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "POSIF Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pflg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PflgSpec;
impl crate::RegisterSpec for PflgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pflg::R`](R) reader structure"]
impl crate::Readable for PflgSpec {}
#[doc = "`reset()` method sets PFLG to value 0"]
impl crate::Resettable for PflgSpec {
    const RESET_VALUE: u32 = 0;
}
