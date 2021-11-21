#[doc = "Register `PRSTAT3` reader"]
pub struct R(crate::R<PRSTAT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSTAT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSTAT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSTAT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "EBU Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBURS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<EBURS_A> for bool {
    #[inline(always)]
    fn from(variant: EBURS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBURS` reader - EBU Reset Status"]
pub struct EBURS_R(crate::FieldReader<bool, EBURS_A>);
impl EBURS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EBURS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EBURS_A {
        match self.bits {
            false => EBURS_A::VALUE1,
            true => EBURS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EBURS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EBURS_A::VALUE2
    }
}
impl core::ops::Deref for EBURS_R {
    type Target = crate::FieldReader<bool, EBURS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 2 - EBU Reset Status"]
    #[inline(always)]
    pub fn eburs(&self) -> EBURS_R {
        EBURS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "RCU Peripheral 3 Reset Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstat3](index.html) module"]
pub struct PRSTAT3_SPEC;
impl crate::RegisterSpec for PRSTAT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prstat3::R](R) reader structure"]
impl crate::Readable for PRSTAT3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRSTAT3 to value 0x04"]
impl crate::Resettable for PRSTAT3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
