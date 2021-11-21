#[doc = "Register `TRAPRAW` reader"]
pub struct R(crate::R<TRAPRAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRAPRAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRAPRAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRAPRAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "OSC_HP Oscillator Watchdog Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCWDGT_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<SOSCWDGT_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCWDGT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOSCWDGT` reader - OSC_HP Oscillator Watchdog Trap Raw Status"]
pub struct SOSCWDGT_R(crate::FieldReader<bool, SOSCWDGT_A>);
impl SOSCWDGT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOSCWDGT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCWDGT_A {
        match self.bits {
            false => SOSCWDGT_A::VALUE1,
            true => SOSCWDGT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SOSCWDGT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SOSCWDGT_A::VALUE2
    }
}
impl core::ops::Deref for SOSCWDGT_R {
    type Target = crate::FieldReader<bool, SOSCWDGT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "System VCO Lock Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCOLCKT_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<SVCOLCKT_A> for bool {
    #[inline(always)]
    fn from(variant: SVCOLCKT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCOLCKT` reader - System VCO Lock Trap Raw Status"]
pub struct SVCOLCKT_R(crate::FieldReader<bool, SVCOLCKT_A>);
impl SVCOLCKT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SVCOLCKT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVCOLCKT_A {
        match self.bits {
            false => SVCOLCKT_A::VALUE1,
            true => SVCOLCKT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SVCOLCKT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SVCOLCKT_A::VALUE2
    }
}
impl core::ops::Deref for SVCOLCKT_R {
    type Target = crate::FieldReader<bool, SVCOLCKT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "USB VCO Lock Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UVCOLCKT_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<UVCOLCKT_A> for bool {
    #[inline(always)]
    fn from(variant: UVCOLCKT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UVCOLCKT` reader - USB VCO Lock Trap Raw Status"]
pub struct UVCOLCKT_R(crate::FieldReader<bool, UVCOLCKT_A>);
impl UVCOLCKT_R {
    pub(crate) fn new(bits: bool) -> Self {
        UVCOLCKT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UVCOLCKT_A {
        match self.bits {
            false => UVCOLCKT_A::VALUE1,
            true => UVCOLCKT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == UVCOLCKT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == UVCOLCKT_A::VALUE2
    }
}
impl core::ops::Deref for UVCOLCKT_R {
    type Target = crate::FieldReader<bool, UVCOLCKT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Parity Error Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PET_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<PET_A> for bool {
    #[inline(always)]
    fn from(variant: PET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PET` reader - Parity Error Trap Raw Status"]
pub struct PET_R(crate::FieldReader<bool, PET_A>);
impl PET_R {
    pub(crate) fn new(bits: bool) -> Self {
        PET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PET_A {
        match self.bits {
            false => PET_A::VALUE1,
            true => PET_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PET_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PET_A::VALUE2
    }
}
impl core::ops::Deref for PET_R {
    type Target = crate::FieldReader<bool, PET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Brown Out Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRWNT_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<BRWNT_A> for bool {
    #[inline(always)]
    fn from(variant: BRWNT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRWNT` reader - Brown Out Trap Raw Status"]
pub struct BRWNT_R(crate::FieldReader<bool, BRWNT_A>);
impl BRWNT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRWNT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRWNT_A {
        match self.bits {
            false => BRWNT_A::VALUE1,
            true => BRWNT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BRWNT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BRWNT_A::VALUE2
    }
}
impl core::ops::Deref for BRWNT_R {
    type Target = crate::FieldReader<bool, BRWNT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "OSC_ULP Oscillator Watchdog Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULPWDGT_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<ULPWDGT_A> for bool {
    #[inline(always)]
    fn from(variant: ULPWDGT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDGT` reader - OSC_ULP Oscillator Watchdog Trap Raw Status"]
pub struct ULPWDGT_R(crate::FieldReader<bool, ULPWDGT_A>);
impl ULPWDGT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ULPWDGT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ULPWDGT_A {
        match self.bits {
            false => ULPWDGT_A::VALUE1,
            true => ULPWDGT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ULPWDGT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ULPWDGT_A::VALUE2
    }
}
impl core::ops::Deref for ULPWDGT_R {
    type Target = crate::FieldReader<bool, ULPWDGT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Peripheral Bridge 0 Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWERR0T_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<BWERR0T_A> for bool {
    #[inline(always)]
    fn from(variant: BWERR0T_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR0T` reader - Peripheral Bridge 0 Trap Raw Status"]
pub struct BWERR0T_R(crate::FieldReader<bool, BWERR0T_A>);
impl BWERR0T_R {
    pub(crate) fn new(bits: bool) -> Self {
        BWERR0T_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWERR0T_A {
        match self.bits {
            false => BWERR0T_A::VALUE1,
            true => BWERR0T_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BWERR0T_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BWERR0T_A::VALUE2
    }
}
impl core::ops::Deref for BWERR0T_R {
    type Target = crate::FieldReader<bool, BWERR0T_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Peripheral Bridge 1 Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWERR1T_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<BWERR1T_A> for bool {
    #[inline(always)]
    fn from(variant: BWERR1T_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR1T` reader - Peripheral Bridge 1 Trap Raw Status"]
pub struct BWERR1T_R(crate::FieldReader<bool, BWERR1T_A>);
impl BWERR1T_R {
    pub(crate) fn new(bits: bool) -> Self {
        BWERR1T_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWERR1T_A {
        match self.bits {
            false => BWERR1T_A::VALUE1,
            true => BWERR1T_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BWERR1T_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BWERR1T_A::VALUE2
    }
}
impl core::ops::Deref for BWERR1T_R {
    type Target = crate::FieldReader<bool, BWERR1T_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "EtherCat Reset 0 Trap Raw Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECAT0RST_A {
    #[doc = "0: No pending trap request"]
    VALUE1 = 0,
    #[doc = "1: Pending trap request"]
    VALUE2 = 1,
}
impl From<ECAT0RST_A> for bool {
    #[inline(always)]
    fn from(variant: ECAT0RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECAT0RST` reader - EtherCat Reset 0 Trap Raw Status"]
pub struct ECAT0RST_R(crate::FieldReader<bool, ECAT0RST_A>);
impl ECAT0RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECAT0RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECAT0RST_A {
        match self.bits {
            false => ECAT0RST_A::VALUE1,
            true => ECAT0RST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ECAT0RST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ECAT0RST_A::VALUE2
    }
}
impl core::ops::Deref for ECAT0RST_R {
    type Target = crate::FieldReader<bool, ECAT0RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Raw Status"]
    #[inline(always)]
    pub fn soscwdgt(&self) -> SOSCWDGT_R {
        SOSCWDGT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - System VCO Lock Trap Raw Status"]
    #[inline(always)]
    pub fn svcolckt(&self) -> SVCOLCKT_R {
        SVCOLCKT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Raw Status"]
    #[inline(always)]
    pub fn uvcolckt(&self) -> UVCOLCKT_R {
        UVCOLCKT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Parity Error Trap Raw Status"]
    #[inline(always)]
    pub fn pet(&self) -> PET_R {
        PET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Brown Out Trap Raw Status"]
    #[inline(always)]
    pub fn brwnt(&self) -> BRWNT_R {
        BRWNT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Raw Status"]
    #[inline(always)]
    pub fn ulpwdgt(&self) -> ULPWDGT_R {
        ULPWDGT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Raw Status"]
    #[inline(always)]
    pub fn bwerr0t(&self) -> BWERR0T_R {
        BWERR0T_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Raw Status"]
    #[inline(always)]
    pub fn bwerr1t(&self) -> BWERR1T_R {
        BWERR1T_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - EtherCat Reset 0 Trap Raw Status"]
    #[inline(always)]
    pub fn ecat0rst(&self) -> ECAT0RST_R {
        ECAT0RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "Trap Raw Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trapraw](index.html) module"]
pub struct TRAPRAW_SPEC;
impl crate::RegisterSpec for TRAPRAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trapraw::R](R) reader structure"]
impl crate::Readable for TRAPRAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRAPRAW to value 0"]
impl crate::Resettable for TRAPRAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
