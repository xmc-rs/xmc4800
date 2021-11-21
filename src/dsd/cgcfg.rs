#[doc = "Register `CGCFG` reader"]
pub struct R(crate::R<CGCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CGCFG` writer"]
pub struct W(crate::W<CGCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGCFG_SPEC>;
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
impl From<crate::W<CGCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Carrier Generator Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CGMOD_A {
    #[doc = "0: Stopped"]
    VALUE1 = 0,
    #[doc = "1: Square wave"]
    VALUE2 = 1,
    #[doc = "2: Triangle"]
    VALUE3 = 2,
    #[doc = "3: Sine wave"]
    VALUE4 = 3,
}
impl From<CGMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: CGMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CGMOD` reader - Carrier Generator Operating Mode"]
pub struct CGMOD_R(crate::FieldReader<u8, CGMOD_A>);
impl CGMOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CGMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CGMOD_A {
        match self.bits {
            0 => CGMOD_A::VALUE1,
            1 => CGMOD_A::VALUE2,
            2 => CGMOD_A::VALUE3,
            3 => CGMOD_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CGMOD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CGMOD_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == CGMOD_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == CGMOD_A::VALUE4
    }
}
impl core::ops::Deref for CGMOD_R {
    type Target = crate::FieldReader<u8, CGMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CGMOD` writer - Carrier Generator Operating Mode"]
pub struct CGMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CGMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGMOD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Stopped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CGMOD_A::VALUE1)
    }
    #[doc = "Square wave"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CGMOD_A::VALUE2)
    }
    #[doc = "Triangle"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CGMOD_A::VALUE3)
    }
    #[doc = "Sine wave"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CGMOD_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Bit-Reverse PWM Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BREV_A {
    #[doc = "0: Normal mode"]
    VALUE1 = 0,
    #[doc = "1: Bit-reverse mode"]
    VALUE2 = 1,
}
impl From<BREV_A> for bool {
    #[inline(always)]
    fn from(variant: BREV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BREV` reader - Bit-Reverse PWM Generation"]
pub struct BREV_R(crate::FieldReader<bool, BREV_A>);
impl BREV_R {
    pub(crate) fn new(bits: bool) -> Self {
        BREV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BREV_A {
        match self.bits {
            false => BREV_A::VALUE1,
            true => BREV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BREV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BREV_A::VALUE2
    }
}
impl core::ops::Deref for BREV_R {
    type Target = crate::FieldReader<bool, BREV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BREV` writer - Bit-Reverse PWM Generation"]
pub struct BREV_W<'a> {
    w: &'a mut W,
}
impl<'a> BREV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BREV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BREV_A::VALUE1)
    }
    #[doc = "Bit-reverse mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BREV_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Signal Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIGPOL_A {
    #[doc = "0: Normal: carrier signal begins with +1"]
    VALUE1 = 0,
    #[doc = "1: Inverted: carrier signal begins with -1"]
    VALUE2 = 1,
}
impl From<SIGPOL_A> for bool {
    #[inline(always)]
    fn from(variant: SIGPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIGPOL` reader - Signal Polarity"]
pub struct SIGPOL_R(crate::FieldReader<bool, SIGPOL_A>);
impl SIGPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIGPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIGPOL_A {
        match self.bits {
            false => SIGPOL_A::VALUE1,
            true => SIGPOL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SIGPOL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SIGPOL_A::VALUE2
    }
}
impl core::ops::Deref for SIGPOL_R {
    type Target = crate::FieldReader<bool, SIGPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIGPOL` writer - Signal Polarity"]
pub struct SIGPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIGPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal: carrier signal begins with +1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIGPOL_A::VALUE1)
    }
    #[doc = "Inverted: carrier signal begins with -1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIGPOL_A::VALUE2)
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
#[doc = "Divider Factor for the PWM Pattern Signal Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVCG_A {
    #[doc = "0: fCG = fCLK / 2"]
    VALUE1 = 0,
    #[doc = "1: fCG = fCLK / 4"]
    VALUE2 = 1,
    #[doc = "2: fCG = fCLK / 6"]
    VALUE3 = 2,
    #[doc = "15: fCG = fCLK / 32"]
    VALUE4 = 15,
}
impl From<DIVCG_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVCG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIVCG` reader - Divider Factor for the PWM Pattern Signal Generator"]
pub struct DIVCG_R(crate::FieldReader<u8, DIVCG_A>);
impl DIVCG_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVCG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVCG_A> {
        match self.bits {
            0 => Some(DIVCG_A::VALUE1),
            1 => Some(DIVCG_A::VALUE2),
            2 => Some(DIVCG_A::VALUE3),
            15 => Some(DIVCG_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DIVCG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DIVCG_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == DIVCG_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == DIVCG_A::VALUE4
    }
}
impl core::ops::Deref for DIVCG_R {
    type Target = crate::FieldReader<u8, DIVCG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVCG` writer - Divider Factor for the PWM Pattern Signal Generator"]
pub struct DIVCG_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVCG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVCG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "fCG = fCLK / 2"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVCG_A::VALUE1)
    }
    #[doc = "fCG = fCLK / 4"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVCG_A::VALUE2)
    }
    #[doc = "fCG = fCLK / 6"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DIVCG_A::VALUE3)
    }
    #[doc = "fCG = fCLK / 32"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(DIVCG_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Run Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUN_A {
    #[doc = "0: Stopped (cleared at the end of a period)"]
    VALUE1 = 0,
    #[doc = "1: Running"]
    VALUE2 = 1,
}
impl From<RUN_A> for bool {
    #[inline(always)]
    fn from(variant: RUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN` reader - Run Indicator"]
pub struct RUN_R(crate::FieldReader<bool, RUN_A>);
impl RUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RUN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUN_A {
        match self.bits {
            false => RUN_A::VALUE1,
            true => RUN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RUN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RUN_A::VALUE2
    }
}
impl core::ops::Deref for RUN_R {
    type Target = crate::FieldReader<bool, RUN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BITCOUNT` reader - Bit Counter"]
pub struct BITCOUNT_R(crate::FieldReader<u8, u8>);
impl BITCOUNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        BITCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BITCOUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STEPCOUNT` reader - Step Counter"]
pub struct STEPCOUNT_R(crate::FieldReader<u8, u8>);
impl STEPCOUNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        STEPCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STEPCOUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Step Counter Sign\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STEPS_A {
    #[doc = "0: Step counter value is positive"]
    VALUE1 = 0,
    #[doc = "1: Step counter value is negative"]
    VALUE2 = 1,
}
impl From<STEPS_A> for bool {
    #[inline(always)]
    fn from(variant: STEPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STEPS` reader - Step Counter Sign"]
pub struct STEPS_R(crate::FieldReader<bool, STEPS_A>);
impl STEPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        STEPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STEPS_A {
        match self.bits {
            false => STEPS_A::VALUE1,
            true => STEPS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == STEPS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STEPS_A::VALUE2
    }
}
impl core::ops::Deref for STEPS_R {
    type Target = crate::FieldReader<bool, STEPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Step Counter Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STEPD_A {
    #[doc = "0: Step counter is counting up"]
    VALUE1 = 0,
    #[doc = "1: Step counter is counting down"]
    VALUE2 = 1,
}
impl From<STEPD_A> for bool {
    #[inline(always)]
    fn from(variant: STEPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STEPD` reader - Step Counter Direction"]
pub struct STEPD_R(crate::FieldReader<bool, STEPD_A>);
impl STEPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        STEPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STEPD_A {
        match self.bits {
            false => STEPD_A::VALUE1,
            true => STEPD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == STEPD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STEPD_A::VALUE2
    }
}
impl core::ops::Deref for STEPD_R {
    type Target = crate::FieldReader<bool, STEPD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sign Signal from Carrier Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SGNCG_A {
    #[doc = "0: Positive values"]
    VALUE1 = 0,
    #[doc = "1: Negative values"]
    VALUE2 = 1,
}
impl From<SGNCG_A> for bool {
    #[inline(always)]
    fn from(variant: SGNCG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SGNCG` reader - Sign Signal from Carrier Generator"]
pub struct SGNCG_R(crate::FieldReader<bool, SGNCG_A>);
impl SGNCG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SGNCG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGNCG_A {
        match self.bits {
            false => SGNCG_A::VALUE1,
            true => SGNCG_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SGNCG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SGNCG_A::VALUE2
    }
}
impl core::ops::Deref for SGNCG_R {
    type Target = crate::FieldReader<bool, SGNCG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Carrier Generator Operating Mode"]
    #[inline(always)]
    pub fn cgmod(&self) -> CGMOD_R {
        CGMOD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Bit-Reverse PWM Generation"]
    #[inline(always)]
    pub fn brev(&self) -> BREV_R {
        BREV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Signal Polarity"]
    #[inline(always)]
    pub fn sigpol(&self) -> SIGPOL_R {
        SIGPOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Divider Factor for the PWM Pattern Signal Generator"]
    #[inline(always)]
    pub fn divcg(&self) -> DIVCG_R {
        DIVCG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Run Indicator"]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Bit Counter"]
    #[inline(always)]
    pub fn bitcount(&self) -> BITCOUNT_R {
        BITCOUNT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - Step Counter"]
    #[inline(always)]
    pub fn stepcount(&self) -> STEPCOUNT_R {
        STEPCOUNT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Step Counter Sign"]
    #[inline(always)]
    pub fn steps(&self) -> STEPS_R {
        STEPS_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Step Counter Direction"]
    #[inline(always)]
    pub fn stepd(&self) -> STEPD_R {
        STEPD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Sign Signal from Carrier Generator"]
    #[inline(always)]
    pub fn sgncg(&self) -> SGNCG_R {
        SGNCG_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Carrier Generator Operating Mode"]
    #[inline(always)]
    pub fn cgmod(&mut self) -> CGMOD_W {
        CGMOD_W { w: self }
    }
    #[doc = "Bit 2 - Bit-Reverse PWM Generation"]
    #[inline(always)]
    pub fn brev(&mut self) -> BREV_W {
        BREV_W { w: self }
    }
    #[doc = "Bit 3 - Signal Polarity"]
    #[inline(always)]
    pub fn sigpol(&mut self) -> SIGPOL_W {
        SIGPOL_W { w: self }
    }
    #[doc = "Bits 4:7 - Divider Factor for the PWM Pattern Signal Generator"]
    #[inline(always)]
    pub fn divcg(&mut self) -> DIVCG_W {
        DIVCG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Carrier Generator Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgcfg](index.html) module"]
pub struct CGCFG_SPEC;
impl crate::RegisterSpec for CGCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgcfg::R](R) reader structure"]
impl crate::Readable for CGCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgcfg::W](W) writer structure"]
impl crate::Writable for CGCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CGCFG to value 0x0710_0000"]
impl crate::Resettable for CGCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0710_0000
    }
}
