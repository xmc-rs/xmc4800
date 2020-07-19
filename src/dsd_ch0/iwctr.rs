#[doc = "Reader of register IWCTR"]
pub type R = crate::R<u32, super::IWCTR>;
#[doc = "Writer for register IWCTR"]
pub type W = crate::W<u32, super::IWCTR>;
#[doc = "Register IWCTR `reset()`'s with value 0"]
impl crate::ResetValue for super::IWCTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NVALCNT`"]
pub type NVALCNT_R = crate::R<u8, u8>;
#[doc = "Integration Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN_A {
    #[doc = "0: Integration stopped. INTEN is cleared at the end of the integration window, i.e. upon the inverse trigger event transition of the external trigger signal."]
    VALUE1 = 0,
    #[doc = "1: Integration enabled. INTEN is set upon the defined trigger event."]
    VALUE2 = 1,
}
impl From<INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTEN`"]
pub type INTEN_R = crate::R<bool, INTEN_A>;
impl INTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN_A {
        match self.bits {
            false => INTEN_A::VALUE1,
            true => INTEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INTEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INTEN_A::VALUE2
    }
}
#[doc = "Reader of field `REPCNT`"]
pub type REPCNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `REPVAL`"]
pub type REPVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REPVAL`"]
pub struct REPVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> REPVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `NVALDIS`"]
pub type NVALDIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NVALDIS`"]
pub struct NVALDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NVALDIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Integration Window SIze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IWS_A {
    #[doc = "0: Internal control: stop integrator after REPVAL+1 integration cycles"]
    VALUE1 = 0,
    #[doc = "1: External control: stop integrator when bit INTEN becomes 0"]
    VALUE2 = 1,
}
impl From<IWS_A> for bool {
    #[inline(always)]
    fn from(variant: IWS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IWS`"]
pub type IWS_R = crate::R<bool, IWS_A>;
impl IWS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWS_A {
        match self.bits {
            false => IWS_A::VALUE1,
            true => IWS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IWS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IWS_A::VALUE2
    }
}
#[doc = "Write proxy for field `IWS`"]
pub struct IWS_W<'a> {
    w: &'a mut W,
}
impl<'a> IWS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IWS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Internal control: stop integrator after REPVAL+1 integration cycles"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IWS_A::VALUE1)
    }
    #[doc = "External control: stop integrator when bit INTEN becomes 0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IWS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `NVALINT`"]
pub type NVALINT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NVALINT`"]
pub struct NVALINT_W<'a> {
    w: &'a mut W,
}
impl<'a> NVALINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Number of Values Counted"]
    #[inline(always)]
    pub fn nvalcnt(&self) -> NVALCNT_R {
        NVALCNT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Integration Enable"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Integration Cycle Counter"]
    #[inline(always)]
    pub fn repcnt(&self) -> REPCNT_R {
        REPCNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Number of Integration Cycles"]
    #[inline(always)]
    pub fn repval(&self) -> REPVAL_R {
        REPVAL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Number of Values Discarded"]
    #[inline(always)]
    pub fn nvaldis(&self) -> NVALDIS_R {
        NVALDIS_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Integration Window SIze"]
    #[inline(always)]
    pub fn iws(&self) -> IWS_R {
        IWS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:29 - Number of Values Integrated"]
    #[inline(always)]
    pub fn nvalint(&self) -> NVALINT_R {
        NVALINT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Number of Integration Cycles"]
    #[inline(always)]
    pub fn repval(&mut self) -> REPVAL_W {
        REPVAL_W { w: self }
    }
    #[doc = "Bits 16:21 - Number of Values Discarded"]
    #[inline(always)]
    pub fn nvaldis(&mut self) -> NVALDIS_W {
        NVALDIS_W { w: self }
    }
    #[doc = "Bit 23 - Integration Window SIze"]
    #[inline(always)]
    pub fn iws(&mut self) -> IWS_W {
        IWS_W { w: self }
    }
    #[doc = "Bits 24:29 - Number of Values Integrated"]
    #[inline(always)]
    pub fn nvalint(&mut self) -> NVALINT_W {
        NVALINT_W { w: self }
    }
}
