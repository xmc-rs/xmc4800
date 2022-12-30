#[doc = "Register `CLC` reader"]
pub struct R(crate::R<CLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLC` writer"]
pub struct W(crate::W<CLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLC_SPEC>;
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
impl From<crate::W<CLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISR` reader - EBU Disable Request Bit"]
pub type DISR_R = crate::BitReader<DISR_A>;
#[doc = "EBU Disable Request Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISR_A {
    #[doc = "0: EBU disable is not requested"]
    VALUE1 = 0,
    #[doc = "1: EBU disable is requested"]
    VALUE2 = 1,
}
impl From<DISR_A> for bool {
    #[inline(always)]
    fn from(variant: DISR_A) -> Self {
        variant as u8 != 0
    }
}
impl DISR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISR_A {
        match self.bits {
            false => DISR_A::VALUE1,
            true => DISR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DISR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DISR_A::VALUE2
    }
}
#[doc = "Field `DISR` writer - EBU Disable Request Bit"]
pub type DISR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLC_SPEC, DISR_A, O>;
impl<'a, const O: u8> DISR_W<'a, O> {
    #[doc = "EBU disable is not requested"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DISR_A::VALUE1)
    }
    #[doc = "EBU disable is requested"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DISR_A::VALUE2)
    }
}
#[doc = "Field `DISS` reader - EBU Disable Status Bit"]
pub type DISS_R = crate::BitReader<DISS_A>;
#[doc = "EBU Disable Status Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISS_A {
    #[doc = "0: EBU is enabled (default after reset)"]
    VALUE1 = 0,
    #[doc = "1: EBU is disabled"]
    VALUE2 = 1,
}
impl From<DISS_A> for bool {
    #[inline(always)]
    fn from(variant: DISS_A) -> Self {
        variant as u8 != 0
    }
}
impl DISS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISS_A {
        match self.bits {
            false => DISS_A::VALUE1,
            true => DISS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DISS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DISS_A::VALUE2
    }
}
#[doc = "Field `SYNC` reader - EBU Clocking Mode"]
pub type SYNC_R = crate::BitReader<SYNC_A>;
#[doc = "EBU Clocking Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC_A {
    #[doc = "0: request EBU to run asynchronously to AHB bus clock and use separate clock source"]
    VALUE1 = 0,
    #[doc = "1: request EBU to run synchronously to ARM processor (default after reset)"]
    VALUE2 = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::VALUE1,
            true => SYNC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYNC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNC_A::VALUE2
    }
}
#[doc = "Field `SYNC` writer - EBU Clocking Mode"]
pub type SYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLC_SPEC, SYNC_A, O>;
impl<'a, const O: u8> SYNC_W<'a, O> {
    #[doc = "request EBU to run asynchronously to AHB bus clock and use separate clock source"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SYNC_A::VALUE1)
    }
    #[doc = "request EBU to run synchronously to ARM processor (default after reset)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SYNC_A::VALUE2)
    }
}
#[doc = "Field `DIV2` reader - DIV2 Clocking Mode"]
pub type DIV2_R = crate::BitReader<DIV2_A>;
#[doc = "DIV2 Clocking Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIV2_A {
    #[doc = "0: standard clocking mode. clock input selected by SYNC bitfield (default after reset)."]
    VALUE1 = 0,
    #[doc = "1: request EBU to run off AHB bus clock divided by 2."]
    VALUE2 = 1,
}
impl From<DIV2_A> for bool {
    #[inline(always)]
    fn from(variant: DIV2_A) -> Self {
        variant as u8 != 0
    }
}
impl DIV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIV2_A {
        match self.bits {
            false => DIV2_A::VALUE1,
            true => DIV2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIV2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIV2_A::VALUE2
    }
}
#[doc = "Field `DIV2` writer - DIV2 Clocking Mode"]
pub type DIV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLC_SPEC, DIV2_A, O>;
impl<'a, const O: u8> DIV2_W<'a, O> {
    #[doc = "standard clocking mode. clock input selected by SYNC bitfield (default after reset)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIV2_A::VALUE1)
    }
    #[doc = "request EBU to run off AHB bus clock divided by 2."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIV2_A::VALUE2)
    }
}
#[doc = "Field `EBUDIV` reader - EBU Clock Divide Ratio"]
pub type EBUDIV_R = crate::FieldReader<u8, EBUDIV_A>;
#[doc = "EBU Clock Divide Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EBUDIV_A {
    #[doc = "0: request EBU to run off input clock (default after reset)"]
    VALUE1 = 0,
    #[doc = "1: request EBU to run off input clock divided by 2"]
    VALUE2 = 1,
    #[doc = "2: request EBU to run off input clock divided by 3"]
    VALUE3 = 2,
    #[doc = "3: request EBU to run off input clock divided by 4"]
    VALUE4 = 3,
}
impl From<EBUDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: EBUDIV_A) -> Self {
        variant as _
    }
}
impl EBUDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EBUDIV_A {
        match self.bits {
            0 => EBUDIV_A::VALUE1,
            1 => EBUDIV_A::VALUE2,
            2 => EBUDIV_A::VALUE3,
            3 => EBUDIV_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EBUDIV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EBUDIV_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EBUDIV_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EBUDIV_A::VALUE4
    }
}
#[doc = "Field `EBUDIV` writer - EBU Clock Divide Ratio"]
pub type EBUDIV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CLC_SPEC, u8, EBUDIV_A, 2, O>;
impl<'a, const O: u8> EBUDIV_W<'a, O> {
    #[doc = "request EBU to run off input clock (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EBUDIV_A::VALUE1)
    }
    #[doc = "request EBU to run off input clock divided by 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EBUDIV_A::VALUE2)
    }
    #[doc = "request EBU to run off input clock divided by 3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EBUDIV_A::VALUE3)
    }
    #[doc = "request EBU to run off input clock divided by 4"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EBUDIV_A::VALUE4)
    }
}
#[doc = "Field `SYNCACK` reader - EBU Clocking Mode Status"]
pub type SYNCACK_R = crate::BitReader<SYNCACK_A>;
#[doc = "EBU Clocking Mode Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCACK_A {
    #[doc = "0: the EBU is asynchronous to the AHB bus clock and is using a separate clock source"]
    VALUE1 = 0,
    #[doc = "1: EBU is synchronous to the AHB bus clock (default after reset)"]
    VALUE2 = 1,
}
impl From<SYNCACK_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCACK_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNCACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCACK_A {
        match self.bits {
            false => SYNCACK_A::VALUE1,
            true => SYNCACK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYNCACK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNCACK_A::VALUE2
    }
}
#[doc = "Field `DIV2ACK` reader - DIV2 Clocking Mode Status"]
pub type DIV2ACK_R = crate::BitReader<DIV2ACK_A>;
#[doc = "DIV2 Clocking Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIV2ACK_A {
    #[doc = "0: EBU is using standard clocking mode. clock input selected by SYNC bitfield (default after reset)."]
    VALUE1 = 0,
    #[doc = "1: EBU is running off AHB bus clock divided by 2."]
    VALUE2 = 1,
}
impl From<DIV2ACK_A> for bool {
    #[inline(always)]
    fn from(variant: DIV2ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl DIV2ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIV2ACK_A {
        match self.bits {
            false => DIV2ACK_A::VALUE1,
            true => DIV2ACK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIV2ACK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIV2ACK_A::VALUE2
    }
}
#[doc = "Field `EBUDIVACK` reader - EBU Clock Divide Ratio Status"]
pub type EBUDIVACK_R = crate::FieldReader<u8, EBUDIVACK_A>;
#[doc = "EBU Clock Divide Ratio Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EBUDIVACK_A {
    #[doc = "0: EBU is running off input clock (default after reset)"]
    VALUE1 = 0,
    #[doc = "1: EBU is running off input clock divided by 2"]
    VALUE2 = 1,
    #[doc = "2: EBU is running off input clock divided by 3"]
    VALUE3 = 2,
    #[doc = "3: EBU is running off input clock divided by 4"]
    VALUE4 = 3,
}
impl From<EBUDIVACK_A> for u8 {
    #[inline(always)]
    fn from(variant: EBUDIVACK_A) -> Self {
        variant as _
    }
}
impl EBUDIVACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EBUDIVACK_A {
        match self.bits {
            0 => EBUDIVACK_A::VALUE1,
            1 => EBUDIVACK_A::VALUE2,
            2 => EBUDIVACK_A::VALUE3,
            3 => EBUDIVACK_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EBUDIVACK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EBUDIVACK_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EBUDIVACK_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EBUDIVACK_A::VALUE4
    }
}
impl R {
    #[doc = "Bit 0 - EBU Disable Request Bit"]
    #[inline(always)]
    pub fn disr(&self) -> DISR_R {
        DISR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EBU Disable Status Bit"]
    #[inline(always)]
    pub fn diss(&self) -> DISS_R {
        DISS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - EBU Clocking Mode"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DIV2 Clocking Mode"]
    #[inline(always)]
    pub fn div2(&self) -> DIV2_R {
        DIV2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - EBU Clock Divide Ratio"]
    #[inline(always)]
    pub fn ebudiv(&self) -> EBUDIV_R {
        EBUDIV_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - EBU Clocking Mode Status"]
    #[inline(always)]
    pub fn syncack(&self) -> SYNCACK_R {
        SYNCACK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DIV2 Clocking Mode Status"]
    #[inline(always)]
    pub fn div2ack(&self) -> DIV2ACK_R {
        DIV2ACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - EBU Clock Divide Ratio Status"]
    #[inline(always)]
    pub fn ebudivack(&self) -> EBUDIVACK_R {
        EBUDIVACK_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EBU Disable Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn disr(&mut self) -> DISR_W<0> {
        DISR_W::new(self)
    }
    #[doc = "Bit 16 - EBU Clocking Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SYNC_W<16> {
        SYNC_W::new(self)
    }
    #[doc = "Bit 17 - DIV2 Clocking Mode"]
    #[inline(always)]
    #[must_use]
    pub fn div2(&mut self) -> DIV2_W<17> {
        DIV2_W::new(self)
    }
    #[doc = "Bits 18:19 - EBU Clock Divide Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn ebudiv(&mut self) -> EBUDIV_W<18> {
        EBUDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBU Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clc](index.html) module"]
pub struct CLC_SPEC;
impl crate::RegisterSpec for CLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clc::R](R) reader structure"]
impl crate::Readable for CLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clc::W](W) writer structure"]
impl crate::Writable for CLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLC to value 0x0011_0000"]
impl crate::Resettable for CLC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0011_0000;
}
