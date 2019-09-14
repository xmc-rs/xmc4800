#[doc = "Reader of register CLC"]
pub type R = crate::R<u32, super::CLC>;
#[doc = "Writer for register CLC"]
pub type W = crate::W<u32, super::CLC>;
#[doc = "Register CLC `reset()`'s with value 0x0011_0000"]
impl crate::ResetValue for super::CLC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0011_0000
    }
}
#[doc = "EBU Disable Request Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISR_A {
    #[doc = "0: EBU disable is not requested"]
    VALUE1,
    #[doc = "1: EBU disable is requested"]
    VALUE2,
}
impl From<DISR_A> for bool {
    #[inline(always)]
    fn from(variant: DISR_A) -> Self {
        match variant {
            DISR_A::VALUE1 => false,
            DISR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DISR`"]
pub type DISR_R = crate::R<bool, DISR_A>;
impl DISR_R {
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
        *self == DISR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DISR_A::VALUE2
    }
}
#[doc = "Write proxy for field `DISR`"]
pub struct DISR_W<'a> {
    w: &'a mut W,
}
impl<'a> DISR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "EBU Disable Status Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISS_A {
    #[doc = "0: EBU is enabled (default after reset)"]
    VALUE1,
    #[doc = "1: EBU is disabled"]
    VALUE2,
}
impl From<DISS_A> for bool {
    #[inline(always)]
    fn from(variant: DISS_A) -> Self {
        match variant {
            DISS_A::VALUE1 => false,
            DISS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DISS`"]
pub type DISS_R = crate::R<bool, DISS_A>;
impl DISS_R {
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
        *self == DISS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DISS_A::VALUE2
    }
}
#[doc = "EBU Clocking Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_A {
    #[doc = "0: request EBU to run asynchronously to AHB bus clock and use separate clock source"]
    VALUE1,
    #[doc = "1: request EBU to run synchronously to ARM processor (default after reset)"]
    VALUE2,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        match variant {
            SYNC_A::VALUE1 => false,
            SYNC_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SYNC`"]
pub type SYNC_R = crate::R<bool, SYNC_A>;
impl SYNC_R {
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
        *self == SYNC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNC_A::VALUE2
    }
}
#[doc = "Write proxy for field `SYNC`"]
pub struct SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "DIV2 Clocking Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV2_A {
    #[doc = "0: standard clocking mode. clock input selected by SYNC bitfield (default after reset)."]
    VALUE1,
    #[doc = "1: request EBU to run off AHB bus clock divided by 2."]
    VALUE2,
}
impl From<DIV2_A> for bool {
    #[inline(always)]
    fn from(variant: DIV2_A) -> Self {
        match variant {
            DIV2_A::VALUE1 => false,
            DIV2_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DIV2`"]
pub type DIV2_R = crate::R<bool, DIV2_A>;
impl DIV2_R {
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
        *self == DIV2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIV2_A::VALUE2
    }
}
#[doc = "Write proxy for field `DIV2`"]
pub struct DIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIV2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "EBU Clock Divide Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBUDIV_A {
    #[doc = "0: request EBU to run off input clock (default after reset)"]
    VALUE1,
    #[doc = "1: request EBU to run off input clock divided by 2"]
    VALUE2,
    #[doc = "2: request EBU to run off input clock divided by 3"]
    VALUE3,
    #[doc = "3: request EBU to run off input clock divided by 4"]
    VALUE4,
}
impl From<EBUDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: EBUDIV_A) -> Self {
        match variant {
            EBUDIV_A::VALUE1 => 0,
            EBUDIV_A::VALUE2 => 1,
            EBUDIV_A::VALUE3 => 2,
            EBUDIV_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `EBUDIV`"]
pub type EBUDIV_R = crate::R<u8, EBUDIV_A>;
impl EBUDIV_R {
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
#[doc = "Write proxy for field `EBUDIV`"]
pub struct EBUDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> EBUDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EBUDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "EBU Clocking Mode Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCACK_A {
    #[doc = "0: the EBU is asynchronous to the AHB bus clock and is using a separate clock source"]
    VALUE1,
    #[doc = "1: EBU is synchronous to the AHB bus clock (default after reset)"]
    VALUE2,
}
impl From<SYNCACK_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCACK_A) -> Self {
        match variant {
            SYNCACK_A::VALUE1 => false,
            SYNCACK_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SYNCACK`"]
pub type SYNCACK_R = crate::R<bool, SYNCACK_A>;
impl SYNCACK_R {
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
        *self == SYNCACK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNCACK_A::VALUE2
    }
}
#[doc = "DIV2 Clocking Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV2ACK_A {
    #[doc = "0: EBU is using standard clocking mode. clock input selected by SYNC bitfield (default after reset)."]
    VALUE1,
    #[doc = "1: EBU is running off AHB bus clock divided by 2."]
    VALUE2,
}
impl From<DIV2ACK_A> for bool {
    #[inline(always)]
    fn from(variant: DIV2ACK_A) -> Self {
        match variant {
            DIV2ACK_A::VALUE1 => false,
            DIV2ACK_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DIV2ACK`"]
pub type DIV2ACK_R = crate::R<bool, DIV2ACK_A>;
impl DIV2ACK_R {
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
        *self == DIV2ACK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIV2ACK_A::VALUE2
    }
}
#[doc = "EBU Clock Divide Ratio Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBUDIVACK_A {
    #[doc = "0: EBU is running off input clock (default after reset)"]
    VALUE1,
    #[doc = "1: EBU is running off input clock divided by 2"]
    VALUE2,
    #[doc = "2: EBU is running off input clock divided by 3"]
    VALUE3,
    #[doc = "3: EBU is running off input clock divided by 4"]
    VALUE4,
}
impl From<EBUDIVACK_A> for u8 {
    #[inline(always)]
    fn from(variant: EBUDIVACK_A) -> Self {
        match variant {
            EBUDIVACK_A::VALUE1 => 0,
            EBUDIVACK_A::VALUE2 => 1,
            EBUDIVACK_A::VALUE3 => 2,
            EBUDIVACK_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `EBUDIVACK`"]
pub type EBUDIVACK_R = crate::R<u8, EBUDIVACK_A>;
impl EBUDIVACK_R {
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
}
