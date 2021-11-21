#[doc = "Register `SM_CONTROL` reader"]
pub struct R(crate::R<SM_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SM_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SM_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SM_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OP_MODE_A {
    #[doc = "0: Buffered (3 buffer mode)"]
    VALUE1 = 0,
    #[doc = "2: Mailbox (Single buffer mode)"]
    VALUE3 = 2,
}
impl From<OP_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OP_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OP_MODE` reader - Operation Mode"]
pub struct OP_MODE_R(crate::FieldReader<u8, OP_MODE_A>);
impl OP_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        OP_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OP_MODE_A> {
        match self.bits {
            0 => Some(OP_MODE_A::VALUE1),
            2 => Some(OP_MODE_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == OP_MODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == OP_MODE_A::VALUE3
    }
}
impl core::ops::Deref for OP_MODE_R {
    type Target = crate::FieldReader<u8, OP_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIR_A {
    #[doc = "0: Read: ECAT read access, PDI write access"]
    VALUE1 = 0,
    #[doc = "1: Write: ECAT write access, PDI read access"]
    VALUE2 = 1,
}
impl From<DIR_A> for u8 {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIR` reader - Direction"]
pub struct DIR_R(crate::FieldReader<u8, DIR_A>);
impl DIR_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIR_A> {
        match self.bits {
            0 => Some(DIR_A::VALUE1),
            1 => Some(DIR_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DIR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DIR_A::VALUE2
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<u8, DIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt in ECAT Event Request Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_ECAT_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<INT_ECAT_A> for bool {
    #[inline(always)]
    fn from(variant: INT_ECAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_ECAT` reader - Interrupt in ECAT Event Request Register"]
pub struct INT_ECAT_R(crate::FieldReader<bool, INT_ECAT_A>);
impl INT_ECAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT_ECAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_ECAT_A {
        match self.bits {
            false => INT_ECAT_A::VALUE1,
            true => INT_ECAT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == INT_ECAT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == INT_ECAT_A::VALUE2
    }
}
impl core::ops::Deref for INT_ECAT_R {
    type Target = crate::FieldReader<bool, INT_ECAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt in PDI Event Request Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_PDI_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<INT_PDI_A> for bool {
    #[inline(always)]
    fn from(variant: INT_PDI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_PDI` reader - Interrupt in PDI Event Request Register"]
pub struct INT_PDI_R(crate::FieldReader<bool, INT_PDI_A>);
impl INT_PDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT_PDI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_PDI_A {
        match self.bits {
            false => INT_PDI_A::VALUE1,
            true => INT_PDI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == INT_PDI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == INT_PDI_A::VALUE2
    }
}
impl core::ops::Deref for INT_PDI_R {
    type Target = crate::FieldReader<bool, INT_PDI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Watchdog Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WD_TRG_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<WD_TRG_A> for bool {
    #[inline(always)]
    fn from(variant: WD_TRG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WD_TRG` reader - Watchdog Trigger Enable"]
pub struct WD_TRG_R(crate::FieldReader<bool, WD_TRG_A>);
impl WD_TRG_R {
    pub(crate) fn new(bits: bool) -> Self {
        WD_TRG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WD_TRG_A {
        match self.bits {
            false => WD_TRG_A::VALUE1,
            true => WD_TRG_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == WD_TRG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WD_TRG_A::VALUE2
    }
}
impl core::ops::Deref for WD_TRG_R {
    type Target = crate::FieldReader<bool, WD_TRG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Operation Mode"]
    #[inline(always)]
    pub fn op_mode(&self) -> OP_MODE_R {
        OP_MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Interrupt in ECAT Event Request Register"]
    #[inline(always)]
    pub fn int_ecat(&self) -> INT_ECAT_R {
        INT_ECAT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt in PDI Event Request Register"]
    #[inline(always)]
    pub fn int_pdi(&self) -> INT_PDI_R {
        INT_PDI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Watchdog Trigger Enable"]
    #[inline(always)]
    pub fn wd_trg(&self) -> WD_TRG_R {
        WD_TRG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
#[doc = "Control Register SyncManager 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm_control](index.html) module"]
pub struct SM_CONTROL_SPEC;
impl crate::RegisterSpec for SM_CONTROL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sm_control::R](R) reader structure"]
impl crate::Readable for SM_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SM_CONTROL to value 0"]
impl crate::Resettable for SM_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
