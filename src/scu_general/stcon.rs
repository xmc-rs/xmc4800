#[doc = "Register `STCON` reader"]
pub struct R(crate::R<STCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STCON` writer"]
pub struct W(crate::W<STCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STCON_SPEC>;
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
impl From<crate::W<STCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HW Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HWCON_A {
    #[doc = "0: Normal mode, JTAG"]
    VALUE1 = 0,
    #[doc = "1: ASC BSL enabled"]
    VALUE2 = 1,
    #[doc = "2: BMI customized boot enabled"]
    VALUE3 = 2,
    #[doc = "3: CAN BSL enabled"]
    VALUE4 = 3,
}
impl From<HWCON_A> for u8 {
    #[inline(always)]
    fn from(variant: HWCON_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HWCON` reader - HW Configuration"]
pub struct HWCON_R(crate::FieldReader<u8, HWCON_A>);
impl HWCON_R {
    pub(crate) fn new(bits: u8) -> Self {
        HWCON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWCON_A {
        match self.bits {
            0 => HWCON_A::VALUE1,
            1 => HWCON_A::VALUE2,
            2 => HWCON_A::VALUE3,
            3 => HWCON_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == HWCON_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HWCON_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == HWCON_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == HWCON_A::VALUE4
    }
}
impl core::ops::Deref for HWCON_R {
    type Target = crate::FieldReader<u8, HWCON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SW Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWCON_A {
    #[doc = "0: Normal mode, boot from Boot ROM"]
    VALUE1 = 0,
    #[doc = "1: ASC BSL enabled"]
    VALUE2 = 1,
    #[doc = "2: BMI customized boot enabled"]
    VALUE3 = 2,
    #[doc = "3: CAN BSL enabled"]
    VALUE4 = 3,
    #[doc = "4: Boot from Code SRAM"]
    VALUE5 = 4,
    #[doc = "8: Boot from alternate Flash Address 0"]
    VALUE6 = 8,
    #[doc = "12: Boot from alternate Flash Address 1"]
    VALUE7 = 12,
    #[doc = "14: Enable fallback Alternate Boot Mode (ABM)"]
    VALUE8 = 14,
}
impl From<SWCON_A> for u8 {
    #[inline(always)]
    fn from(variant: SWCON_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SWCON` reader - SW Configuration"]
pub struct SWCON_R(crate::FieldReader<u8, SWCON_A>);
impl SWCON_R {
    pub(crate) fn new(bits: u8) -> Self {
        SWCON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWCON_A> {
        match self.bits {
            0 => Some(SWCON_A::VALUE1),
            1 => Some(SWCON_A::VALUE2),
            2 => Some(SWCON_A::VALUE3),
            3 => Some(SWCON_A::VALUE4),
            4 => Some(SWCON_A::VALUE5),
            8 => Some(SWCON_A::VALUE6),
            12 => Some(SWCON_A::VALUE7),
            14 => Some(SWCON_A::VALUE8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SWCON_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SWCON_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SWCON_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == SWCON_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == SWCON_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == SWCON_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        **self == SWCON_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        **self == SWCON_A::VALUE8
    }
}
impl core::ops::Deref for SWCON_R {
    type Target = crate::FieldReader<u8, SWCON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWCON` writer - SW Configuration"]
pub struct SWCON_W<'a> {
    w: &'a mut W,
}
impl<'a> SWCON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWCON_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal mode, boot from Boot ROM"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SWCON_A::VALUE1)
    }
    #[doc = "ASC BSL enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SWCON_A::VALUE2)
    }
    #[doc = "BMI customized boot enabled"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SWCON_A::VALUE3)
    }
    #[doc = "CAN BSL enabled"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SWCON_A::VALUE4)
    }
    #[doc = "Boot from Code SRAM"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(SWCON_A::VALUE5)
    }
    #[doc = "Boot from alternate Flash Address 0"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(SWCON_A::VALUE6)
    }
    #[doc = "Boot from alternate Flash Address 1"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(SWCON_A::VALUE7)
    }
    #[doc = "Enable fallback Alternate Boot Mode (ABM)"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(SWCON_A::VALUE8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - HW Configuration"]
    #[inline(always)]
    pub fn hwcon(&self) -> HWCON_R {
        HWCON_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - SW Configuration"]
    #[inline(always)]
    pub fn swcon(&self) -> SWCON_R {
        SWCON_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - SW Configuration"]
    #[inline(always)]
    pub fn swcon(&mut self) -> SWCON_W {
        SWCON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Startup Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcon](index.html) module"]
pub struct STCON_SPEC;
impl crate::RegisterSpec for STCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stcon::R](R) reader structure"]
impl crate::Readable for STCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stcon::W](W) writer structure"]
impl crate::Writable for STCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STCON to value 0"]
impl crate::Resettable for STCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
