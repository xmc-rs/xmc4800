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
#[doc = "EBU Disable Request Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DISR` reader - EBU Disable Request Bit"]
pub struct DISR_R(crate::FieldReader<bool, DISR_A>);
impl DISR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == DISR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DISR_A::VALUE2
    }
}
impl core::ops::Deref for DISR_R {
    type Target = crate::FieldReader<bool, DISR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISR` writer - EBU Disable Request Bit"]
pub struct DISR_W<'a> {
    w: &'a mut W,
}
impl<'a> DISR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISR_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "EBU Disable Status Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DISS` reader - EBU Disable Status Bit"]
pub struct DISS_R(crate::FieldReader<bool, DISS_A>);
impl DISS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == DISS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DISS_A::VALUE2
    }
}
impl core::ops::Deref for DISS_R {
    type Target = crate::FieldReader<bool, DISS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "EBU Clocking Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `SYNC` reader - EBU Clocking Mode"]
pub struct SYNC_R(crate::FieldReader<bool, SYNC_A>);
impl SYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == SYNC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SYNC_A::VALUE2
    }
}
impl core::ops::Deref for SYNC_R {
    type Target = crate::FieldReader<bool, SYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNC` writer - EBU Clocking Mode"]
pub struct SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "DIV2 Clocking Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DIV2` reader - DIV2 Clocking Mode"]
pub struct DIV2_R(crate::FieldReader<bool, DIV2_A>);
impl DIV2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIV2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == DIV2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DIV2_A::VALUE2
    }
}
impl core::ops::Deref for DIV2_R {
    type Target = crate::FieldReader<bool, DIV2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV2` writer - DIV2 Clocking Mode"]
pub struct DIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIV2_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "EBU Clock Divide Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `EBUDIV` reader - EBU Clock Divide Ratio"]
pub struct EBUDIV_R(crate::FieldReader<u8, EBUDIV_A>);
impl EBUDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        EBUDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == EBUDIV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EBUDIV_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == EBUDIV_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == EBUDIV_A::VALUE4
    }
}
impl core::ops::Deref for EBUDIV_R {
    type Target = crate::FieldReader<u8, EBUDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EBUDIV` writer - EBU Clock Divide Ratio"]
pub struct EBUDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> EBUDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EBUDIV_A) -> &'a mut W {
        self.bits(variant.into())
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "EBU Clocking Mode Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `SYNCACK` reader - EBU Clocking Mode Status"]
pub struct SYNCACK_R(crate::FieldReader<bool, SYNCACK_A>);
impl SYNCACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNCACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == SYNCACK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SYNCACK_A::VALUE2
    }
}
impl core::ops::Deref for SYNCACK_R {
    type Target = crate::FieldReader<bool, SYNCACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "DIV2 Clocking Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DIV2ACK` reader - DIV2 Clocking Mode Status"]
pub struct DIV2ACK_R(crate::FieldReader<bool, DIV2ACK_A>);
impl DIV2ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIV2ACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == DIV2ACK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DIV2ACK_A::VALUE2
    }
}
impl core::ops::Deref for DIV2ACK_R {
    type Target = crate::FieldReader<bool, DIV2ACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "EBU Clock Divide Ratio Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `EBUDIVACK` reader - EBU Clock Divide Ratio Status"]
pub struct EBUDIVACK_R(crate::FieldReader<u8, EBUDIVACK_A>);
impl EBUDIVACK_R {
    pub(crate) fn new(bits: u8) -> Self {
        EBUDIVACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == EBUDIVACK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EBUDIVACK_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == EBUDIVACK_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == EBUDIVACK_A::VALUE4
    }
}
impl core::ops::Deref for EBUDIVACK_R {
    type Target = crate::FieldReader<u8, EBUDIVACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - EBU Disable Request Bit"]
    #[inline(always)]
    pub fn disr(&self) -> DISR_R {
        DISR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EBU Disable Status Bit"]
    #[inline(always)]
    pub fn diss(&self) -> DISS_R {
        DISS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - EBU Clocking Mode"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DIV2 Clocking Mode"]
    #[inline(always)]
    pub fn div2(&self) -> DIV2_R {
        DIV2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - EBU Clock Divide Ratio"]
    #[inline(always)]
    pub fn ebudiv(&self) -> EBUDIV_R {
        EBUDIV_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 20 - EBU Clocking Mode Status"]
    #[inline(always)]
    pub fn syncack(&self) -> SYNCACK_R {
        SYNCACK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - DIV2 Clocking Mode Status"]
    #[inline(always)]
    pub fn div2ack(&self) -> DIV2ACK_R {
        DIV2ACK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - EBU Clock Divide Ratio Status"]
    #[inline(always)]
    pub fn ebudivack(&self) -> EBUDIVACK_R {
        EBUDIVACK_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EBU Disable Request Bit"]
    #[inline(always)]
    pub fn disr(&mut self) -> DISR_W {
        DISR_W { w: self }
    }
    #[doc = "Bit 16 - EBU Clocking Mode"]
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W {
        SYNC_W { w: self }
    }
    #[doc = "Bit 17 - DIV2 Clocking Mode"]
    #[inline(always)]
    pub fn div2(&mut self) -> DIV2_W {
        DIV2_W { w: self }
    }
    #[doc = "Bits 18:19 - EBU Clock Divide Ratio"]
    #[inline(always)]
    pub fn ebudiv(&mut self) -> EBUDIV_W {
        EBUDIV_W { w: self }
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
}
#[doc = "`reset()` method sets CLC to value 0x0011_0000"]
impl crate::Resettable for CLC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0011_0000
    }
}
