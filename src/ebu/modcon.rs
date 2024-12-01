#[doc = "Register `MODCON` reader"]
pub type R = crate::R<MODCON_SPEC>;
#[doc = "Register `MODCON` writer"]
pub type W = crate::W<MODCON_SPEC>;
#[doc = "Field `STS` reader - Memory Status Bit"]
pub type STS_R = crate::BitReader;
#[doc = "Field `LCKABRT` reader - Lock Abort"]
pub type LCKABRT_R = crate::BitReader;
#[doc = "SDRAM Tristate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDTRI_A {
    #[doc = "0: SDRAM control signals are driven by the EBU when the EBU does not own the external bus. SDRAM cannot be shared."]
    VALUE1 = 0,
    #[doc = "1: SDRAM control signals are tri-stated by the EBU when the EBU does not own the external bus. The SDRAM can be shared."]
    VALUE2 = 1,
}
impl From<SDTRI_A> for bool {
    #[inline(always)]
    fn from(variant: SDTRI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDTRI` reader - SDRAM Tristate"]
pub type SDTRI_R = crate::BitReader<SDTRI_A>;
impl SDTRI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDTRI_A {
        match self.bits {
            false => SDTRI_A::VALUE1,
            true => SDTRI_A::VALUE2,
        }
    }
    #[doc = "SDRAM control signals are driven by the EBU when the EBU does not own the external bus. SDRAM cannot be shared."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDTRI_A::VALUE1
    }
    #[doc = "SDRAM control signals are tri-stated by the EBU when the EBU does not own the external bus. The SDRAM can be shared."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SDTRI_A::VALUE2
    }
}
#[doc = "Field `SDTRI` writer - SDRAM Tristate"]
pub type SDTRI_W<'a, REG> = crate::BitWriter<'a, REG, SDTRI_A>;
impl<'a, REG> SDTRI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDRAM control signals are driven by the EBU when the EBU does not own the external bus. SDRAM cannot be shared."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SDTRI_A::VALUE1)
    }
    #[doc = "SDRAM control signals are tri-stated by the EBU when the EBU does not own the external bus. The SDRAM can be shared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SDTRI_A::VALUE2)
    }
}
#[doc = "External Bus Lock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTLOCK_A {
    #[doc = "0: External bus is not locked after the EBU gains ownership"]
    VALUE1 = 0,
    #[doc = "1: External bus is locked after the EBU gains ownership"]
    VALUE2 = 1,
}
impl From<EXTLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: EXTLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTLOCK` reader - External Bus Lock Control"]
pub type EXTLOCK_R = crate::BitReader<EXTLOCK_A>;
impl EXTLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTLOCK_A {
        match self.bits {
            false => EXTLOCK_A::VALUE1,
            true => EXTLOCK_A::VALUE2,
        }
    }
    #[doc = "External bus is not locked after the EBU gains ownership"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EXTLOCK_A::VALUE1
    }
    #[doc = "External bus is locked after the EBU gains ownership"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EXTLOCK_A::VALUE2
    }
}
#[doc = "Field `EXTLOCK` writer - External Bus Lock Control"]
pub type EXTLOCK_W<'a, REG> = crate::BitWriter<'a, REG, EXTLOCK_A>;
impl<'a, REG> EXTLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External bus is not locked after the EBU gains ownership"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTLOCK_A::VALUE1)
    }
    #[doc = "External bus is locked after the EBU gains ownership"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTLOCK_A::VALUE2)
    }
}
#[doc = "Arbitration Signal Synchronization Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARBSYNC_A {
    #[doc = "0: Arbitration inputs are synchronous"]
    VALUE1 = 0,
    #[doc = "1: Arbitration inputs are asynchronous"]
    VALUE2 = 1,
}
impl From<ARBSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: ARBSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARBSYNC` reader - Arbitration Signal Synchronization Control"]
pub type ARBSYNC_R = crate::BitReader<ARBSYNC_A>;
impl ARBSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARBSYNC_A {
        match self.bits {
            false => ARBSYNC_A::VALUE1,
            true => ARBSYNC_A::VALUE2,
        }
    }
    #[doc = "Arbitration inputs are synchronous"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ARBSYNC_A::VALUE1
    }
    #[doc = "Arbitration inputs are asynchronous"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ARBSYNC_A::VALUE2
    }
}
#[doc = "Field `ARBSYNC` writer - Arbitration Signal Synchronization Control"]
pub type ARBSYNC_W<'a, REG> = crate::BitWriter<'a, REG, ARBSYNC_A>;
impl<'a, REG> ARBSYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Arbitration inputs are synchronous"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ARBSYNC_A::VALUE1)
    }
    #[doc = "Arbitration inputs are asynchronous"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ARBSYNC_A::VALUE2)
    }
}
#[doc = "Arbitration Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARBMODE_A {
    #[doc = "0: No Bus arbitration mode selected"]
    VALUE1 = 0,
    #[doc = "1: Arbiter Mode arbitration mode selected"]
    VALUE2 = 1,
    #[doc = "2: Participant arbitration mode selected"]
    VALUE3 = 2,
    #[doc = "3: Sole Master arbitration mode selected"]
    VALUE4 = 3,
}
impl From<ARBMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ARBMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ARBMODE_A {
    type Ux = u8;
}
impl crate::IsEnum for ARBMODE_A {}
#[doc = "Field `ARBMODE` reader - Arbitration Mode Selection"]
pub type ARBMODE_R = crate::FieldReader<ARBMODE_A>;
impl ARBMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARBMODE_A {
        match self.bits {
            0 => ARBMODE_A::VALUE1,
            1 => ARBMODE_A::VALUE2,
            2 => ARBMODE_A::VALUE3,
            3 => ARBMODE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "No Bus arbitration mode selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ARBMODE_A::VALUE1
    }
    #[doc = "Arbiter Mode arbitration mode selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ARBMODE_A::VALUE2
    }
    #[doc = "Participant arbitration mode selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ARBMODE_A::VALUE3
    }
    #[doc = "Sole Master arbitration mode selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ARBMODE_A::VALUE4
    }
}
#[doc = "Field `ARBMODE` writer - Arbitration Mode Selection"]
pub type ARBMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ARBMODE_A, crate::Safe>;
impl<'a, REG> ARBMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Bus arbitration mode selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ARBMODE_A::VALUE1)
    }
    #[doc = "Arbiter Mode arbitration mode selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ARBMODE_A::VALUE2)
    }
    #[doc = "Participant arbitration mode selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(ARBMODE_A::VALUE3)
    }
    #[doc = "Sole Master arbitration mode selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(ARBMODE_A::VALUE4)
    }
}
#[doc = "Bus Time-out Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMEOUTC_A {
    #[doc = "0: Time-out is disabled."]
    VALUE1 = 0,
    #[doc = "1: Time-out is generated after 1 8 clock cycles."]
    VALUE2 = 1,
    #[doc = "255: Time-out is generated after 255 8 clock cycles."]
    VALUE3 = 255,
}
impl From<TIMEOUTC_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUTC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMEOUTC_A {
    type Ux = u8;
}
impl crate::IsEnum for TIMEOUTC_A {}
#[doc = "Field `TIMEOUTC` reader - Bus Time-out Control"]
pub type TIMEOUTC_R = crate::FieldReader<TIMEOUTC_A>;
impl TIMEOUTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIMEOUTC_A> {
        match self.bits {
            0 => Some(TIMEOUTC_A::VALUE1),
            1 => Some(TIMEOUTC_A::VALUE2),
            255 => Some(TIMEOUTC_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Time-out is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TIMEOUTC_A::VALUE1
    }
    #[doc = "Time-out is generated after 1 8 clock cycles."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TIMEOUTC_A::VALUE2
    }
    #[doc = "Time-out is generated after 255 8 clock cycles."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TIMEOUTC_A::VALUE3
    }
}
#[doc = "Field `TIMEOUTC` writer - Bus Time-out Control"]
pub type TIMEOUTC_W<'a, REG> = crate::FieldWriter<'a, REG, 8, TIMEOUTC_A>;
impl<'a, REG> TIMEOUTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Time-out is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUTC_A::VALUE1)
    }
    #[doc = "Time-out is generated after 1 8 clock cycles."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUTC_A::VALUE2)
    }
    #[doc = "Time-out is generated after 255 8 clock cycles."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUTC_A::VALUE3)
    }
}
#[doc = "Field `LOCKTIMEOUT` reader - Lock Timeout Counter Preload"]
pub type LOCKTIMEOUT_R = crate::FieldReader;
#[doc = "Field `LOCKTIMEOUT` writer - Lock Timeout Counter Preload"]
pub type LOCKTIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GLOBALCS` reader - Global Chip Select Enable"]
pub type GLOBALCS_R = crate::FieldReader;
#[doc = "Field `GLOBALCS` writer - Global Chip Select Enable"]
pub type GLOBALCS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACCSINH` reader - Access Inhibit request"]
pub type ACCSINH_R = crate::BitReader;
#[doc = "Field `ACCSINH` writer - Access Inhibit request"]
pub type ACCSINH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCSINHACK` reader - Access inhibit acknowledge"]
pub type ACCSINHACK_R = crate::BitReader;
#[doc = "ALE Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALE_A {
    #[doc = "0: Output is ADV"]
    VALUE1 = 0,
    #[doc = "1: Output is ALE"]
    VALUE2 = 1,
}
impl From<ALE_A> for bool {
    #[inline(always)]
    fn from(variant: ALE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALE` reader - ALE Mode"]
pub type ALE_R = crate::BitReader<ALE_A>;
impl ALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALE_A {
        match self.bits {
            false => ALE_A::VALUE1,
            true => ALE_A::VALUE2,
        }
    }
    #[doc = "Output is ADV"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALE_A::VALUE1
    }
    #[doc = "Output is ALE"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALE_A::VALUE2
    }
}
#[doc = "Field `ALE` writer - ALE Mode"]
pub type ALE_W<'a, REG> = crate::BitWriter<'a, REG, ALE_A>;
impl<'a, REG> ALE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is ADV"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALE_A::VALUE1)
    }
    #[doc = "Output is ALE"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALE_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Memory Status Bit"]
    #[inline(always)]
    pub fn sts(&self) -> STS_R {
        STS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Abort"]
    #[inline(always)]
    pub fn lckabrt(&self) -> LCKABRT_R {
        LCKABRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDRAM Tristate"]
    #[inline(always)]
    pub fn sdtri(&self) -> SDTRI_R {
        SDTRI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - External Bus Lock Control"]
    #[inline(always)]
    pub fn extlock(&self) -> EXTLOCK_R {
        EXTLOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Arbitration Signal Synchronization Control"]
    #[inline(always)]
    pub fn arbsync(&self) -> ARBSYNC_R {
        ARBSYNC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Arbitration Mode Selection"]
    #[inline(always)]
    pub fn arbmode(&self) -> ARBMODE_R {
        ARBMODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Bus Time-out Control"]
    #[inline(always)]
    pub fn timeoutc(&self) -> TIMEOUTC_R {
        TIMEOUTC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Lock Timeout Counter Preload"]
    #[inline(always)]
    pub fn locktimeout(&self) -> LOCKTIMEOUT_R {
        LOCKTIMEOUT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Global Chip Select Enable"]
    #[inline(always)]
    pub fn globalcs(&self) -> GLOBALCS_R {
        GLOBALCS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Access Inhibit request"]
    #[inline(always)]
    pub fn accsinh(&self) -> ACCSINH_R {
        ACCSINH_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Access inhibit acknowledge"]
    #[inline(always)]
    pub fn accsinhack(&self) -> ACCSINHACK_R {
        ACCSINHACK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - ALE Mode"]
    #[inline(always)]
    pub fn ale(&self) -> ALE_R {
        ALE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SDRAM Tristate"]
    #[inline(always)]
    pub fn sdtri(&mut self) -> SDTRI_W<MODCON_SPEC> {
        SDTRI_W::new(self, 2)
    }
    #[doc = "Bit 4 - External Bus Lock Control"]
    #[inline(always)]
    pub fn extlock(&mut self) -> EXTLOCK_W<MODCON_SPEC> {
        EXTLOCK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Arbitration Signal Synchronization Control"]
    #[inline(always)]
    pub fn arbsync(&mut self) -> ARBSYNC_W<MODCON_SPEC> {
        ARBSYNC_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Arbitration Mode Selection"]
    #[inline(always)]
    pub fn arbmode(&mut self) -> ARBMODE_W<MODCON_SPEC> {
        ARBMODE_W::new(self, 6)
    }
    #[doc = "Bits 8:15 - Bus Time-out Control"]
    #[inline(always)]
    pub fn timeoutc(&mut self) -> TIMEOUTC_W<MODCON_SPEC> {
        TIMEOUTC_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Lock Timeout Counter Preload"]
    #[inline(always)]
    pub fn locktimeout(&mut self) -> LOCKTIMEOUT_W<MODCON_SPEC> {
        LOCKTIMEOUT_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Global Chip Select Enable"]
    #[inline(always)]
    pub fn globalcs(&mut self) -> GLOBALCS_W<MODCON_SPEC> {
        GLOBALCS_W::new(self, 24)
    }
    #[doc = "Bit 28 - Access Inhibit request"]
    #[inline(always)]
    pub fn accsinh(&mut self) -> ACCSINH_W<MODCON_SPEC> {
        ACCSINH_W::new(self, 28)
    }
    #[doc = "Bit 31 - ALE Mode"]
    #[inline(always)]
    pub fn ale(&mut self) -> ALE_W<MODCON_SPEC> {
        ALE_W::new(self, 31)
    }
}
#[doc = "EBU Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`modcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODCON_SPEC;
impl crate::RegisterSpec for MODCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modcon::R`](R) reader structure"]
impl crate::Readable for MODCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modcon::W`](W) writer structure"]
impl crate::Writable for MODCON_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODCON to value 0x20"]
impl crate::Resettable for MODCON_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
