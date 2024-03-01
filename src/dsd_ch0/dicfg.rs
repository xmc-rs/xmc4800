#[doc = "Register `DICFG` reader"]
pub type R = crate::R<DicfgSpec>;
#[doc = "Register `DICFG` writer"]
pub type W = crate::W<DicfgSpec>;
#[doc = "Input Data Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dsrc {
    #[doc = "0: Disconnected"]
    Value1 = 0,
    #[doc = "2: External, from input A, direct"]
    Value2 = 2,
    #[doc = "3: External, from input A, inverted"]
    Value3 = 3,
    #[doc = "4: External, from input B, direct"]
    Value4 = 4,
    #[doc = "5: External, from input B, inverted"]
    Value5 = 5,
}
impl From<Dsrc> for u8 {
    #[inline(always)]
    fn from(variant: Dsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dsrc {
    type Ux = u8;
}
#[doc = "Field `DSRC` reader - Input Data Source Select"]
pub type DsrcR = crate::FieldReader<Dsrc>;
impl DsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dsrc> {
        match self.bits {
            0 => Some(Dsrc::Value1),
            2 => Some(Dsrc::Value2),
            3 => Some(Dsrc::Value3),
            4 => Some(Dsrc::Value4),
            5 => Some(Dsrc::Value5),
            _ => None,
        }
    }
    #[doc = "Disconnected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dsrc::Value1
    }
    #[doc = "External, from input A, direct"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dsrc::Value2
    }
    #[doc = "External, from input A, inverted"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Dsrc::Value3
    }
    #[doc = "External, from input B, direct"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Dsrc::Value4
    }
    #[doc = "External, from input B, inverted"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Dsrc::Value5
    }
}
#[doc = "Field `DSRC` writer - Input Data Source Select"]
pub type DsrcW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dsrc>;
impl<'a, REG> DsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disconnected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsrc::Value1)
    }
    #[doc = "External, from input A, direct"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dsrc::Value2)
    }
    #[doc = "External, from input A, inverted"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Dsrc::Value3)
    }
    #[doc = "External, from input B, direct"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Dsrc::Value4)
    }
    #[doc = "External, from input B, inverted"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Dsrc::Value5)
    }
}
#[doc = "Write Control for Data Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dswc {
    #[doc = "0: No write access to data parameters"]
    Value1 = 0,
    #[doc = "1: Bitfield DSRC can be written"]
    Value2 = 1,
}
impl From<Dswc> for bool {
    #[inline(always)]
    fn from(variant: Dswc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSWC` writer - Write Control for Data Selection"]
pub type DswcW<'a, REG> = crate::BitWriter<'a, REG, Dswc>;
impl<'a, REG> DswcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write access to data parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dswc::Value1)
    }
    #[doc = "Bitfield DSRC can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dswc::Value2)
    }
}
#[doc = "Integrator Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Itrmode {
    #[doc = "0: No integration trigger, integrator bypassed, INTEN = 0 all the time"]
    Value1 = 0,
    #[doc = "1: Trigger event upon a falling edge"]
    Value2 = 1,
    #[doc = "2: Trigger event upon a rising edge"]
    Value3 = 2,
    #[doc = "3: No trigger, integrator active all the time, INTEN = 1 all the time"]
    Value4 = 3,
}
impl From<Itrmode> for u8 {
    #[inline(always)]
    fn from(variant: Itrmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Itrmode {
    type Ux = u8;
}
#[doc = "Field `ITRMODE` reader - Integrator Trigger Mode"]
pub type ItrmodeR = crate::FieldReader<Itrmode>;
impl ItrmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Itrmode {
        match self.bits {
            0 => Itrmode::Value1,
            1 => Itrmode::Value2,
            2 => Itrmode::Value3,
            3 => Itrmode::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "No integration trigger, integrator bypassed, INTEN = 0 all the time"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Itrmode::Value1
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Itrmode::Value2
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Itrmode::Value3
    }
    #[doc = "No trigger, integrator active all the time, INTEN = 1 all the time"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Itrmode::Value4
    }
}
#[doc = "Field `ITRMODE` writer - Integrator Trigger Mode"]
pub type ItrmodeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Itrmode>;
impl<'a, REG> ItrmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No integration trigger, integrator bypassed, INTEN = 0 all the time"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Itrmode::Value1)
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Itrmode::Value2)
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Itrmode::Value3)
    }
    #[doc = "No trigger, integrator active all the time, INTEN = 1 all the time"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Itrmode::Value4)
    }
}
#[doc = "Timestamp Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tstrmode {
    #[doc = "0: No timestamp trigger"]
    Value1 = 0,
    #[doc = "1: Trigger event upon a falling edge"]
    Value2 = 1,
    #[doc = "2: Trigger event upon a rising edge"]
    Value3 = 2,
    #[doc = "3: Trigger event upon each edge"]
    Value4 = 3,
}
impl From<Tstrmode> for u8 {
    #[inline(always)]
    fn from(variant: Tstrmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tstrmode {
    type Ux = u8;
}
#[doc = "Field `TSTRMODE` reader - Timestamp Trigger Mode"]
pub type TstrmodeR = crate::FieldReader<Tstrmode>;
impl TstrmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstrmode {
        match self.bits {
            0 => Tstrmode::Value1,
            1 => Tstrmode::Value2,
            2 => Tstrmode::Value3,
            3 => Tstrmode::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "No timestamp trigger"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tstrmode::Value1
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tstrmode::Value2
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Tstrmode::Value3
    }
    #[doc = "Trigger event upon each edge"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Tstrmode::Value4
    }
}
#[doc = "Field `TSTRMODE` writer - Timestamp Trigger Mode"]
pub type TstrmodeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Tstrmode>;
impl<'a, REG> TstrmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No timestamp trigger"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tstrmode::Value1)
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tstrmode::Value2)
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Tstrmode::Value3)
    }
    #[doc = "Trigger event upon each edge"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Tstrmode::Value4)
    }
}
#[doc = "Field `TRSEL` reader - Trigger Select"]
pub type TrselR = crate::FieldReader;
#[doc = "Field `TRSEL` writer - Trigger Select"]
pub type TrselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Write Control for Trigger Parameters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trwc {
    #[doc = "0: No write access to trigger parameters"]
    Value1 = 0,
    #[doc = "1: Bitfields TRSEL, TSTRMODE, ITRMODE can be written"]
    Value2 = 1,
}
impl From<Trwc> for bool {
    #[inline(always)]
    fn from(variant: Trwc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRWC` writer - Write Control for Trigger Parameters"]
pub type TrwcW<'a, REG> = crate::BitWriter<'a, REG, Trwc>;
impl<'a, REG> TrwcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write access to trigger parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Trwc::Value1)
    }
    #[doc = "Bitfields TRSEL, TSTRMODE, ITRMODE can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Trwc::Value2)
    }
}
#[doc = "Sample Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csrc {
    #[doc = "1: External, from input A"]
    Value2 = 1,
    #[doc = "2: External, from input B"]
    Value3 = 2,
    #[doc = "3: External, from input C"]
    Value4 = 3,
    #[doc = "4: External, from input D"]
    Value5 = 4,
    #[doc = "15: Internal clock"]
    Value6 = 15,
}
impl From<Csrc> for u8 {
    #[inline(always)]
    fn from(variant: Csrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csrc {
    type Ux = u8;
}
#[doc = "Field `CSRC` reader - Sample Clock Source Select"]
pub type CsrcR = crate::FieldReader<Csrc>;
impl CsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Csrc> {
        match self.bits {
            1 => Some(Csrc::Value2),
            2 => Some(Csrc::Value3),
            3 => Some(Csrc::Value4),
            4 => Some(Csrc::Value5),
            15 => Some(Csrc::Value6),
            _ => None,
        }
    }
    #[doc = "External, from input A"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Csrc::Value2
    }
    #[doc = "External, from input B"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Csrc::Value3
    }
    #[doc = "External, from input C"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Csrc::Value4
    }
    #[doc = "External, from input D"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Csrc::Value5
    }
    #[doc = "Internal clock"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Csrc::Value6
    }
}
#[doc = "Field `CSRC` writer - Sample Clock Source Select"]
pub type CsrcW<'a, REG> = crate::FieldWriter<'a, REG, 4, Csrc>;
impl<'a, REG> CsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External, from input A"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Csrc::Value2)
    }
    #[doc = "External, from input B"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Csrc::Value3)
    }
    #[doc = "External, from input C"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Csrc::Value4)
    }
    #[doc = "External, from input D"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Csrc::Value5)
    }
    #[doc = "Internal clock"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Csrc::Value6)
    }
}
#[doc = "Data Strobe Generatoion Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Strobe {
    #[doc = "0: No data strobe"]
    Value1 = 0,
    #[doc = "1: Direct clock, a sample trigger is generated at each rising clock edge"]
    Value2 = 1,
    #[doc = "2: Direct clock, a sample trigger is generated at each falling clock edge"]
    Value3 = 2,
    #[doc = "3: Double data, a sample trigger is generated at each rising and falling clock edge"]
    Value4 = 3,
    #[doc = "5: Double clock, a sample trigger is generated at every 2nd rising clock edge"]
    Value6 = 5,
    #[doc = "6: Double clock, a sample trigger is generated at every 2nd falling clock edge"]
    Value7 = 6,
}
impl From<Strobe> for u8 {
    #[inline(always)]
    fn from(variant: Strobe) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Strobe {
    type Ux = u8;
}
#[doc = "Field `STROBE` reader - Data Strobe Generatoion Mode"]
pub type StrobeR = crate::FieldReader<Strobe>;
impl StrobeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Strobe> {
        match self.bits {
            0 => Some(Strobe::Value1),
            1 => Some(Strobe::Value2),
            2 => Some(Strobe::Value3),
            3 => Some(Strobe::Value4),
            5 => Some(Strobe::Value6),
            6 => Some(Strobe::Value7),
            _ => None,
        }
    }
    #[doc = "No data strobe"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Strobe::Value1
    }
    #[doc = "Direct clock, a sample trigger is generated at each rising clock edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Strobe::Value2
    }
    #[doc = "Direct clock, a sample trigger is generated at each falling clock edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Strobe::Value3
    }
    #[doc = "Double data, a sample trigger is generated at each rising and falling clock edge"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Strobe::Value4
    }
    #[doc = "Double clock, a sample trigger is generated at every 2nd rising clock edge"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Strobe::Value6
    }
    #[doc = "Double clock, a sample trigger is generated at every 2nd falling clock edge"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Strobe::Value7
    }
}
#[doc = "Field `STROBE` writer - Data Strobe Generatoion Mode"]
pub type StrobeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Strobe>;
impl<'a, REG> StrobeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No data strobe"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Strobe::Value1)
    }
    #[doc = "Direct clock, a sample trigger is generated at each rising clock edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Strobe::Value2)
    }
    #[doc = "Direct clock, a sample trigger is generated at each falling clock edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Strobe::Value3)
    }
    #[doc = "Double data, a sample trigger is generated at each rising and falling clock edge"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Strobe::Value4)
    }
    #[doc = "Double clock, a sample trigger is generated at every 2nd rising clock edge"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Strobe::Value6)
    }
    #[doc = "Double clock, a sample trigger is generated at every 2nd falling clock edge"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Strobe::Value7)
    }
}
#[doc = "Write Control for Strobe/Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scwc {
    #[doc = "0: No write access to strobe/clock parameters"]
    Value1 = 0,
    #[doc = "1: Bitfields STROBE, CSRC can be written"]
    Value2 = 1,
}
impl From<Scwc> for bool {
    #[inline(always)]
    fn from(variant: Scwc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCWC` writer - Write Control for Strobe/Clock Selection"]
pub type ScwcW<'a, REG> = crate::BitWriter<'a, REG, Scwc>;
impl<'a, REG> ScwcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write access to strobe/clock parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Scwc::Value1)
    }
    #[doc = "Bitfields STROBE, CSRC can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Scwc::Value2)
    }
}
impl R {
    #[doc = "Bits 0:3 - Input Data Source Select"]
    #[inline(always)]
    pub fn dsrc(&self) -> DsrcR {
        DsrcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Integrator Trigger Mode"]
    #[inline(always)]
    pub fn itrmode(&self) -> ItrmodeR {
        ItrmodeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Timestamp Trigger Mode"]
    #[inline(always)]
    pub fn tstrmode(&self) -> TstrmodeR {
        TstrmodeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Trigger Select"]
    #[inline(always)]
    pub fn trsel(&self) -> TrselR {
        TrselR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Sample Clock Source Select"]
    #[inline(always)]
    pub fn csrc(&self) -> CsrcR {
        CsrcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Data Strobe Generatoion Mode"]
    #[inline(always)]
    pub fn strobe(&self) -> StrobeR {
        StrobeR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input Data Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn dsrc(&mut self) -> DsrcW<DicfgSpec> {
        DsrcW::new(self, 0)
    }
    #[doc = "Bit 7 - Write Control for Data Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dswc(&mut self) -> DswcW<DicfgSpec> {
        DswcW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Integrator Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn itrmode(&mut self) -> ItrmodeW<DicfgSpec> {
        ItrmodeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Timestamp Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tstrmode(&mut self) -> TstrmodeW<DicfgSpec> {
        TstrmodeW::new(self, 10)
    }
    #[doc = "Bits 12:14 - Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn trsel(&mut self) -> TrselW<DicfgSpec> {
        TrselW::new(self, 12)
    }
    #[doc = "Bit 15 - Write Control for Trigger Parameters"]
    #[inline(always)]
    #[must_use]
    pub fn trwc(&mut self) -> TrwcW<DicfgSpec> {
        TrwcW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Sample Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn csrc(&mut self) -> CsrcW<DicfgSpec> {
        CsrcW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Data Strobe Generatoion Mode"]
    #[inline(always)]
    #[must_use]
    pub fn strobe(&mut self) -> StrobeW<DicfgSpec> {
        StrobeW::new(self, 20)
    }
    #[doc = "Bit 31 - Write Control for Strobe/Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn scwc(&mut self) -> ScwcW<DicfgSpec> {
        ScwcW::new(self, 31)
    }
}
#[doc = "Demodulator Input Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dicfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dicfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DicfgSpec;
impl crate::RegisterSpec for DicfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dicfg::R`](R) reader structure"]
impl crate::Readable for DicfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dicfg::W`](W) writer structure"]
impl crate::Writable for DicfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DICFG to value 0"]
impl crate::Resettable for DicfgSpec {
    const RESET_VALUE: u32 = 0;
}
