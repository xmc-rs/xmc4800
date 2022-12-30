#[doc = "Register `MODCON` reader"]
pub struct R(crate::R<MODCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODCON` writer"]
pub struct W(crate::W<MODCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MODCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STS` reader - Memory Status Bit"]
pub type STS_R = crate::BitReader<bool>;
#[doc = "Field `LCKABRT` reader - Lock Abort"]
pub type LCKABRT_R = crate::BitReader<bool>;
#[doc = "Field `SDTRI` reader - SDRAM Tristate"]
pub type SDTRI_R = crate::BitReader<SDTRI_A>;
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
impl SDTRI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDTRI_A {
        match self.bits {
            false => SDTRI_A::VALUE1,
            true => SDTRI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDTRI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SDTRI_A::VALUE2
    }
}
#[doc = "Field `SDTRI` writer - SDRAM Tristate"]
pub type SDTRI_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODCON_SPEC, SDTRI_A, O>;
impl<'a, const O: u8> SDTRI_W<'a, O> {
    #[doc = "SDRAM control signals are driven by the EBU when the EBU does not own the external bus. SDRAM cannot be shared."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SDTRI_A::VALUE1)
    }
    #[doc = "SDRAM control signals are tri-stated by the EBU when the EBU does not own the external bus. The SDRAM can be shared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SDTRI_A::VALUE2)
    }
}
#[doc = "Field `EXTLOCK` reader - External Bus Lock Control"]
pub type EXTLOCK_R = crate::BitReader<EXTLOCK_A>;
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
impl EXTLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTLOCK_A {
        match self.bits {
            false => EXTLOCK_A::VALUE1,
            true => EXTLOCK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EXTLOCK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EXTLOCK_A::VALUE2
    }
}
#[doc = "Field `EXTLOCK` writer - External Bus Lock Control"]
pub type EXTLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODCON_SPEC, EXTLOCK_A, O>;
impl<'a, const O: u8> EXTLOCK_W<'a, O> {
    #[doc = "External bus is not locked after the EBU gains ownership"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXTLOCK_A::VALUE1)
    }
    #[doc = "External bus is locked after the EBU gains ownership"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXTLOCK_A::VALUE2)
    }
}
#[doc = "Field `ARBSYNC` reader - Arbitration Signal Synchronization Control"]
pub type ARBSYNC_R = crate::BitReader<ARBSYNC_A>;
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
impl ARBSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBSYNC_A {
        match self.bits {
            false => ARBSYNC_A::VALUE1,
            true => ARBSYNC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ARBSYNC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ARBSYNC_A::VALUE2
    }
}
#[doc = "Field `ARBSYNC` writer - Arbitration Signal Synchronization Control"]
pub type ARBSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODCON_SPEC, ARBSYNC_A, O>;
impl<'a, const O: u8> ARBSYNC_W<'a, O> {
    #[doc = "Arbitration inputs are synchronous"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARBSYNC_A::VALUE1)
    }
    #[doc = "Arbitration inputs are asynchronous"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARBSYNC_A::VALUE2)
    }
}
#[doc = "Field `ARBMODE` reader - Arbitration Mode Selection"]
pub type ARBMODE_R = crate::FieldReader<u8, ARBMODE_A>;
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
impl ARBMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBMODE_A {
        match self.bits {
            0 => ARBMODE_A::VALUE1,
            1 => ARBMODE_A::VALUE2,
            2 => ARBMODE_A::VALUE3,
            3 => ARBMODE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ARBMODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ARBMODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ARBMODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ARBMODE_A::VALUE4
    }
}
#[doc = "Field `ARBMODE` writer - Arbitration Mode Selection"]
pub type ARBMODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MODCON_SPEC, u8, ARBMODE_A, 2, O>;
impl<'a, const O: u8> ARBMODE_W<'a, O> {
    #[doc = "No Bus arbitration mode selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARBMODE_A::VALUE1)
    }
    #[doc = "Arbiter Mode arbitration mode selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARBMODE_A::VALUE2)
    }
    #[doc = "Participant arbitration mode selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ARBMODE_A::VALUE3)
    }
    #[doc = "Sole Master arbitration mode selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ARBMODE_A::VALUE4)
    }
}
#[doc = "Field `TIMEOUTC` reader - Bus Time-out Control"]
pub type TIMEOUTC_R = crate::FieldReader<u8, TIMEOUTC_A>;
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
impl TIMEOUTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMEOUTC_A> {
        match self.bits {
            0 => Some(TIMEOUTC_A::VALUE1),
            1 => Some(TIMEOUTC_A::VALUE2),
            255 => Some(TIMEOUTC_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TIMEOUTC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TIMEOUTC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TIMEOUTC_A::VALUE3
    }
}
#[doc = "Field `TIMEOUTC` writer - Bus Time-out Control"]
pub type TIMEOUTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODCON_SPEC, u8, TIMEOUTC_A, 8, O>;
impl<'a, const O: u8> TIMEOUTC_W<'a, O> {
    #[doc = "Time-out is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TIMEOUTC_A::VALUE1)
    }
    #[doc = "Time-out is generated after 1 8 clock cycles."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TIMEOUTC_A::VALUE2)
    }
    #[doc = "Time-out is generated after 255 8 clock cycles."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TIMEOUTC_A::VALUE3)
    }
}
#[doc = "Field `LOCKTIMEOUT` reader - Lock Timeout Counter Preload"]
pub type LOCKTIMEOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCKTIMEOUT` writer - Lock Timeout Counter Preload"]
pub type LOCKTIMEOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODCON_SPEC, u8, u8, 8, O>;
#[doc = "Field `GLOBALCS` reader - Global Chip Select Enable"]
pub type GLOBALCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GLOBALCS` writer - Global Chip Select Enable"]
pub type GLOBALCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODCON_SPEC, u8, u8, 4, O>;
#[doc = "Field `ACCSINH` reader - Access Inhibit request"]
pub type ACCSINH_R = crate::BitReader<bool>;
#[doc = "Field `ACCSINH` writer - Access Inhibit request"]
pub type ACCSINH_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODCON_SPEC, bool, O>;
#[doc = "Field `ACCSINHACK` reader - Access inhibit acknowledge"]
pub type ACCSINHACK_R = crate::BitReader<bool>;
#[doc = "Field `ALE` reader - ALE Mode"]
pub type ALE_R = crate::BitReader<ALE_A>;
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
impl ALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALE_A {
        match self.bits {
            false => ALE_A::VALUE1,
            true => ALE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALE_A::VALUE2
    }
}
#[doc = "Field `ALE` writer - ALE Mode"]
pub type ALE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODCON_SPEC, ALE_A, O>;
impl<'a, const O: u8> ALE_W<'a, O> {
    #[doc = "Output is ADV"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALE_A::VALUE1)
    }
    #[doc = "Output is ALE"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
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
    #[must_use]
    pub fn sdtri(&mut self) -> SDTRI_W<2> {
        SDTRI_W::new(self)
    }
    #[doc = "Bit 4 - External Bus Lock Control"]
    #[inline(always)]
    #[must_use]
    pub fn extlock(&mut self) -> EXTLOCK_W<4> {
        EXTLOCK_W::new(self)
    }
    #[doc = "Bit 5 - Arbitration Signal Synchronization Control"]
    #[inline(always)]
    #[must_use]
    pub fn arbsync(&mut self) -> ARBSYNC_W<5> {
        ARBSYNC_W::new(self)
    }
    #[doc = "Bits 6:7 - Arbitration Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn arbmode(&mut self) -> ARBMODE_W<6> {
        ARBMODE_W::new(self)
    }
    #[doc = "Bits 8:15 - Bus Time-out Control"]
    #[inline(always)]
    #[must_use]
    pub fn timeoutc(&mut self) -> TIMEOUTC_W<8> {
        TIMEOUTC_W::new(self)
    }
    #[doc = "Bits 16:23 - Lock Timeout Counter Preload"]
    #[inline(always)]
    #[must_use]
    pub fn locktimeout(&mut self) -> LOCKTIMEOUT_W<16> {
        LOCKTIMEOUT_W::new(self)
    }
    #[doc = "Bits 24:27 - Global Chip Select Enable"]
    #[inline(always)]
    #[must_use]
    pub fn globalcs(&mut self) -> GLOBALCS_W<24> {
        GLOBALCS_W::new(self)
    }
    #[doc = "Bit 28 - Access Inhibit request"]
    #[inline(always)]
    #[must_use]
    pub fn accsinh(&mut self) -> ACCSINH_W<28> {
        ACCSINH_W::new(self)
    }
    #[doc = "Bit 31 - ALE Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ale(&mut self) -> ALE_W<31> {
        ALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBU Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modcon](index.html) module"]
pub struct MODCON_SPEC;
impl crate::RegisterSpec for MODCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modcon::R](R) reader structure"]
impl crate::Readable for MODCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modcon::W](W) writer structure"]
impl crate::Writable for MODCON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODCON to value 0x20"]
impl crate::Resettable for MODCON_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
