#[doc = "Register `CGCFG` reader"]
pub type R = crate::R<CgcfgSpec>;
#[doc = "Register `CGCFG` writer"]
pub type W = crate::W<CgcfgSpec>;
#[doc = "Carrier Generator Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cgmod {
    #[doc = "0: Stopped"]
    Value1 = 0,
    #[doc = "1: Square wave"]
    Value2 = 1,
    #[doc = "2: Triangle"]
    Value3 = 2,
    #[doc = "3: Sine wave"]
    Value4 = 3,
}
impl From<Cgmod> for u8 {
    #[inline(always)]
    fn from(variant: Cgmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cgmod {
    type Ux = u8;
}
impl crate::IsEnum for Cgmod {}
#[doc = "Field `CGMOD` reader - Carrier Generator Operating Mode"]
pub type CgmodR = crate::FieldReader<Cgmod>;
impl CgmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cgmod {
        match self.bits {
            0 => Cgmod::Value1,
            1 => Cgmod::Value2,
            2 => Cgmod::Value3,
            3 => Cgmod::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Stopped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cgmod::Value1
    }
    #[doc = "Square wave"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cgmod::Value2
    }
    #[doc = "Triangle"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cgmod::Value3
    }
    #[doc = "Sine wave"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cgmod::Value4
    }
}
#[doc = "Field `CGMOD` writer - Carrier Generator Operating Mode"]
pub type CgmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cgmod, crate::Safe>;
impl<'a, REG> CgmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Stopped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmod::Value1)
    }
    #[doc = "Square wave"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmod::Value2)
    }
    #[doc = "Triangle"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmod::Value3)
    }
    #[doc = "Sine wave"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmod::Value4)
    }
}
#[doc = "Bit-Reverse PWM Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brev {
    #[doc = "0: Normal mode"]
    Value1 = 0,
    #[doc = "1: Bit-reverse mode"]
    Value2 = 1,
}
impl From<Brev> for bool {
    #[inline(always)]
    fn from(variant: Brev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BREV` reader - Bit-Reverse PWM Generation"]
pub type BrevR = crate::BitReader<Brev>;
impl BrevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brev {
        match self.bits {
            false => Brev::Value1,
            true => Brev::Value2,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Brev::Value1
    }
    #[doc = "Bit-reverse mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Brev::Value2
    }
}
#[doc = "Field `BREV` writer - Bit-Reverse PWM Generation"]
pub type BrevW<'a, REG> = crate::BitWriter<'a, REG, Brev>;
impl<'a, REG> BrevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Brev::Value1)
    }
    #[doc = "Bit-reverse mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Brev::Value2)
    }
}
#[doc = "Signal Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sigpol {
    #[doc = "0: Normal: carrier signal begins with +1"]
    Value1 = 0,
    #[doc = "1: Inverted: carrier signal begins with -1"]
    Value2 = 1,
}
impl From<Sigpol> for bool {
    #[inline(always)]
    fn from(variant: Sigpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIGPOL` reader - Signal Polarity"]
pub type SigpolR = crate::BitReader<Sigpol>;
impl SigpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sigpol {
        match self.bits {
            false => Sigpol::Value1,
            true => Sigpol::Value2,
        }
    }
    #[doc = "Normal: carrier signal begins with +1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sigpol::Value1
    }
    #[doc = "Inverted: carrier signal begins with -1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sigpol::Value2
    }
}
#[doc = "Field `SIGPOL` writer - Signal Polarity"]
pub type SigpolW<'a, REG> = crate::BitWriter<'a, REG, Sigpol>;
impl<'a, REG> SigpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal: carrier signal begins with +1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sigpol::Value1)
    }
    #[doc = "Inverted: carrier signal begins with -1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sigpol::Value2)
    }
}
#[doc = "Divider Factor for the PWM Pattern Signal Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divcg {
    #[doc = "0: fCG = fCLK / 2"]
    Value1 = 0,
    #[doc = "1: fCG = fCLK / 4"]
    Value2 = 1,
    #[doc = "2: fCG = fCLK / 6"]
    Value3 = 2,
    #[doc = "15: fCG = fCLK / 32"]
    Value4 = 15,
}
impl From<Divcg> for u8 {
    #[inline(always)]
    fn from(variant: Divcg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Divcg {
    type Ux = u8;
}
impl crate::IsEnum for Divcg {}
#[doc = "Field `DIVCG` reader - Divider Factor for the PWM Pattern Signal Generator"]
pub type DivcgR = crate::FieldReader<Divcg>;
impl DivcgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Divcg> {
        match self.bits {
            0 => Some(Divcg::Value1),
            1 => Some(Divcg::Value2),
            2 => Some(Divcg::Value3),
            15 => Some(Divcg::Value4),
            _ => None,
        }
    }
    #[doc = "fCG = fCLK / 2"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Divcg::Value1
    }
    #[doc = "fCG = fCLK / 4"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Divcg::Value2
    }
    #[doc = "fCG = fCLK / 6"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Divcg::Value3
    }
    #[doc = "fCG = fCLK / 32"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Divcg::Value4
    }
}
#[doc = "Field `DIVCG` writer - Divider Factor for the PWM Pattern Signal Generator"]
pub type DivcgW<'a, REG> = crate::FieldWriter<'a, REG, 4, Divcg>;
impl<'a, REG> DivcgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fCG = fCLK / 2"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Divcg::Value1)
    }
    #[doc = "fCG = fCLK / 4"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Divcg::Value2)
    }
    #[doc = "fCG = fCLK / 6"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Divcg::Value3)
    }
    #[doc = "fCG = fCLK / 32"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Divcg::Value4)
    }
}
#[doc = "Run Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Run {
    #[doc = "0: Stopped (cleared at the end of a period)"]
    Value1 = 0,
    #[doc = "1: Running"]
    Value2 = 1,
}
impl From<Run> for bool {
    #[inline(always)]
    fn from(variant: Run) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN` reader - Run Indicator"]
pub type RunR = crate::BitReader<Run>;
impl RunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Run {
        match self.bits {
            false => Run::Value1,
            true => Run::Value2,
        }
    }
    #[doc = "Stopped (cleared at the end of a period)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Run::Value1
    }
    #[doc = "Running"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Run::Value2
    }
}
#[doc = "Field `BITCOUNT` reader - Bit Counter"]
pub type BitcountR = crate::FieldReader;
#[doc = "Field `STEPCOUNT` reader - Step Counter"]
pub type StepcountR = crate::FieldReader;
#[doc = "Step Counter Sign\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Steps {
    #[doc = "0: Step counter value is positive"]
    Value1 = 0,
    #[doc = "1: Step counter value is negative"]
    Value2 = 1,
}
impl From<Steps> for bool {
    #[inline(always)]
    fn from(variant: Steps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STEPS` reader - Step Counter Sign"]
pub type StepsR = crate::BitReader<Steps>;
impl StepsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Steps {
        match self.bits {
            false => Steps::Value1,
            true => Steps::Value2,
        }
    }
    #[doc = "Step counter value is positive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Steps::Value1
    }
    #[doc = "Step counter value is negative"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Steps::Value2
    }
}
#[doc = "Step Counter Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stepd {
    #[doc = "0: Step counter is counting up"]
    Value1 = 0,
    #[doc = "1: Step counter is counting down"]
    Value2 = 1,
}
impl From<Stepd> for bool {
    #[inline(always)]
    fn from(variant: Stepd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STEPD` reader - Step Counter Direction"]
pub type StepdR = crate::BitReader<Stepd>;
impl StepdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stepd {
        match self.bits {
            false => Stepd::Value1,
            true => Stepd::Value2,
        }
    }
    #[doc = "Step counter is counting up"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stepd::Value1
    }
    #[doc = "Step counter is counting down"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stepd::Value2
    }
}
#[doc = "Sign Signal from Carrier Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sgncg {
    #[doc = "0: Positive values"]
    Value1 = 0,
    #[doc = "1: Negative values"]
    Value2 = 1,
}
impl From<Sgncg> for bool {
    #[inline(always)]
    fn from(variant: Sgncg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SGNCG` reader - Sign Signal from Carrier Generator"]
pub type SgncgR = crate::BitReader<Sgncg>;
impl SgncgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sgncg {
        match self.bits {
            false => Sgncg::Value1,
            true => Sgncg::Value2,
        }
    }
    #[doc = "Positive values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sgncg::Value1
    }
    #[doc = "Negative values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sgncg::Value2
    }
}
impl R {
    #[doc = "Bits 0:1 - Carrier Generator Operating Mode"]
    #[inline(always)]
    pub fn cgmod(&self) -> CgmodR {
        CgmodR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Bit-Reverse PWM Generation"]
    #[inline(always)]
    pub fn brev(&self) -> BrevR {
        BrevR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Signal Polarity"]
    #[inline(always)]
    pub fn sigpol(&self) -> SigpolR {
        SigpolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Divider Factor for the PWM Pattern Signal Generator"]
    #[inline(always)]
    pub fn divcg(&self) -> DivcgR {
        DivcgR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Run Indicator"]
    #[inline(always)]
    pub fn run(&self) -> RunR {
        RunR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Bit Counter"]
    #[inline(always)]
    pub fn bitcount(&self) -> BitcountR {
        BitcountR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - Step Counter"]
    #[inline(always)]
    pub fn stepcount(&self) -> StepcountR {
        StepcountR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Step Counter Sign"]
    #[inline(always)]
    pub fn steps(&self) -> StepsR {
        StepsR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Step Counter Direction"]
    #[inline(always)]
    pub fn stepd(&self) -> StepdR {
        StepdR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Sign Signal from Carrier Generator"]
    #[inline(always)]
    pub fn sgncg(&self) -> SgncgR {
        SgncgR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Carrier Generator Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cgmod(&mut self) -> CgmodW<CgcfgSpec> {
        CgmodW::new(self, 0)
    }
    #[doc = "Bit 2 - Bit-Reverse PWM Generation"]
    #[inline(always)]
    #[must_use]
    pub fn brev(&mut self) -> BrevW<CgcfgSpec> {
        BrevW::new(self, 2)
    }
    #[doc = "Bit 3 - Signal Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn sigpol(&mut self) -> SigpolW<CgcfgSpec> {
        SigpolW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Divider Factor for the PWM Pattern Signal Generator"]
    #[inline(always)]
    #[must_use]
    pub fn divcg(&mut self) -> DivcgW<CgcfgSpec> {
        DivcgW::new(self, 4)
    }
}
#[doc = "Carrier Generator Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CgcfgSpec;
impl crate::RegisterSpec for CgcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgcfg::R`](R) reader structure"]
impl crate::Readable for CgcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cgcfg::W`](W) writer structure"]
impl crate::Writable for CgcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CGCFG to value 0x0710_0000"]
impl crate::Resettable for CgcfgSpec {
    const RESET_VALUE: u32 = 0x0710_0000;
}
