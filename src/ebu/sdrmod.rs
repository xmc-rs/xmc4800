#[doc = "Register `SDRMOD` reader"]
pub struct R(crate::R<SDRMOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDRMOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDRMOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDRMOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDRMOD` writer"]
pub struct W(crate::W<SDRMOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDRMOD_SPEC>;
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
impl From<crate::W<SDRMOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDRMOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XBA` reader - Extended Operation Bank Select"]
pub struct XBA_R(crate::FieldReader<u8, u8>);
impl XBA_R {
    pub(crate) fn new(bits: u8) -> Self {
        XBA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XBA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XBA` writer - Extended Operation Bank Select"]
pub struct XBA_W<'a> {
    w: &'a mut W,
}
impl<'a> XBA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `XOPM` reader - Extended Operation Mode"]
pub struct XOPM_R(crate::FieldReader<u16, u16>);
impl XOPM_R {
    pub(crate) fn new(bits: u16) -> Self {
        XOPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOPM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOPM` writer - Extended Operation Mode"]
pub struct XOPM_W<'a> {
    w: &'a mut W,
}
impl<'a> XOPM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `COLDSTART` writer - SDRAM coldstart"]
pub struct COLDSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> COLDSTART_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPMODE_A {
    #[doc = "0: Only this value must be written (default after reset)"]
    VALUE1 = 0,
}
impl From<OPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OPMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OPMODE` reader - Operation Mode"]
pub struct OPMODE_R(crate::FieldReader<u8, OPMODE_A>);
impl OPMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        OPMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPMODE_A> {
        match self.bits {
            0 => Some(OPMODE_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == OPMODE_A::VALUE1
    }
}
impl core::ops::Deref for OPMODE_R {
    type Target = crate::FieldReader<u8, OPMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPMODE` writer - Operation Mode"]
pub struct OPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Only this value must be written (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OPMODE_A::VALUE1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 7)) | ((value as u32 & 0x7f) << 7);
        self.w
    }
}
#[doc = "CAS latency\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CASLAT_A {
    #[doc = "2: Two clocks (default after reset)"]
    VALUE1 = 2,
    #[doc = "3: Three clocks"]
    VALUE2 = 3,
}
impl From<CASLAT_A> for u8 {
    #[inline(always)]
    fn from(variant: CASLAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CASLAT` reader - CAS latency"]
pub struct CASLAT_R(crate::FieldReader<u8, CASLAT_A>);
impl CASLAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CASLAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CASLAT_A> {
        match self.bits {
            2 => Some(CASLAT_A::VALUE1),
            3 => Some(CASLAT_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CASLAT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CASLAT_A::VALUE2
    }
}
impl core::ops::Deref for CASLAT_R {
    type Target = crate::FieldReader<u8, CASLAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CASLAT` writer - CAS latency"]
pub struct CASLAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CASLAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CASLAT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Two clocks (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CASLAT_A::VALUE1)
    }
    #[doc = "Three clocks"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CASLAT_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Burst type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTYP_A {
    #[doc = "0: Only this value should be written (default after reset)"]
    VALUE1 = 0,
}
impl From<BTYP_A> for bool {
    #[inline(always)]
    fn from(variant: BTYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BTYP` reader - Burst type"]
pub struct BTYP_R(crate::FieldReader<bool, BTYP_A>);
impl BTYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BTYP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BTYP_A> {
        match self.bits {
            false => Some(BTYP_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BTYP_A::VALUE1
    }
}
impl core::ops::Deref for BTYP_R {
    type Target = crate::FieldReader<bool, BTYP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTYP` writer - Burst type"]
pub struct BTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> BTYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BTYP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Only this value should be written (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BTYP_A::VALUE1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Burst length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BURSTL_A {
    #[doc = "0: 1 (default after reset)"]
    VALUE1 = 0,
    #[doc = "1: 2"]
    VALUE2 = 1,
    #[doc = "2: 4"]
    VALUE3 = 2,
    #[doc = "3: 8"]
    VALUE4 = 3,
    #[doc = "4: 16"]
    VALUE5 = 4,
}
impl From<BURSTL_A> for u8 {
    #[inline(always)]
    fn from(variant: BURSTL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BURSTL` reader - Burst length"]
pub struct BURSTL_R(crate::FieldReader<u8, BURSTL_A>);
impl BURSTL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BURSTL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BURSTL_A> {
        match self.bits {
            0 => Some(BURSTL_A::VALUE1),
            1 => Some(BURSTL_A::VALUE2),
            2 => Some(BURSTL_A::VALUE3),
            3 => Some(BURSTL_A::VALUE4),
            4 => Some(BURSTL_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BURSTL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BURSTL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == BURSTL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == BURSTL_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == BURSTL_A::VALUE5
    }
}
impl core::ops::Deref for BURSTL_R {
    type Target = crate::FieldReader<u8, BURSTL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BURSTL` writer - Burst length"]
pub struct BURSTL_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURSTL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BURSTL_A::VALUE1)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BURSTL_A::VALUE2)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BURSTL_A::VALUE3)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BURSTL_A::VALUE4)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(BURSTL_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - Extended Operation Bank Select"]
    #[inline(always)]
    pub fn xba(&self) -> XBA_R {
        XBA_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - Extended Operation Mode"]
    #[inline(always)]
    pub fn xopm(&self) -> XOPM_R {
        XOPM_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 7:13 - Operation Mode"]
    #[inline(always)]
    pub fn opmode(&self) -> OPMODE_R {
        OPMODE_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 4:6 - CAS latency"]
    #[inline(always)]
    pub fn caslat(&self) -> CASLAT_R {
        CASLAT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - Burst type"]
    #[inline(always)]
    pub fn btyp(&self) -> BTYP_R {
        BTYP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - Burst length"]
    #[inline(always)]
    pub fn burstl(&self) -> BURSTL_R {
        BURSTL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - Extended Operation Bank Select"]
    #[inline(always)]
    pub fn xba(&mut self) -> XBA_W {
        XBA_W { w: self }
    }
    #[doc = "Bits 16:27 - Extended Operation Mode"]
    #[inline(always)]
    pub fn xopm(&mut self) -> XOPM_W {
        XOPM_W { w: self }
    }
    #[doc = "Bit 15 - SDRAM coldstart"]
    #[inline(always)]
    pub fn coldstart(&mut self) -> COLDSTART_W {
        COLDSTART_W { w: self }
    }
    #[doc = "Bits 7:13 - Operation Mode"]
    #[inline(always)]
    pub fn opmode(&mut self) -> OPMODE_W {
        OPMODE_W { w: self }
    }
    #[doc = "Bits 4:6 - CAS latency"]
    #[inline(always)]
    pub fn caslat(&mut self) -> CASLAT_W {
        CASLAT_W { w: self }
    }
    #[doc = "Bit 3 - Burst type"]
    #[inline(always)]
    pub fn btyp(&mut self) -> BTYP_W {
        BTYP_W { w: self }
    }
    #[doc = "Bits 0:2 - Burst length"]
    #[inline(always)]
    pub fn burstl(&mut self) -> BURSTL_W {
        BURSTL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBU SDRAM Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdrmod](index.html) module"]
pub struct SDRMOD_SPEC;
impl crate::RegisterSpec for SDRMOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdrmod::R](R) reader structure"]
impl crate::Readable for SDRMOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdrmod::W](W) writer structure"]
impl crate::Writable for SDRMOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDRMOD to value 0x20"]
impl crate::Resettable for SDRMOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
