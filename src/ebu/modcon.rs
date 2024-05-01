#[doc = "Register `MODCON` reader"]
pub type R = crate::R<ModconSpec>;
#[doc = "Register `MODCON` writer"]
pub type W = crate::W<ModconSpec>;
#[doc = "Field `STS` reader - Memory Status Bit"]
pub type StsR = crate::BitReader;
#[doc = "Field `LCKABRT` reader - Lock Abort"]
pub type LckabrtR = crate::BitReader;
#[doc = "SDRAM Tristate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdtri {
    #[doc = "0: SDRAM control signals are driven by the EBU when the EBU does not own the external bus. SDRAM cannot be shared."]
    Value1 = 0,
    #[doc = "1: SDRAM control signals are tri-stated by the EBU when the EBU does not own the external bus. The SDRAM can be shared."]
    Value2 = 1,
}
impl From<Sdtri> for bool {
    #[inline(always)]
    fn from(variant: Sdtri) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDTRI` reader - SDRAM Tristate"]
pub type SdtriR = crate::BitReader<Sdtri>;
impl SdtriR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdtri {
        match self.bits {
            false => Sdtri::Value1,
            true => Sdtri::Value2,
        }
    }
    #[doc = "SDRAM control signals are driven by the EBU when the EBU does not own the external bus. SDRAM cannot be shared."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sdtri::Value1
    }
    #[doc = "SDRAM control signals are tri-stated by the EBU when the EBU does not own the external bus. The SDRAM can be shared."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sdtri::Value2
    }
}
#[doc = "Field `SDTRI` writer - SDRAM Tristate"]
pub type SdtriW<'a, REG> = crate::BitWriter<'a, REG, Sdtri>;
impl<'a, REG> SdtriW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDRAM control signals are driven by the EBU when the EBU does not own the external bus. SDRAM cannot be shared."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdtri::Value1)
    }
    #[doc = "SDRAM control signals are tri-stated by the EBU when the EBU does not own the external bus. The SDRAM can be shared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sdtri::Value2)
    }
}
#[doc = "External Bus Lock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extlock {
    #[doc = "0: External bus is not locked after the EBU gains ownership"]
    Value1 = 0,
    #[doc = "1: External bus is locked after the EBU gains ownership"]
    Value2 = 1,
}
impl From<Extlock> for bool {
    #[inline(always)]
    fn from(variant: Extlock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTLOCK` reader - External Bus Lock Control"]
pub type ExtlockR = crate::BitReader<Extlock>;
impl ExtlockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extlock {
        match self.bits {
            false => Extlock::Value1,
            true => Extlock::Value2,
        }
    }
    #[doc = "External bus is not locked after the EBU gains ownership"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Extlock::Value1
    }
    #[doc = "External bus is locked after the EBU gains ownership"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Extlock::Value2
    }
}
#[doc = "Field `EXTLOCK` writer - External Bus Lock Control"]
pub type ExtlockW<'a, REG> = crate::BitWriter<'a, REG, Extlock>;
impl<'a, REG> ExtlockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External bus is not locked after the EBU gains ownership"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Extlock::Value1)
    }
    #[doc = "External bus is locked after the EBU gains ownership"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Extlock::Value2)
    }
}
#[doc = "Arbitration Signal Synchronization Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arbsync {
    #[doc = "0: Arbitration inputs are synchronous"]
    Value1 = 0,
    #[doc = "1: Arbitration inputs are asynchronous"]
    Value2 = 1,
}
impl From<Arbsync> for bool {
    #[inline(always)]
    fn from(variant: Arbsync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARBSYNC` reader - Arbitration Signal Synchronization Control"]
pub type ArbsyncR = crate::BitReader<Arbsync>;
impl ArbsyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arbsync {
        match self.bits {
            false => Arbsync::Value1,
            true => Arbsync::Value2,
        }
    }
    #[doc = "Arbitration inputs are synchronous"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Arbsync::Value1
    }
    #[doc = "Arbitration inputs are asynchronous"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Arbsync::Value2
    }
}
#[doc = "Field `ARBSYNC` writer - Arbitration Signal Synchronization Control"]
pub type ArbsyncW<'a, REG> = crate::BitWriter<'a, REG, Arbsync>;
impl<'a, REG> ArbsyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Arbitration inputs are synchronous"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Arbsync::Value1)
    }
    #[doc = "Arbitration inputs are asynchronous"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Arbsync::Value2)
    }
}
#[doc = "Arbitration Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Arbmode {
    #[doc = "0: No Bus arbitration mode selected"]
    Value1 = 0,
    #[doc = "1: Arbiter Mode arbitration mode selected"]
    Value2 = 1,
    #[doc = "2: Participant arbitration mode selected"]
    Value3 = 2,
    #[doc = "3: Sole Master arbitration mode selected"]
    Value4 = 3,
}
impl From<Arbmode> for u8 {
    #[inline(always)]
    fn from(variant: Arbmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Arbmode {
    type Ux = u8;
}
impl crate::IsEnum for Arbmode {}
#[doc = "Field `ARBMODE` reader - Arbitration Mode Selection"]
pub type ArbmodeR = crate::FieldReader<Arbmode>;
impl ArbmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arbmode {
        match self.bits {
            0 => Arbmode::Value1,
            1 => Arbmode::Value2,
            2 => Arbmode::Value3,
            3 => Arbmode::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "No Bus arbitration mode selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Arbmode::Value1
    }
    #[doc = "Arbiter Mode arbitration mode selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Arbmode::Value2
    }
    #[doc = "Participant arbitration mode selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Arbmode::Value3
    }
    #[doc = "Sole Master arbitration mode selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Arbmode::Value4
    }
}
#[doc = "Field `ARBMODE` writer - Arbitration Mode Selection"]
pub type ArbmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Arbmode, crate::Safe>;
impl<'a, REG> ArbmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Bus arbitration mode selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Arbmode::Value1)
    }
    #[doc = "Arbiter Mode arbitration mode selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Arbmode::Value2)
    }
    #[doc = "Participant arbitration mode selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Arbmode::Value3)
    }
    #[doc = "Sole Master arbitration mode selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Arbmode::Value4)
    }
}
#[doc = "Bus Time-out Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timeoutc {
    #[doc = "0: Time-out is disabled."]
    Value1 = 0,
    #[doc = "1: Time-out is generated after 1 8 clock cycles."]
    Value2 = 1,
    #[doc = "255: Time-out is generated after 255 8 clock cycles."]
    Value3 = 255,
}
impl From<Timeoutc> for u8 {
    #[inline(always)]
    fn from(variant: Timeoutc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timeoutc {
    type Ux = u8;
}
impl crate::IsEnum for Timeoutc {}
#[doc = "Field `TIMEOUTC` reader - Bus Time-out Control"]
pub type TimeoutcR = crate::FieldReader<Timeoutc>;
impl TimeoutcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Timeoutc> {
        match self.bits {
            0 => Some(Timeoutc::Value1),
            1 => Some(Timeoutc::Value2),
            255 => Some(Timeoutc::Value3),
            _ => None,
        }
    }
    #[doc = "Time-out is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Timeoutc::Value1
    }
    #[doc = "Time-out is generated after 1 8 clock cycles."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Timeoutc::Value2
    }
    #[doc = "Time-out is generated after 255 8 clock cycles."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Timeoutc::Value3
    }
}
#[doc = "Field `TIMEOUTC` writer - Bus Time-out Control"]
pub type TimeoutcW<'a, REG> = crate::FieldWriter<'a, REG, 8, Timeoutc>;
impl<'a, REG> TimeoutcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Time-out is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutc::Value1)
    }
    #[doc = "Time-out is generated after 1 8 clock cycles."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutc::Value2)
    }
    #[doc = "Time-out is generated after 255 8 clock cycles."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutc::Value3)
    }
}
#[doc = "Field `LOCKTIMEOUT` reader - Lock Timeout Counter Preload"]
pub type LocktimeoutR = crate::FieldReader;
#[doc = "Field `LOCKTIMEOUT` writer - Lock Timeout Counter Preload"]
pub type LocktimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GLOBALCS` reader - Global Chip Select Enable"]
pub type GlobalcsR = crate::FieldReader;
#[doc = "Field `GLOBALCS` writer - Global Chip Select Enable"]
pub type GlobalcsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACCSINH` reader - Access Inhibit request"]
pub type AccsinhR = crate::BitReader;
#[doc = "Field `ACCSINH` writer - Access Inhibit request"]
pub type AccsinhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCSINHACK` reader - Access inhibit acknowledge"]
pub type AccsinhackR = crate::BitReader;
#[doc = "ALE Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ale {
    #[doc = "0: Output is ADV"]
    Value1 = 0,
    #[doc = "1: Output is ALE"]
    Value2 = 1,
}
impl From<Ale> for bool {
    #[inline(always)]
    fn from(variant: Ale) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALE` reader - ALE Mode"]
pub type AleR = crate::BitReader<Ale>;
impl AleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ale {
        match self.bits {
            false => Ale::Value1,
            true => Ale::Value2,
        }
    }
    #[doc = "Output is ADV"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ale::Value1
    }
    #[doc = "Output is ALE"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ale::Value2
    }
}
#[doc = "Field `ALE` writer - ALE Mode"]
pub type AleW<'a, REG> = crate::BitWriter<'a, REG, Ale>;
impl<'a, REG> AleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is ADV"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ale::Value1)
    }
    #[doc = "Output is ALE"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ale::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Memory Status Bit"]
    #[inline(always)]
    pub fn sts(&self) -> StsR {
        StsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Abort"]
    #[inline(always)]
    pub fn lckabrt(&self) -> LckabrtR {
        LckabrtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDRAM Tristate"]
    #[inline(always)]
    pub fn sdtri(&self) -> SdtriR {
        SdtriR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - External Bus Lock Control"]
    #[inline(always)]
    pub fn extlock(&self) -> ExtlockR {
        ExtlockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Arbitration Signal Synchronization Control"]
    #[inline(always)]
    pub fn arbsync(&self) -> ArbsyncR {
        ArbsyncR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Arbitration Mode Selection"]
    #[inline(always)]
    pub fn arbmode(&self) -> ArbmodeR {
        ArbmodeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Bus Time-out Control"]
    #[inline(always)]
    pub fn timeoutc(&self) -> TimeoutcR {
        TimeoutcR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Lock Timeout Counter Preload"]
    #[inline(always)]
    pub fn locktimeout(&self) -> LocktimeoutR {
        LocktimeoutR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Global Chip Select Enable"]
    #[inline(always)]
    pub fn globalcs(&self) -> GlobalcsR {
        GlobalcsR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Access Inhibit request"]
    #[inline(always)]
    pub fn accsinh(&self) -> AccsinhR {
        AccsinhR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Access inhibit acknowledge"]
    #[inline(always)]
    pub fn accsinhack(&self) -> AccsinhackR {
        AccsinhackR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - ALE Mode"]
    #[inline(always)]
    pub fn ale(&self) -> AleR {
        AleR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SDRAM Tristate"]
    #[inline(always)]
    #[must_use]
    pub fn sdtri(&mut self) -> SdtriW<ModconSpec> {
        SdtriW::new(self, 2)
    }
    #[doc = "Bit 4 - External Bus Lock Control"]
    #[inline(always)]
    #[must_use]
    pub fn extlock(&mut self) -> ExtlockW<ModconSpec> {
        ExtlockW::new(self, 4)
    }
    #[doc = "Bit 5 - Arbitration Signal Synchronization Control"]
    #[inline(always)]
    #[must_use]
    pub fn arbsync(&mut self) -> ArbsyncW<ModconSpec> {
        ArbsyncW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Arbitration Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn arbmode(&mut self) -> ArbmodeW<ModconSpec> {
        ArbmodeW::new(self, 6)
    }
    #[doc = "Bits 8:15 - Bus Time-out Control"]
    #[inline(always)]
    #[must_use]
    pub fn timeoutc(&mut self) -> TimeoutcW<ModconSpec> {
        TimeoutcW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Lock Timeout Counter Preload"]
    #[inline(always)]
    #[must_use]
    pub fn locktimeout(&mut self) -> LocktimeoutW<ModconSpec> {
        LocktimeoutW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Global Chip Select Enable"]
    #[inline(always)]
    #[must_use]
    pub fn globalcs(&mut self) -> GlobalcsW<ModconSpec> {
        GlobalcsW::new(self, 24)
    }
    #[doc = "Bit 28 - Access Inhibit request"]
    #[inline(always)]
    #[must_use]
    pub fn accsinh(&mut self) -> AccsinhW<ModconSpec> {
        AccsinhW::new(self, 28)
    }
    #[doc = "Bit 31 - ALE Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ale(&mut self) -> AleW<ModconSpec> {
        AleW::new(self, 31)
    }
}
#[doc = "EBU Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modcon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modcon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModconSpec;
impl crate::RegisterSpec for ModconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modcon::R`](R) reader structure"]
impl crate::Readable for ModconSpec {}
#[doc = "`write(|w| ..)` method takes [`modcon::W`](W) writer structure"]
impl crate::Writable for ModconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODCON to value 0x20"]
impl crate::Resettable for ModconSpec {
    const RESET_VALUE: u32 = 0x20;
}
